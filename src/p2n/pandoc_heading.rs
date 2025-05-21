use crate::p2n::pandoc_text::PandocTextConverter;
use notion_client::objects::block::TextColor;
use notion_client::objects::block::{Block as NotionBlock, BlockType, HeadingsValue};
use notion_client::objects::rich_text::RichText;
use pandoc_types::definition::{Attr, Block as PandocBlock};
use std::error::Error;

/// Builder for Notion heading blocks
pub struct NotionHeadingBuilder {
    rich_text: Vec<RichText>,
    color: Option<TextColor>,
    is_toggleable: Option<bool>,
    level: u8,
    parent_id: Option<String>,
}

impl NotionHeadingBuilder {
    /// Create a new NotionHeadingBuilder with default values
    pub fn new() -> Self {
        Self {
            rich_text: Vec::new(),
            color: None,
            is_toggleable: None,
            level: 1,
            parent_id: None,
        }
    }

    /// Set the rich text content for the heading
    pub fn rich_text(mut self, rich_text: Vec<RichText>) -> Self {
        self.rich_text = rich_text;
        self
    }

    /// Set the heading color
    pub fn color(mut self, color: TextColor) -> Self {
        self.color = Some(color);
        self
    }

    /// Set whether the heading is toggleable
    pub fn is_toggleable(mut self, toggleable: bool) -> Self {
        self.is_toggleable = Some(toggleable);
        self
    }

    /// Set the heading level (1-3 in Notion)
    pub fn level(mut self, level: u8) -> Self {
        // Notion only supports heading levels 1-3
        self.level = level.min(3).max(1);
        self
    }

    /// Set the parent ID for the block
    pub fn parent_id(mut self, id: String) -> Self {
        self.parent_id = Some(id);
        self
    }

    /// Build the Notion heading block
    pub fn build(self) -> NotionBlock {
        let heading_value = HeadingsValue {
            rich_text: self.rich_text,
            color: self.color,
            is_toggleable: self.is_toggleable,
        };

        let block_type = match self.level {
            1 => BlockType::Heading1 {
                heading_1: heading_value,
            },
            2 => BlockType::Heading2 {
                heading_2: heading_value,
            },
            3 => BlockType::Heading3 {
                heading_3: heading_value,
            },
            _ => BlockType::Heading1 {
                heading_1: heading_value,
            }, // Default fallback
        };

        // Create parent if specified
        let parent = self.parent_id.map(|id| {
            use notion_client::objects::parent::Parent;
            Parent::PageId { page_id: id }
        });

        NotionBlock {
            object: Some("block".to_string()),
            id: Some(String::new()), // Will be filled by Notion API
            parent,
            created_time: None,
            created_by: None,
            last_edited_time: None,
            last_edited_by: None,
            archived: Some(false),
            has_children: Some(false),
            block_type,
        }
    }
}

/// Converter for Pandoc heading blocks to Notion heading blocks
pub struct PandocHeadingConverter {
    text_converter: PandocTextConverter,
}

impl Default for PandocHeadingConverter {
    fn default() -> Self {
        Self::new()
    }
}

impl PandocHeadingConverter {
    /// Create a new heading converter
    pub fn new() -> Self {
        Self {
            text_converter: PandocTextConverter::default(),
        }
    }

    /// Convert a Pandoc heading to a Notion heading
    pub fn convert(
        &self,
        block: &PandocBlock,
        parent_id: Option<String>,
    ) -> Result<Option<NotionBlock>, Box<dyn Error>> {
        match block {
            PandocBlock::Header(level, attr, inlines) => {
                // Convert inline elements to rich text using PandocTextConverter
                let rich_text = self.text_converter.convert(inlines)?;

                // Extract color and toggleable state from attributes
                let color = self.extract_color_from_attr(attr);
                let is_toggleable = self.is_toggleable_from_classes(&attr.classes);

                // Build and return the Notion heading
                let mut builder = NotionHeadingBuilder::new()
                    .rich_text(rich_text)
                    .level(*level as u8);

                if let Some(color_value) = color {
                    builder = builder.color(color_value);
                }

                if let Some(toggleable) = is_toggleable {
                    builder = builder.is_toggleable(toggleable);
                }

                if let Some(id) = parent_id {
                    builder = builder.parent_id(id);
                }

                Ok(Some(builder.build()))
            }
            _ => Ok(None),
        }
    }

    /// Try to convert any Pandoc block to a Notion heading
    pub fn try_convert(
        &self,
        block: &PandocBlock,
        parent_id: Option<String>,
    ) -> Result<Option<NotionBlock>, Box<dyn Error>> {
        self.convert(block, parent_id)
    }

    /// Extract color from Pandoc attributes if present
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

    /// Check if heading is toggleable based on classes
    fn is_toggleable_from_classes(&self, classes: &[String]) -> Option<bool> {
        if classes.iter().any(|class| class == "toggleable") {
            return Some(true);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pandoc_types::definition::Inline;

    #[test]
    fn test_convert_simple_heading() {
        let converter = PandocHeadingConverter::new();

        // Create a simple H1 heading
        let heading = PandocBlock::Header(
            1,
            Attr::default(),
            vec![Inline::Str("Test Heading".to_string())],
        );

        // Convert to Notion heading
        let result = converter.convert(&heading, None).unwrap().unwrap();

        // Verify the level and text content
        match result.block_type {
            BlockType::Heading1 { heading_1 } => {
                assert_eq!(heading_1.rich_text.len(), 1);
                assert_eq!(heading_1.rich_text[0].plain_text().unwrap(), "Test Heading");
                assert_eq!(heading_1.is_toggleable, None);
            }
            _ => panic!("Expected Heading1 block type"),
        }
    }

    #[test]
    fn test_convert_toggleable_heading() {
        let converter = PandocHeadingConverter::new();

        // Create a toggleable H2 heading
        let mut attr = Attr::default();
        attr.classes.push("toggleable".to_string());

        let heading =
            PandocBlock::Header(2, attr, vec![Inline::Str("Toggleable Heading".to_string())]);

        // Convert to Notion heading
        let result = converter.convert(&heading, None).unwrap().unwrap();

        // Verify it's a level 2 heading with toggleable property
        match result.block_type {
            BlockType::Heading2 { heading_2 } => {
                assert_eq!(heading_2.rich_text.len(), 1);
                assert_eq!(
                    heading_2.rich_text[0].plain_text().unwrap(),
                    "Toggleable Heading"
                );
                assert_eq!(heading_2.is_toggleable, Some(true));
            }
            _ => panic!("Expected Heading2 block type"),
        }
    }

    #[test]
    fn test_convert_colored_heading() {
        let converter = PandocHeadingConverter::new();

        // Create a colored H3 heading
        let mut attr = Attr::default();
        attr.attributes
            .push(("data-color".to_string(), "red".to_string()));

        let heading =
            PandocBlock::Header(3, attr, vec![Inline::Str("Colored Heading".to_string())]);

        // Convert to Notion heading
        let result = converter.convert(&heading, None).unwrap().unwrap();

        // Verify it's a level 3 heading with color
        match result.block_type {
            BlockType::Heading3 { heading_3 } => {
                assert_eq!(heading_3.rich_text.len(), 1);
                assert_eq!(
                    heading_3.rich_text[0].plain_text().unwrap(),
                    "Colored Heading"
                );
                assert_eq!(heading_3.color, Some(TextColor::Red));
            }
            _ => panic!("Expected Heading3 block type"),
        }
    }

    #[test]
    fn test_level_clamping() {
        let converter = PandocHeadingConverter::new();

        // Create a heading with level 6 (beyond Notion's support)
        let heading = PandocBlock::Header(
            6,
            Attr::default(),
            vec![Inline::Str("Level 6 Heading".to_string())],
        );

        // Convert to Notion heading (should be clamped to level 3)
        let result = converter.convert(&heading, None).unwrap().unwrap();

        // Verify it's been clamped to level 3
        match result.block_type {
            BlockType::Heading3 { heading_3 } => {
                assert_eq!(heading_3.rich_text.len(), 1);
                assert_eq!(
                    heading_3.rich_text[0].plain_text().unwrap(),
                    "Level 6 Heading"
                );
            }
            _ => panic!("Expected Heading3 block type due to clamping"),
        }
    }
}
