// pandoc-notion/tests/fixtures/roundtrip_utils.rs

#![allow(dead_code, unused_imports, unused_variables)]

use pandoc_notion::n2p::notion_block_visitor::NotionToPandocVisitor;
use pandoc_notion::p2n::pandoc_block_visitor::PandocToNotionVisitor;
use pandoc_notion::prelude::*;
use pandoc_types::definition::{Attr, Block, Inline, ListAttributes, Pandoc, QuoteType};
use serde_json::{self, Value};
use similar::{ChangeTag, TextDiff};
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::time::Duration;
use tempfile::NamedTempFile;
use tokio::time::sleep;

/// Represents the results of a roundtrip test
pub struct RoundtripResult {
    pub sanitized_markdown: String,
    pub input_ast: Pandoc,
    pub output_ast: Pandoc,
    pub input_markdown: String,
    pub output_markdown: String,
    pub ast_differences: Vec<String>,
}

impl RoundtripResult {
    /// Print a readable report of the roundtrip results
    pub fn print_report(&self) {
        println!("\nSANITIZED MARKDOWN:\n{}", self.sanitized_markdown);

        // Print AST summary (not the full AST to avoid overwhelming output)
        println!("\nINPUT AST: {} blocks", self.input_ast.blocks.len());
        println!("OUTPUT AST: {} blocks", self.output_ast.blocks.len());

        // Print first block type for both ASTs
        if !self.input_ast.blocks.is_empty() {
            println!("INPUT FIRST BLOCK TYPE: {:?}", self.input_ast.blocks[0]);
        }
        if !self.output_ast.blocks.is_empty() {
            println!("OUTPUT FIRST BLOCK TYPE: {:?}", self.output_ast.blocks[0]);
        }

        // Display markdown diff
        println!("\nMARKDOWN DIFF:");
        let diff = TextDiff::from_lines(&self.input_markdown, &self.output_markdown);

        for change in diff.iter_all_changes() {
            let sign = match change.tag() {
                ChangeTag::Delete => "- ",
                ChangeTag::Insert => "+ ",
                ChangeTag::Equal => "  ",
            };

            print!("{}{}", sign, change);
        }

        // Display AST differences
        if self.ast_differences.is_empty() {
            println!("\nAST DIFF: No differences detected");
        } else {
            println!("\nAST DIFF: Differences detected!");
            for diff in &self.ast_differences {
                println!("{}", diff);
            }
            
            // Print more detailed structure comparison for debugging
            if self.input_ast.blocks.len() == self.output_ast.blocks.len() {
                for (i, (input_block, output_block)) in self.input_ast.blocks.iter()
                    .zip(self.output_ast.blocks.iter()).enumerate() {
                    println!("\nDETAILED COMPARISON OF BLOCK {}:", i);
                    println!("INPUT: {:#?}", input_block);
                    println!("OUTPUT: {:#?}", output_block);
                }
            }
        }
    }

    /// Returns true if there are no differences between input and output
    pub fn is_identical(&self) -> bool {
        self.ast_differences.is_empty()
    }

    /// Save input and output to files for further inspection
    pub fn save_to_files(
        &self,
        input_path: &Path,
        output_path: &Path,
    ) -> Result<(), Box<dyn Error>> {
        fs::write(input_path, &self.input_markdown)?;
        fs::write(output_path, &self.output_markdown)?;
        Ok(())
    }
}

/// Run a markdown string through the full roundtrip process:
/// 1. Sanitize through pandoc with commonmark_x
/// 2. Convert to Pandoc AST
/// 3. Convert to Notion blocks
/// 4. Convert back to Pandoc AST
/// 5. Compare results
pub fn run_markdown_roundtrip(markdown: &str) -> Result<RoundtripResult, Box<dyn Error>> {
    // Step 1: Sanitize the markdown through pandoc
    let sanitized = sanitize_markdown(markdown)?;

    // Step 2: Convert sanitized markdown to Pandoc AST
    let input_ast = markdown_to_pandoc_ast(&sanitized)?;

    // Step 3: Convert Pandoc AST to Notion blocks
    let p2n_visitor = PandocToNotionVisitor::new();
    let notion_blocks = p2n_visitor.convert_blocks(&input_ast.blocks)?;

    // Step 4: Convert Notion blocks back to Pandoc AST
    let n2p_visitor = NotionToPandocVisitor::new();
    let output_blocks = n2p_visitor.convert_blocks(&notion_blocks);

    // Create a new Pandoc AST with the round-tripped blocks
    let output_ast = Pandoc {
        meta: input_ast.meta.clone(), // Keep the same metadata
        blocks: output_blocks,
    };

    // Step 5: Convert both ASTs back to markdown for human-readable diff
    let processor = create_text_processor();
    let input_markdown = processor.ast_to_text(&input_ast, TextFormat::Markdown)?;
    let output_markdown = processor.ast_to_text(&output_ast, TextFormat::Markdown)?;

    // Step 6: Compare ASTs to check for semantic differences
    let ast_differences = compare_pandoc_ast(&input_ast, &output_ast);

    Ok(RoundtripResult {
        sanitized_markdown: sanitized,
        input_ast,
        output_ast,
        input_markdown,
        output_markdown,
        ast_differences,
    })
}

/// Run a markdown file through the full roundtrip process
pub fn run_file_roundtrip(file_path: &Path) -> Result<RoundtripResult, Box<dyn Error>> {
    let markdown = fs::read_to_string(file_path)?;
    run_markdown_roundtrip(&markdown)
}

/// Sanitize markdown by running it through pandoc with commonmark_x format
pub fn sanitize_markdown(markdown: &str) -> Result<String, Box<dyn Error>> {
    // Create a temporary file for the input
    let mut input_file = NamedTempFile::new()?;
    write!(input_file, "{}", markdown)?;

    // Create a temporary file for the output
    let output_file = NamedTempFile::new()?;

    // Run pandoc to convert the markdown to commonmark_x and back
    let status = Command::new("pandoc")
        .arg(input_file.path())
        .arg("--from=commonmark_x")
        .arg("--to=commonmark_x")
        .arg("-o")
        .arg(output_file.path())
        .status()?;

    if !status.success() {
        return Err("Failed to sanitize markdown with pandoc".into());
    }

    // Read the sanitized markdown
    let sanitized = fs::read_to_string(output_file.path())?;
    Ok(sanitized)
}

/// Convert markdown to Pandoc AST
pub fn markdown_to_pandoc_ast(markdown: &str) -> Result<Pandoc, Box<dyn Error>> {
    // Create a temporary file for the input
    let mut input_file = NamedTempFile::new()?;
    write!(input_file, "{}", markdown)?;

    // Create a temporary file for the output JSON
    let output_file = NamedTempFile::new()?;

    // Run pandoc to convert the markdown to JSON AST
    let status = Command::new("pandoc")
        .arg(input_file.path())
        .arg("--from=commonmark_x")
        .arg("--to=json")
        .arg("-o")
        .arg(output_file.path())
        .status()?;

    if !status.success() {
        return Err("Failed to convert markdown to Pandoc AST".into());
    }

    // Read the JSON AST
    let json_ast = fs::read_to_string(output_file.path())?;

    // Deserialize into Pandoc struct
    let pandoc_ast: Pandoc = serde_json::from_str(&json_ast)?;
    Ok(pandoc_ast)
}

/// Semantic AST comparison that returns a list of differences
/// This comparison ignores empty spans, treating them as equivalent to their content
pub fn compare_pandoc_ast(ast1: &Pandoc, ast2: &Pandoc) -> Vec<String> {
    let mut differences = Vec::new();

    // Create normalized versions of the ASTs with empty spans removed
    let normalized_ast1 = normalize_ast(ast1);
    let normalized_ast2 = normalize_ast(ast2);

    // Convert to JSON for easier structural comparison
    let json1 = serde_json::to_value(&normalized_ast1).unwrap_or(Value::Null);
    let json2 = serde_json::to_value(&normalized_ast2).unwrap_or(Value::Null);

    // Compare block count
    let blocks1 = normalized_ast1.blocks.len();
    let blocks2 = normalized_ast2.blocks.len();

    if blocks1 != blocks2 {
        differences.push(format!("Block count mismatch: {} vs {}", blocks1, blocks2));
    }

    // Quick check for basic structure
    if json1 != json2 {
        // Compare individual blocks for more specific differences
        let min_blocks = blocks1.min(blocks2);
        for i in 0..min_blocks {
            let block1 = serde_json::to_value(&normalized_ast1.blocks[i]).unwrap_or(Value::Null);
            let block2 = serde_json::to_value(&normalized_ast2.blocks[i]).unwrap_or(Value::Null);

            if block1 != block2 {
                differences.push(format!("Block {} differs", i));

                // Add more detailed comparison if needed
                // This is a simplified diff - a real implementation would be more sophisticated
                if let Some(block1_type) = block1.get("t") {
                    if let Some(block2_type) = block2.get("t") {
                        if block1_type != block2_type {
                            differences.push(format!(
                                "  Type mismatch: {:?} vs {:?}",
                                block1_type, block2_type
                            ));
                        }
                    }
                }
            }
        }
    }

    differences
}

/// Creates a normalized copy of a Pandoc AST with empty spans removed
fn normalize_ast(ast: &Pandoc) -> Pandoc {
    let mut normalized = ast.clone();
    
    // Process each block to normalize it
    for i in 0..normalized.blocks.len() {
        normalized.blocks[i] = normalize_block(&normalized.blocks[i]);
    }
    
    normalized
}

/// Normalizes a block by removing empty spans
fn normalize_block(block: &Block) -> Block {
    match block {
        Block::Plain(inlines) => Block::Plain(normalize_inlines(inlines)),
        Block::Para(inlines) => Block::Para(normalize_inlines(inlines)),
        Block::LineBlock(lines) => Block::LineBlock(
            lines.iter().map(|line| normalize_inlines(line)).collect()
        ),
        Block::BulletList(items) => Block::BulletList(
            items.iter().map(|item| item.iter().map(normalize_block).collect()).collect()
        ),
        Block::OrderedList(attrs, items) => Block::OrderedList(
            attrs.clone(),
            items.iter().map(|item| item.iter().map(normalize_block).collect()).collect()
        ),
        Block::Div(attrs, blocks) => Block::Div(
            attrs.clone(),
            blocks.iter().map(normalize_block).collect()
        ),
        Block::BlockQuote(blocks) => Block::BlockQuote(
            blocks.iter().map(normalize_block).collect()
        ),
        // For other block types, just return the original
        _ => block.clone(),
    }
}

/// Normalizes a list of inlines by removing empty spans
fn normalize_inlines(inlines: &[Inline]) -> Vec<Inline> {
    let mut result = Vec::new();
    
    for inline in inlines {
        match inline {
            // If this is a Span with empty attributes, replace it with its content
            Inline::Span(attr, content) if is_empty_attr(attr) => {
                // Add the normalized content directly
                result.extend(normalize_inlines(content));
            },
            // For other inline types that can contain inlines, normalize recursively
            Inline::Emph(content) => {
                result.push(Inline::Emph(normalize_inlines(content)));
            },
            Inline::Strong(content) => {
                result.push(Inline::Strong(normalize_inlines(content)));
            },
            Inline::Strikeout(content) => {
                result.push(Inline::Strikeout(normalize_inlines(content)));
            },
            Inline::Superscript(content) => {
                result.push(Inline::Superscript(normalize_inlines(content)));
            },
            Inline::Subscript(content) => {
                result.push(Inline::Subscript(normalize_inlines(content)));
            },
            Inline::SmallCaps(content) => {
                result.push(Inline::SmallCaps(normalize_inlines(content)));
            },
            Inline::Quoted(quote_type, content) => {
                result.push(Inline::Quoted(quote_type.clone(), normalize_inlines(content)));
            },
            Inline::Cite(citations, content) => {
                result.push(Inline::Cite(citations.clone(), normalize_inlines(content)));
            },
            Inline::Link(attr, content, target) => {
                result.push(Inline::Link(attr.clone(), normalize_inlines(content), target.clone()));
            },
            Inline::Image(attr, content, target) => {
                result.push(Inline::Image(attr.clone(), normalize_inlines(content), target.clone()));
            },
            Inline::Span(attr, content) => {
                result.push(Inline::Span(attr.clone(), normalize_inlines(content)));
            },
            // For other inline types, just add them as-is
            _ => {
                result.push(inline.clone());
            },
        }
    }
    
    result
}

/// Checks if an attribute is empty (no identifier, classes, or attributes)
fn is_empty_attr(attr: &Attr) -> bool {
    attr.identifier.is_empty() && attr.classes.is_empty() && attr.attributes.is_empty()
}

/// Adds a delay between API calls to respect Notion's rate limit
/// 
/// Notion's API has a rate limit of 3 requests per second per integration.
/// This function adds a 350ms delay between calls to stay safely under that limit.
pub async fn respect_rate_limit() {
    // Add a 350ms delay to stay under 3 requests per second
    sleep(Duration::from_millis(350)).await;
}
