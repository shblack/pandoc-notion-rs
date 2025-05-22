#![allow(dead_code, unused_imports)]

mod fixtures {
    pub mod notion_test_helpers;
    pub mod pretty_diff;
    pub mod roundtrip_utils;
}

use fixtures::notion_test_helpers::{
    notion_credentials_available, get_notion_credentials, create_notion_client,
    setup_test_page, teardown_test_page,
};
use fixtures::pretty_diff::print_side_by_side_diff;
use fixtures::roundtrip_utils::{sanitize_markdown, respect_rate_limit};

use pandoc_notion::prelude::*;
use std::error::Error;
use tokio::fs;
use tempfile::tempdir;

// Helper function to run a Notion syntax roundtrip test
async fn test_notion_roundtrip(
    test_name: &str, 
    markdown: &str
) -> Result<(), Box<dyn Error>> {
    // Skip if credentials not available
    if !notion_credentials_available() {
        println!("Notion credentials not available, skipping test: {}", test_name);
        return Ok(());
    }
    
    println!("\n===== TESTING NOTION ROUNDTRIP: {} =====", test_name);
    println!("Original markdown:\n{}", markdown);
    
    // Get credentials and create client
    let (token, parent_page_id) = get_notion_credentials()?;
    let notion_client = create_notion_client(&token)?;
    
    // Create a test page
    let page_id = setup_test_page(&notion_client, &parent_page_id, test_name).await?;
    
    // Respect API rate limits
    respect_rate_limit().await;
    
    // Create a converter and configure it with the Notion client
    let mut converter = create_converter();
    converter.configure_notion_client(token.clone())?;
    
    // Convert markdown to Notion blocks
    println!("Converting markdown to Notion blocks...");
    let notion_blocks = converter.text_to_notion_blocks(markdown, TextFormat::Markdown)?;
    println!("Created {} Notion blocks", notion_blocks.len());
    
    // Save Notion blocks to file for debugging and future tests
    if test_name == "bulleted_list" {
        println!("===== SAVING NOTION BLOCKS FOR DEBUGGING =====");
        let blocks_json = serde_json::to_string_pretty(&notion_blocks)?;
        fs::write(format!("{}_notion_blocks.json", test_name), blocks_json).await?;
        println!("Saved Notion blocks to {}_notion_blocks.json", test_name);
    }
    
    // Upload blocks to Notion
    println!("Uploading blocks to Notion page {}...", page_id);
    converter.upload_blocks_to_notion(&page_id, notion_blocks).await?;
    
    // Respect API rate limits
    respect_rate_limit().await;
    
    // Wait briefly to ensure changes are processed
    println!("Waiting for Notion to process changes...");
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    
    // Fetch the content back from Notion
    println!("Fetching content back from Notion...");
    let retrieved_pandoc = converter.notion_blocks_to_pandoc(&page_id).await?;
    
    // Respect API rate limits
    respect_rate_limit().await;
    
    // Save Pandoc AST to file for debugging and future tests
    if test_name == "bulleted_list" {
        println!("===== SAVING PANDOC AST FOR DEBUGGING =====");
        let pandoc_json = serde_json::to_string_pretty(&retrieved_pandoc)?;
        fs::write(format!("{}_pandoc_ast.json", test_name), pandoc_json).await?;
        println!("Saved Pandoc AST to {}_pandoc_ast.json", test_name);
    }
    
    // Convert to markdown
    let processor = create_text_processor();
    let retrieved_markdown = processor.ast_to_text(&retrieved_pandoc, TextFormat::Markdown)?;
    
    // Sanitize both the original and retrieved markdown for fair comparison
    let sanitized_original = sanitize_markdown(markdown)?;
    let sanitized_retrieved = sanitize_markdown(&retrieved_markdown)?;
    
    // Show comparison with clear markers for both versions
    println!("\n===== COMPARING ORIGINAL AND RETRIEVED CONTENT =====");
    println!("\n--- ORIGINAL CONTENT ---");
    println!("{}", sanitized_original);
    println!("\n--- RETRIEVED CONTENT ---");
    println!("{}", sanitized_retrieved);
    
    // Show a more reliable unified diff format
    println!("\n--- UNIFIED DIFF ---");
    let diff = similar::TextDiff::from_lines(&sanitized_original, &sanitized_retrieved);
    for change in diff.iter_all_changes() {
        let prefix = match change.tag() {
            similar::ChangeTag::Delete => "- ",
            similar::ChangeTag::Insert => "+ ",
            similar::ChangeTag::Equal => "  ",
        };
        print!("{}{}", prefix, change);
    }
    
    // Save to temp files for inspection if needed
    let temp_dir = tempdir()?;
    let original_path = temp_dir.path().join(format!("{}_original.md", test_name));
    let retrieved_path = temp_dir.path().join(format!("{}_retrieved.md", test_name));
    
    fs::write(&original_path, &sanitized_original).await?;
    fs::write(&retrieved_path, &sanitized_retrieved).await?;
    
    println!("\nSaved to temporary files for inspection:");
    println!("  Original:  {}", original_path.display());
    println!("  Retrieved: {}", retrieved_path.display());
    
    // Clean up by archiving the test page
    teardown_test_page(&notion_client, &page_id).await?;
    
    // Respect API rate limits after last API call
    respect_rate_limit().await;
    
    Ok(())
}

#[tokio::test]
async fn test_headings() -> Result<(), Box<dyn Error>> {
    let markdown = r#"
# Heading 1

## Heading 2

### Heading 3
"#;
    
    test_notion_roundtrip("headings", markdown).await
}

#[tokio::test]
async fn test_text_formatting() -> Result<(), Box<dyn Error>> {
    let markdown = r#"
This is **bold text**.

This is *italic text*.

This is ***bold and italic text***.

This is ~~strikethrough text~~.

This is `inline code`.
"#;
    
    test_notion_roundtrip("text_formatting", markdown).await
}

#[tokio::test]
async fn test_bulleted_list() -> Result<(), Box<dyn Error>> {
    let markdown = r#"
- First item
- Second item
- Third item
  - Nested item 1
  - Nested item 2
    - Deeply nested item
"#;
    
    test_notion_roundtrip("bulleted_list", markdown).await
}

#[tokio::test]
async fn test_numbered_list() -> Result<(), Box<dyn Error>> {
    let markdown = r#"
1. First item
2. Second item
   1. Nested numbered item 1
   2. Nested numbered item 2
3. Third item
"#;
    
    test_notion_roundtrip("numbered_list", markdown).await
}

#[tokio::test]
async fn test_blockquote() -> Result<(), Box<dyn Error>> {
    let markdown = r#"
> This is a blockquote.
>
> It can have multiple paragraphs.

> This is another blockquote
>
> > With a nested blockquote
>
> And back to the original level.
"#;
    
    test_notion_roundtrip("blockquote", markdown).await
}

#[tokio::test]
async fn test_code_block() -> Result<(), Box<dyn Error>> {
    let markdown = r#"
```rust
fn main() {
    println!("Hello, world!");
    
    // This is a comment
    let x = 42;
}
```

```text
Plain code block without language specification
Multiple lines
```
"#;
    
    test_notion_roundtrip("code_block", markdown).await
}

#[tokio::test]
async fn test_links() -> Result<(), Box<dyn Error>> {
    let markdown = r#"
[Link with title](https://example.com "Example Website")

[Plain link](https://example.com)

<https://example.com>
"#;
    
    test_notion_roundtrip("links", markdown).await
}

#[tokio::test]
async fn test_mixed_list_types() -> Result<(), Box<dyn Error>> {
    let markdown = r#"
1. First ordered item
   - Unordered sub-item
   - Another unordered sub-item
2. Second ordered item
   1. Ordered sub-item
      - Deeply nested unordered item
"#;
    
    test_notion_roundtrip("mixed_list_types", markdown).await
}

#[tokio::test]
async fn test_todo_list() -> Result<(), Box<dyn Error>> {
    let markdown = r#"
- [ ] Unchecked task item
- [x] Checked task item
  - [ ] Nested unchecked task
  - [x] Nested checked task
    - Regular nested item
"#;
    
    test_notion_roundtrip("todo_list", markdown).await
}

#[tokio::test]
async fn test_paragraphs_and_breaks() -> Result<(), Box<dyn Error>> {
    let markdown = r#"
This is a paragraph.

This is another paragraph with a line  
break in the middle.

---

This paragraph is separated by a horizontal rule.
"#;
    
    test_notion_roundtrip("paragraphs_and_breaks", markdown).await
}

#[tokio::test]
async fn test_nested_formatting() -> Result<(), Box<dyn Error>> {
    let markdown = r#"
- List item with **bold text**
  - Nested item with *italic text*
    - Deep nested with ***bold italic***

> Blockquote with **bold** and *italic*
>
> > Nested quote with `code`
"#;
    
    test_notion_roundtrip("nested_formatting", markdown).await
}