use crate::n2p::notion_text::NotionTextConverter;
use notion_client::objects::block::{Block as NotionBlock, BlockType, QuoteValue, TextColor};
use pandoc_types::definition::{Block as PandocBlock, Inline};

/// Builder for converting Notion quotes to Pandoc block quotes
pub struct QuoteBuilder {
    content: Vec<PandocBlock>,
}

impl QuoteBuilder {
    /// Create a new QuoteBuilder with default values
    pub fn new() -> Self {
        Self {
            content: Vec::new(),
        }
    }

    /// Set the blocks to be included in the quote
    pub fn content(mut self, blocks: Vec<PandocBlock>) -> Self {
        self.content = blocks;
        self
    }

    /// Add a block to the quote content
    pub fn add_block(mut self, block: PandocBlock) -> Self {
        self.content.push(block);
        self
    }

    /// Build the Pandoc BlockQuote
    pub fn build(self) -> PandocBlock {
        PandocBlock::BlockQuote(self.content)
    }
}

/// Convert a Notion quote to a Pandoc block quote
pub fn convert_notion_quote(block: &NotionBlock) -> Option<Vec<PandocBlock>> {
    match &block.block_type {
        BlockType::Quote { quote } => {
            let main_quote = build_quote_from_notion(quote);

            // We only return the main quote block here
            // Children will be processed by the visitor separately
            Some(vec![main_quote])
        }
        _ => None,
    }
}

/// Helper function to build a block quote from Notion quote data
fn build_quote_from_notion(quote: &QuoteValue) -> PandocBlock {
    // Convert rich_text to Pandoc inlines
    let inlines = NotionTextConverter::convert(&quote.rich_text);

    // Map Notion color attribute to pandoc Attr
    let processed_inlines = handle_quote_color(inlines, &quote.color);

    // Create a paragraph from the processed inlines
    let paragraph = PandocBlock::Para(processed_inlines);

    // Wrap the paragraph in a BlockQuote
    QuoteBuilder::new().add_block(paragraph).build()

    // children: Option<Vec<Block>> are handled by visitor
}

/// Handle quote color by wrapping content in an appropriate Span
fn handle_quote_color(inlines: Vec<Inline>, color: &TextColor) -> Vec<Inline> {
    // If there are no inlines, just return empty vector
    if inlines.is_empty() {
        return Vec::new();
    }

    // Create attributes for the color
    let attr = pandoc_types::definition::Attr {
        identifier: String::new(),
        classes: Vec::new(),
        attributes: vec![("data-color".to_string(), format!("{:?}", color))],
    };

    // Return a single Span containing all inlines
    vec![Inline::Span(attr, inlines)]
}

/// Convenience function to directly convert any block to a quote if it is one
pub fn try_convert_to_quote(block: &NotionBlock) -> Option<PandocBlock> {
    convert_notion_quote(block).map(|blocks| blocks[0].clone())
}
