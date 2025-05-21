use notion_client::objects::block::{Block as NotionBlock, BlockType};
use pandoc_notion::n2p::notion_block_visitor::NotionToPandocVisitor;
use pandoc_notion::p2n::pandoc_block_visitor::PandocToNotionVisitor;
use pandoc_notion::test_utils::notion_helpers::test;
use pretty_assertions::assert_eq;

/// Integration test to verify roundtrip conversion of paragraph blocks: Notion -> Pandoc -> Notion
/// Tests that paragraph structure and content is preserved through conversions
#[test]
fn test_paragraph_roundtrip() {
    // Create visitors for conversions
    let n2p_visitor = NotionToPandocVisitor::new();
    let p2n_visitor = PandocToNotionVisitor::new();

    // Create test cases with different paragraph structures
    let test_cases = vec![
        create_simple_paragraph("Simple paragraph text"),
        create_paragraph_with_formatted_text(),
        create_paragraph_with_multiple_text_segments(),
        create_paragraph_with_children(),
        create_empty_paragraph(),
    ];

    // Test each case
    for (i, original_notion_block) in test_cases.iter().enumerate() {
        println!("\n---------- PARAGRAPH TEST CASE {} ----------", i);
        println!("ORIGINAL NOTION PARAGRAPH:");
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
        verify_paragraph_roundtrip(i, original_notion_block, &roundtrip_notion_blocks);
    }
}

/// Creates a simple paragraph block with plain text
fn create_simple_paragraph(content: &str) -> NotionBlock {
    test::create_paragraph_block(content, Some("simple_paragraph_id"))
}

/// Creates a paragraph with formatted text (bold, italic, etc.)
fn create_paragraph_with_formatted_text() -> NotionBlock {
    let mut block = test::create_paragraph_block("", Some("formatted_paragraph_id"));
    
    if let BlockType::Paragraph { paragraph } = &mut block.block_type {
        paragraph.rich_text = vec![
            test::create_rich_text("This is "),
            test::create_formatted_rich_text("bold", true, false, false, false, false, None),
            test::create_rich_text(" and "),
            test::create_formatted_rich_text("italic", false, true, false, false, false, None),
            test::create_rich_text(" text."),
        ];
    }
    
    block
}

/// Creates a paragraph with multiple text segments
fn create_paragraph_with_multiple_text_segments() -> NotionBlock {
    let mut block = test::create_paragraph_block("", Some("multi_segment_paragraph_id"));
    
    if let BlockType::Paragraph { paragraph } = &mut block.block_type {
        paragraph.rich_text = vec![
            test::create_rich_text("First segment. "),
            test::create_rich_text("Second segment."),
        ];
    }
    
    block
}

/// Creates a paragraph with child blocks
fn create_paragraph_with_children() -> NotionBlock {
    let children = vec![
        test::create_paragraph_block("This is a child paragraph.", Some("child_paragraph_id")),
    ];
    
    let mut block = test::create_paragraph_block("", Some("parent_paragraph_id"));
    
    if let BlockType::Paragraph { paragraph } = &mut block.block_type {
        paragraph.rich_text = vec![test::create_rich_text("Parent paragraph with children.")];
        paragraph.children = Some(children);
        
        // Make sure the has_children flag is set
        block.has_children = Some(true);
    }
    
    block
}

/// Creates an empty paragraph
fn create_empty_paragraph() -> NotionBlock {
    let mut block = test::create_paragraph_block("", Some("empty_paragraph_id"));
    
    if let BlockType::Paragraph { paragraph } = &mut block.block_type {
        paragraph.rich_text = vec![];
    }
    
    block
}

/// Verifies the results of the roundtrip conversion
fn verify_paragraph_roundtrip(
    case_index: usize,
    original: &NotionBlock,
    roundtrip: &[NotionBlock],
) {
    assert!(!roundtrip.is_empty(), "No blocks returned in roundtrip");
    
    // Helper closure to extract text from a paragraph
    let get_text = |block: &NotionBlock| -> String {
        match &block.block_type {
            BlockType::Paragraph { paragraph } => {
                paragraph.rich_text.iter()
                    .map(|rt| rt.plain_text().unwrap_or_default())
                    .collect()
            },
            _ => panic!("Expected a paragraph block"),
        }
    };
    
    // For cases with children, the hierarchy may be flattened
    if case_index == 3 { // paragraph with children
        // Verify the first block is a paragraph
        verify_block_type(&roundtrip[0], "paragraph");
        
        // Check content is preserved
        let original_text = get_text(original);
        let roundtrip_text = get_text(&roundtrip[0]);
        
        assert_eq!(
            original_text,
            roundtrip_text,
            "Parent paragraph text not preserved"
        );
        
        // In a Pandoc roundtrip, child blocks are preserved but flattened
        println!("Note: For paragraphs with children, hierarchical structure may not be preserved in Pandoc roundtrip");
    } else {
        // For simple cases, verify exact content
        verify_block_type(&roundtrip[0], "paragraph");
        
        // Handle empty paragraph case
        if case_index == 4 {  // empty paragraph
            match &roundtrip[0].block_type {
                BlockType::Paragraph { paragraph } => {
                    assert!(
                        paragraph.rich_text.is_empty() || 
                        paragraph.rich_text[0].plain_text().unwrap_or_default().trim().is_empty(),
                        "Empty paragraph should remain empty"
                    );
                },
                _ => panic!("Expected paragraph block"),
            }
        } else {
            // Compare content based on the case
            let original_text = get_text(original);
            let roundtrip_text = get_text(&roundtrip[0]);
            
            assert_eq!(
                original_text,
                roundtrip_text,
                "Paragraph text not preserved in case {}", case_index
            );
        }
    }
}

/// Verifies that a block has the expected type
fn verify_block_type(block: &NotionBlock, expected_type: &str) {
    match &block.block_type {
        BlockType::Paragraph { .. } if expected_type == "paragraph" => {},
        _ => panic!("Expected a {} block, but got {:?}", expected_type, block.block_type),
    }
}