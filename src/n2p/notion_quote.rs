use crate::n2p::notion_text::NotionTextConverter;
use crate::n2p::ConversionConfig;
use notion_client::objects::block::{Block as NotionBlock, BlockType, QuoteValue, TextColor};
use pandoc_types::definition::{Block as PandocBlock, Inline};

/// Builder for constructing properly nested blockquote structures in Pandoc format
/// 
/// Handles preserving the correct order of content within blockquotes and
/// ensures proper nesting according to Pandoc's expectations.
pub struct QuoteBuilder {
    content: Vec<PandocBlock>,
}

impl QuoteBuilder {
    /// Create a new empty builder
    pub fn new() -> Self {
        Self {
            content: Vec::new(),
        }
    }
    
    /// Add a block to the content of the quote
    pub fn add_content(mut self, block: PandocBlock) -> Self {
        self.content.push(block);
        self
    }
    
    /// Add multiple blocks to the content at once
    pub fn add_content_blocks(mut self, blocks: Vec<PandocBlock>) -> Self {
        self.content.extend(blocks);
        self
    }
    
    /// Build the final blockquote
    pub fn build(self) -> PandocBlock {
        PandocBlock::BlockQuote(self.content)
    }
    
    /// Static method to process a document and ensure proper blockquote nesting
    /// 
    /// This preserves the original order of blocks within quotes, which is
    /// crucial for maintaining nested blockquote structure.
    pub fn process_document_quotes(blocks: Vec<PandocBlock>) -> Vec<PandocBlock> {
        blocks.into_iter()
            .map(|block| Self::process_block(block))
            .collect()
    }
    
    /// Process a single block, recursively handling any nested blockquotes
    fn process_block(block: PandocBlock) -> PandocBlock {
        match block {
            PandocBlock::BlockQuote(content) => {
                // Process the content of this blockquote
                let processed_content = content.into_iter()
                    .map(|inner_block| Self::process_block(inner_block))
                    .collect();
                
                // Create a new blockquote with the processed content
                PandocBlock::BlockQuote(processed_content)
            },
            PandocBlock::Para(inlines) => PandocBlock::Para(inlines),
            PandocBlock::Plain(inlines) => PandocBlock::Plain(inlines),
            PandocBlock::BulletList(items) => {
                let processed_items = items.into_iter()
                    .map(|item_blocks| {
                        item_blocks.into_iter()
                            .map(|item_block| Self::process_block(item_block))
                            .collect()
                    })
                    .collect();
                
                PandocBlock::BulletList(processed_items)
            },
            PandocBlock::OrderedList(attrs, items) => {
                let processed_items = items.into_iter()
                    .map(|item_blocks| {
                        item_blocks.into_iter()
                            .map(|item_block| Self::process_block(item_block))
                            .collect()
                    })
                    .collect();
                
                PandocBlock::OrderedList(attrs, processed_items)
            },
            // For other block types, return as-is
            other => other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pandoc_types::definition::{Inline};
    
    #[test]
    fn test_simple_quote_building() {
        // Create a simple quote with a paragraph
        let text = vec![Inline::Str("Test quote".to_string())];
        let para = PandocBlock::Para(text);
        
        let builder = QuoteBuilder::new().add_content(para);
        let result = builder.build();
        
        // Check the result is a blockquote containing our paragraph
        match result {
            PandocBlock::BlockQuote(content) => {
                assert_eq!(content.len(), 1);
                match &content[0] {
                    PandocBlock::Para(inlines) => {
                        match &inlines[0] {
                            Inline::Str(s) => assert_eq!(s, "Test quote"),
                            _ => panic!("Expected Str inline"),
                        }
                    },
                    _ => panic!("Expected Para block"),
                }
            },
            _ => panic!("Expected BlockQuote"),
        }
    }
    
    #[test]
    fn test_nested_quote_order() {
        // Create a two-level nested quote
        let lvl1_text = vec![Inline::Str("First level".to_string())];
        let lvl1_para = PandocBlock::Para(lvl1_text);
        
        let lvl2_text = vec![Inline::Str("Second level".to_string())];
        let lvl2_para = PandocBlock::Para(lvl2_text);
        let lvl2_quote = PandocBlock::BlockQuote(vec![lvl2_para]);
        
        let final_text = vec![Inline::Str("Back to first".to_string())];
        let final_para = PandocBlock::Para(final_text);
        
        // Build the quote with correct order
        let builder = QuoteBuilder::new()
            .add_content(lvl1_para)
            .add_content(lvl2_quote)
            .add_content(final_para);
        
        let result = builder.build();
        
        // Verify the order is preserved
        match result {
            PandocBlock::BlockQuote(content) => {
                assert_eq!(content.len(), 3);
                
                // First paragraph
                match &content[0] {
                    PandocBlock::Para(inlines) => {
                        match &inlines[0] {
                            Inline::Str(s) => assert_eq!(s, "First level"),
                            _ => panic!("Expected Str inline"),
                        }
                    },
                    _ => panic!("Expected Para block"),
                }
                
                // Nested quote
                match &content[1] {
                    PandocBlock::BlockQuote(_) => {},
                    _ => panic!("Expected BlockQuote"),
                }
                
                // Final paragraph
                match &content[2] {
                    PandocBlock::Para(inlines) => {
                        match &inlines[0] {
                            Inline::Str(s) => assert_eq!(s, "Back to first"),
                            _ => panic!("Expected Str inline"),
                        }
                    },
                    _ => panic!("Expected Para block"),
                }
            },
            _ => panic!("Expected BlockQuote"),
        }
    }
}

/// Convert a Notion quote to a Pandoc block quote
pub fn convert_notion_quote(
    block: &NotionBlock, 
    config: &ConversionConfig,
    children_blocks: Vec<PandocBlock>
) -> Option<PandocBlock> {
    match &block.block_type {
        BlockType::Quote { quote } => {
            // Process nested quotes with special handling to ensure proper nesting
            let main_quote = build_quote_from_notion(quote, config, children_blocks);

            // Return the quote block with its nested children
            Some(main_quote)
        }
        _ => None,
    }
}

/// Helper function to build a block quote from Notion quote data
fn build_quote_from_notion(
    quote: &QuoteValue, 
    config: &ConversionConfig,
    children_blocks: Vec<PandocBlock>
) -> PandocBlock {
    // Convert rich_text to Pandoc inlines
    let inlines = NotionTextConverter::convert(&quote.rich_text);

    // Map Notion color attribute to pandoc Attr
    let processed_inlines = handle_quote_color(inlines, &quote.color, config);

    // Create a paragraph from the processed inlines
    let paragraph = PandocBlock::Para(processed_inlines);

    // Use the quote builder to maintain the original order of content
    // This is crucial for preserving nested structure
    let builder = QuoteBuilder::new()
        .add_content(paragraph)
        .add_content_blocks(children_blocks);
    
    // Build the complete BlockQuote with proper ordering
    builder.build()
}

/// Handle quote color by wrapping content in an appropriate Span
fn handle_quote_color(inlines: Vec<Inline>, color: &TextColor, config: &ConversionConfig) -> Vec<Inline> {
    // If there are no inlines, just return empty vector
    if inlines.is_empty() {
        return Vec::new();
    }

    // Create attributes based on configuration
    let attr = if config.preserve_attributes {
        pandoc_types::definition::Attr {
            identifier: String::new(),
            classes: Vec::new(),
            attributes: vec![("data-color".to_string(), format!("{:?}", color))],
        }
    } else {
        pandoc_types::definition::Attr::default()
    };

    // Return a single Span containing all inlines
    vec![Inline::Span(attr, inlines)]
}

/// Convenience function to directly convert any block to a quote if it is one
pub fn try_convert_to_quote(block: &NotionBlock, config: &ConversionConfig) -> Option<PandocBlock> {
    convert_notion_quote(block, config, Vec::new())
}
