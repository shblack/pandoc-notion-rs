use crate::n2p::notion_text::NotionTextConverter;
use crate::n2p::ConversionConfig;
use notion_client::objects::block::{Block as NotionBlock, BlockType, TextColor};
use pandoc_types::definition::{
    Attr, Block as PandocBlock, Inline, ListAttributes, ListNumberDelim, ListNumberStyle,
};

/// Convert a Notion bulleted list item to a Pandoc bullet list item
pub fn convert_notion_bulleted_list(block: &NotionBlock, config: &ConversionConfig) -> Option<PandocBlock> {
    match &block.block_type {
        BlockType::BulletedListItem { bulleted_list_item } => {
            // Convert rich text to Pandoc inlines
            let inlines = NotionTextConverter::convert(&bulleted_list_item.rich_text);

            // Apply color formatting if needed
            let styled_inlines = apply_text_color(inlines, &bulleted_list_item.color, config);

            // Create a Plain block with the styled inlines (not Para)
            let plain = PandocBlock::Plain(styled_inlines);

            // Create a bullet list with a single item
            Some(PandocBlock::BulletList(vec![vec![plain]]))
        }
        _ => None,
    }
}

/// Convert a Notion numbered list item to a Pandoc ordered list item
pub fn convert_notion_numbered_list(block: &NotionBlock, config: &ConversionConfig) -> Option<PandocBlock> {
    match &block.block_type {
        BlockType::NumberedListItem { numbered_list_item } => {
            // Convert rich text to Pandoc inlines
            let inlines = NotionTextConverter::convert(&numbered_list_item.rich_text);

            // Apply color formatting if needed
            let styled_inlines = apply_text_color(inlines, &numbered_list_item.color, config);

            // Create a Plain block with the styled inlines (not Para)
            let plain = PandocBlock::Plain(styled_inlines);

            // Create default list attributes (starting at 1)
            let list_attrs = ListAttributes {
                start_number: 1,
                style: ListNumberStyle::Decimal,
                delim: ListNumberDelim::Period,
            };

            // Create an ordered list with a single item
            Some(PandocBlock::OrderedList(list_attrs, vec![vec![plain]]))
        }
        _ => None,
    }
}

/// Convert a Notion to-do item to a Pandoc bullet list item with checkbox
pub fn convert_notion_todo(block: &NotionBlock, config: &ConversionConfig) -> Option<PandocBlock> {
    match &block.block_type {
        BlockType::ToDo { to_do } => {
            // Create vector for inlines
            let mut inlines = Vec::new();

            // Add checkbox character based on status (using Unicode characters)
            let checkbox = if to_do.checked.unwrap_or(false) {
                Inline::Str("☒".to_string()) // Checked box (Unicode)
            } else {
                Inline::Str("☐".to_string()) // Unchecked box (Unicode)
            };
            inlines.push(checkbox);

            // Add space after checkbox
            inlines.push(Inline::Space);

            // Add the rest of the text
            let todo_inlines = NotionTextConverter::convert(&to_do.rich_text);
            inlines.extend(todo_inlines);

            // Apply color formatting if needed
            let styled_inlines = if let Some(color) = &to_do.color {
                apply_text_color(inlines, color, config)
            } else {
                inlines
            };

            // Create a Plain block with the styled inlines (not Para)
            let plain = PandocBlock::Plain(styled_inlines);

            // Create a bullet list with a single item - no special attributes needed
            Some(PandocBlock::BulletList(vec![vec![plain]]))
        }
        _ => None,
    }
}

/// Apply text color to inlines
fn apply_text_color(inlines: Vec<Inline>, color: &TextColor, config: &ConversionConfig) -> Vec<Inline> {
    if inlines.is_empty() {
        return Vec::new();
    }

    // Create attributes based on configuration
    let attr = if config.preserve_attributes {
        Attr {
            identifier: String::new(),
            classes: Vec::new(),
            attributes: vec![("data-color".to_string(), format!("{:?}", color))],
        }
    } else {
        Attr::default()
    };

    // Return a single Span containing all inlines
    vec![Inline::Span(attr, inlines)]
}

/// Convenience function to convert any block to a bulleted list item if applicable
pub fn try_convert_to_bulleted_list(block: &NotionBlock, config: &ConversionConfig) -> Option<PandocBlock> {
    convert_notion_bulleted_list(block, config)
}

/// Convenience function to convert any block to a numbered list item if applicable
pub fn try_convert_to_numbered_list(block: &NotionBlock, config: &ConversionConfig) -> Option<PandocBlock> {
    convert_notion_numbered_list(block, config)
}

/// Convenience function to convert any block to a to-do item if applicable
pub fn try_convert_to_todo(block: &NotionBlock, config: &ConversionConfig) -> Option<PandocBlock> {
    convert_notion_todo(block, config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::n2p::notion_block_visitor::NotionToPandocVisitor;
    use crate::test_utils::pandoc_helpers::test::assert_inlines_text_eq;
    use notion_client::objects::block::{BulletedListItemValue, NumberedListItemValue, ToDoValue};
    use notion_client::objects::rich_text::{RichText, Text};

    // Helper function to create rich text
    fn create_rich_text(content: &str) -> Vec<RichText> {
        vec![RichText::Text {
            text: Text {
                content: content.to_string(),
                link: None,
            },
            annotations: None,
            plain_text: Some(content.to_string()),
            href: None,
        }]
    }

    // Helper function to create a bulleted list item
    fn create_bulleted_list_item(text: &str, children: Option<Vec<NotionBlock>>) -> NotionBlock {
        NotionBlock {
            object: Some("block".to_string()),
            id: Some(format!("bulleted-list-{}", text)),
            parent: None,
            created_time: None,
            created_by: None,
            last_edited_time: None,
            last_edited_by: None,
            archived: Some(false),
            has_children: Some(children.is_some()),
            block_type: BlockType::BulletedListItem {
                bulleted_list_item: BulletedListItemValue {
                    rich_text: create_rich_text(text),
                    color: TextColor::Default,
                    children,
                },
            },
        }
    }

    // Helper function to create a numbered list item
    fn create_numbered_list_item(text: &str, children: Option<Vec<NotionBlock>>) -> NotionBlock {
        NotionBlock {
            object: Some("block".to_string()),
            id: Some(format!("numbered-list-{}", text)),
            parent: None,
            created_time: None,
            created_by: None,
            last_edited_time: None,
            last_edited_by: None,
            archived: Some(false),
            has_children: Some(children.is_some()),
            block_type: BlockType::NumberedListItem {
                numbered_list_item: NumberedListItemValue {
                    rich_text: create_rich_text(text),
                    color: TextColor::Default,
                    children,
                },
            },
        }
    }

    // Helper function to create a to-do list item
    fn create_todo_item(
        text: &str,
        checked: bool,
        children: Option<Vec<NotionBlock>>,
    ) -> NotionBlock {
        NotionBlock {
            object: Some("block".to_string()),
            id: Some(format!("todo-list-{}", text)),
            parent: None,
            created_time: None,
            created_by: None,
            last_edited_time: None,
            last_edited_by: None,
            archived: Some(false),
            has_children: Some(children.is_some()),
            block_type: BlockType::ToDo {
                to_do: ToDoValue {
                    rich_text: create_rich_text(text),
                    checked: Some(checked),
                    color: Some(TextColor::Default),
                    children,
                },
            },
        }
    }

    // Test simple bulleted list
    // Test bulleted list conversion
    #[test]
    fn test_bulleted_list_conversion() {
        let visitor = NotionToPandocVisitor::new();

        let item = create_bulleted_list_item("Bulleted item", None);
        let result = visitor.convert_blocks(&[item]);

        assert_eq!(result.len(), 1);
        if let PandocBlock::BulletList(items) = &result[0] {
            assert_eq!(items.len(), 1);
            if let PandocBlock::Plain(inlines) = &items[0][0] {
                assert_eq!(inlines.len(), 1);
                if let Inline::Span(_, span_inlines) = &inlines[0] {
                    // Verify the text content
                    assert_inlines_text_eq(span_inlines, "Bulleted item");
                } else {
                    panic!("Expected span");
                }
            } else {
                panic!("Expected plain block");
            }
        } else {
            panic!("Expected bullet list");
        }
    }

    // Test simple numbered list
    // Test numbered list conversion
    #[test]
    fn test_numbered_list_conversion() {
        let visitor = NotionToPandocVisitor::new();

        let item = create_numbered_list_item("Numbered item", None);
        let result = visitor.convert_blocks(&[item]);

        assert_eq!(result.len(), 1);
        if let PandocBlock::OrderedList(attrs, items) = &result[0] {
            assert_eq!(attrs.start_number, 1);
            assert_eq!(items.len(), 1);
            if let PandocBlock::Plain(inlines) = &items[0][0] {
                assert_eq!(inlines.len(), 1);
                if let Inline::Span(_, span_inlines) = &inlines[0] {
                    // Verify the text content
                    assert_inlines_text_eq(span_inlines, "Numbered item");
                } else {
                    panic!("Expected span");
                }
            } else {
                panic!("Expected plain block");
            }
        } else {
            panic!("Expected ordered list");
        }
    }

    // Test simple to-do list
    #[test]
    fn test_todo_list_conversion() {
        let visitor = NotionToPandocVisitor::new();

        // Test unchecked item
        let unchecked = create_todo_item("Unchecked task", false, None);
        let result = visitor.convert_blocks(&[unchecked]);

        assert_eq!(result.len(), 1);
        if let PandocBlock::BulletList(items) = &result[0] {
            assert_eq!(items.len(), 1);
            if let PandocBlock::Plain(inlines) = &items[0][0] {
                assert_eq!(inlines.len(), 1);
                if let Inline::Span(_, span_inlines) = &inlines[0] {
                    // Don't strictly check span count as text may be split into multiple pieces
                    assert!(span_inlines.len() >= 3); // At least checkbox, space, and some text
                    if let Inline::Str(checkbox) = &span_inlines[0] {
                        assert_eq!(checkbox, "☐");
                    } else {
                        panic!("Expected checkbox character");
                    }
                } else {
                    panic!("Expected span");
                }
            } else {
                panic!("Expected plain block");
            }
        } else {
            panic!("Expected bullet list");
        }

        // Test checked item
        let checked = create_todo_item("Checked task", true, None);
        let result = visitor.convert_blocks(&[checked]);

        assert_eq!(result.len(), 1);
        if let PandocBlock::BulletList(items) = &result[0] {
            assert_eq!(items.len(), 1);
            if let PandocBlock::Plain(inlines) = &items[0][0] {
                assert_eq!(inlines.len(), 1);
                if let Inline::Span(_, span_inlines) = &inlines[0] {
                    // Don't strictly check span count as text may be split into multiple pieces
                    assert!(span_inlines.len() >= 3); // At least checkbox, space, and some text
                    if let Inline::Str(checkbox) = &span_inlines[0] {
                        assert_eq!(checkbox, "☒");
                    } else {
                        panic!("Expected checkbox character");
                    }
                } else {
                    panic!("Expected span");
                }
            } else {
                panic!("Expected plain block");
            }
        } else {
            panic!("Expected bullet list");
        }
    }

    // Test nested lists
    #[test]
    fn test_nested_list_conversion() {
        let visitor = NotionToPandocVisitor::new();

        // Create a nested structure
        let child1 = create_bulleted_list_item("Child item 1", None);
        let child2 = create_bulleted_list_item("Child item 2", None);
        let parent = create_bulleted_list_item("Parent item", Some(vec![child1, child2]));

        let result = visitor.convert_blocks(&[parent]);

        // Should have 3 blocks: parent + 2 children
        assert_eq!(result.len(), 3);

        // Verify parent is a bullet list
        if let PandocBlock::BulletList(items) = &result[0] {
            assert_eq!(items.len(), 1);
            if let PandocBlock::Plain(inlines) = &items[0][0] {
                assert_eq!(inlines.len(), 1);
                if let Inline::Span(_, span_inlines) = &inlines[0] {
                    // Verify the text content
                    assert_inlines_text_eq(span_inlines, "Parent item");
                } else {
                    panic!("Expected span");
                }
            } else {
                panic!("Expected plain block");
            }
        } else {
            panic!("Expected bullet list");
        }

        // Verify children are also bullet lists
        for i in 1..3 {
            if let PandocBlock::BulletList(items) = &result[i] {
                assert_eq!(items.len(), 1);
                if let PandocBlock::Plain(inlines) = &items[0][0] {
                    assert_eq!(inlines.len(), 1);
                    if let Inline::Span(_, span_inlines) = &inlines[0] {
                        // Verify the text content
                        assert_inlines_text_eq(span_inlines, &format!("Child item {}", i));
                    } else {
                        panic!("Expected span");
                    }
                } else {
                    panic!("Expected plain block");
                }
            } else {
                panic!("Expected bullet list");
            }
        }
    }

    // Test deeply nested lists (3+ levels)
    #[test]
    fn test_deeply_nested_lists() {
        let visitor = NotionToPandocVisitor::new();

        // Create a deeply nested structure (3 levels)
        let grandchild = create_bulleted_list_item("Grandchild item", None);
        let child = create_bulleted_list_item("Child item", Some(vec![grandchild]));
        let parent = create_bulleted_list_item("Parent item", Some(vec![child]));

        // Convert the parent
        let result = visitor.convert_blocks(&[parent]);

        // Should have 3 blocks: parent + child + grandchild
        assert_eq!(result.len(), 3);

        // Verify items at each level
        let expected_texts = ["Parent item", "Child item", "Grandchild item"];

        for (i, expected) in expected_texts.iter().enumerate() {
            if let PandocBlock::BulletList(items) = &result[i] {
                assert_eq!(items.len(), 1);
                if let PandocBlock::Plain(inlines) = &items[0][0] {
                    assert_eq!(inlines.len(), 1);
                    if let Inline::Span(_, span_inlines) = &inlines[0] {
                        // Verify the text content
                        assert_inlines_text_eq(span_inlines, expected);
                    } else {
                        panic!("Expected span");
                    }
                } else {
                    panic!("Expected plain block");
                }
            } else {
                panic!("Expected bullet list");
            }
        }
    }

    // Test mixed list types
    #[test]
    fn test_mixed_list_types() {
        let visitor = NotionToPandocVisitor::new();

        // Create a mix of list types
        let bulleted = create_bulleted_list_item("Bulleted item", None);
        let numbered = create_numbered_list_item("Numbered item", None);
        let todo = create_todo_item("Todo item", false, None);

        let result = visitor.convert_blocks(&[bulleted, numbered, todo]);

        // Should have 3 blocks, one for each list item
        assert_eq!(result.len(), 3);

        // Verify first item is a bullet list
        if let PandocBlock::BulletList(_) = &result[0] {
            // Good
        } else {
            panic!("Expected bullet list");
        }

        // Verify second item is an ordered list
        if let PandocBlock::OrderedList(_, _) = &result[1] {
            // Good
        } else {
            panic!("Expected ordered list");
        }

        // Verify third item is a bullet list (todo is represented as bullet list)
        if let PandocBlock::BulletList(_) = &result[2] {
            // Good
        } else {
            panic!("Expected bullet list for todo");
        }
    }
}
