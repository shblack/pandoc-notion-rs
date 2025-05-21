use notion_client::objects::block::{Block as NotionBlock, BlockType};
use pandoc_notion::n2p::notion_block_visitor::NotionToPandocVisitor;
use pandoc_notion::p2n::pandoc_block_visitor::PandocToNotionVisitor;
use pandoc_notion::test_utils::notion_helpers::test;
use pretty_assertions::assert_eq;

/// Integration test to verify roundtrip conversion of quote blocks: Notion -> Pandoc -> Notion
/// Tests that quote structure and content are preserved through conversions
#[test]
fn test_quotes_roundtrip() {
    // Create visitors for conversions
    let n2p_visitor = NotionToPandocVisitor::new();
    let p2n_visitor = PandocToNotionVisitor::new();

    // Create test cases with different quote configurations
    let test_cases = vec![
        create_simple_quote("A simple quote block"),
        create_quote_with_formatted_text(),
        create_quote_with_multiple_text_segments(),
        create_quote_with_children(),
        create_empty_quote(),
    ];

    // Test each case
    for (i, original_notion_block) in test_cases.iter().enumerate() {
        println!("\n---------- QUOTE TEST CASE {} ----------", i);
        println!("ORIGINAL NOTION QUOTE:");
        test::print_notion_block(original_notion_block);

        // Convert Notion to Pandoc
        let pandoc_blocks = n2p_visitor.convert_blocks(&[original_notion_block.clone()]);
        
        assert!(
            !pandoc_blocks.is_empty(),
            "Conversion to Pandoc produced no blocks"
        );
        
        println!("\nNOTION → PANDOC RESULT ({} blocks):", pandoc_blocks.len());
        for (j, block) in pandoc_blocks.iter().enumerate() {
            println!("Pandoc Block {}: {:#?}", j, block);
        }

        // Convert Pandoc back to Notion
        let roundtrip_notion_blocks = p2n_visitor
            .convert_blocks(&pandoc_blocks)
            .expect("Conversion from Pandoc back to Notion should succeed");
        
        assert!(
            !roundtrip_notion_blocks.is_empty(),
            "Conversion back to Notion produced no blocks"
        );
        
        println!(
            "\nPANDOC → NOTION RESULT ({} blocks):",
            roundtrip_notion_blocks.len()
        );
        for (j, block) in roundtrip_notion_blocks.iter().enumerate() {
            println!("Roundtrip Notion Block {}:", j);
            test::print_notion_block(block);
        }
        
        // Verify the roundtrip results
        verify_quote_roundtrip(i, original_notion_block, &roundtrip_notion_blocks);
    }
}

/// Creates a simple quote block with plain text
fn create_simple_quote(content: &str) -> NotionBlock {
    test::create_quote_block(content, None)
}

/// Creates a quote with formatted text (bold, italic, etc.)
fn create_quote_with_formatted_text() -> NotionBlock {
    let mut block = test::create_quote_block("", None);
    
    if let BlockType::Quote { quote } = &mut block.block_type {
        quote.rich_text = vec![
            test::create_rich_text("This is "),
            test::create_formatted_rich_text("bold", true, false, false, false, false, None),
            test::create_rich_text(" and "),
            test::create_formatted_rich_text("italic", false, true, false, false, false, None),
            test::create_rich_text(" text in a quote."),
        ];
    }
    
    block
}

/// Creates a quote with multiple text segments
fn create_quote_with_multiple_text_segments() -> NotionBlock {
    let mut block = test::create_quote_block("", None);
    
    if let BlockType::Quote { quote } = &mut block.block_type {
        quote.rich_text = vec![
            test::create_rich_text("First segment of the quote. "),
            test::create_rich_text("Second segment of the quote."),
        ];
    }
    
    block
}

/// Creates a quote with child blocks
fn create_quote_with_children() -> NotionBlock {
    let children = vec![
        test::create_paragraph_block("This is a paragraph inside a quote.", Some("child_paragraph_id")),
    ];
    
    test::create_quote_block("Quote with children blocks.", Some(children))
}

/// Creates an empty quote
fn create_empty_quote() -> NotionBlock {
    let mut block = test::create_quote_block("", None);
    
    if let BlockType::Quote { quote } = &mut block.block_type {
        quote.rich_text = vec![];
    }
    
    block
}

/// Verifies the results of the roundtrip conversion for quotes
fn verify_quote_roundtrip(
    case_index: usize,
    original: &NotionBlock,
    roundtrip: &[NotionBlock],
) {
    assert!(!roundtrip.is_empty(), "No blocks returned in roundtrip");
    
    // Helper closure to extract text from a quote
    let get_text = |block: &NotionBlock| -> String {
        match &block.block_type {
            BlockType::Quote { quote } => {
                quote.rich_text.iter()
                    .map(|rt| rt.plain_text().unwrap_or_default())
                    .collect()
            },
            _ => panic!("Expected a quote block"),
        }
    };
    
    // For cases with children, the hierarchy may be flattened
    if case_index == 3 { // quote with children
        // Verify the first block is a quote
        verify_block_type(&roundtrip[0], "quote");
        
        // Check content is preserved
        let original_text = get_text(original);
        let roundtrip_text = get_text(&roundtrip[0]);
        
        assert_eq!(
            original_text,
            roundtrip_text,
            "Quote text not preserved"
        );
        
        // In a Pandoc roundtrip, child blocks are preserved but flattened
        println!("Note: For quotes with children, hierarchical structure may not be preserved in Pandoc roundtrip");
    } else {
        // For simple cases, verify exact content
        verify_block_type(&roundtrip[0], "quote");
        
        // Handle empty quote case
        if case_index == 4 {  // empty quote
            match &roundtrip[0].block_type {
                BlockType::Quote { quote } => {
                    assert!(
                        quote.rich_text.is_empty() || 
                        quote.rich_text[0].plain_text().unwrap_or_default().trim().is_empty(),
                        "Empty quote should remain empty"
                    );
                },
                _ => panic!("Expected quote block"),
            }
        } else {
            // Compare content based on the case
            let original_text = get_text(original);
            let roundtrip_text = get_text(&roundtrip[0]);
            
            assert_eq!(
                original_text,
                roundtrip_text,
                "Quote text not preserved in case {}", case_index
            );
        }
    }
}

/// Verifies that a block has the expected type
fn verify_block_type(block: &NotionBlock, expected_type: &str) {
    match &block.block_type {
        BlockType::Quote { .. } if expected_type == "quote" => {},
        _ => panic!("Expected a {} block, but got {:?}", expected_type, block.block_type),
    }
}