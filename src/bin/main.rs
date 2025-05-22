use clap::{Parser, Subcommand};
use log::{debug, info};
use pandoc_notion::{create_converter, TextFormat};
use std::env;
use std::error::Error;
use std::io::{self, Read};
use std::path::PathBuf;

/// Parse format string to TextFormat enum
fn parse_format(format: &str) -> Result<TextFormat, Box<dyn Error>> {
    match format.to_lowercase().as_str() {
        "markdown" | "md" => Ok(TextFormat::Markdown),
        "commonmark_x" => Ok(TextFormat::Markdown), // commonmark_x is handled by TextFormat::Markdown
        "commonmark" => Ok(TextFormat::CommonMark),
        "gfm" | "github" => Ok(TextFormat::GithubMarkdown),
        "html" | "htm" => Ok(TextFormat::Html),
        "plain" | "txt" => Ok(TextFormat::PlainText),
        "latex" | "tex" => Ok(TextFormat::Latex),
        "rst" => Ok(TextFormat::Rst),
        "org" => Ok(TextFormat::Org),
        // For other formats, try to use them as a custom format
        fmt => Ok(TextFormat::Custom(Box::leak(fmt.to_string().into_boxed_str()))),
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Notion API token (or set NOTION_TOKEN environment variable)
    #[arg(short, long)]
    token: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Download content from a Notion page/block and convert to markdown
    Download {
        /// Notion page/block ID to download from
        #[arg(short, long)]
        block_id: String,

        /// Output file (stdout if not specified)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Output format (markdown, commonmark_x, html, etc.)
        #[arg(short, long, default_value = "markdown")]
        format: String,

        /// Enable debug output
        #[arg(short, long)]
        debug: bool,
    },

    /// Upload content to a Notion page/block
    Upload {
        /// Notion page/block ID to upload to
        #[arg(short, long)]
        block_id: String,

        /// Input file (stdin if not specified)
        #[arg(short, long)]
        input: Option<PathBuf>,

        /// Input format (markdown, commonmark_x, html, etc.)
        #[arg(short, long, default_value = "markdown")]
        format: String,

        /// Enable debug output
        #[arg(short, long)]
        debug: bool,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    // Initialize logger with appropriate level based on debug flag
    let debug = match &cli.command {
        Commands::Download { debug, .. } => *debug,
        Commands::Upload { debug, .. } => *debug,
    };
    
    env_logger::Builder::new()
        .filter_level(if debug { log::LevelFilter::Debug } else { log::LevelFilter::Info })
        .init();

    // Get token from args or environment
    let token = cli.token.or_else(|| env::var("NOTION_TOKEN").ok())
        .ok_or("Notion API token not provided. Use --token or set NOTION_TOKEN environment variable")?;

    match &cli.command {
        Commands::Download {
            block_id,
            output,
            format,
            debug: _,
        } => {
            download(&token, block_id, output, format).await?;
        }
        Commands::Upload {
            block_id,
            input,
            format,
            debug: _,
        } => {
            upload(&token, block_id, input, format).await?;
        }
    }

    Ok(())
}

async fn download(
    token: &str,
    block_id: &str,
    output: &Option<PathBuf>,
    format: &str,
) -> Result<(), Box<dyn Error>> {
    // Create converter and configure it with the token
    let mut converter = create_converter();
    converter.configure_notion_client(token.to_string())?;
    
    debug!("Downloading content from Notion block ID: {}", block_id);

    // If output is specified, use notion_to_file, otherwise get text and print to stdout
    match output {
        Some(path) => {
            let format = parse_format(format)?;
            converter.notion_to_file(block_id, path, Some(format)).await?;
            info!("Content saved to: {}", path.display());
        }
        None => {
            // For stdout, we need to get the text and print it
            let format = parse_format(format)?;
            let text = converter.notion_blocks_to_text(block_id, format).await?;
            println!("{}", text);
        }
    }

    Ok(())
}

async fn upload(
    token: &str,
    block_id: &str,
    input: &Option<PathBuf>,
    format: &str,
) -> Result<(), Box<dyn Error>> {
    // Create converter and configure it with the token
    let mut converter = create_converter();
    converter.configure_notion_client(token.to_string())?;
    
    debug!("Preparing to upload to Notion block ID: {}", block_id);

    match input {
        Some(path) => {
            // If input file is specified, use file_to_notion
            let format = parse_format(format)?;
            converter.file_to_notion(path, block_id, Some(format)).await?;
        }
        None => {
            // If using stdin, read all content and use text_to_notion_blocks + upload
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            
            let format = parse_format(format)?;
            let blocks = converter.text_to_notion_blocks(&buffer, format)?;
            
            debug!("Converted to {} Notion blocks", blocks.len());
            
            converter.upload_blocks_to_notion(block_id, blocks).await?;
        }
    }
    
    info!("Upload complete!");
    Ok(())
}