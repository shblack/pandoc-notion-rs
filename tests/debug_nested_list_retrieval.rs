use notion_client::endpoints::Client as NotionClient;
use notion_client::objects::block::{Block as NotionBlock};
use notion_client::objects::rich_text::RichText;
use notion_client::objects::rich_text::Text;
use notion_client::objects::page::PageProperty;
use notion_client::objects::parent::Parent;
use notion_client::endpoints::pages::create::request::CreateAPageRequest;
use std::collections::BTreeMap;
use pandoc_notion::converter::create_converter;
use pandoc_notion::notion_block_fetcher::create_debug_block_fetcher;
use pandoc_notion::text::TextFormat;
use std::env;
use std::fs;
use tokio::runtime::Runtime;
use chrono::Utc;
use tempfile::tempdir;

#[test]
fn debug_nested_list_retrieval() {
    // Use tokio runtime for async operations
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        println!("\n===== STARTING NESTED LIST RETRIEVAL DEBUG TEST =====");
        
        // Get environment variables
        let notion_token = env::var("NOTION_TOKEN").expect("NOTION_TOKEN env var not set");
        let parent_page_id = env::var("NOTION_PARENT_PAGE").expect("NOTION_PARENT_PAGE not set");
        
        // Create Notion client
        let notion_client = NotionClient::new(notion_token, None)
            .expect("Failed to create Notion client");
        
        // Create a temporary directory for test files
        let temp_dir = tempdir().expect("Failed to create temp directory");
        let temp_path = temp_dir.path();
        
        // Create a test page name with timestamp
        let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let page_title = format!("Nested List Test - {}", timestamp);
        
        println!("Creating test page: {}", page_title);
        
        // Use the test utility to create a page (simplified approach)
        let page_id = create_test_page(&notion_client, &parent_page_id, &page_title).await;
        println!("Created test page with ID: {}", page_id);
        
        // Create a simple markdown file with nested lists - 3 levels
        let markdown = r#"
# Nested List Test

- Level 1 item
  - Level 2 item
    - Level 3 item
"#;
        
        let md_file_path = temp_path.join("nested_list_test.md");
        fs::write(&md_file_path, markdown).expect("Failed to write markdown file");
        println!("Created test markdown file at: {}", md_file_path.display());
        println!("Content:\n{}", markdown);
        
        // Create converter
        let converter = create_converter()
            .with_notion_client(notion_client.clone());
        
        // Convert markdown to Notion blocks
        println!("\n===== CONVERTING MARKDOWN TO NOTION BLOCKS =====");
        let blocks = converter.file_to_notion_blocks(&md_file_path, None)
            .expect("Failed to convert markdown to blocks");
        
        println!("Converted markdown to {} Notion blocks", blocks.len());
        print_block_hierarchy(&blocks, 0);
        
        // Upload blocks to Notion
        println!("\n===== UPLOADING BLOCKS TO NOTION =====");
        converter.upload_blocks_to_notion(&page_id, blocks.clone())
            .await
            .expect("Failed to upload blocks to Notion");
        
        println!("Successfully uploaded blocks to Notion page");
        
        // Wait briefly to ensure blocks are available
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        
        // Fetch blocks back from Notion
        println!("\n===== RETRIEVING BLOCKS FROM NOTION =====");
        let block_fetcher = create_debug_block_fetcher(notion_client.clone());
        let retrieved_blocks = block_fetcher.fetch_block_with_children(&page_id)
            .await
            .expect("Failed to fetch blocks from Notion");
        
        println!("Retrieved {} blocks from Notion", retrieved_blocks.len());
        print_block_hierarchy(&retrieved_blocks, 0);
        
        // Convert back to markdown
        println!("\n===== CONVERTING BACK TO MARKDOWN =====");
        let output_file_path = temp_path.join("retrieved_list.md");
        converter.notion_page_to_file(&page_id, &output_file_path, TextFormat::Markdown)
            .await
            .expect("Failed to convert retrieved blocks to markdown");
        
        let retrieved_markdown = fs::read_to_string(&output_file_path)
            .expect("Failed to read retrieved markdown file");
        
        println!("Retrieved markdown:\n{}", retrieved_markdown);
        
        // Check for expected nested list markers
        println!("\n===== CHECKING NESTED LIST PRESERVATION =====");
        let expected_markers = [
            "- Level 1 item",
            "  - Level 2 item",
            "    - Level 3 item"
        ];
        
        for marker in &expected_markers {
            let contains = retrieved_markdown.contains(marker);
            println!("{}: {}", marker, if contains { "FOUND ✓" } else { "MISSING ✗" });
            
            if !contains {
                println!("ERROR: Expected marker '{}' not found in retrieved markdown", marker);
            }
        }
    });
}

// Helper function to create a test page
async fn create_test_page(client: &NotionClient, parent_id: &str, title: &str) -> String {
    // Create the page title as rich text
    let title_text = Text {
        content: title.to_string(),
        link: None,
    };
    
    let title_rich_text = RichText::Text {
        text: title_text,
        annotations: None,
        plain_text: None,
        href: None,
    };
    
    // Create the page properties (only title is required)
    let mut properties = BTreeMap::new();
    properties.insert(
        "title".to_string(),
        PageProperty::Title {
            id: None,
            title: vec![title_rich_text],
        },
    );
    
    // Create parent reference
    let parent = Parent::PageId {
        page_id: parent_id.to_string(),
    };
    
    // Create the page request
    let request = CreateAPageRequest {
        parent,
        properties,
        children: None,
        icon: None,
        cover: None,
    };
    
    // Send the request to create the page
    let response = client.pages.create_a_page(request)
        .await
        .expect("Failed to create test page");
    
    response.id
}

// Helper function to print the block hierarchy
fn print_block_hierarchy(blocks: &[NotionBlock], indent: usize) {
    for (i, block) in blocks.iter().enumerate() {
        let indent_str = " ".repeat(indent * 2);
        println!("{}Block {}: {:?}", indent_str, i, block.block_type);
        
        // Check for children
        let children = match &block.block_type {
            notion_client::objects::block::BlockType::BulletedListItem { bulleted_list_item } => &bulleted_list_item.children,
            notion_client::objects::block::BlockType::NumberedListItem { numbered_list_item } => &numbered_list_item.children,
            notion_client::objects::block::BlockType::ToDo { to_do } => &to_do.children,
            notion_client::objects::block::BlockType::Toggle { toggle } => &toggle.children,
            notion_client::objects::block::BlockType::Paragraph { paragraph } => &paragraph.children,
            notion_client::objects::block::BlockType::Quote { quote } => &quote.children,
            _ => &None,
        };
        
        if let Some(children) = children {
            if !children.is_empty() {
                println!("{}Children:", indent_str);
                print_block_hierarchy(children, indent + 1);
            }
        }
    }
}