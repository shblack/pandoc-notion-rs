use notion_client::objects::block::{Block as NotionBlock, BlockType, TextColor as BlockTextColor};
use pandoc_notion::n2p::notion_block_visitor::NotionToPandocVisitor;
use pandoc_notion::p2n::pandoc_block_visitor::PandocToNotionVisitor;
use pandoc_notion::test_utils::notion_helpers::test;
use pretty_assertions::assert_eq;

/// Integration test to verify roundtrip conversion of list blocks: Notion -> Pandoc -> Notion
/// Tests that list structure and content are preserved through conversions
#[test]
fn test_lists_roundtrip() {
    // Create visitors for conversions
    let n2p_visitor = NotionToPandocVisitor::new();
    let p2n_visitor = PandocToNotionVisitor::new();

    // Create test cases with different list types
    let test_cases = vec![
        create_bulleted_list_item("Simple bulleted list item"),
        create_numbered_list_item("Simple numbered list item"),
        create_todo_list_item("Simple to-do item", false),
        create_todo_list_item("Completed to-do item", true),
        create_bulleted_list_with_children(),
        create_nested_bulleted_list(),
        create_numbered_list_with_children(),
        create_mixed_list_types(),
    ];

    // Test each case
    for (i, original_notion_block) in test_cases.iter().enumerate() {
        println!("\n---------- LIST TEST CASE {} ----------", i);
        println!("ORIGINAL NOTION LIST ITEM:");
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
        verify_list_roundtrip(i, original_notion_block, &roundtrip_notion_blocks);
    }
}

/// Creates a simple bulleted list item
fn create_bulleted_list_item(content: &str) -> NotionBlock {
    test::create_bulleted_list_item(content, None, None)
}

/// Creates a simple numbered list item
fn create_numbered_list_item(content: &str) -> NotionBlock {
    test::create_numbered_list_item(content, None, None)
}

/// Creates a to-do list item with the specified check state
fn create_todo_list_item(content: &str, checked: bool) -> NotionBlock {
    test::create_todo_item(content, checked, None)
}

/// Creates a bulleted list item with paragraph children
fn create_bulleted_list_with_children() -> NotionBlock {
    // Create paragraph children
    let children = vec![
        test::create_paragraph_block("First child paragraph", Some("child_para_1")),
        test::create_paragraph_block("Second child paragraph", Some("child_para_2")),
    ];

    test::create_bulleted_list_item(
        "Bulleted list with children",
        Some(BlockTextColor::Default),
        Some(children),
    )
}

/// Creates a nested bulleted list structure
fn create_nested_bulleted_list() -> NotionBlock {
    // Create inner list items (level 2)
    let nested_items = vec![
        test::create_bulleted_list_item("Nested item 1", None, None),
        test::create_bulleted_list_item("Nested item 2", None, None),
    ];

    // Create top-level list item with nested items as children
    test::create_bulleted_list_item(
        "Parent list item with nested list",
        Some(BlockTextColor::Default),
        Some(nested_items),
    )
}

/// Creates a numbered list with children
fn create_numbered_list_with_children() -> NotionBlock {
    // Create inner list items (level 2)
    let nested_items = vec![
        test::create_numbered_list_item("Nested numbered item 1", None, None),
        test::create_numbered_list_item("Nested numbered item 2", None, None),
    ];

    // Create top-level list item with nested items as children
    test::create_numbered_list_item(
        "Parent numbered list item",
        Some(BlockTextColor::Default),
        Some(nested_items),
    )
}

/// Creates a list with mixed types (to-do parent with bulleted children)
fn create_mixed_list_types() -> NotionBlock {
    // Create bulleted list children
    let children = vec![
        test::create_bulleted_list_item("Child bulleted item", None, None),
    ];

    // Create parent to-do item
    test::create_todo_item("Parent to-do with mixed children", false, Some(children))
}

/// Verifies the results of the roundtrip conversion for lists
fn verify_list_roundtrip(
    case_index: usize,
    original: &NotionBlock,
    roundtrip: &[NotionBlock],
) {
    assert!(!roundtrip.is_empty(), "No blocks returned in roundtrip");
    
    // Helper closure to extract text from a list item
    let get_text = |block: &NotionBlock| -> String {
        match &block.block_type {
            BlockType::BulletedListItem { bulleted_list_item } => {
                if bulleted_list_item.rich_text.is_empty() {
                    "".to_string()
                } else {
                    bulleted_list_item.rich_text[0].plain_text().unwrap_or_default()
                }
            },
            BlockType::NumberedListItem { numbered_list_item } => {
                if numbered_list_item.rich_text.is_empty() {
                    "".to_string()
                } else {
                    numbered_list_item.rich_text[0].plain_text().unwrap_or_default()
                }
            },
            BlockType::ToDo { to_do } => {
                if to_do.rich_text.is_empty() {
                    "".to_string()
                } else {
                    to_do.rich_text[0].plain_text().unwrap_or_default()
                }
            },
            _ => panic!("Expected a list item type"),
        }
    };
    
    // Simple list items (cases 0-3)
    if case_index <= 3 {
        match &original.block_type {
            BlockType::BulletedListItem { .. } => {
                match &roundtrip[0].block_type {
                    BlockType::BulletedListItem { .. } => {
                        assert_eq!(
                            get_text(original),
                            get_text(&roundtrip[0]),
                            "Bulleted list item text not preserved"
                        );
                    },
                    _ => panic!("Expected a bulleted list item in roundtrip"),
                }
            },
            BlockType::NumberedListItem { .. } => {
                match &roundtrip[0].block_type {
                    BlockType::NumberedListItem { .. } => {
                        assert_eq!(
                            get_text(original),
                            get_text(&roundtrip[0]),
                            "Numbered list item text not preserved"
                        );
                    },
                    _ => panic!("Expected a numbered list item in roundtrip"),
                }
            },
            BlockType::ToDo { to_do } => {
                match &roundtrip[0].block_type {
                    BlockType::ToDo { to_do: rt_todo } => {
                        assert_eq!(
                            get_text(original),
                            get_text(&roundtrip[0]),
                            "To-do list item text not preserved"
                        );
                        
                        // For to-do items, also check if checked state is preserved
                        if case_index == 2 { // Simple to-do item (unchecked)
                            assert_eq!(
                                to_do.checked,
                                rt_todo.checked,
                                "To-do checked state not preserved"
                            );
                        } else if case_index == 3 { // Completed to-do item (checked)
                            assert_eq!(
                                to_do.checked,
                                rt_todo.checked,
                                "To-do checked state not preserved"
                            );
                        }
                    },
                    _ => panic!("Expected a to-do list item in roundtrip"),
                }
            },
            _ => panic!("Original block is not a list item"),
        }
    }
    // Complex list structures (cases 4-7)
    else {
        // First verify the parent list item is preserved
        match &original.block_type {
            BlockType::BulletedListItem { .. } => {
                match &roundtrip[0].block_type {
                    BlockType::BulletedListItem { .. } => {
                        assert_eq!(
                            get_text(original),
                            get_text(&roundtrip[0]),
                            "Bulleted list parent text not preserved"
                        );
                    },
                    _ => panic!("Expected a bulleted list item in roundtrip"),
                }
            },
            BlockType::NumberedListItem { .. } => {
                match &roundtrip[0].block_type {
                    BlockType::NumberedListItem { .. } => {
                        assert_eq!(
                            get_text(original),
                            get_text(&roundtrip[0]),
                            "Numbered list parent text not preserved"
                        );
                    },
                    _ => panic!("Expected a numbered list item in roundtrip"),
                }
            },
            BlockType::ToDo { .. } => {
                match &roundtrip[0].block_type {
                    BlockType::ToDo { .. } => {
                        assert_eq!(
                            get_text(original),
                            get_text(&roundtrip[0]),
                            "To-do list parent text not preserved"
                        );
                    },
                    _ => panic!("Expected a to-do list item in roundtrip"),
                }
            },
            _ => panic!("Original block is not a list item"),
        }
        
        // Then verify we have child blocks (the exact representation may differ)
        if roundtrip.len() > 1 {
            println!("Child blocks found in the roundtrip");
        } else {
            assert!(
                roundtrip[0].has_children == Some(true),
                "Children missing from roundtrip"
            );
        }
    }
}