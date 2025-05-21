use crate::p2n::pandoc_text::PandocTextConverter;
use notion_client::objects::block::{Block as NotionBlock, BlockType, ParagraphValue, TextColor};
use notion_client::objects::rich_text::RichText;
use pandoc_types::definition::{Attr, Block as PandocBlock, Inline};
use std::error::Error;

/// Builder for Notion paragraph blocks
pub struct NotionParagraphBuilder {
    rich_text: Vec<RichText>,
    color: Option<TextColor>,
    children: Option<Vec<NotionBlock>>,
    parent_id: Option<String>,
}

impl NotionParagraphBuilder {
    /// Create a new NotionParagraphBuilder with default values
    pub fn new() -> Self {
        Self {
            rich_text: Vec::new(),
            color: None,
            children: None,
            parent_id: None,
        }
    }

    /// Set the rich text content for the paragraph
    pub fn rich_text(mut self, rich_text: Vec<RichText>) -> Self {
        self.rich_text = rich_text;
        self
    }

    /// Add a rich text element to the paragraph
    pub fn add_rich_text(mut self, text: RichText) -> Self {
        self.rich_text.push(text);
        self
    }

    /// Set the paragraph color
    pub fn color(mut self, color: TextColor) -> Self {
        self.color = Some(color);
        self
    }

    /// Set the children blocks
    pub fn children(mut self, children: Vec<NotionBlock>) -> Self {
        if !children.is_empty() {
            self.children = Some(children);
        }
        self
    }

    /// Set the parent ID for the block
    pub fn parent_id(mut self, id: String) -> Self {
        self.parent_id = Some(id);
        self
    }

    /// Build the Notion paragraph block
    pub fn build(self) -> NotionBlock {
        let paragraph_value = ParagraphValue {
            rich_text: self.rich_text,
            color: self.color,
            children: self.children,
        };

        // Create parent if specified
        let parent = self.parent_id.map(|id| {
            use notion_client::objects::parent::Parent;
            Parent::PageId { page_id: id }
        });

        // Create has_children flag
        let has_children = if let Some(children) = &paragraph_value.children {
            Some(!children.is_empty())
        } else {
            Some(false)
        };

        NotionBlock {
            object: Some("block".to_string()),
            id: Some(String::new()), // Will be filled by Notion API
            parent,
            created_time: None,
            created_by: None,
            last_edited_time: None,
            last_edited_by: None,
            archived: Some(false),
            has_children,
            block_type: BlockType::Paragraph {
                paragraph: paragraph_value,
            },
        }
    }
}

/// Converter for Pandoc paragraph blocks to Notion paragraph blocks
pub struct PandocParagraphConverter {
    text_converter: PandocTextConverter,
}

impl Default for PandocParagraphConverter {
    fn default() -> Self {
        Self::new()
    }
}

impl PandocParagraphConverter {
    /// Create a new paragraph converter
    pub fn new() -> Self {
        Self {
            text_converter: PandocTextConverter::default(),
        }
    }

    /// Convert a Pandoc paragraph to a Notion paragraph
    pub fn convert(
        &self,
        block: &PandocBlock,
        parent_id: Option<String>,
    ) -> Result<Option<NotionBlock>, Box<dyn Error>> {
        match block {
            PandocBlock::Para(inlines) => {
                // Convert inline elements to rich text using PandocTextConverter
                let rich_text = self.text_converter.convert(inlines)?;

                // Extract color from span if present
                let color = self.extract_color_from_inlines(inlines);

                // Build and return the Notion paragraph
                let mut builder = NotionParagraphBuilder::new().rich_text(rich_text);

                if let Some(color_value) = color {
                    builder = builder.color(color_value);
                }

                if let Some(id) = parent_id {
                    builder = builder.parent_id(id);
                }

                Ok(Some(builder.build()))
            }
            _ => Ok(None),
        }
    }

    /// Try to convert any Pandoc block to a Notion paragraph
    pub fn try_convert(
        &self,
        block: &PandocBlock,
        parent_id: Option<String>,
    ) -> Result<Option<NotionBlock>, Box<dyn Error>> {
        self.convert(block, parent_id)
    }

    /// Extract color from Pandoc inlines if wrapped in a Span with color attributes
    fn extract_color_from_inlines(&self, inlines: &[Inline]) -> Option<TextColor> {
        // Check if the paragraph content is wrapped in a single Span with color attributes
        if inlines.len() == 1 {
            if let Inline::Span(attr, _) = &inlines[0] {
                return self.extract_color_from_attr(attr);
            }
        }
        None
    }

    /// Extract color from Pandoc attributes
    fn extract_color_from_attr(&self, attr: &Attr) -> Option<TextColor> {
        for (key, value) in &attr.attributes {
            if key == "data-color" || key == "color" {
                return match value.to_lowercase().as_str() {
                    "blue" => Some(TextColor::Blue),
                    "brown" => Some(TextColor::Brown),
                    "default" => Some(TextColor::Default),
                    "gray" => Some(TextColor::Gray),
                    "green" => Some(TextColor::Green),
                    "orange" => Some(TextColor::Orange),
                    "pink" => Some(TextColor::Pink),
                    "purple" => Some(TextColor::Purple),
                    "red" => Some(TextColor::Red),
                    "yellow" => Some(TextColor::Yellow),
                    "blue_background" | "bluebackground" => Some(TextColor::BlueBackground),
                    "brown_background" | "brownbackground" => Some(TextColor::BrownBackground),
                    "gray_background" | "graybackground" => Some(TextColor::GrayBackground),
                    "green_background" | "greenbackground" => Some(TextColor::GreenBackground),
                    "orange_background" | "orangebackground" => Some(TextColor::OrangeBackground),
                    "pink_background" | "pinkbackground" => Some(TextColor::PinkBackground),
                    "purple_background" | "purplebackground" => Some(TextColor::PurpleBackground),
                    "red_background" | "redbackground" => Some(TextColor::RedBackground),
                    "yellow_background" | "yellowbackground" => Some(TextColor::YellowBackground),
                    _ => Some(TextColor::Default),
                };
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pandoc_types::definition::Inline;

    #[test]
    fn test_convert_simple_paragraph() {
        let converter = PandocParagraphConverter::new();

        // Create a simple paragraph
        let paragraph =
            PandocBlock::Para(vec![Inline::Str("This is a test paragraph.".to_string())]);

        // Convert to Notion paragraph
        let result = converter.convert(&paragraph, None).unwrap().unwrap();

        // Verify the text content
        match result.block_type {
            BlockType::Paragraph { paragraph } => {
                assert_eq!(paragraph.rich_text.len(), 1);
                assert_eq!(
                    paragraph.rich_text[0].plain_text().unwrap(),
                    "This is a test paragraph."
                );
                assert_eq!(paragraph.color, None);
            }
            _ => panic!("Expected Paragraph block type"),
        }
    }

    #[test]
    fn test_convert_colored_paragraph() {
        let converter = PandocParagraphConverter::new();

        // Create a colored paragraph (using Span wrapper)
        let mut attr = Attr::default();
        attr.attributes
            .push(("data-color".to_string(), "blue".to_string()));

        let inner_content = vec![Inline::Str("This is a blue paragraph.".to_string())];
        let paragraph = PandocBlock::Para(vec![Inline::Span(attr, inner_content)]);

        // Convert to Notion paragraph
        let result = converter.convert(&paragraph, None).unwrap().unwrap();

        // Verify it has the correct color
        match result.block_type {
            BlockType::Paragraph { paragraph } => {
                assert_eq!(paragraph.rich_text.len(), 1);
                assert_eq!(
                    paragraph.rich_text[0].plain_text().unwrap(),
                    "This is a blue paragraph."
                );
                assert_eq!(paragraph.color, Some(TextColor::Blue));
            }
            _ => panic!("Expected Paragraph block type"),
        }
    }

    #[test]
    fn test_convert_with_parent() {
        let converter = PandocParagraphConverter::new();

        // Create a simple paragraph
        let paragraph = PandocBlock::Para(vec![Inline::Str("Paragraph with parent.".to_string())]);

        // Set a parent ID
        let parent_id = "test-page-id-123".to_string();

        // Convert to Notion paragraph with parent
        let result = converter
            .convert(&paragraph, Some(parent_id.clone()))
            .unwrap()
            .unwrap();

        // Verify it has the parent set
        assert!(result.parent.is_some());
        if let Some(parent) = result.parent {
            use notion_client::objects::parent::Parent;
            match parent {
                Parent::PageId { page_id } => {
                    assert_eq!(page_id, parent_id);
                }
                _ => panic!("Expected page_id parent type"),
            }
        }
    }

    #[test]
    fn test_convert_complex_paragraph() {
        let converter = PandocParagraphConverter::new();

        // Create a paragraph with multiple inline elements
        let paragraph = PandocBlock::Para(vec![
            Inline::Str("This is ".to_string()),
            Inline::Emph(vec![Inline::Str("emphasized".to_string())]),
            Inline::Str(" and ".to_string()),
            Inline::Strong(vec![Inline::Str("strong".to_string())]),
            Inline::Str(" text.".to_string()),
        ]);

        // Convert to Notion paragraph
        let result = converter.convert(&paragraph, None).unwrap().unwrap();

        // Verify rich text elements (just check that we have multiple elements)
        match result.block_type {
            BlockType::Paragraph { paragraph } => {
                // We should have at least 3 rich text elements (could be more depending on implementation)
                assert!(paragraph.rich_text.len() >= 3);

                // Verify the combined text content
                let full_text: String = paragraph
                    .rich_text
                    .iter()
                    .filter_map(|rt| rt.plain_text())
                    .collect();

                assert_eq!(full_text, "This is emphasized and strong text.");
            }
            _ => panic!("Expected Paragraph block type"),
        }
    }
}
