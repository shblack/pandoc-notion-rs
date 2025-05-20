use pandoc_notion::n2p::notion_text::NotionTextConverter;
use pandoc_notion::p2n::pandoc_text::PandocTextConverter;
use pandoc_types::definition::Inline;

#[test]
fn test_strong_conversion() {
    // Create a simple Strong element with nested Str
    let original_pandoc = vec![Inline::Strong(vec![Inline::Str("Bold".to_string())])];

    println!("Original Pandoc: {:?}", original_pandoc);

    // Create converters
    let pandoc_to_notion = PandocTextConverter::new();

    // Convert Pandoc to Notion
    println!("Converting Pandoc to Notion");
    let notion_result = pandoc_to_notion.convert(&original_pandoc);

    match notion_result {
        Ok(notion_rich_text) => {
            println!("Conversion to Notion succeeded: {:?}", notion_rich_text);

            // Convert Notion back to Pandoc
            println!("Converting Notion back to Pandoc");
            let roundtrip_pandoc = NotionTextConverter::convert(&notion_rich_text);

            println!("Roundtrip result: {:?}", roundtrip_pandoc);

            // Compare the results
            assert_eq!(
                original_pandoc, roundtrip_pandoc,
                "Roundtrip conversion failed."
            );
        }
        Err(e) => {
            println!("Conversion to Notion failed: {:?}", e);
            panic!("Failed to convert to Notion: {:?}", e);
        }
    }
}

// Test with a more complex nested structure
#[test]
fn test_complex_strong() {
    let original_pandoc = vec![Inline::Strong(vec![
        Inline::Str("Bold".to_string()),
        Inline::Space,
        Inline::Str("text".to_string()),
    ])];

    println!("Complex Strong Original: {:?}", original_pandoc);

    let pandoc_to_notion = PandocTextConverter::new();

    println!("Converting Complex Strong to Notion");
    let notion_result = pandoc_to_notion.convert(&original_pandoc);

    match notion_result {
        Ok(rich_text) => {
            println!("Complex Strong conversion succeeded: {:?}", rich_text);

            let roundtrip = NotionTextConverter::convert(&rich_text);
            println!("Complex Strong roundtrip: {:?}", roundtrip);

            assert_eq!(original_pandoc, roundtrip);
        }
        Err(e) => {
            println!("Complex Strong conversion failed: {:?}", e);
            panic!("Complex Strong conversion failed: {:?}", e);
        }
    }
}
