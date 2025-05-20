use crate::n2p::block_converter::convert_blocks;
use crate::n2p::notion_text::NotionTextConverter;
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
pub fn convert_notion_paragraph(block: &NotionBlock) -> Option<Vec<PandocBlock>> {
    // Create text converter
    let text_converter = NotionTextConverter;

    match &block.block_type {
        BlockType::Paragraph { paragraph } => {
            let mut result = Vec::new();

            // Convert the main paragraph
            let main_para = build_paragraph_from_notion(paragraph, &text_converter);
            result.push(main_para);

            // Handle any children (nested blocks)
            if let Some(children) = &paragraph.children {
                if !children.is_empty() {
                    // Convert children to Pandoc blocks and add them after the paragraph
                    let child_blocks = convert_blocks(children);
                    result.extend(child_blocks);
                }
            }

            Some(result)
        }
        _ => None,
    }
}

/// Helper function to build a paragraph from Notion paragraph data
fn build_paragraph_from_notion(
    paragraph: &ParagraphValue,
    text_converter: &NotionTextConverter,
) -> PandocBlock {
    // Convert rich_text to Pandoc inlines using NotionTextConverter
    let inlines = text_converter.convert(&paragraph.rich_text);

    // In Pandoc, paragraph styling is handled at the inline level,
    // so we would need to wrap content in Span elements with attributes
    // if we want to preserve Notion's paragraph color
    let styled_inlines = if let Some(color) = &paragraph.color {
        // If color is present, wrap inlines in a Span with appropriate attributes
        handle_paragraph_color(inlines, color)
    } else {
        inlines
    };

    ParagraphBuilder::new().inlines(styled_inlines).build()
}

/// Handle paragraph color by wrapping content in an appropriate Span
fn handle_paragraph_color(inlines: Vec<Inline>, color: &TextColor) -> Vec<Inline> {
    // If there are no inlines, just return empty vector
    if inlines.is_empty() {
        return Vec::new();
    }

    // Create a Span with color attribute to wrap the inlines
    let attr = Attr {
        identifier: String::new(),
        classes: Vec::new(),
        attributes: vec![("data-color".to_string(), format!("{:?}", color))],
    };

    // Return a single Span containing all inlines
    vec![Inline::Span(attr, inlines)]
}

/// Convenience function to directly convert any block to a paragraph if it is one
pub fn try_convert_to_paragraph(block: &NotionBlock) -> Option<Vec<PandocBlock>> {
    convert_notion_paragraph(block)
}
