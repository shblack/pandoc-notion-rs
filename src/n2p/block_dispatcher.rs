// src/n2p/block_dispatcher.rs
use notion_client::objects::block::{Block as NotionBlock, BlockType};
use pandoc_types::definition::Block as PandocBlock;
use std::collections::HashMap;
use std::sync::Arc;

use crate::n2p::notion_heading::convert_notion_heading;
use crate::n2p::notion_paragraph::convert_notion_paragraph;
// Import other converters as needed

/// Type for conversion functions
type ConversionFn = Arc<dyn Fn(&NotionBlock) -> Option<Vec<PandocBlock>> + Send + Sync>;

/// Dispatcher for converting Notion blocks to Pandoc blocks
pub struct NotionToPandocDispatcher {
    converters: HashMap<&'static str, ConversionFn>,
}

impl NotionToPandocDispatcher {
    /// Create a new dispatcher with all registered converters
    pub fn new() -> Self {
        let mut dispatcher = Self {
            converters: HashMap::new(),
        };

        // Register all converters
        dispatcher.register_default_converters();

        dispatcher
    }

    /// Register the default set of converters
    fn register_default_converters(&mut self) {
        // Register heading converter
        self.register_converter(
            "heading_1",
            Arc::new(|block| convert_notion_heading(block).map(|b| vec![b])),
        );

        self.register_converter(
            "heading_2",
            Arc::new(|block| convert_notion_heading(block).map(|b| vec![b])),
        );

        self.register_converter(
            "heading_3",
            Arc::new(|block| convert_notion_heading(block).map(|b| vec![b])),
        );

        // Register paragraph converter
        self.register_converter(
            "paragraph",
            Arc::new(|block| convert_notion_paragraph(block)),
        );

        // Register other converters here
        // self.register_converter("bulleted_list_item", Arc::new(|block| { ... }));
        // etc.
    }

    /// Register a custom converter for a block type
    pub fn register_converter(&mut self, block_type: &'static str, converter: ConversionFn) {
        self.converters.insert(block_type, converter);
    }

    /// Convert a Notion block to Pandoc blocks
    pub fn convert(&self, block: &NotionBlock) -> Vec<PandocBlock> {
        let block_type = self.get_block_type(block);

        if let Some(converter) = self.converters.get(block_type) {
            if let Some(blocks) = converter(block) {
                return blocks;
            }
        }

        // If no converter found or conversion failed, return empty vector
        Vec::new()
    }

    /// Get the string representation of the block type
    fn get_block_type(&self, block: &NotionBlock) -> &'static str {
        match &block.block_type {
            BlockType::Heading1 { .. } => "heading_1",
            BlockType::Heading2 { .. } => "heading_2",
            BlockType::Heading3 { .. } => "heading_3",
            BlockType::Paragraph { .. } => "paragraph",
            BlockType::BulletedListItem { .. } => "bulleted_list_item",
            BlockType::NumberedListItem { .. } => "numbered_list_item",
            BlockType::ToDo { .. } => "to_do",
            BlockType::Toggle { .. } => "toggle",
            BlockType::Code { .. } => "code",
            BlockType::Quote { .. } => "quote",
            BlockType::Callout { .. } => "callout",
            BlockType::Divider { .. } => "divider",
            BlockType::Image { .. } => "image",
            BlockType::Video { .. } => "video",
            BlockType::File { .. } => "file",
            BlockType::Bookmark { .. } => "bookmark",
            BlockType::Equation { .. } => "equation",
            // Add other block types as needed
            _ => "unsupported",
        }
    }

    /// Convert multiple Notion blocks to Pandoc blocks
    pub fn convert_blocks(&self, blocks: &[NotionBlock]) -> Vec<PandocBlock> {
        blocks
            .iter()
            .flat_map(|block| self.convert(block))
            .collect()
    }
}

/// Convenience function for one-off block conversions
pub fn convert_block(block: &NotionBlock) -> Vec<PandocBlock> {
    NotionToPandocDispatcher::new().convert(block)
}

/// Convenience function for converting multiple blocks
pub fn convert_blocks(blocks: &[NotionBlock]) -> Vec<PandocBlock> {
    NotionToPandocDispatcher::new().convert_blocks(blocks)
}
