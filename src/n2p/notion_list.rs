use crate::n2p::notion_text::NotionTextConverter;
use crate::n2p::ConversionConfig;
use notion_client::objects::block::{Block as NotionBlock, BlockType, TextColor};
use pandoc_types::definition::{
    Attr, Block as PandocBlock, Inline, ListAttributes, ListNumberDelim, ListNumberStyle,
};

/// Builder for constructing properly nested list structures in Pandoc format
/// 
/// Handles grouping of consecutive items of the same list type and
/// maintains proper nesting structure according to Pandoc's expectations.
pub struct ListBuilder {
    bullet_items: Vec<Vec<PandocBlock>>,
    ordered_items: Vec<Vec<PandocBlock>>,
    ordered_attrs: Option<ListAttributes>,
    result: Vec<PandocBlock>,
}

impl ListBuilder {
    /// Create a new empty builder
    pub fn new() -> Self {
        Self {
            bullet_items: Vec::new(),
            ordered_items: Vec::new(),
            ordered_attrs: None,
            result: Vec::new(),
        }
    }
    
    /// Add a block to the builder
    pub fn add_block(mut self, block: PandocBlock) -> Self {
        match block {
            PandocBlock::BulletList(items) => self.add_bullet_items(items),
            PandocBlock::OrderedList(attrs, items) => self.add_ordered_items(attrs, items),
            other => {
                self.flush_lists();
                self.result.push(other);
                self
            }
        }
    }
    
    /// Add multiple blocks at once
    pub fn add_blocks(mut self, blocks: Vec<PandocBlock>) -> Self {
        for block in blocks {
            self = self.add_block(block);
        }
        self
    }
    
    /// Add bullet list items
    fn add_bullet_items(mut self, items: Vec<Vec<PandocBlock>>) -> Self {
        // If we have ordered items, flush them first
        if !self.ordered_items.is_empty() {
            self.flush_ordered_list();
        }
        
        // Add the items to our collection
        self.bullet_items.extend(items);
        self
    }
    
    /// Add ordered list items
    fn add_ordered_items(mut self, attrs: ListAttributes, items: Vec<Vec<PandocBlock>>) -> Self {
        // If we have bullet items, flush them first
        if !self.bullet_items.is_empty() {
            self.flush_bullet_list();
        }
        
        // If we already have ordered items with different attributes, flush them
        if let Some(current_attrs) = &self.ordered_attrs {
            if *current_attrs != attrs {
                self.flush_ordered_list();
            }
        }
        
        // Set or update the attributes
        self.ordered_attrs = Some(attrs);
        
        // Add the items to our collection
        self.ordered_items.extend(items);
        self
    }
    
    /// Flush any bullet list items to the result
    fn flush_bullet_list(&mut self) {
        if !self.bullet_items.is_empty() {
            self.result.push(PandocBlock::BulletList(self.bullet_items.clone()));
            self.bullet_items.clear();
        }
    }
    
    /// Flush any ordered list items to the result
    fn flush_ordered_list(&mut self) {
        if !self.ordered_items.is_empty() && self.ordered_attrs.is_some() {
            self.result.push(PandocBlock::OrderedList(
                self.ordered_attrs.clone().unwrap(),
                self.ordered_items.clone()
            ));
            self.ordered_items.clear();
        }
    }
    
    /// Flush all pending lists
    fn flush_lists(&mut self) {
        self.flush_bullet_list();
        self.flush_ordered_list();
    }
    
    /// Build the final list of blocks
    pub fn build(mut self) -> Vec<PandocBlock> {
        // Flush any pending lists
        self.flush_lists();
        self.result
    }
    
    /// Static method to collect and merge top-level lists in a document
    /// 
    /// This is used as a second pass after individual blocks have been converted,
    /// to merge consecutive list blocks of the same type into a single list.
    pub fn collect_document_lists(blocks: Vec<PandocBlock>) -> Vec<PandocBlock> {
        if blocks.is_empty() {
            return Vec::new();
        }
        
        let mut result = Vec::new();
        let mut current_bullet_items: Vec<Vec<PandocBlock>> = Vec::new();
        let mut current_ordered_items: Vec<Vec<PandocBlock>> = Vec::new();
        let mut current_ordered_attrs: Option<ListAttributes> = None;
        
        // Helper function to flush bullet list items
        fn flush_bullet_list(result: &mut Vec<PandocBlock>, items: &mut Vec<Vec<PandocBlock>>) {
            if !items.is_empty() {
                result.push(PandocBlock::BulletList(items.clone()));
                items.clear();
            }
        }
        
        // Helper function to flush ordered list items
        fn flush_ordered_list(result: &mut Vec<PandocBlock>, items: &mut Vec<Vec<PandocBlock>>, attrs: &mut Option<ListAttributes>) {
            if !items.is_empty() && attrs.is_some() {
                // Clone the attributes to avoid ownership issues
                let attrs_clone = attrs.clone().unwrap();
                result.push(PandocBlock::OrderedList(attrs_clone, items.clone()));
                items.clear();
                *attrs = None;
            }
        }
        
        for block in blocks {
            match &block {
                PandocBlock::BulletList(items) => {
                    // If we have ordered items, flush them first
                    flush_ordered_list(&mut result, &mut current_ordered_items, &mut current_ordered_attrs);
                    
                    // Add the new bullet items to our collection
                    if items.len() == 1 {  // Typical output from our list converters
                        current_bullet_items.push(items[0].clone());
                    } else {
                        current_bullet_items.extend(items.clone());
                    }
                },
                PandocBlock::OrderedList(attrs, items) => {
                    // If we have bullet items, flush them first
                    flush_bullet_list(&mut result, &mut current_bullet_items);
                    
                    // If we already have ordered items with different attributes, flush them
                    if let Some(ref current_attrs) = current_ordered_attrs {
                        if current_attrs != attrs {
                            flush_ordered_list(&mut result, &mut current_ordered_items, &mut current_ordered_attrs);
                        }
                    }
                    
                    // Set or update the attributes
                    current_ordered_attrs = Some(attrs.clone());
                    
                    // Add the new ordered items to our collection
                    if items.len() == 1 {  // Typical output from our list converters
                        current_ordered_items.push(items[0].clone());
                    } else {
                        current_ordered_items.extend(items.clone());
                    }
                },
                _ => {
                    // For any other block type, flush existing lists first
                    flush_bullet_list(&mut result, &mut current_bullet_items);
                    flush_ordered_list(&mut result, &mut current_ordered_items, &mut current_ordered_attrs);
                    
                    // Add the block to the result
                    result.push(block);
                }
            }
        }
        
        // Flush any remaining list items
        flush_bullet_list(&mut result, &mut current_bullet_items);
        flush_ordered_list(&mut result, &mut current_ordered_items, &mut current_ordered_attrs);
        
        result
    }
}

/// Convert a Notion bulleted list item to a Pandoc bullet list item
pub fn convert_notion_bulleted_list(
    block: &NotionBlock, 
    config: &ConversionConfig,
    children_blocks: Vec<PandocBlock>
) -> Option<PandocBlock> {
    match &block.block_type {
        BlockType::BulletedListItem { bulleted_list_item } => {
            // Convert rich text to Pandoc inlines
            let inlines = NotionTextConverter::convert(&bulleted_list_item.rich_text);

            // Apply color formatting if needed
            let styled_inlines = apply_text_color(inlines, &bulleted_list_item.color, config);

            // Create a Plain block with the styled inlines (not Para)
            let plain = PandocBlock::Plain(styled_inlines);

            // Create the content of this list item
            let mut item_content = vec![plain];
            
            // Group child blocks by list type using ListBuilder
            let grouped_children = ListBuilder::new()
                .add_blocks(children_blocks)
                .build();
            
            // Add all processed child blocks to the item content
            item_content.extend(grouped_children);

            // Create a bullet list with a single item
            Some(PandocBlock::BulletList(vec![item_content]))
        }
        _ => None,
    }
}

/// Convert a Notion numbered list item to a Pandoc ordered list item
pub fn convert_notion_numbered_list(
    block: &NotionBlock, 
    config: &ConversionConfig,
    children_blocks: Vec<PandocBlock>
) -> Option<PandocBlock> {
    match &block.block_type {
        BlockType::NumberedListItem { numbered_list_item } => {
            // Convert rich text to Pandoc inlines
            let inlines = NotionTextConverter::convert(&numbered_list_item.rich_text);

            // Apply color formatting if needed
            let styled_inlines = apply_text_color(inlines, &numbered_list_item.color, config);

            // Create a Plain block with the styled inlines (not Para)
            let plain = PandocBlock::Plain(styled_inlines);

            // Create the content of this list item
            let mut item_content = vec![plain];
            
            // Group child blocks by list type using ListBuilder
            let grouped_children = ListBuilder::new()
                .add_blocks(children_blocks)
                .build();
            
            // Add all processed child blocks to the item content
            item_content.extend(grouped_children);

            // Create default list attributes (starting at 1)
            let list_attrs = ListAttributes {
                start_number: 1,
                style: ListNumberStyle::Decimal,
                delim: ListNumberDelim::Period,
            };

            // Create an ordered list with a single item
            Some(PandocBlock::OrderedList(list_attrs, vec![item_content]))
        }
        _ => None,
    }
}

/// Convert a Notion to-do item to a Pandoc bullet list item with checkbox
pub fn convert_notion_todo(
    block: &NotionBlock, 
    config: &ConversionConfig,
    children_blocks: Vec<PandocBlock>
) -> Option<PandocBlock> {
    match &block.block_type {
        BlockType::ToDo { to_do } => {
            // Create vector for inlines
            let mut inlines = Vec::new();

            // Add opening bracket for CommonMark X format
            inlines.push(Inline::Str("[".to_string()));
            
            // Add checkbox content (space or x) based on status
            if to_do.checked.unwrap_or(false) {
                inlines.push(Inline::Str("x".to_string())); // Checked with 'x'
            } else {
                inlines.push(Inline::Space); // Unchecked with space
            }
            
            // Add closing bracket
            inlines.push(Inline::Str("]".to_string()));
            
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

            // Create the content of this list item
            let mut item_content = vec![plain];
            
            // Group child blocks by list type using ListBuilder
            let grouped_children = ListBuilder::new()
                .add_blocks(children_blocks)
                .build();
            
            // Add all processed child blocks to the item content
            item_content.extend(grouped_children);

            // Create a bullet list with a single item - no special attributes needed
            Some(PandocBlock::BulletList(vec![item_content]))
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

/// Convenience function to convert any block to a bulleted list if it is one
pub fn try_convert_to_bulleted_list(block: &NotionBlock, config: &ConversionConfig) -> Option<PandocBlock> {
    convert_notion_bulleted_list(block, config, Vec::new())
}

/// Convenience function to convert any block to a numbered list if it is one
pub fn try_convert_to_numbered_list(block: &NotionBlock, config: &ConversionConfig) -> Option<PandocBlock> {
    convert_notion_numbered_list(block, config, Vec::new())
}

/// Convenience function to convert any block to a todo item if it is one
pub fn try_convert_to_todo(block: &NotionBlock, config: &ConversionConfig) -> Option<PandocBlock> {
    convert_notion_todo(block, config, Vec::new())
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
                    assert!(span_inlines.len() >= 4); // At least [, space, ], space, and some text
                    if let Inline::Str(open_bracket) = &span_inlines[0] {
                        assert_eq!(open_bracket, "[");
                        if let Inline::Space = &span_inlines[1] {
                            if let Inline::Str(close_bracket) = &span_inlines[2] {
                                assert_eq!(close_bracket, "]");
                            } else {
                                panic!("Expected closing bracket");
                            }
                        } else {
                            panic!("Expected space in checkbox");
                        }
                    } else {
                        panic!("Expected opening bracket");
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
                    assert!(span_inlines.len() >= 4); // At least [, x, ], space, and some text
                    if let Inline::Str(open_bracket) = &span_inlines[0] {
                        assert_eq!(open_bracket, "[");
                        if let Inline::Str(x) = &span_inlines[1] {
                            assert_eq!(x, "x");
                            if let Inline::Str(close_bracket) = &span_inlines[2] {
                                assert_eq!(close_bracket, "]");
                            } else {
                                panic!("Expected closing bracket");
                            }
                        } else {
                            panic!("Expected 'x' in checkbox");
                        }
                    } else {
                        panic!("Expected opening bracket");
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

        // Should have 1 block: parent with nested children
        assert_eq!(result.len(), 1);

        // Verify parent is a bullet list
        if let PandocBlock::BulletList(items) = &result[0] {
            assert_eq!(items.len(), 1);
            
            // The item should have 2 blocks: Plain text + 1 merged bullet list with 2 items
            assert_eq!(items[0].len(), 2);
            
            // Check the parent text
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
            
            // Verify the merged bullet list containing the two child items
            if let PandocBlock::BulletList(nested_items) = &items[0][1] {
                assert_eq!(nested_items.len(), 2, "Should have 2 nested items");
            
                // Check each child item
                for (i, nested_item) in nested_items.iter().enumerate() {
                    if let PandocBlock::Plain(inlines) = &nested_item[0] {
                        assert_eq!(inlines.len(), 1);
                        if let Inline::Span(_, span_inlines) = &inlines[0] {
                            // Verify the text content (adding 1 to i since our items are 1-indexed)
                            assert_inlines_text_eq(span_inlines, &format!("Child item {}", i+1));
                        } else {
                            panic!("Expected span");
                        }
                    } else {
                        panic!("Expected plain block");
                    }
                }
            } else {
                panic!("Expected bullet list");
            }
        } else {
            panic!("Expected bullet list");
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

        // Should have 1 block: parent with nested children
        assert_eq!(result.len(), 1);

        // Verify the parent list
        if let PandocBlock::BulletList(parent_items) = &result[0] {
            assert_eq!(parent_items.len(), 1);
            assert_eq!(parent_items[0].len(), 2); // Plain text + nested list
            
            // Check parent text
            if let PandocBlock::Plain(inlines) = &parent_items[0][0] {
                assert_eq!(inlines.len(), 1);
                if let Inline::Span(_, span_inlines) = &inlines[0] {
                    assert_inlines_text_eq(span_inlines, "Parent item");
                } else {
                    panic!("Expected span");
                }
            } else {
                panic!("Expected plain block");
            }
            
            // Check the child list
            if let PandocBlock::BulletList(child_items) = &parent_items[0][1] {
                assert_eq!(child_items.len(), 1);
                assert_eq!(child_items[0].len(), 2); // Plain text + nested list
                
                // Check child text
                if let PandocBlock::Plain(inlines) = &child_items[0][0] {
                    assert_eq!(inlines.len(), 1);
                    if let Inline::Span(_, span_inlines) = &inlines[0] {
                        assert_inlines_text_eq(span_inlines, "Child item");
                    } else {
                        panic!("Expected span");
                    }
                } else {
                    panic!("Expected plain block");
                }
                
                // Check the grandchild list
                if let PandocBlock::BulletList(grandchild_items) = &child_items[0][1] {
                    assert_eq!(grandchild_items.len(), 1);
                    assert_eq!(grandchild_items[0].len(), 1); // Just plain text, no more nesting
                    
                    // Check grandchild text
                    if let PandocBlock::Plain(inlines) = &grandchild_items[0][0] {
                        assert_eq!(inlines.len(), 1);
                        if let Inline::Span(_, span_inlines) = &inlines[0] {
                            assert_inlines_text_eq(span_inlines, "Grandchild item");
                        } else {
                            panic!("Expected span");
                        }
                    } else {
                        panic!("Expected plain block");
                    }
                } else {
                    panic!("Expected bullet list for grandchild");
                }
            } else {
                panic!("Expected bullet list for child");
            }
        } else {
            panic!("Expected bullet list for parent");
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
