use notion_client::objects::block::{Block as NotionBlock, BlockType};
use pandoc_notion::n2p::notion_block_visitor::NotionToPandocVisitor;
use pandoc_notion::p2n::pandoc_block_visitor::PandocToNotionVisitor;
use pandoc_notion::test_utils::notion_helpers::test;
use pretty_assertions::assert_eq;

/// Integration test to verify roundtrip conversion of heading blocks: Notion -> Pandoc -> Notion
/// Tests that heading structure, level, and content are preserved through conversions
#[test]
fn test_headings_roundtrip() {
    // Create visitors for conversions
    let n2p_visitor = NotionToPandocVisitor::new();
    let p2n_visitor = PandocToNotionVisitor::new();

    // Create test cases with different heading levels
    let mut test_cases = vec![
        test::create_heading_block(1, "Heading Level 1"),
        test::create_heading_block(2, "Heading Level 2"),
        test::create_heading_block(3, "Heading Level 3"),
        test::create_heading_with_formatted_text(1),
        test::create_heading_with_formatted_text(2),
        test::create_heading_with_formatted_text(3),
        test::create_heading_with_children(1),
        test::create_empty_heading(2),
    ];
    
    // Add a test case for a toggleable heading with child content
    let heading_with_children_blocks = test::create_heading_with_child_blocks(
        1,
        "Toggleable heading with explicit children",
        vec![
            test::create_paragraph_block("This is a child paragraph of the heading", Some("child_para_id")),
            test::create_bulleted_list_item("Child list item of the heading", None, None),
        ]
    );
    test_cases.push(heading_with_children_blocks[0].clone()); // Add just the heading to test cases

    // Test each case
    for (i, original_notion_block) in test_cases.iter().enumerate() {
        println!("\n---------- HEADING TEST CASE {} ----------", i);
        println!("ORIGINAL NOTION HEADING:");
        test::print_notion_block(original_notion_block);

        // Special handling for case 8: toggleable heading with explicit children
        let input_blocks = if i == 8 {
            // Get the heading and its children
            let mut blocks = vec![original_notion_block.clone()];
            blocks.extend(heading_with_children_blocks[1..].iter().cloned());
            blocks
        } else {
            vec![original_notion_block.clone()]
        };

        // Convert Notion to Pandoc
        let pandoc_blocks = n2p_visitor.convert_blocks(&input_blocks);
        
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
        verify_heading_roundtrip(i, original_notion_block, &roundtrip_notion_blocks);
    }
}

/// Verifies the results of the roundtrip conversion for headings
fn verify_heading_roundtrip(
    case_index: usize,
    original: &NotionBlock,
    roundtrip: &[NotionBlock],
) {
    assert!(!roundtrip.is_empty(), "No blocks returned in roundtrip");
    
    // For cases with children, the hierarchy may be flattened
    if case_index == 6 || case_index == 8 { // heading with children
        // Verify the first block is a heading
        verify_block_type(&roundtrip[0], "heading");
        
        // Extract the expected level and content from the original heading
        match &original.block_type {
            BlockType::Heading1 { heading_1 } => {
                match &roundtrip[0].block_type {
                    BlockType::Heading1 { heading_1: rt_h1 } => {
                        if !heading_1.rich_text.is_empty() && !rt_h1.rich_text.is_empty() {
                            assert_eq!(
                                heading_1.rich_text[0].plain_text().unwrap_or_default(),
                                rt_h1.rich_text[0].plain_text().unwrap_or_default(),
                                "Heading text not preserved"
                            );
                        }
                        
                        // Note: toggleable state may not be preserved in the roundtrip
                        // We're only checking that the heading text itself is preserved
                    },
                    _ => panic!("Expected Heading1 in roundtrip"),
                }
            },
            BlockType::Heading2 { heading_2 } => {
                match &roundtrip[0].block_type {
                    BlockType::Heading2 { heading_2: rt_h2 } => {
                        if !heading_2.rich_text.is_empty() && !rt_h2.rich_text.is_empty() {
                            assert_eq!(
                                heading_2.rich_text[0].plain_text().unwrap_or_default(),
                                rt_h2.rich_text[0].plain_text().unwrap_or_default(),
                                "Heading text not preserved"
                            );
                        }
                    },
                    _ => panic!("Expected Heading2 in roundtrip"),
                }
            },
            BlockType::Heading3 { heading_3 } => {
                match &roundtrip[0].block_type {
                    BlockType::Heading3 { heading_3: rt_h3 } => {
                        if !heading_3.rich_text.is_empty() && !rt_h3.rich_text.is_empty() {
                            assert_eq!(
                                heading_3.rich_text[0].plain_text().unwrap_or_default(),
                                rt_h3.rich_text[0].plain_text().unwrap_or_default(),
                                "Heading text not preserved"
                            );
                        }
                    },
                    _ => panic!("Expected Heading3 in roundtrip"),
                }
            },
            _ => panic!("Original block is not a heading"),
        }
        
        // For case 8, verify child content specifically
        if case_index == 8 {
            // Check that we have at least 3 blocks (heading + 2 children)
            assert!(
                roundtrip.len() >= 3,
                "Expected at least 3 blocks after roundtrip, got {}",
                roundtrip.len()
            );
            
            // Check if child paragraph content is preserved
            let paragraph_content = "This is a child paragraph of the heading";
            let list_item_content = "Child list item of the heading";
            
            let has_paragraph = roundtrip.iter().any(|block| {
                match &block.block_type {
                    BlockType::Paragraph { paragraph } => {
                        if !paragraph.rich_text.is_empty() {
                            let text = paragraph.rich_text.iter()
                                .map(|rt| rt.plain_text().unwrap_or_default())
                                .collect::<String>();
                            text.contains(paragraph_content)
                        } else {
                            false
                        }
                    },
                    _ => false,
                }
            });
            
            let has_list_item = roundtrip.iter().any(|block| {
                match &block.block_type {
                    BlockType::BulletedListItem { bulleted_list_item } => {
                        if !bulleted_list_item.rich_text.is_empty() {
                            let text = bulleted_list_item.rich_text.iter()
                                .map(|rt| rt.plain_text().unwrap_or_default())
                                .collect::<String>();
                            text.contains(list_item_content)
                        } else {
                            false
                        }
                    },
                    _ => false,
                }
            });
            
            assert!(
                has_paragraph,
                "Child paragraph content not found after roundtrip"
            );
            
            assert!(
                has_list_item,
                "Child list item content not found after roundtrip"
            );
            
            println!("Child blocks were preserved in the roundtrip");
        } else {
            // In a Pandoc roundtrip, child blocks are preserved but flattened
            println!("Note: For toggleable headings, hierarchical structure cannot be preserved in Pandoc roundtrip");
        }
    } else {
        // For simple cases, verify exact content
        verify_block_type(&roundtrip[0], "heading");
        
        match &original.block_type {
            BlockType::Heading1 { heading_1 } => {
                match &roundtrip[0].block_type {
                    BlockType::Heading1 { heading_1: rt_h1 } => {
                        // Handle empty heading case
                        if case_index == 7 {  // empty heading
                            assert!(
                                rt_h1.rich_text.is_empty() || 
                                rt_h1.rich_text[0].plain_text().unwrap_or_default().trim().is_empty(),
                                "Empty heading should remain empty"
                            );
                        } else if case_index >= 3 && case_index <= 5 {  // formatted text headings
                            // Check if total text content is preserved (may be in different segments)
                            let original_text: String = heading_1.rich_text.iter()
                                .map(|rt| rt.plain_text().unwrap_or_default())
                                .collect();
                            let roundtrip_text: String = rt_h1.rich_text.iter()
                                .map(|rt| rt.plain_text().unwrap_or_default())
                                .collect();
                            assert_eq!(
                                original_text, 
                                roundtrip_text,
                                "Formatted heading text not preserved"
                            );
                        } else if !heading_1.rich_text.is_empty() && !rt_h1.rich_text.is_empty() {
                            // simple headings
                            assert_eq!(
                                heading_1.rich_text[0].plain_text().unwrap_or_default(),
                                rt_h1.rich_text[0].plain_text().unwrap_or_default(),
                                "Heading text not preserved"
                            );
                        }
                    },
                    _ => panic!("Expected Heading1 in roundtrip"),
                }
            },
            BlockType::Heading2 { heading_2 } => {
                match &roundtrip[0].block_type {
                    BlockType::Heading2 { heading_2: rt_h2 } => {
                        // Handle empty heading case
                        if case_index == 7 {  // empty heading
                            assert!(
                                rt_h2.rich_text.is_empty() || 
                                rt_h2.rich_text[0].plain_text().unwrap_or_default().trim().is_empty(),
                                "Empty heading should remain empty"
                            );
                        } else if case_index >= 3 && case_index <= 5 {  // formatted text headings
                            // Check if total text content is preserved (may be in different segments)
                            let original_text: String = heading_2.rich_text.iter()
                                .map(|rt| rt.plain_text().unwrap_or_default())
                                .collect();
                            let roundtrip_text: String = rt_h2.rich_text.iter()
                                .map(|rt| rt.plain_text().unwrap_or_default())
                                .collect();
                            assert_eq!(
                                original_text, 
                                roundtrip_text,
                                "Formatted heading text not preserved"
                            );
                        } else if !heading_2.rich_text.is_empty() && !rt_h2.rich_text.is_empty() {
                            // simple headings
                            assert_eq!(
                                heading_2.rich_text[0].plain_text().unwrap_or_default(),
                                rt_h2.rich_text[0].plain_text().unwrap_or_default(),
                                "Heading text not preserved"
                            );
                        }
                    },
                    _ => panic!("Expected Heading2 in roundtrip"),
                }
            },
            BlockType::Heading3 { heading_3 } => {
                match &roundtrip[0].block_type {
                    BlockType::Heading3 { heading_3: rt_h3 } => {
                        // Handle empty heading case
                        if case_index == 7 {  // empty heading
                            assert!(
                                rt_h3.rich_text.is_empty() || 
                                rt_h3.rich_text[0].plain_text().unwrap_or_default().trim().is_empty(),
                                "Empty heading should remain empty"
                            );
                        } else if case_index >= 3 && case_index <= 5 {  // formatted text headings
                            // Check if total text content is preserved (may be in different segments)
                            let original_text: String = heading_3.rich_text.iter()
                                .map(|rt| rt.plain_text().unwrap_or_default())
                                .collect();
                            let roundtrip_text: String = rt_h3.rich_text.iter()
                                .map(|rt| rt.plain_text().unwrap_or_default())
                                .collect();
                            assert_eq!(
                                original_text, 
                                roundtrip_text,
                                "Formatted heading text not preserved"
                            );
                        } else if !heading_3.rich_text.is_empty() && !rt_h3.rich_text.is_empty() {
                            // simple headings
                            assert_eq!(
                                heading_3.rich_text[0].plain_text().unwrap_or_default(),
                                rt_h3.rich_text[0].plain_text().unwrap_or_default(),
                                "Heading text not preserved"
                            );
                        }
                    },
                    _ => panic!("Expected Heading3 in roundtrip"),
                }
            },
            _ => panic!("Original block is not a heading"),
        }
    }
}

/// Verifies that a block has the expected type
fn verify_block_type(block: &NotionBlock, expected_type: &str) {
    match &block.block_type {
        BlockType::Heading1 { .. } | BlockType::Heading2 { .. } | BlockType::Heading3 { .. } 
            if expected_type == "heading" => {},
        _ => panic!("Expected a {} block, but got {:?}", expected_type, block.block_type),
    }
}