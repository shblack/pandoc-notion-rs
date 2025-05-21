use crate::n2p::notion_text::NotionTextConverter;
use crate::n2p::ConversionConfig;
use notion_client::objects::block::{Block as NotionBlock, BlockType, ParagraphValue, TextColor};
use pandoc_types::definition::{Attr, Block as PandocBlock, Inline};

/// Builder for converting Notion paragraphs to Pandoc paragraphs
pub struct ParagraphBuilder {
    inlines: Vec<Inline>,
}

impl ParagraphBuilder {
    /// Create a new ParagraphBuilder with default values
    pub fn new() -> Self {
        Self {
            inlines: Vec::new(),
        }
    }

    /// Set the inline content for the paragraph
    pub fn inlines(mut self, inlines: Vec<Inline>) -> Self {
        self.inlines = inlines;
        self
    }

    /// Add an inline element to the paragraph
    pub fn add_inline(mut self, inline: Inline) -> Self {
        self.inlines.push(inline);
        self
    }

    /// Build the Pandoc paragraph
    pub fn build(self) -> PandocBlock {
        PandocBlock::Para(self.inlines)
    }
}

/// Convert a Notion paragraph to a Pandoc paragraph
pub fn convert_notion_paragraph(block: &NotionBlock, config: &ConversionConfig) -> Option<PandocBlock> {
    match &block.block_type {
        BlockType::Paragraph { paragraph } => {
            // Only convert the main paragraph without handling children
            let main_para = build_paragraph_from_notion(paragraph, config);

            // Return just the paragraph - children will be handled by visitor
            Some(main_para)
        }
        _ => None,
    }
}
/// Helper function to build a paragraph from Notion paragraph data
fn build_paragraph_from_notion(paragraph: &ParagraphValue, config: &ConversionConfig) -> PandocBlock {
    // Convert rich_text to Pandoc inlines using NotionTextConverter
    let inlines = NotionTextConverter::convert(&paragraph.rich_text);
    // In Pandoc, paragraph styling is handled at the inline level,
    // so we would need to wrap content in Span elements with attributes
    // if we want to preserve Notion's paragraph color
    let styled_inlines = if let Some(color) = &paragraph.color {
        // If color is present, wrap inlines in a Span with appropriate attributes
        handle_paragraph_color(inlines, color, config)
    } else {
        inlines
    };

    ParagraphBuilder::new().inlines(styled_inlines).build()
}

/// Handle paragraph color by wrapping content in an appropriate Span
fn handle_paragraph_color(inlines: Vec<Inline>, color: &TextColor, config: &ConversionConfig) -> Vec<Inline> {
    // If there are no inlines, just return empty vector
    if inlines.is_empty() {
        return Vec::new();
    }

    // Create attributes based on configuration
    let attr = if config.preserve_attributes {
        // Create a Span with color attribute to wrap the inlines
        Attr {
            identifier: String::new(),
            classes: Vec::new(),
            attributes: vec![("data-color".to_string(), format!("{:?}", color))],
        }
    } else {
        // Empty attributes when not preserving
        Attr::default()
    };

    // Return a single Span containing all inlines
    vec![Inline::Span(attr, inlines)]
}

/// Convenience function to directly convert any block to a paragraph if it is one
pub fn try_convert_to_paragraph(block: &NotionBlock, config: &ConversionConfig) -> Option<PandocBlock> {
    convert_notion_paragraph(block, config)
}
