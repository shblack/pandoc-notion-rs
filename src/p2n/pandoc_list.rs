use crate::p2n::pandoc_text::PandocTextConverter;
use notion_client::objects::block::{
    Block as NotionBlock, BlockType, BulletedListItemValue, NumberedListItemValue, TextColor,
    ToDoValue,
};
use notion_client::objects::rich_text::RichText;
use pandoc_types::definition::{Block as PandocBlock, Inline};
use std::error::Error;

/// Builder for Notion bulleted list item blocks
pub struct NotionBulletedListBuilder {
    rich_text: Vec<RichText>,
    color: Option<TextColor>,
    children: Option<Vec<NotionBlock>>,
}

impl NotionBulletedListBuilder {
    /// Create a new bulleted list builder with default values
    pub fn new() -> Self {
        Self {
            rich_text: Vec::new(),
            color: None,
            children: None,
        }
    }

    /// Set the rich text content for the list item
    pub fn rich_text(mut self, rich_text: Vec<RichText>) -> Self {
        self.rich_text = rich_text;
        self
    }

    /// Add a rich text element to the list item
    pub fn add_rich_text(mut self, text: RichText) -> Self {
        self.rich_text.push(text);
        self
    }

    /// Set the list item color
    pub fn color(mut self, color: TextColor) -> Self {
        self.color = Some(color);
        self
    }

    /// Set the children blocks
    pub fn children(mut self, children: Vec<NotionBlock>) -> Self {
        if !children.is_empty() {
            self.children = Some(children);
        }
        self
    }

    /// Build the Notion bulleted list item block
    pub fn build(self) -> NotionBlock {
        let list_item_value = BulletedListItemValue {
            rich_text: self.rich_text,
            color: self.color.unwrap_or(TextColor::Default),
            children: self.children,
        };

        // No parent_id needed for our use case
        let parent = None;

        // Create has_children flag
        let has_children = if let Some(children) = &list_item_value.children {
            Some(!children.is_empty())
        } else {
            Some(false)
        };

        NotionBlock {
            object: Some("block".to_string()),
            id: None, // Allow Notion API to generate a new UUID
            parent,
            created_time: None,
            created_by: None,
            last_edited_time: None,
            last_edited_by: None,
            archived: Some(false),
            has_children,
            block_type: BlockType::BulletedListItem {
                bulleted_list_item: list_item_value,
            },
        }
    }
}

/// Builder for Notion numbered list item blocks
pub struct NotionNumberedListBuilder {
    rich_text: Vec<RichText>,
    color: Option<TextColor>,
    children: Option<Vec<NotionBlock>>,
}

impl NotionNumberedListBuilder {
    /// Create a new NotionNumberedListBuilder with default values
    pub fn new() -> Self {
        Self {
            rich_text: Vec::new(),
            color: None,
            children: None,
        }
    }

    /// Set the rich text content for the list item
    pub fn rich_text(mut self, rich_text: Vec<RichText>) -> Self {
        self.rich_text = rich_text;
        self
    }

    /// Add a rich text element to the list item
    pub fn add_rich_text(mut self, text: RichText) -> Self {
        self.rich_text.push(text);
        self
    }

    /// Set the list item color
    pub fn color(mut self, color: TextColor) -> Self {
        self.color = Some(color);
        self
    }

    /// Set the children blocks
    pub fn children(mut self, children: Vec<NotionBlock>) -> Self {
        if !children.is_empty() {
            self.children = Some(children);
        }
        self
    }

    /// Build the Notion numbered list item block
    pub fn build(self) -> NotionBlock {
        let list_item_value = NumberedListItemValue {
            rich_text: self.rich_text,
            color: self.color.unwrap_or(TextColor::Default),
            children: self.children,
        };

        // No parent_id needed for our use case
        let parent = None;

        // Create has_children flag
        let has_children = if let Some(children) = &list_item_value.children {
            Some(!children.is_empty())
        } else {
            Some(false)
        };

        NotionBlock {
            object: Some("block".to_string()),
            id: None, // Allow Notion API to generate a new UUID
            parent,
            created_time: None,
            created_by: None,
            last_edited_time: None,
            last_edited_by: None,
            archived: Some(false),
            has_children,
            block_type: BlockType::NumberedListItem {
                numbered_list_item: list_item_value,
            },
        }
    }
}

/// Builder for Notion todo list item blocks
pub struct NotionTodoListBuilder {
    rich_text: Vec<RichText>,
    color: Option<TextColor>,
    checked: Option<bool>,
    children: Option<Vec<NotionBlock>>,
}

impl NotionTodoListBuilder {
    /// Create a new NotionTodoListBuilder with default values
    pub fn new() -> Self {
        Self {
            rich_text: Vec::new(),
            color: None,
            checked: None,
            children: None,
        }
    }

    /// Set the rich text content for the list item
    pub fn rich_text(mut self, rich_text: Vec<RichText>) -> Self {
        self.rich_text = rich_text;
        self
    }

    /// Add a rich text element to the list item
    pub fn add_rich_text(mut self, text: RichText) -> Self {
        self.rich_text.push(text);
        self
    }

    /// Set the list item color
    pub fn color(mut self, color: TextColor) -> Self {
        self.color = Some(color);
        self
    }

    /// Set whether the todo item is checked
    pub fn with_checked(mut self, checked: bool) -> Self {
        self.checked = Some(checked);
        self
    }

    /// Set the children blocks
    pub fn children(mut self, children: Vec<NotionBlock>) -> Self {
        if !children.is_empty() {
            self.children = Some(children);
        }
        self
    }

    /// Build the Notion todo list item block
    pub fn build(self) -> NotionBlock {
        let checked = self.checked.unwrap_or(false);

        let list_item_value = ToDoValue {
            rich_text: self.rich_text,
            color: Some(self.color.unwrap_or(TextColor::Default)),
            checked: Some(checked),
            children: self.children,
        };

        // No parent_id needed for our use case
        let parent = None;

        // Create has_children flag
        let has_children = if let Some(children) = &list_item_value.children {
            Some(!children.is_empty())
        } else {
            Some(false)
        };

        NotionBlock {
            object: Some("block".to_string()),
            id: None, // Allow Notion API to generate a new UUID
            parent,
            created_time: None,
            created_by: None,
            last_edited_time: None,
            last_edited_by: None,
            archived: Some(false),
            has_children,
            block_type: BlockType::ToDo {
                to_do: list_item_value,
            },
        }
    }
}

/// Converter for Pandoc list blocks to Notion list blocks
pub struct PandocListConverter {
    text_converter: PandocTextConverter,
}

impl Default for PandocListConverter {
    fn default() -> Self {
        Self::new()
    }
}

impl PandocListConverter {
    /// Create a new list converter
    pub fn new() -> Self {
        Self {
            text_converter: PandocTextConverter::default(),
        }
    }

    /// Convert a single PandocBlock to a single Notion bulleted list item
    pub fn convert_bullet_list_item(
        &self,
        first_block: &PandocBlock,
    ) -> Result<NotionBlock, Box<dyn Error>> {
        // Extract content to rich text
        let rich_text = match first_block {
            PandocBlock::Plain(inlines) | PandocBlock::Para(inlines) => {
                self.text_converter.convert(inlines)?
            }
            _ => Vec::new(), // Empty if not plain/para
        };

        // Build the list item without children
        let builder = NotionBulletedListBuilder::new().rich_text(rich_text);
        
        Ok(builder.build())
    }

    /// Convert a single PandocBlock to a single Notion numbered list item
    pub fn convert_ordered_list_item(
        &self,
        first_block: &PandocBlock,
        _attrs: &pandoc_types::definition::ListAttributes,
    ) -> Result<NotionBlock, Box<dyn Error>> {
        // Extract content to rich text
        let rich_text = match first_block {
            PandocBlock::Plain(inlines) | PandocBlock::Para(inlines) => {
                self.text_converter.convert(inlines)?
            }
            _ => Vec::new(), // Empty if not plain/para
        };

        // Build the list item without children
        let builder = NotionNumberedListBuilder::new().rich_text(rich_text);
        
        Ok(builder.build())
    }

    /// Checks if a PandocBlock is a Todo list item and converts it if so.
    /// This looks for special characters like ☐ and ☒ at the beginning of text.
    pub fn try_convert_todo_item(
        &self,
        first_block: &PandocBlock,
    ) -> Result<Option<NotionBlock>, Box<dyn Error>> {
        match first_block {
            PandocBlock::Plain(inlines) | PandocBlock::Para(inlines) => {
                if let Some(Inline::Str(checkbox)) = inlines.first() {
                    if checkbox == "☐" || checkbox == "☒" {
                        let checked = checkbox == "☒";
                        
                        // Extract the content after the checkbox
                        let mut content_inlines = inlines.clone();
                        if content_inlines.len() >= 2 {
                            content_inlines.remove(0); // Remove checkbox
                            if let Some(Inline::Space) = content_inlines.first() {
                                content_inlines.remove(0); // Remove space after checkbox
                            }
                        }
                        
                        // Convert remaining content to rich text
                        let rich_text = self.text_converter.convert(&content_inlines)?;
                        
                        // Build the todo item without children
                        let builder = NotionTodoListBuilder::new()
                            .rich_text(rich_text)
                            .with_checked(checked);
                            
                        return Ok(Some(builder.build()));
                    }
                }
            }
            _ => {}
        }
        
        Ok(None)
    }

    /// Process a potential todo item from a sequence of PandocBlocks
    pub fn process_potential_todo_item(
        &self,
        item: &[PandocBlock],
    ) -> Result<Option<NotionBlock>, Box<dyn Error>> {
        // Check if this might be a todo item
        if let Some(first_block) = item.first() {
            return self.try_convert_todo_item(first_block);
        }
        
        Ok(None)
    }

    /// Convert a pandoc block to a Notion list block
    pub fn convert(
        &self,
        block: &PandocBlock,
    ) -> Result<Vec<NotionBlock>, Box<dyn Error>> {
        match block {
            PandocBlock::BulletList(items) => {
                let mut result = Vec::new();
                for item in items {
                    // Check for todo item first
                    if let Some(first_block) = item.first() {
                        // Extract any nested blocks (all blocks after the first)
                        let nested_blocks = self.extract_nested_blocks(item);
                        let mut children = Vec::new();
                        
                        // Process nested blocks recursively
                        for nested_block in nested_blocks {
                            let nested_results = self.convert(nested_block)?;
                            children.extend(nested_results);
                        }
                        
                        // Process the content block
                        if let Some(todo) = self.try_convert_todo_item(first_block)? {
                            // Add children to the todo item
                            let todo_with_children = self.add_children_to_block(todo, children)?;
                            result.push(todo_with_children);
                            continue;
                        }
                        
                        // Convert to bulleted list item
                        let block = self.convert_bullet_list_item(first_block)?;
                        
                        // Add children to the bullet item
                        let block_with_children = self.add_children_to_block(block, children)?;
                        result.push(block_with_children);
                    }
                }
                Ok(result)
            },
            PandocBlock::OrderedList(attrs, items) => {
                let mut result = Vec::new();
                for item in items {
                    // Check for todo item first
                    if let Some(first_block) = item.first() {
                        // Extract any nested blocks (all blocks after the first)
                        let nested_blocks = self.extract_nested_blocks(item);
                        let mut children = Vec::new();
                        
                        // Process nested blocks recursively
                        for nested_block in nested_blocks {
                            let nested_results = self.convert(nested_block)?;
                            children.extend(nested_results);
                        }
                        
                        // Process the content block
                        if let Some(todo) = self.try_convert_todo_item(first_block)? {
                            // Add children to the todo item
                            let todo_with_children = self.add_children_to_block(todo, children)?;
                            result.push(todo_with_children);
                            continue;
                        }
                        
                        // Convert to numbered list item
                        let block = self.convert_ordered_list_item(first_block, attrs)?;
                        
                        // Add children to the numbered item
                        let block_with_children = self.add_children_to_block(block, children)?;
                        result.push(block_with_children);
                    }
                }
                Ok(result)
            },
            _ => Ok(vec![]),
        }
    }



    /// Extract the first block from a list item
    pub fn extract_first_block<'a>(
        &self,
        item: &'a [PandocBlock],
    ) -> Option<&'a PandocBlock> {
        item.first()
    }
    
    /// Extract nested blocks from a list item (all blocks except the first)
    pub fn extract_nested_blocks<'a>(
        &self,
        item: &'a [PandocBlock],
    ) -> Vec<&'a PandocBlock> {
        if item.len() <= 1 {
            return Vec::new();
        }
        
        item[1..].iter().collect()
    }
    
    /// Add children to a Notion block
    pub fn add_children_to_block(
        &self,
        block: NotionBlock,
        children: Vec<NotionBlock>,
    ) -> Result<NotionBlock, Box<dyn Error>> {
        if children.is_empty() {
            return Ok(block);
        }
        
        // Create new block with children based on block type
        match block.block_type {
            BlockType::BulletedListItem { bulleted_list_item } => {
                let mut builder = NotionBulletedListBuilder::new()
                    .rich_text(bulleted_list_item.rich_text)
                    .children(children);
                    
                // Set the color directly
                builder = builder.color(bulleted_list_item.color);
                
                Ok(builder.build())
            },
            BlockType::NumberedListItem { numbered_list_item } => {
                let mut builder = NotionNumberedListBuilder::new()
                    .rich_text(numbered_list_item.rich_text)
                    .children(children);
                    
                // Set the color directly
                builder = builder.color(numbered_list_item.color);
                
                Ok(builder.build())
            },
            BlockType::ToDo { to_do } => {
                let mut builder = NotionTodoListBuilder::new()
                    .rich_text(to_do.rich_text)
                    .with_checked(to_do.checked.unwrap_or(false))
                    .children(children);
                    
                // to_do.color is an Option<TextColor>
                if let Some(color) = to_do.color {
                    builder = builder.color(color);
                }
                
                Ok(builder.build())
            },
            _ => {
                // For other block types, we can't add children
                // Return the original block
                Ok(block)
            }
        }
    }




}

#[cfg(test)]
mod tests {
    use super::*;
    use pandoc_types::definition::{Inline};

    #[test]
    fn test_convert_simple_bullet_list() {
        let converter = PandocListConverter::new();

        // Create a simple bullet list
        let item1 = vec![PandocBlock::Plain(vec![Inline::Str("Item 1".to_string())])];
        let item2 = vec![PandocBlock::Plain(vec![Inline::Str("Item 2".to_string())])];
        let bullet_list = PandocBlock::BulletList(vec![item1, item2]);

        // Convert to Notion bulleted list items
        let result = converter.convert(&bullet_list).unwrap();

        // Verify the result
        assert_eq!(result.len(), 2);

        // Check first item
        match &result[0].block_type {
            BlockType::BulletedListItem { bulleted_list_item } => {
                assert_eq!(bulleted_list_item.rich_text.len(), 1);
                assert_eq!(
                    bulleted_list_item.rich_text[0].plain_text().unwrap(),
                    "Item 1"
                );
                assert_eq!(bulleted_list_item.color, TextColor::Default);
            }
            _ => panic!("Expected BulletedListItem block type"),
        }

        // Check second item
        match &result[1].block_type {
            BlockType::BulletedListItem { bulleted_list_item } => {
                assert_eq!(bulleted_list_item.rich_text.len(), 1);
                assert_eq!(
                    bulleted_list_item.rich_text[0].plain_text().unwrap(),
                    "Item 2"
                );
                assert_eq!(bulleted_list_item.color, TextColor::Default);
            }
            _ => panic!("Expected BulletedListItem block type"),
        }
    }

    #[test]
    fn test_convert_simple_ordered_list() {
        let converter = PandocListConverter::new();

        // Create a simple ordered list
        let attrs = pandoc_types::definition::ListAttributes {
            start_number: 1,
            style: pandoc_types::definition::ListNumberStyle::Decimal,
            delim: pandoc_types::definition::ListNumberDelim::Period,
        };

        let item1 = vec![PandocBlock::Plain(vec![Inline::Str(
            "First item".to_string(),
        )])];
        let item2 = vec![PandocBlock::Plain(vec![Inline::Str(
            "Second item".to_string(),
        )])];
        let ordered_list = PandocBlock::OrderedList(attrs, vec![item1, item2]);

        // Convert to Notion numbered list items
        let result = converter.convert(&ordered_list).unwrap();

        // Verify the result
        assert_eq!(result.len(), 2);

        // Check first item
        match &result[0].block_type {
            BlockType::NumberedListItem { numbered_list_item } => {
                assert_eq!(numbered_list_item.rich_text.len(), 1);
                assert_eq!(
                    numbered_list_item.rich_text[0].plain_text().unwrap(),
                    "First item"
                );
                assert_eq!(numbered_list_item.color, TextColor::Default);
            }
            _ => panic!("Expected NumberedListItem block type"),
        }

        // Check second item
        match &result[1].block_type {
            BlockType::NumberedListItem { numbered_list_item } => {
                assert_eq!(numbered_list_item.rich_text.len(), 1);
                assert_eq!(
                    numbered_list_item.rich_text[0].plain_text().unwrap(),
                    "Second item"
                );
                assert_eq!(numbered_list_item.color, TextColor::Default);
            }
            _ => panic!("Expected NumberedListItem block type"),
        }
    }

    #[test]
    fn test_convert_todo_item() {
        let converter = PandocListConverter::new();

        // Create a todo list item (unchecked)
        let item = vec![PandocBlock::Plain(vec![
            Inline::Str("☐".to_string()),
            Inline::Space,
            Inline::Str("Unchecked task".to_string()),
        ])];

        // Process potential todo item
        let result = converter.process_potential_todo_item(&item).unwrap();

        // Verify the result
        assert!(result.is_some());
        let todo_block = result.unwrap();

        match todo_block.block_type {
            BlockType::ToDo { to_do } => {
                assert_eq!(to_do.rich_text.len(), 1);
                assert_eq!(to_do.rich_text[0].plain_text().unwrap(), "Unchecked task");
                assert_eq!(to_do.checked, Some(false));
            }
            _ => panic!("Expected ToDo block type"),
        }

        // Create a todo list item (checked)
        let item = vec![PandocBlock::Plain(vec![
            Inline::Str("☒".to_string()),
            Inline::Space,
            Inline::Str("Checked task".to_string()),
        ])];

        // Process potential todo item
        let result = converter.process_potential_todo_item(&item).unwrap();

        // Verify the result
        assert!(result.is_some());
        let todo_block = result.unwrap();

        match todo_block.block_type {
            BlockType::ToDo { to_do } => {
                assert_eq!(to_do.rich_text.len(), 1);
                assert_eq!(to_do.rich_text[0].plain_text().unwrap(), "Checked task");
                assert_eq!(to_do.checked, Some(true));
            }
            _ => panic!("Expected ToDo block type"),
        }
    }

    #[test]
    fn test_nested_list() {
        let converter = PandocListConverter::new();

        // Create a bullet list with a nested ordered list
        // Create a nested structure
        let nested_item1 = vec![PandocBlock::Plain(vec![Inline::Str(
            "Nested 1".to_string(),
        )])];
        let nested_item2 = vec![PandocBlock::Plain(vec![Inline::Str(
            "Nested 2".to_string(),
        )])];

        let attrs = pandoc_types::definition::ListAttributes {
            start_number: 1,
            style: pandoc_types::definition::ListNumberStyle::Decimal,
            delim: pandoc_types::definition::ListNumberDelim::Period,
        };

        let nested_list = PandocBlock::OrderedList(attrs, vec![nested_item1, nested_item2]);

        let parent_item = vec![
            PandocBlock::Plain(vec![Inline::Str("Parent item".to_string())]),
            nested_list,
        ];

        let bullet_list = PandocBlock::BulletList(vec![parent_item]);

        // Convert to Notion bulleted list items
        let result = converter.convert(&bullet_list).unwrap();

        // Verify the result
        assert_eq!(result.len(), 1);

        // Check parent item
        match &result[0].block_type {
            BlockType::BulletedListItem { bulleted_list_item } => {
                assert_eq!(bulleted_list_item.rich_text.len(), 1);
                assert_eq!(
                    bulleted_list_item.rich_text[0].plain_text().unwrap(),
                    "Parent item"
                );

                // Check that it has children
                assert!(bulleted_list_item.children.is_some(), "Children is None, expected Some");
                let children = bulleted_list_item.children.as_ref().unwrap();
                assert_eq!(children.len(), 2);

                // Verify first child (should be a numbered list item)
                match &children[0].block_type {
                    BlockType::NumberedListItem { numbered_list_item } => {
                        assert_eq!(numbered_list_item.rich_text.len(), 1);
                        assert_eq!(
                            numbered_list_item.rich_text[0].plain_text().unwrap(),
                            "Nested 1"
                        );
                    }
                    _ => panic!("Expected NumberedListItem block type for first child"),
                }

                // Verify second child
                match &children[1].block_type {
                    BlockType::NumberedListItem { numbered_list_item } => {
                        assert_eq!(numbered_list_item.rich_text.len(), 1);
                        assert_eq!(
                            numbered_list_item.rich_text[0].plain_text().unwrap(),
                            "Nested 2"
                        );
                    }
                    _ => panic!("Expected NumberedListItem block type for second child"),
                }
            }
            _ => panic!("Expected BulletedListItem block type"),
        }
    }
}
