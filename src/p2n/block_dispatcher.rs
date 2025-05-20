// src/p2n/block_dispatcher.rs
use notion_client::objects::block::Block as NotionBlock;
use pandoc_types::definition::Block as PandocBlock;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

use crate::p2n::pandoc_heading::PandocHeadingConverter;
use crate::p2n::pandoc_paragraph::PandocParagraphConverter;
// Import other converters as needed

/// Type for conversion functions
type ConversionFn = Arc<
    dyn Fn(&PandocBlock, Option<String>) -> Result<Option<NotionBlock>, Box<dyn Error>>
        + Send
        + Sync,
>;

/// Dispatcher for converting Pandoc blocks to Notion blocks
pub struct PandocToNotionDispatcher {
    converters: HashMap<&'static str, ConversionFn>,
    heading_converter: PandocHeadingConverter,
    paragraph_converter: PandocParagraphConverter,
    // Add other converters as needed
}

impl PandocToNotionDispatcher {
    /// Create a new dispatcher with all registered converters
    pub fn new() -> Self {
        let mut dispatcher = Self {
            converters: HashMap::new(),
            heading_converter: PandocHeadingConverter::new(),
            paragraph_converter: PandocParagraphConverter::new(),
            // Initialize other converters
        };

        // Register all converters
        dispatcher.register_default_converters();

        dispatcher
    }

    /// Register the default set of converters
    fn register_default_converters(&mut self) {
        let heading_converter = self.heading_converter.clone();
        let paragraph_converter = self.paragraph_converter.clone();

        // Register Header converter
        self.register_converter(
            "Header",
            Arc::new(move |block, parent_id| heading_converter.convert(block, parent_id)),
        );

        // Register Para converter
        self.register_converter(
            "Para",
            Arc::new(move |block, parent_id| paragraph_converter.convert(block, parent_id)),
        );

        // Register other converters here
    }

    /// Register a custom converter for a block type
    pub fn register_converter(&mut self, block_type: &'static str, converter: ConversionFn) {
        self.converters.insert(block_type, converter);
    }

    /// Convert a Pandoc block to a Notion block
    pub fn convert(
        &self,
        block: &PandocBlock,
        parent_id: Option<String>,
    ) -> Result<Option<NotionBlock>, Box<dyn Error>> {
        let block_type = self.get_block_type(block);

        if let Some(converter) = self.converters.get(block_type) {
            return converter(block, parent_id);
        }

        // If no converter found, return None
        Ok(None)
    }

    /// Get the string representation of the block type
    fn get_block_type(&self, block: &PandocBlock) -> &'static str {
        match block {
            PandocBlock::Header(_, _, _) => "Header",
            PandocBlock::Para(_) => "Para",
            PandocBlock::Plain(_) => "Plain",
            PandocBlock::LineBlock(_) => "LineBlock",
            PandocBlock::CodeBlock(_, _) => "CodeBlock",
            PandocBlock::RawBlock(_, _) => "RawBlock",
            PandocBlock::BlockQuote(_) => "BlockQuote",
            PandocBlock::OrderedList(_, _) => "OrderedList",
            PandocBlock::BulletList(_) => "BulletList",
            PandocBlock::DefinitionList(_) => "DefinitionList",
            PandocBlock::HorizontalRule => "HorizontalRule",
            PandocBlock::Table(_) => "Table",
            PandocBlock::Figure(_, _, _) => "Figure",
            PandocBlock::Div(_, _) => "Div",
            PandocBlock::Null => "Null",
        }
    }

    /// Convert multiple Pandoc blocks to Notion blocks
    pub fn convert_blocks(
        &self,
        blocks: &[PandocBlock],
        parent_id: Option<String>,
    ) -> Result<Vec<NotionBlock>, Box<dyn Error>> {
        let mut result = Vec::new();

        for block in blocks {
            if let Some(notion_block) = self.convert(block, parent_id.clone())? {
                result.push(notion_block);
            }
        }

        Ok(result)
    }
}

impl Clone for PandocToNotionDispatcher {
    fn clone(&self) -> Self {
        Self {
            converters: self.converters.clone(),
            heading_converter: self.heading_converter.clone(),
            paragraph_converter: self.paragraph_converter.clone(),
            // Clone other converters
        }
    }
}

// Implement Clone for the converters
impl Clone for PandocHeadingConverter {
    fn clone(&self) -> Self {
        Self::new()
    }
}

impl Clone for PandocParagraphConverter {
    fn clone(&self) -> Self {
        Self::new()
    }
}

/// Convenience function for one-off block conversions
pub fn convert_block(
    block: &PandocBlock,
    parent_id: Option<String>,
) -> Result<Option<NotionBlock>, Box<dyn Error>> {
    PandocToNotionDispatcher::new().convert(block, parent_id)
}

/// Convenience function for converting multiple blocks
pub fn convert_blocks(
    blocks: &[PandocBlock],
    parent_id: Option<String>,
) -> Result<Vec<NotionBlock>, Box<dyn Error>> {
    PandocToNotionDispatcher::new().convert_blocks(blocks, parent_id)
}
