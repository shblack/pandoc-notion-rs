use crate::n2p::notion_text::NotionTextConverter;
use notion_client::objects::block::{Block as NotionBlock, BlockType, HeadingsValue};
use pandoc_types::definition::{Attr, Block as PandocBlock, Inline};

/// Builder for converting Notion headings to Pandoc headings
pub struct HeadingBuilder {
    level: i32,
    rich_text: Vec<Inline>,
    identifier: String,
    classes: Vec<String>,
    attributes: Vec<(String, String)>,
}

impl HeadingBuilder {
    /// Create a new HeadingBuilder with default values
    pub fn new() -> Self {
        Self {
            level: 1,
            rich_text: Vec::new(),
            identifier: String::new(),
            classes: Vec::new(),
            attributes: Vec::new(),
        }
    }

    /// Set the heading level (1-6)
    pub fn level(mut self, level: i32) -> Self {
        self.level = level;
        self
    }

    /// Set the rich text content for the heading
    pub fn rich_text(mut self, rich_text: Vec<Inline>) -> Self {
        self.rich_text = rich_text;
        self
    }

    /// Set the identifier (for linking)
    pub fn identifier(mut self, identifier: String) -> Self {
        self.identifier = identifier;
        self
    }

    /// Add CSS classes
    pub fn classes(mut self, classes: Vec<String>) -> Self {
        self.classes = classes;
        self
    }

    /// Add a single class
    pub fn add_class(mut self, class: String) -> Self {
        self.classes.push(class);
        self
    }

    /// Add attributes
    pub fn attributes(mut self, attributes: Vec<(String, String)>) -> Self {
        self.attributes = attributes;
        self
    }

    /// Add a single attribute
    pub fn add_attribute(mut self, key: String, value: String) -> Self {
        self.attributes.push((key, value));
        self
    }

    /// Build the Pandoc heading
    pub fn build(self) -> PandocBlock {
        PandocBlock::Header(
            self.level,
            Attr {
                identifier: self.identifier,
                classes: self.classes,
                attributes: self.attributes,
            },
            self.rich_text,
        )
    }
}

/// Convert a Notion heading to a Pandoc heading
pub fn convert_notion_heading(block: &NotionBlock) -> Option<PandocBlock> {
    // Create text converter
    let text_converter = NotionTextConverter;

    match &block.block_type {
        BlockType::Heading1 { heading_1 } => {
            Some(build_heading_from_notion(heading_1, 1, &text_converter))
        }
        BlockType::Heading2 { heading_2 } => {
            Some(build_heading_from_notion(heading_2, 2, &text_converter))
        }
        BlockType::Heading3 { heading_3 } => {
            Some(build_heading_from_notion(heading_3, 3, &text_converter))
        }
        _ => None,
    }
}

/// Helper function to build a heading from Notion heading data
fn build_heading_from_notion(
    heading: &HeadingsValue,
    level: i32,
    text_converter: &NotionTextConverter,
) -> PandocBlock {
    // Convert rich text to inlines using NotionTextConverter
    let inlines = text_converter.convert(&heading.rich_text);

    let mut builder = HeadingBuilder::new().level(level).rich_text(inlines);

    // Add a unique identifier based on text content (for anchor links)
    if !heading.rich_text.is_empty() {
        let id = generate_heading_id(&heading.rich_text);
        builder = builder.identifier(id);
    }

    // Handle Notion's color if present
    if let Some(color) = &heading.color {
        builder = builder.add_attribute("data-color".to_string(), format!("{:?}", color));
    }

    // Handle Notion's toggleable feature
    if let Some(true) = heading.is_toggleable {
        builder = builder.add_class("toggleable".to_string());
    }

    builder.build()
}

/// Generate a heading ID from rich text content for anchor links
fn generate_heading_id(rich_text: &[notion_client::objects::rich_text::RichText]) -> String {
    // Extract plain text from rich text
    let plain_text: String = rich_text
        .iter()
        .filter_map(|rt| rt.plain_text())
        .collect::<Vec<_>>()
        .join("");

    // Create a slug-like ID
    plain_text
        .to_lowercase()
        .chars()
        .map(|c| match c {
            ' ' => '-',
            c if c.is_alphanumeric() => c,
            _ => '-',
        })
        .collect::<String>()
        .replace("--", "-")
}

/// Convenience function to directly convert any block to a heading if it is one
pub fn try_convert_to_heading(block: &NotionBlock) -> Option<PandocBlock> {
    convert_notion_heading(block)
}
