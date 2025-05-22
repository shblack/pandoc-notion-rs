use crate::n2p::ConversionConfig;
use notion_client::objects::block::{Block as NotionBlock, BlockType};
use pandoc_types::definition::{Block as PandocBlock};

/// Convert a Notion divider to a Pandoc horizontal rule
pub fn convert_notion_divider(block: &NotionBlock, _config: &ConversionConfig) -> Option<PandocBlock> {
    match &block.block_type {
        BlockType::Divider { divider: _ } => {
            // Dividers don't have any configurable properties in Notion
            // so we just create a simple horizontal rule
            Some(PandocBlock::HorizontalRule)
        }
        _ => None,
    }
}

/// Convenience function to directly convert any block to a divider if it is one
pub fn try_convert_to_divider(block: &NotionBlock, config: &ConversionConfig) -> Option<PandocBlock> {
    convert_notion_divider(block, config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use notion_client::objects::block::DividerValue;

    #[test]
    fn test_convert_divider() {
        // Create a test divider block
        let block = create_divider_block();
        let config = ConversionConfig::default();

        // Convert it
        let result = convert_notion_divider(&block, &config);

        // Verify it's a horizontal rule
        assert!(result.is_some());
        match result.unwrap() {
            PandocBlock::HorizontalRule => (), // Success
            other => panic!("Expected HorizontalRule, got {:?}", other),
        }
    }

    fn create_divider_block() -> NotionBlock {
        NotionBlock {
            object: Some("block".to_string()),
            id: Some("test_divider_id".to_string()),
            parent: None,
            created_time: None,
            last_edited_time: None,
            created_by: None,
            last_edited_by: None,
            has_children: Some(false),
            archived: Some(false),
            block_type: BlockType::Divider { divider: DividerValue {} },
        }
    }
}