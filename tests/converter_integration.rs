use pandoc_notion::prelude::*;
use pandoc_types::definition::Pandoc;
use similar::{ChangeTag, TextDiff};
use std::env;
use std::fs;
use std::path::Path;
use tempfile::tempdir;
use notion_client::endpoints::Client as NotionClient;
use notion_client::endpoints::pages::create::request::CreateAPageRequest;
use notion_client::endpoints::pages::update::request::UpdatePagePropertiesRequest;
use notion_client::objects::parent::Parent;
use notion_client::objects::rich_text::{RichText, Text};
use notion_client::objects::page::PageProperty;
use std::collections::BTreeMap;
use chrono::Utc;
use serde_json;

/// Integration test for NotionConverter with the actual Notion API
///
/// Requirements:
/// - A valid Notion API token in the NOTION_TOKEN environment variable
/// - A Notion page ID in the NOTION_TEST_PAGE_ID environment variable that will be the parent for the test page
///
/// This test performs real operations against the Notion API:
/// 1. Creates a child page under the specified parent page
/// 2. Reads content from a test markdown file
/// 3. Converts it to Notion blocks
/// 4. Uploads the blocks to the child page in Notion
/// 5. Downloads the blocks from Notion
/// 6. Converts them back to markdown
/// 7. Displays a diff between the original and round-trip versions
/// 8. Archives the child page when finished
#[tokio::test]
async fn test_converter_with_actual_notion_api() {
    // Get environment variables for Notion API
    let token = match env::var("NOTION_TOKEN") {
        Ok(token) => token,
        Err(_) => {
            println!("NOTION_TOKEN not set, skipping integration test");
            return;
        }
    };

    let parent_page_id = match env::var("NOTION_TEST_PAGE_ID") {
        Ok(id) => id,
        Err(_) => {
            println!("NOTION_TEST_PAGE_ID not set, skipping integration test");
            return;
        }
    };
    
    // Create a Notion client for creating and archiving the child page
    let notion_client = NotionClient::new(token.clone(), None)
        .expect("Should create Notion client");
    
    // Create a timestamp for the test page name
    let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let test_page_title = format!("Integration Test - {}", timestamp);
    
    // Create a child page for testing
    println!("Creating child page for testing: {}", test_page_title);
    let child_page = create_child_page(&notion_client, &parent_page_id, &test_page_title).await
        .expect("Should create child page");
    
    let target_page_id = child_page.get("id").and_then(|id| id.as_str()).unwrap_or_default().to_string();
    println!("Created child page with ID: {}", target_page_id);

    println!("Starting integration test with Notion API");

    // 1. Read the test markdown file
    let test_file_path = Path::new("tests/fixtures/test_format.md");
    println!("Reading test file: {}", test_file_path.display());
    let original_markdown =
        fs::read_to_string(test_file_path).expect("Should read test markdown file");

    println!("Read test file, length: {} bytes", original_markdown.len());

    // 2. Create a converter with Notion client
    let mut converter = create_converter();
    converter
        .configure_notion_client(token)
        .expect("Should configure Notion client");

    // 3. Convert markdown to Notion blocks
    println!("Converting markdown to Notion blocks");
    let notion_blocks = converter
        .text_to_notion_blocks(&original_markdown, TextFormat::Markdown)
        .expect("Should convert markdown to Notion blocks");

    println!(
        "Converted markdown to {} Notion blocks",
        notion_blocks.len()
    );

    // 4. Upload blocks to target page
    println!(
        "Uploading {} blocks to target Notion page: {}",
        notion_blocks.len(),
        target_page_id
    );
    converter
        .upload_blocks_to_notion(&target_page_id, notion_blocks)
        .await
        .expect("Should upload blocks to Notion");

    println!("Successfully uploaded blocks to Notion");

    // 5. Fetch the newly uploaded content to verify
    println!("Fetching blocks from target page to verify");
    let verification_pandoc = converter
        .notion_blocks_to_pandoc(&target_page_id)
        .await
        .expect("Should fetch blocks from target page");

    // 6. Convert to markdown for comparison
    let processor = create_text_processor();
    let verification_markdown = processor
        .ast_to_text(&verification_pandoc, TextFormat::Markdown)
        .expect("Should convert verification AST to markdown");

    println!("Round-trip verification complete");
    println!("Original markdown length: {}", original_markdown.len());
    println!(
        "Verification markdown length: {}",
        verification_markdown.len()
    );

    // 7. Show diff between original and round-trip versions
    println!("\n====== ORIGINAL vs ROUND-TRIP DIFF ======\n");
    let diff = TextDiff::from_lines(&original_markdown, &verification_markdown);

    for change in diff.iter_all_changes() {
        let sign = match change.tag() {
            ChangeTag::Delete => "- ",
            ChangeTag::Insert => "+ ",
            ChangeTag::Equal => "  ",
        };

        print!("{}{}", sign, change);
    }

    // 8. Save both versions to temporary files for manual inspection
    let temp_dir = tempdir().expect("Should create temp directory");
    let original_file = temp_dir.path().join("original.md");
    let verification_file = temp_dir.path().join("verification.md");

    fs::write(&original_file, &original_markdown).expect("Should write original markdown to file");
    fs::write(&verification_file, &verification_markdown)
        .expect("Should write verification markdown to file");

    println!("\nSaved files for manual comparison:");
    println!("  Original:    {}", original_file.display());
    println!("  Round-trip:  {}", verification_file.display());

    // Note: We don't assert exact equality because Notion's API may introduce subtle differences
    // in formatting. Instead, we just verify the operation completed successfully.

    // Archive the child page when done
    println!("Archiving child page: {}", target_page_id);
    archive_page(&notion_client, &target_page_id).await
        .expect("Should archive child page");
    println!("Successfully archived child page");
    
    println!("Integration test completed successfully");
}

/// Create a child page under the specified parent page
async fn create_child_page(
    client: &NotionClient,
    parent_id: &str,
    title: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
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
    let page = client.pages.create_a_page(request).await?;
    // Convert the page to JSON value
    Ok(serde_json::to_value(page)?)
}

/// Archive a page
async fn archive_page(
    client: &NotionClient,
    page_id: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    // Create the update page request
    let request = UpdatePagePropertiesRequest {
        properties: BTreeMap::new(), // No property changes
        archived: Some(true),        // Archive the page
        icon: None,
        cover: None,
    };
    
    // Send the request to update the page
    let page = client.pages.update_page_properties(page_id, request).await?;
    Ok(serde_json::to_value(page)?)
}

// Test the file-based operations of NotionConverter
#[test]
fn test_file_conversions() {
    // Create a converter
    let converter = create_converter();

    // Read the test markdown file
    let test_file_path = Path::new("tests/fixtures/test_format.md");
    let test_markdown = fs::read_to_string(test_file_path).expect("Should read test markdown file");

    // Create a temporary directory for test files
    let temp_dir = tempdir().expect("Should create temp directory");
    let input_file = temp_dir.path().join("input.md");
    fs::write(&input_file, &test_markdown).expect("Should write test markdown to file");

    // Convert markdown to Notion blocks
    let notion_blocks = converter
        .file_to_notion_blocks(&input_file, Some(TextFormat::Markdown))
        .expect("Should convert markdown file to Notion blocks");

    println!(
        "Converted markdown to {} Notion blocks",
        notion_blocks.len()
    );

    // Verify block types
    assert!(!notion_blocks.is_empty(), "Should produce Notion blocks");

    // Convert to Pandoc AST for verification
    let visitor = pandoc_notion::n2p::NotionToPandocVisitor::new();
    let pandoc_blocks = visitor.convert_blocks(&notion_blocks);

    // Create a Pandoc AST from the blocks
    let pandoc = Pandoc {
        meta: Default::default(),
        blocks: pandoc_blocks,
    };

    // Convert back to markdown
    let output_file = temp_dir.path().join("output.md");
    let processor = create_text_processor();
    processor
        .ast_to_file_with_format(&pandoc, &output_file, TextFormat::Markdown)
        .expect("Should write output markdown file");

    // Read the output markdown
    let output_markdown =
        fs::read_to_string(&output_file).expect("Should read output markdown file");

    // Print diff
    println!("\n====== FILE CONVERSION TEST: ORIGINAL vs CONVERTED DIFF ======\n");
    let diff = TextDiff::from_lines(&test_markdown, &output_markdown);

    for change in diff.iter_all_changes() {
        let sign = match change.tag() {
            ChangeTag::Delete => "- ",
            ChangeTag::Insert => "+ ",
            ChangeTag::Equal => "  ",
        };

        print!("{}{}", sign, change);
    }

    // Verify that essential content is preserved
    assert!(
        output_markdown.contains("Pandoc-Notion Test Document"),
        "Output should contain the title"
    );
    assert!(
        output_markdown.contains("Text Formatting"),
        "Output should contain section headings"
    );
    assert!(
        output_markdown.contains("bold text"),
        "Output should contain 'bold text'"
    );
    assert!(
        output_markdown.contains("italic text"),
        "Output should contain 'italic text'"
    );
    assert!(
        output_markdown.contains("Blockquotes"),
        "Output should contain 'Blockquotes'"
    );
    assert!(
        output_markdown.contains("Unordered Lists"),
        "Output should contain 'Unordered Lists'"
    );
    assert!(
        output_markdown.contains("Ordered Lists"),
        "Output should contain 'Ordered Lists'"
    );

    println!("File conversion test completed successfully");
}