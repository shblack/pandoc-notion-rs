use notion_client::objects::block::{Block as NotionBlock, BlockType, QuoteValue, TextColor};
use notion_client::objects::rich_text::RichText;
use pandoc_types::definition::{Block as PandocBlock};
use std::error::Error;

use crate::p2n::pandoc_text::PandocTextConverter;

/// Builder for constructing Notion quote blocks from Pandoc BlockQuote
pub struct NotionQuoteBuilder {
    rich_text: Vec<RichText>,
    color: TextColor,
    children: Option<Vec<NotionBlock>>,
}

impl NotionQuoteBuilder {
    /// Create a new NotionQuoteBuilder with default values
    pub fn new() -> Self {
        Self {
            rich_text: Vec::new(),
            color: TextColor::Default,
            children: None,
        }
    }

    /// Set the rich text content
    pub fn rich_text(mut self, rich_text: Vec<RichText>) -> Self {
        self.rich_text = rich_text;
        self
    }

    /// Add a rich text element to the quote
    pub fn add_rich_text(mut self, text: RichText) -> Self {
        self.rich_text.push(text);
        self
    }

    /// Set the color
    pub fn color(mut self, color: TextColor) -> Self {
        self.color = color;
        self
    }

    /// Set children blocks
    pub fn children(mut self, children: Vec<NotionBlock>) -> Self {
        if !children.is_empty() {
            self.children = Some(children);
        }
        self
    }

    /// Build the Notion quote block
    pub fn build(self) -> NotionBlock {
        let quote_value = QuoteValue {
            rich_text: self.rich_text,
            color: self.color,
            children: self.children,
        };

        // Parent is set by Notion API, not during conversion
        let parent = None;

        // Create has_children flag
        let has_children = if let Some(children) = &quote_value.children {
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
            block_type: BlockType::Quote {
                quote: quote_value,
            },
        }
    }
}

/// Converter for transforming Pandoc block quotes to Notion quote blocks
pub struct PandocQuoteConverter {
    text_converter: PandocTextConverter,
}

impl Default for PandocQuoteConverter {
    fn default() -> Self {
        Self::new()
    }
}

impl PandocQuoteConverter {
    /// Create a new PandocQuoteConverter
    pub fn new() -> Self {
        Self {
            text_converter: PandocTextConverter::default(),
        }
    }

    /// Convert a Pandoc BlockQuote to a Notion quote block
    pub fn convert(&self, blocks: &[PandocBlock]) -> Result<NotionBlock, Box<dyn Error>> {
        if blocks.is_empty() {
            // If blocks are empty, return an empty quote
            return Ok(NotionQuoteBuilder::new().build());
        }

        // Extract text content from the first paragraph in the block quote
        let mut builder = NotionQuoteBuilder::new();
        let mut children = Vec::new();
        
        // Process the first block for the quote content
        match blocks.first() {
            Some(PandocBlock::Para(inlines)) => {
                // Convert paragraph inlines to rich text for the quote
                let rich_text = self.text_converter.convert(inlines)?;
                
                // Extract color if present
                let color = self.extract_color_from_inlines(inlines);
                
                builder = builder.rich_text(rich_text);
                builder = builder.color(color);
            },
            Some(PandocBlock::Plain(inlines)) => {
                // Convert plain inlines to rich text for the quote
                let rich_text = self.text_converter.convert(inlines)?;
                builder = builder.rich_text(rich_text);
            },
            Some(_) => {
                // For other block types, use default builder with empty content
            },
            None => {},
        }
        
        // Process remaining blocks as children
        if blocks.len() > 1 {
            // Process each block after the first one as a child
            for child_block in &blocks[1..] {
                match child_block {
                    // For nested BlockQuotes, process them recursively
                    PandocBlock::BlockQuote(nested_blocks) => {
                        let nested_quote = self.convert(nested_blocks)?;
                        children.push(nested_quote);
                    },
                    // For other block types, they need to be processed by the visitor
                    // but since we don't have access to it here, we'll create simple text blocks
                    PandocBlock::Para(inlines) | PandocBlock::Plain(inlines) => {
                        let rich_text = self.text_converter.convert(inlines)?;
                        let paragraph_block = crate::p2n::pandoc_paragraph::NotionParagraphBuilder::new()
                            .rich_text(rich_text)
                            .build();
                        children.push(paragraph_block);
                    },
                    // Other block types would need similar handling
                    _ => {
                        // Add a placeholder for unsupported types
                        let unsupported_msg = format!("Unsupported block in quote: {:?}", child_block);
                        let text = notion_client::objects::rich_text::RichText::Text {
                            text: notion_client::objects::rich_text::Text {
                                content: unsupported_msg,
                                link: None,
                            },
                            annotations: None,
                            plain_text: None,
                            href: None,
                        };
                        let paragraph_block = crate::p2n::pandoc_paragraph::NotionParagraphBuilder::new()
                            .add_rich_text(text)
                            .build();
                        children.push(paragraph_block);
                    }
                }
            }
        }
        
        // Add all collected children to the builder
        if !children.is_empty() {
            builder = builder.children(children);
        }
        
        Ok(builder.build())
    }
    
    /// Extract color from Pandoc inlines if wrapped in a Span with color attributes
    fn extract_color_from_inlines(&self, inlines: &[pandoc_types::definition::Inline]) -> TextColor {
        // Check if the paragraph content is wrapped in a single Span with color attributes
        if inlines.len() == 1 {
            if let pandoc_types::definition::Inline::Span(attr, _) = &inlines[0] {
                return self.extract_color_from_attr(attr);
            }
        }
        TextColor::Default
    }

    /// Extract color from Pandoc attributes
    fn extract_color_from_attr(&self, attr: &pandoc_types::definition::Attr) -> TextColor {
        for (key, value) in &attr.attributes {
            if key == "data-color" || key == "color" {
                return match value.to_lowercase().as_str() {
                    "blue" => TextColor::Blue,
                    "brown" => TextColor::Brown,
                    "default" => TextColor::Default,
                    "gray" => TextColor::Gray,
                    "green" => TextColor::Green,
                    "orange" => TextColor::Orange,
                    "pink" => TextColor::Pink,
                    "purple" => TextColor::Purple,
                    "red" => TextColor::Red,
                    "yellow" => TextColor::Yellow,
                    "blue_background" | "bluebackground" => TextColor::BlueBackground,
                    "brown_background" | "brownbackground" => TextColor::BrownBackground,
                    "gray_background" | "graybackground" => TextColor::GrayBackground,
                    "green_background" | "greenbackground" => TextColor::GreenBackground,
                    "orange_background" | "orangebackground" => TextColor::OrangeBackground,
                    "pink_background" | "pinkbackground" => TextColor::PinkBackground,
                    "purple_background" | "purplebackground" => TextColor::PurpleBackground,
                    "red_background" | "redbackground" => TextColor::RedBackground,
                    "yellow_background" | "yellowbackground" => TextColor::YellowBackground,
                    _ => TextColor::Default,
                };
            }
        }
        TextColor::Default
    }

    /// Try to convert any Pandoc block to a Notion quote if it's a BlockQuote
    pub fn try_convert(
        &self,
        block: &PandocBlock,
        _parent_id: Option<String>, // Kept for API compatibility but not used
    ) -> Result<Option<NotionBlock>, Box<dyn Error>> {
        match block {
            PandocBlock::BlockQuote(blocks) => {
                let result = self.convert(blocks)?;
                Ok(Some(result))
            },
            _ => Ok(None),
        }
    }

    /// Recursive helper to process nested blockquotes with proper nesting
    pub fn process_nested_quote(&self, blocks: &[PandocBlock]) -> Result<NotionBlock, Box<dyn Error>> {
        // This is just a wrapper for convert to make the recursive intent clearer
        self.convert(blocks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pandoc_types::definition::Inline;
    
    #[test]
    fn test_convert_simple_quote() {
        let quote_converter = PandocQuoteConverter::new();
        
        // Create a simple BlockQuote with a single paragraph
        let inlines = vec![
            Inline::Str("This is a test quote".to_string())
        ];
        let para = PandocBlock::Para(inlines);
        let block_quote = vec![para];
        
        // Convert to Notion
        let result = quote_converter.convert(&block_quote).unwrap();
        
        // Verify the result
        match &result.block_type {
            BlockType::Quote { quote } => {
                assert_eq!(quote.rich_text.len(), 1);
                assert_eq!(quote.rich_text[0].plain_text().unwrap(), "This is a test quote");
                assert_eq!(quote.color, TextColor::Default);
                assert!(quote.children.is_none());
            },
            _ => panic!("Expected a Quote block")
        }
    }
    
    #[test]
    fn test_convert_colored_quote() {
        let quote_converter = PandocQuoteConverter::new();
        
        // Create a colored quote (using Span wrapper)
        let mut attr = pandoc_types::definition::Attr::default();
        attr.attributes.push(("data-color".to_string(), "blue".to_string()));
        
        let inner_content = vec![Inline::Str("This is a blue quote".to_string())];
        let span = Inline::Span(attr, inner_content);
        let para = PandocBlock::Para(vec![span]);
        let block_quote = vec![para];
        
        // Convert to Notion
        let result = quote_converter.convert(&block_quote).unwrap();
        
        // Verify it has the correct color
        match &result.block_type {
            BlockType::Quote { quote } => {
                assert_eq!(quote.rich_text.len(), 1);
                assert_eq!(quote.rich_text[0].plain_text().unwrap(), "This is a blue quote");
                assert_eq!(quote.color, TextColor::Blue);
            },
            _ => panic!("Expected a Quote block")
        }
    }
    
    #[test]
    fn test_try_convert() {
        let quote_converter = PandocQuoteConverter::new();
        
        // Create a BlockQuote
        let inlines = vec![Inline::Str("Quote content".to_string())];
        let para = PandocBlock::Para(inlines);
        let block = PandocBlock::BlockQuote(vec![para]);
        
        // Try to convert it
        let result = quote_converter.try_convert(&block, None).unwrap();
        
        // Should return Some(NotionBlock)
        assert!(result.is_some());
        
        // Create a non-BlockQuote block
        let non_quote_block = PandocBlock::Para(vec![Inline::Str("Not a quote".to_string())]);
        
        // Try to convert it
        let result = quote_converter.try_convert(&non_quote_block, None).unwrap();
        
        // Should return None
        assert!(result.is_none());
    }
}