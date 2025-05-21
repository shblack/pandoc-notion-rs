use notion_client::objects::block::{
    Block as NotionBlock, BlockType, HeadingsValue, ParagraphValue,
};
use notion_client::objects::rich_text::{RichText, Text};
use pandoc_notion::n2p::notion_block_visitor::NotionToPandocVisitor;
use pandoc_notion::p2n::pandoc_block_visitor::PandocToNotionVisitor;
use pretty_assertions::assert_eq;

/// Integration test to verify roundtrip conversion: Notion -> Pandoc -> Notion
/// Tests that block structure and content is preserved through conversions
/// Note: Children will be flattened in the process as Pandoc doesn't have the same hierarchy model
#[test]
fn test_notion_to_pandoc_to_notion_roundtrip() {
    // Create a NotionToPandocVisitor
    let n2p_visitor = NotionToPandocVisitor::new();

    // Create a PandocToNotionVisitor
    let p2n_visitor = PandocToNotionVisitor::new();

    // Create test cases with different block structures
    let test_cases = vec![
        create_simple_paragraph_block(),
        create_simple_heading_block(1, "Heading 1"),
        create_simple_heading_block(2, "Heading 2"),
        create_simple_heading_block(3, "Heading 3"),
        create_paragraph_with_children(),
    ];

    // Test each case
    for (i, original_notion_block) in test_cases.iter().enumerate() {
        println!("\n---------- CASE {} ----------", i);
        println!("ORIGINAL NOTION BLOCK:");
        print_notion_block(original_notion_block);

        // Step 1: Convert Notion to Pandoc
        let pandoc_blocks = n2p_visitor.convert_blocks(&[original_notion_block.clone()]);

        // Step A: Verify the conversion produced some output
        assert!(
            !pandoc_blocks.is_empty(),
            "Conversion to Pandoc produced no blocks"
        );
        println!("\nNOTION → PANDOC RESULT ({} blocks):", pandoc_blocks.len());
        for (j, block) in pandoc_blocks.iter().enumerate() {
            println!("Pandoc Block {}: {:#?}", j, block);
        }

        // Step 2: Convert Pandoc back to Notion
        let roundtrip_notion_blocks = p2n_visitor
            .convert_blocks(&pandoc_blocks, None)
            .expect("Conversion from Pandoc back to Notion should succeed");

        // Step B: Verify the conversion produced some output
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
            print_notion_block(block);
        }
        println!("-------------------------------\n");

        // Compare the results based on the test case
        verify_roundtrip_results(i, original_notion_block, &roundtrip_notion_blocks);
    }
}

/// Create a simple paragraph block with text
fn create_simple_paragraph_block() -> NotionBlock {
    NotionBlock {
        object: Some("block".to_string()),
        id: Some("test-paragraph-id".to_string()),
        parent: None,
        created_time: None,
        created_by: None,
        last_edited_time: None,
        last_edited_by: None,
        archived: Some(false),
        has_children: Some(false),
        block_type: BlockType::Paragraph {
            paragraph: ParagraphValue {
                rich_text: vec![RichText::Text {
                    text: Text {
                        content: "Simple paragraph text".to_string(),
                        link: None,
                    },
                    annotations: None,
                    plain_text: Some("Simple paragraph text".to_string()),
                    href: None,
                }],
                color: None,
                children: None,
            },
        },
    }
}

/// Create a simple heading block with specified level and text
fn create_simple_heading_block(level: u8, text: &str) -> NotionBlock {
    let heading_value = HeadingsValue {
        rich_text: vec![RichText::Text {
            text: Text {
                content: text.to_string(),
                link: None,
            },
            annotations: None,
            plain_text: Some(text.to_string()),
            href: None,
        }],
        color: None,
        is_toggleable: Some(false),
    };

    NotionBlock {
        object: Some("block".to_string()),
        id: Some(format!("test-heading-{}-id", level)),
        parent: None,
        created_time: None,
        created_by: None,
        last_edited_time: None,
        last_edited_by: None,
        archived: Some(false),
        has_children: Some(false),
        block_type: match level {
            1 => BlockType::Heading1 {
                heading_1: heading_value,
            },
            2 => BlockType::Heading2 {
                heading_2: heading_value,
            },
            3 => BlockType::Heading3 {
                heading_3: heading_value,
            },
            _ => panic!("Unsupported heading level"),
        },
    }
}

/// Create a paragraph block with nested child paragraphs
fn create_paragraph_with_children() -> NotionBlock {
    // Create child paragraphs
    let child1 = NotionBlock {
        object: Some("block".to_string()),
        id: Some("child-1-id".to_string()),
        parent: None,
        created_time: None,
        created_by: None,
        last_edited_time: None,
        last_edited_by: None,
        archived: Some(false),
        has_children: Some(false),
        block_type: BlockType::Paragraph {
            paragraph: ParagraphValue {
                rich_text: vec![RichText::Text {
                    text: Text {
                        content: "Child paragraph 1".to_string(),
                        link: None,
                    },
                    annotations: None,
                    plain_text: Some("Child paragraph 1".to_string()),
                    href: None,
                }],
                color: None,
                children: None,
            },
        },
    };

    let child2 = NotionBlock {
        object: Some("block".to_string()),
        id: Some("child-2-id".to_string()),
        parent: None,
        created_time: None,
        created_by: None,
        last_edited_time: None,
        last_edited_by: None,
        archived: Some(false),
        has_children: Some(false),
        block_type: BlockType::Paragraph {
            paragraph: ParagraphValue {
                rich_text: vec![RichText::Text {
                    text: Text {
                        content: "Child paragraph 2".to_string(),
                        link: None,
                    },
                    annotations: None,
                    plain_text: Some("Child paragraph 2".to_string()),
                    href: None,
                }],
                color: None,
                children: None,
            },
        },
    };

    // Create parent with children
    NotionBlock {
        object: Some("block".to_string()),
        id: Some("parent-paragraph-id".to_string()),
        parent: None,
        created_time: None,
        created_by: None,
        last_edited_time: None,
        last_edited_by: None,
        archived: Some(false),
        has_children: Some(true),
        block_type: BlockType::Paragraph {
            paragraph: ParagraphValue {
                rich_text: vec![RichText::Text {
                    text: Text {
                        content: "Parent paragraph".to_string(),
                        link: None,
                    },
                    annotations: None,
                    plain_text: Some("Parent paragraph".to_string()),
                    href: None,
                }],
                color: None,
                children: Some(vec![child1, child2]),
            },
        },
    }
}

/// Verify the results of the roundtrip conversion
fn verify_roundtrip_results(case_index: usize, original: &NotionBlock, roundtrip: &[NotionBlock]) {
    match case_index {
        // Simple paragraph
        0 => {
            assert_eq!(
                roundtrip.len(),
                1,
                "Should have exactly one paragraph block after roundtrip"
            );
            verify_paragraph_content(original, &roundtrip[0]);
        }
        // Heading 1
        1 => {
            assert_eq!(
                roundtrip.len(),
                1,
                "Should have exactly one heading block after roundtrip"
            );
            verify_heading_content(original, &roundtrip[0], 1);
        }
        // Heading 2
        2 => {
            assert_eq!(
                roundtrip.len(),
                1,
                "Should have exactly one heading block after roundtrip"
            );
            verify_heading_content(original, &roundtrip[0], 2);
        }
        // Heading 3
        3 => {
            assert_eq!(
                roundtrip.len(),
                1,
                "Should have exactly one heading block after roundtrip"
            );
            verify_heading_content(original, &roundtrip[0], 3);
        }
        // Paragraph with children
        4 => {
            // In Pandoc, children get flattened, so we should have 3 blocks after roundtrip:
            // Parent paragraph, child1, child2
            assert_eq!(
                roundtrip.len(),
                3,
                "Should have three blocks after flattening hierarchy"
            );

            // Verify the parent paragraph content
            verify_paragraph_content(original, &roundtrip[0]);

            // Get children from original
            let children = match &original.block_type {
                BlockType::Paragraph { paragraph } => paragraph.children.as_ref().unwrap(),
                _ => panic!("Expected paragraph with children"),
            };

            // Verify child 1
            verify_paragraph_content(&children[0], &roundtrip[1]);

            // Verify child 2
            verify_paragraph_content(&children[1], &roundtrip[2]);
        }
        _ => panic!("Unknown test case"),
    }
}

/// Helper to verify paragraph content matches
fn verify_paragraph_content(original: &NotionBlock, roundtrip: &NotionBlock) {
    match (&original.block_type, &roundtrip.block_type) {
        (BlockType::Paragraph { paragraph: p1 }, BlockType::Paragraph { paragraph: p2 }) => {
            // Compare rich_text content
            let original_text = p1.rich_text[0].plain_text().unwrap();
            let roundtrip_text = p2.rich_text[0].plain_text().unwrap();
            assert_eq!(
                original_text, roundtrip_text,
                "Paragraph text content should match"
            );
        }
        _ => panic!("Expected both to be paragraphs"),
    }
}

/// Helper to verify heading content matches
fn verify_heading_content(original: &NotionBlock, roundtrip: &NotionBlock, expected_level: u8) {
    let (original_text, original_level) = match &original.block_type {
        BlockType::Heading1 { heading_1 } => (heading_1.rich_text[0].plain_text().unwrap(), 1),
        BlockType::Heading2 { heading_2 } => (heading_2.rich_text[0].plain_text().unwrap(), 2),
        BlockType::Heading3 { heading_3 } => (heading_3.rich_text[0].plain_text().unwrap(), 3),
        _ => panic!("Expected original to be a heading"),
    };

    let (roundtrip_text, roundtrip_level) = match &roundtrip.block_type {
        BlockType::Heading1 { heading_1 } => (heading_1.rich_text[0].plain_text().unwrap(), 1),
        BlockType::Heading2 { heading_2 } => (heading_2.rich_text[0].plain_text().unwrap(), 2),
        BlockType::Heading3 { heading_3 } => (heading_3.rich_text[0].plain_text().unwrap(), 3),
        _ => panic!("Expected roundtrip to be a heading"),
    };

    assert_eq!(
        original_level, expected_level,
        "Original heading level should match expected"
    );
    assert_eq!(
        roundtrip_level, expected_level,
        "Roundtrip heading level should match expected"
    );
    assert_eq!(
        original_text, roundtrip_text,
        "Heading text content should match"
    );
}

/// Test to verify more complex hierarchies with headings and paragraphs
#[test]
fn test_complex_hierarchy_roundtrip() {
    println!("\n========== COMPLEX HIERARCHY ROUNDTRIP TEST ==========");

    // Create a NotionToPandocVisitor
    let n2p_visitor = NotionToPandocVisitor::new();

    // Create a PandocToNotionVisitor
    let p2n_visitor = PandocToNotionVisitor::new();

    // Create a complex structure with heading and nested content
    let original_blocks = vec![
        create_simple_heading_block(1, "Document Title"),
        create_simple_paragraph_block(), // Top level paragraph
        create_simple_heading_block(2, "Section Heading"),
        create_paragraph_with_children(), // Paragraph with children
    ];

    // Print original blocks
    println!("ORIGINAL NOTION BLOCKS ({} blocks):", original_blocks.len());
    for (i, block) in original_blocks.iter().enumerate() {
        println!("Original Block {}:", i);
        print_notion_block(block);
    }

    // Step 1: Convert Notion to Pandoc
    let pandoc_blocks = n2p_visitor.convert_blocks(&original_blocks);

    // Verify the conversion produced the expected number of blocks
    // 4 original blocks + 2 children from the paragraph with children = 6 total
    assert_eq!(
        pandoc_blocks.len(),
        6,
        "Should have 6 Pandoc blocks after flattening"
    );

    // Print pandoc blocks
    println!("\nNOTION → PANDOC RESULT ({} blocks):", pandoc_blocks.len());
    for (i, block) in pandoc_blocks.iter().enumerate() {
        println!("Pandoc Block {}: {:#?}", i, block);
    }

    // Step 2: Convert Pandoc back to Notion
    let roundtrip_blocks = p2n_visitor
        .convert_blocks(&pandoc_blocks, None)
        .expect("Conversion from Pandoc back to Notion should succeed");

    // Verify we get 6 blocks back (hierarchy is flattened)
    assert_eq!(
        roundtrip_blocks.len(),
        6,
        "Should have 6 Notion blocks after roundtrip"
    );

    // Print roundtrip blocks
    println!(
        "\nPANDOC → NOTION RESULT ({} blocks):",
        roundtrip_blocks.len()
    );
    for (i, block) in roundtrip_blocks.iter().enumerate() {
        println!("Roundtrip Block {}:", i);
        print_notion_block(block);
    }
    println!("======================================================\n");

    // Verify block types in the expected order
    verify_block_type(&roundtrip_blocks[0], "heading1");
    verify_block_type(&roundtrip_blocks[1], "paragraph");
    verify_block_type(&roundtrip_blocks[2], "heading2");
    verify_block_type(&roundtrip_blocks[3], "paragraph"); // Parent paragraph
    verify_block_type(&roundtrip_blocks[4], "paragraph"); // Child paragraph 1
    verify_block_type(&roundtrip_blocks[5], "paragraph"); // Child paragraph 2

    // Verify content of key blocks
    match &roundtrip_blocks[0].block_type {
        BlockType::Heading1 { heading_1 } => {
            assert_eq!(
                heading_1.rich_text[0].plain_text().unwrap(),
                "Document Title"
            );
        }
        _ => panic!("Expected heading1"),
    }

    match &roundtrip_blocks[2].block_type {
        BlockType::Heading2 { heading_2 } => {
            assert_eq!(
                heading_2.rich_text[0].plain_text().unwrap(),
                "Section Heading"
            );
        }
        _ => panic!("Expected heading2"),
    }

    // Verify parent and child paragraph content
    match &roundtrip_blocks[3].block_type {
        BlockType::Paragraph { paragraph } => {
            assert_eq!(
                paragraph.rich_text[0].plain_text().unwrap(),
                "Parent paragraph"
            );
        }
        _ => panic!("Expected paragraph"),
    }

    match &roundtrip_blocks[4].block_type {
        BlockType::Paragraph { paragraph } => {
            assert_eq!(
                paragraph.rich_text[0].plain_text().unwrap(),
                "Child paragraph 1"
            );
        }
        _ => panic!("Expected paragraph"),
    }

    match &roundtrip_blocks[5].block_type {
        BlockType::Paragraph { paragraph } => {
            assert_eq!(
                paragraph.rich_text[0].plain_text().unwrap(),
                "Child paragraph 2"
            );
        }
        _ => panic!("Expected paragraph"),
    }
}

/// Helper to verify a block is of the expected type
fn verify_block_type(block: &NotionBlock, expected_type: &str) {
    let block_type = match &block.block_type {
        BlockType::Paragraph { .. } => "paragraph",
        BlockType::Heading1 { .. } => "heading1",
        BlockType::Heading2 { .. } => "heading2",
        BlockType::Heading3 { .. } => "heading3",
        _ => "other",
    };

    assert_eq!(
        block_type, expected_type,
        "Block should be of type {}",
        expected_type
    );
}

/// Helper to print a Notion block in a readable format
fn print_notion_block(block: &NotionBlock) {
    // Get block type and content
    match &block.block_type {
        BlockType::Paragraph { paragraph } => {
            println!("  Type: Paragraph");
            if !paragraph.rich_text.is_empty() {
                println!(
                    "  Content: \"{}\"",
                    paragraph.rich_text[0]
                        .plain_text()
                        .unwrap_or("".to_string())
                        .to_string()
                );
            } else {
                println!("  Content: <empty>");
            }
            if let Some(children) = &paragraph.children {
                println!("  Children: {} child blocks", children.len());
                for (i, child) in children.iter().enumerate() {
                    println!("  Child {}:", i);
                    print_notion_block(child);
                }
            }
        }
        BlockType::Heading1 { heading_1 } => {
            println!("  Type: Heading 1");
            if !heading_1.rich_text.is_empty() {
                println!(
                    "  Content: \"{}\"",
                    heading_1.rich_text[0]
                        .plain_text()
                        .unwrap_or("".to_string())
                );
            } else {
                println!("  Content: <empty>");
            }
        }
        BlockType::Heading2 { heading_2 } => {
            println!("  Type: Heading 2");
            if !heading_2.rich_text.is_empty() {
                println!(
                    "  Content: \"{}\"",
                    heading_2.rich_text[0]
                        .plain_text()
                        .unwrap_or("".to_string())
                );
            } else {
                println!("  Content: <empty>");
            }
        }
        BlockType::Heading3 { heading_3 } => {
            println!("  Type: Heading 3");
            if !heading_3.rich_text.is_empty() {
                println!(
                    "  Content: \"{}\"",
                    heading_3.rich_text[0]
                        .plain_text()
                        .unwrap_or("".to_string())
                );
            } else {
                println!("  Content: <empty>");
            }
        }
        _ => println!("  Type: Other block type"),
    }
    println!(
        "  ID: {}",
        block.id.as_ref().unwrap_or(&"<no id>".to_string())
    );
    println!("  Has Children: {}", block.has_children.unwrap_or(false));
}
