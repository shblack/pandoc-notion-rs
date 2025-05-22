use notion_client::objects::block::{Block as NotionBlock, BlockType, DividerValue};
use pandoc_types::definition::Block as PandocBlock;
use std::error::Error;

/// Builder for Notion divider blocks
pub struct NotionDividerBuilder {}

impl NotionDividerBuilder {
    /// Create a new NotionDividerBuilder
    pub fn new() -> Self {
        Self {}
    }

    /// Build the Notion divider block
    pub fn build(self) -> NotionBlock {
        NotionBlock {
            object: Some("block".to_string()),
            id: None, // Allow Notion API to generate a new UUID
            parent: None, // Parent is set by Notion API, not during conversion
            created_time: None,
            created_by: None,
            last_edited_time: None,
            last_edited_by: None,
            archived: Some(false),
            has_children: Some(false),
            block_type: BlockType::Divider {
                divider: DividerValue {},
            },
        }
    }
}

/// Converter for Pandoc horizontal rule blocks to Notion divider blocks
pub struct PandocDividerConverter {}

impl Default for PandocDividerConverter {
    fn default() -> Self {
        Self::new()
    }
}

impl PandocDividerConverter {
    /// Create a new divider converter
    pub fn new() -> Self {
        Self {}
    }

    /// Convert a Pandoc horizontal rule to a Notion divider
    pub fn convert(
        &self,
        block: &PandocBlock,
        _parent_id: Option<String>, // Kept for API compatibility but not used
    ) -> Result<Option<NotionBlock>, Box<dyn Error>> {
        match block {
            PandocBlock::HorizontalRule => {
                // Dividers are simple, just create one
                let builder = NotionDividerBuilder::new();
                Ok(Some(builder.build()))
            }
            _ => Ok(None),
        }
    }

    /// Try to convert any Pandoc block to a Notion divider
    pub fn try_convert(
        &self,
        block: &PandocBlock,
        _parent_id: Option<String>, // Kept for API compatibility but not used
    ) -> Result<Option<NotionBlock>, Box<dyn Error>> {
        self.convert(block, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_horizontal_rule() {
        let converter = PandocDividerConverter::new();

        // Create a horizontal rule
        let hr = PandocBlock::HorizontalRule;

        // Convert to Notion divider
        let result = converter.convert(&hr, None).unwrap().unwrap();

        // Verify it's a divider
        match result.block_type {
            BlockType::Divider { divider: _ } => {
                // Success - divider has no properties to check
            }
            _ => panic!("Expected Divider block type"),
        }
    }

    #[test]
    fn test_non_divider_block() {
        let converter = PandocDividerConverter::new();

        // Create a non-divider block (using a Null block for simplicity)
        let non_divider = PandocBlock::Null;

        // Try to convert to Notion divider
        let result = converter.convert(&non_divider, None).unwrap();

        // Verify it returns None
        assert!(result.is_none());
    }
}