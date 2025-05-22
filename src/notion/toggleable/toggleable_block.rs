//! ToggleableBlock trait definition
//!
//! This file defines a trait for blocks that can be toggled to show/hide children,
//! such as Notion's toggleable headings.

use notion_client::objects::block::{Block as NotionBlock, BlockType};

/// Trait for blocks that can be toggled and contain children
pub trait ToggleableBlock {
    /// Check if the block is toggleable
    fn is_toggleable(&self) -> bool;
    
    /// Get the block ID
    fn block_id(&self) -> Option<&str>;
    
    /// Check if the block has children according to the API
    fn has_children(&self) -> bool;
}

/// Implementation of ToggleableBlock for NotionBlock
impl ToggleableBlock for NotionBlock {
    fn is_toggleable(&self) -> bool {
        match &self.block_type {
            BlockType::Heading1 { heading_1 } => heading_1.is_toggleable.unwrap_or(false),
            BlockType::Heading2 { heading_2 } => heading_2.is_toggleable.unwrap_or(false),
            BlockType::Heading3 { heading_3 } => heading_3.is_toggleable.unwrap_or(false),
            // Toggle blocks are always toggleable (name implies it)
            BlockType::Toggle { .. } => true,
            // All other block types are not toggleable
            _ => false,
        }
    }
    
    fn block_id(&self) -> Option<&str> {
        self.id.as_deref()
    }
    
    fn has_children(&self) -> bool {
        self.has_children.unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use notion_client::objects::block::{HeadingsValue, ToggleValue};
    use notion_client::objects::rich_text::RichText;

    #[test]
    fn test_heading1_toggleable() {
        let block = NotionBlock {
            object: Some("block".to_string()),
            id: Some("test-id".to_string()),
            parent: None,
            has_children: Some(true),
            block_type: BlockType::Heading1 { 
                heading_1: HeadingsValue {
                    rich_text: Vec::<RichText>::new(),
                    color: Some(notion_client::objects::block::TextColor::Default),
                    is_toggleable: Some(true),
                }
            },
            created_time: None,
            created_by: None,
            last_edited_time: None,
            last_edited_by: None,
            archived: None,
        };
        
        assert!(block.is_toggleable());
        assert_eq!(block.block_id(), Some("test-id"));
        assert!(block.has_children());
    }

    #[test]
    fn test_heading1_not_toggleable() {
        let block = NotionBlock {
            object: Some("block".to_string()),
            id: Some("test-id".to_string()),
            parent: None,
            has_children: None,
            block_type: BlockType::Heading1 { 
                heading_1: HeadingsValue {
                    rich_text: Vec::<RichText>::new(),
                    color: Some(notion_client::objects::block::TextColor::Default),
                    is_toggleable: Some(false),
                }
            },
            created_time: None,
            created_by: None,
            last_edited_time: None,
            last_edited_by: None,
            archived: None,
        };
        
        assert!(!block.is_toggleable());
        assert!(!block.has_children());
    }

    #[test]
    fn test_toggle_block() {
        let block = NotionBlock {
            object: Some("block".to_string()),
            id: Some("test-id".to_string()),
            parent: None,
            has_children: Some(true),
            block_type: BlockType::Toggle { 
                toggle: ToggleValue {
                    rich_text: Vec::<RichText>::new(),
                    color: notion_client::objects::block::TextColor::Default,
                    children: None,
                }
            },
            created_time: None,
            created_by: None,
            last_edited_time: None,
            last_edited_by: None,
            archived: None,
        };
        
        assert!(block.is_toggleable());
        assert!(block.has_children());
    }
}