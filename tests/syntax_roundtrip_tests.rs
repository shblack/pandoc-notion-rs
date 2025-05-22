#![allow(dead_code, unused_imports)]

use pandoc_notion::prelude::*;
use std::error::Error;
use tempfile::tempdir;
use std::fs;
use std::path::Path;

// Import test utilities
mod fixtures {
    pub mod roundtrip_utils;
    pub mod pretty_diff;
}
use fixtures::roundtrip_utils::{run_markdown_roundtrip, RoundtripResult};
use fixtures::pretty_diff::{print_side_by_side_diff, DiffConfig, DiffStyle, ColorFormat};

// Test helper function to run roundtrip and check results
fn test_syntax_roundtrip(name: &str, markdown: &str) -> Result<(), Box<dyn Error>> {
    println!("\n=== TESTING ROUNDTRIP: {} ===", name);
    
    // Run the roundtrip conversion
    let result = run_markdown_roundtrip(markdown)?;
    
    // Print a side-by-side diff for better visualization
    println!("\nSIDE-BY-SIDE DIFF:");
    print_side_by_side_diff(&result.input_markdown, &result.output_markdown);
    
    // Assert that the roundtrip was identical
    assert!(
        result.is_identical(),
        "{} roundtrip should preserve content",
        name
    );
    
    Ok(())
}

// Helper to save roundtrip results to temp files for inspection
fn save_test_results(name: &str, result: &RoundtripResult) -> (String, String) {
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let input_path = temp_dir.path().join(format!("{}_input.md", name));
    let output_path = temp_dir.path().join(format!("{}_output.md", name));
    
    fs::write(&input_path, &result.input_markdown).expect("Failed to write input file");
    fs::write(&output_path, &result.output_markdown).expect("Failed to write output file");
    
    (input_path.to_string_lossy().to_string(), output_path.to_string_lossy().to_string())
}

#[test]
fn test_headings_roundtrip() {
    let markdown = r#"
# Heading 1

## Heading 2

### Heading 3
"#;
    
    test_syntax_roundtrip("headings", markdown).expect("Heading roundtrip failed");
}

#[test]
fn test_text_formatting_roundtrip() {
    let markdown = r#"
This is **bold text**.

This is *italic text*.

This is ***bold and italic text***.

This is ~~strikethrough text~~.

This is `inline code`.
"#;
    
    test_syntax_roundtrip("text_formatting", markdown).expect("Text formatting roundtrip failed");
}

#[test]
fn test_bulleted_list_roundtrip() {
    let markdown = r#"
- First item
- Second item
- Third item
  - Nested item 1
  - Nested item 2
    - Deeply nested item
"#;
    
    test_syntax_roundtrip("bulleted_list", markdown).expect("Bulleted list roundtrip failed");
}

#[test]
fn test_numbered_list_roundtrip() {
    let markdown = r#"
1. First item
2. Second item
   1. Nested numbered item 1
   2. Nested numbered item 2
3. Third item
"#;
    
    test_syntax_roundtrip("numbered_list", markdown).expect("Numbered list roundtrip failed");
}

#[test]
fn test_blockquote_roundtrip() {
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
    
    test_syntax_roundtrip("blockquote", markdown).expect("Blockquote roundtrip failed");
}

#[test]
fn test_code_block_roundtrip() {
    let markdown = r#"
```rust
fn main() {
    println!("Hello, world!");
    
    // This is a comment
    let x = 42;
}
```

```
Plain code block without language specification
Multiple lines
```
"#;
    
    test_syntax_roundtrip("code_block", markdown).expect("Code block roundtrip failed");
}

#[test]
fn test_links_roundtrip() {
    let markdown = r#"
[Link with title](https://example.com "Example Website")

[Plain link](https://example.com)

<https://example.com>

https://auto-link-example.com
"#;
    
    test_syntax_roundtrip("links", markdown).expect("Links roundtrip failed");
}

#[test]
fn test_mixed_list_types_roundtrip() {
    let markdown = r#"
1. First ordered item
   - Unordered sub-item
   - Another unordered sub-item
2. Second ordered item
   1. Ordered sub-item
      - Deeply nested unordered item
"#;
    
    test_syntax_roundtrip("mixed_list_types", markdown).expect("Mixed list types roundtrip failed");
}

#[test]
fn test_todo_list_roundtrip() {
    let markdown = r#"
- [ ] Unchecked task item
- [x] Checked task item
  - [ ] Nested unchecked task
  - [x] Nested checked task
    - Regular nested item
"#;
    
    test_syntax_roundtrip("todo_list", markdown).expect("Todo list roundtrip failed");
}

#[test]
fn test_paragraphs_and_breaks_roundtrip() {
    let markdown = r#"
This is a paragraph.

This is another paragraph with a line  
break in the middle.

---

This paragraph is separated by a horizontal rule.
"#;
    
    test_syntax_roundtrip("paragraphs_and_breaks", markdown).expect("Paragraphs and breaks roundtrip failed");
}

#[test]
fn test_nested_formatting_roundtrip() {
    let markdown = r#"
- List item with **bold text**
  - Nested item with *italic text*
    - Deep nested with ***bold italic***

> Blockquote with **bold** and *italic*
>
> > Nested quote with `code`
"#;
    
    test_syntax_roundtrip("nested_formatting", markdown).expect("Nested formatting roundtrip failed");
}