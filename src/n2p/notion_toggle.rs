use crate::n2p::notion_text::NotionTextConverter;
use crate::n2p::ConversionConfig;
use notion_client::objects::block::{Block as NotionBlock, BlockType, ToggleValue};
use pandoc_types::definition::{Attr, Block as PandocBlock, Inline};

/// Builder for converting Notion toggles to Pandoc divs
pub struct ToggleBuilder {
    rich_text: Vec<Inline>,
    identifier: String,
    classes: Vec<String>,
    attributes: Vec<(String, String)>,
    children: Vec<PandocBlock>,
}

impl ToggleBuilder {
    /// Create a new ToggleBuilder with default values
    pub fn new() -> Self {
        Self {
            rich_text: Vec::new(),
            identifier: String::new(),
            classes: vec!["toggle".to_string()], // Default class for toggle blocks
            attributes: Vec::new(),
            children: Vec::new(),
        }
    }

    /// Set the rich text content for the toggle
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

    /// Set the children blocks
    pub fn children(mut self, children: Vec<PandocBlock>) -> Self {
        self.children = children;
        self
    }

    /// Add a single child block
    pub fn add_child(mut self, child: PandocBlock) -> Self {
        self.children.push(child);
        self
    }

    /// Build the Pandoc toggle div
    pub fn build(self) -> PandocBlock {
        // Create a container for all content
        let mut content = Vec::new();
        
        // Handle toggle text content
        if !self.rich_text.is_empty() {
            // If there's text, add it as a paragraph
            content.push(PandocBlock::Para(self.rich_text));
            
            // Add a blank line after text if there are children
            if !self.children.is_empty() {
                content.push(PandocBlock::Para(Vec::new()));
            }
        } else {
            // If there's no text, just add a blank line
            content.push(PandocBlock::Para(Vec::new()));
        }
        
        // Add all children blocks
        content.extend(self.children);
        
        // Create the toggle div
        PandocBlock::Div(
            Attr {
                identifier: self.identifier,
                classes: self.classes,
                attributes: self.attributes,
            },
            content,
        )
    }
}

/// Convert a Notion toggle to a Pandoc div
pub fn convert_notion_toggle(block: &NotionBlock, config: &ConversionConfig) -> Option<PandocBlock> {
    match &block.block_type {
        BlockType::Toggle { toggle } => {
            Some(build_toggle_from_notion(toggle, config))
        }
        _ => None,
    }
}

/// Helper function to build a toggle from Notion toggle data
fn build_toggle_from_notion(
    toggle: &ToggleValue,
    config: &ConversionConfig,
) -> PandocBlock {
    // Convert rich text to inlines using NotionTextConverter
    let inlines = NotionTextConverter::convert(&toggle.rich_text);

    // If render_toggle_div is false, just return text or blank line
    if !config.render_toggle_div {
        if inlines.is_empty() {
            // Empty toggle should just be a blank line
            return PandocBlock::Para(Vec::new());
        } else {
            // For toggle with text, just return the text paragraph
            return PandocBlock::Para(inlines);
        }
    }

    // Otherwise render as div with toggle class
    let mut builder = ToggleBuilder::new().rich_text(inlines);

    // Only add attributes if preserve_attributes is true
    if config.preserve_attributes {
        // Add a unique identifier based on text content (for anchor links)
        if !toggle.rich_text.is_empty() {
            let id = generate_toggle_id(&toggle.rich_text);
            builder = builder.identifier(id);
        }

        // Handle Notion's color
        builder = builder.add_attribute("data-color".to_string(), format!("{:?}", toggle.color));
    }

    builder.build()
}

/// Generate a toggle ID from rich text content for anchor links
fn generate_toggle_id(rich_text: &[notion_client::objects::rich_text::RichText]) -> String {
    // Extract plain text from rich text
    let plain_text: String = rich_text
        .iter()
        .filter_map(|rt| rt.plain_text())
        .collect::<Vec<_>>()
        .join("");

    // Create a slug-like ID
    format!(
        "toggle-{}",
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
    )
}

/// Convenience function to directly convert any block to a toggle if it is one
pub fn try_convert_to_toggle(block: &NotionBlock, config: &ConversionConfig) -> Option<PandocBlock> {
    convert_notion_toggle(block, config)
}