//! Converter for Pandoc inline elements to Notion rich text
//!

use notion_client::objects::rich_text::{Annotations, Equation, Link, RichText, Text, TextColor};
use pandoc_types::definition::Inline;
use std::error::Error;
use std::fmt;

/// Errors that can occur during the conversion process
#[derive(Debug, Clone)]
pub enum ConversionError {
    /// Element type is not supported
    UnsupportedElement(String),
    /// Invalid formatting or structure
    InvalidFormat(String),
}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConversionError::UnsupportedElement(msg) => write!(f, "Unsupported element: {}", msg),
            ConversionError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
        }
    }
}

impl Error for ConversionError {}

/// Configuration options for the conversion process
#[derive(Clone, Default)]
pub struct ConversionConfig {
    // Configuration options can be added here if needed in the future
}

/// Builder for creating Notion rich text objects
#[derive(Default)]
pub struct TextBuilder {
    current_text: String,
    annotations: Annotations,
    link: Option<Link>,
    rich_texts: Vec<RichText>,
}

impl TextBuilder {
    /// Get the current annotations
    pub fn get_annotations(&self) -> Annotations {
        self.annotations.clone()
    }

    /// Set the current annotations
    pub fn set_annotations(&mut self, annotations: Annotations) {
        self.annotations = annotations;
    }

    /// Update annotations using a modifier function
    pub fn update_annotation<F>(&mut self, modifier: F) -> &mut Self
    where
        F: FnOnce(&mut Annotations),
    {
        modifier(&mut self.annotations);
        self
    }
}

impl TextBuilder {
    /// Create a new text builder with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Commit the current text with its annotations
    pub fn commit_current_text(&mut self) {
        if !self.current_text.is_empty() {
            let text = Text {
                content: self.current_text.clone(),
                link: self.link.clone(),
            };

            let rich_text = RichText::Text {
                text,
                annotations: Some(self.annotations.clone()),
                plain_text: Some(self.current_text.clone()),
                href: None,
            };

            self.rich_texts.push(rich_text);
            self.current_text.clear();
        }
    }

    /// Commit an equation to the rich texts
    pub fn commit_equation(&mut self, expression: &str) {
        self.commit_current_text();

        let equation = Equation {
            expression: expression.to_string(),
        };

        let rich_text = RichText::Equation {
            equation,
            annotations: self.annotations.clone(),
            plain_text: expression.to_string(),
            href: None,
        };

        self.rich_texts.push(rich_text);
    }

    /// Apply formatting to a new builder and return it
    pub fn apply_formatting<F>(&self, modifier: F) -> TextBuilder
    where
        F: FnOnce(&mut Annotations),
    {
        let mut new_builder = TextBuilder::new();
        new_builder.annotations = self.annotations.clone();
        modifier(&mut new_builder.annotations);
        new_builder
    }

    /// Append text to the current text buffer
    pub fn append_text(&mut self, text: &str) {
        self.current_text.push_str(text);
    }

    /// Build and return the final rich text objects
    pub fn build(mut self) -> Result<Vec<RichText>, ConversionError> {
        self.commit_current_text();
        Ok(self.rich_texts)
    }

    /// Set the link for the current text
    pub fn set_link(&mut self, link: Link) {
        self.link = Some(link);
    }
}

/// Converter for Pandoc inline elements to Notion rich text
pub struct PandocTextConverter {
    // No need for configuration fields currently
}

impl Default for PandocTextConverter {
    fn default() -> Self {
        Self::with_config(ConversionConfig::default())
    }
}

impl PandocTextConverter {
    /// Create a new converter with default configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new converter with the specified configuration
    pub fn with_config(_config: ConversionConfig) -> Self {
        PandocTextConverter {}
    }

    /// Convert a list of Pandoc inline elements to Notion rich text objects
    pub fn convert(&self, elements: &[Inline]) -> Result<Vec<RichText>, ConversionError> {
        let mut builder = TextBuilder::new();
        self.convert_elements(elements, &mut builder)?;
        builder.build()
    }

    /// Convert a list of elements with a specific formatting applied
    fn convert_with_formatting<F>(
        &self,
        elements: &[Inline],
        builder: &mut TextBuilder,
        modifier: F,
    ) -> Result<(), ConversionError>
    where
        F: FnOnce(&mut Annotations),
    {
        let mut nested_builder = builder.apply_formatting(modifier);
        self.convert_elements(elements, &mut nested_builder)?;

        if let Ok(rich_texts) = nested_builder.build() {
            for rich_text in rich_texts {
                builder.rich_texts.push(rich_text);
            }
        }

        Ok(())
    }

    /// Convert a list of elements with the current formatting
    fn convert_elements(
        &self,
        elements: &[Inline],
        builder: &mut TextBuilder,
    ) -> Result<(), ConversionError> {
        for element in elements {
            match element {
                Inline::Str(text) => {
                    builder.append_text(text);
                }

                Inline::Space => {
                    builder.append_text(" ");
                }

                Inline::SoftBreak => {
                    builder.commit_current_text();
                    builder.append_text(" ");
                }

                Inline::LineBreak => {
                    builder.commit_current_text();
                    builder.append_text("\n");
                }

                Inline::Strong(content) => {
                    builder.commit_current_text();
                    self.convert_with_formatting(content, builder, |annotations| {
                        annotations.bold = true;
                    })?;
                }

                Inline::Emph(content) => {
                    builder.commit_current_text();
                    self.convert_with_formatting(content, builder, |annotations| {
                        annotations.italic = true;
                    })?;
                }

                Inline::Strikeout(content) => {
                    builder.commit_current_text();
                    self.convert_with_formatting(content, builder, |annotations| {
                        annotations.strikethrough = true;
                    })?;
                }

                Inline::Code(_, code_text) => {
                    builder.commit_current_text();
                    let mut new_builder = builder.apply_formatting(|annotations| {
                        annotations.code = true;
                    });
                    new_builder.append_text(code_text);

                    if let Ok(rich_texts) = new_builder.build() {
                        for rich_text in rich_texts {
                            builder.rich_texts.push(rich_text);
                        }
                    }
                }

                Inline::Math(_, expression) => {
                    builder.commit_equation(expression);
                }

                Inline::Link(_, content, target) => {
                    builder.commit_current_text();
                    let mut new_builder = TextBuilder::new();

                    // Set the link
                    let link = Link {
                        url: target.url.clone(),
                    };
                    new_builder.set_link(link);

                    // Process the link text
                    self.convert_elements(content, &mut new_builder)?;

                    if let Ok(rich_texts) = new_builder.build() {
                        for rich_text in rich_texts {
                            builder.rich_texts.push(rich_text);
                        }
                    }
                }

                Inline::Span(attr, content) => {
                    builder.commit_current_text();

                    // Handle color classes
                    let color = self.parse_color(&attr.classes);
                    let is_underline = attr.classes.iter().any(|class| class == "underline");

                    // Create a new builder with the span's formatting
                    let mut new_builder = TextBuilder::new();
                    new_builder.annotations = builder.get_annotations();

                    if is_underline {
                        new_builder.annotations.underline = true;
                    }

                    if let Some(color) = color {
                        new_builder.annotations.color = color;
                    }

                    // Process the span content
                    self.convert_elements(content, &mut new_builder)?;

                    if let Ok(rich_texts) = new_builder.build() {
                        for rich_text in rich_texts {
                            builder.rich_texts.push(rich_text);
                        }
                    }
                }

                _ => {
                    return Err(ConversionError::UnsupportedElement(format!(
                        "{:?}",
                        element
                    )));
                }
            }
        }

        Ok(())
    }

    /// Parse color classes from Pandoc span classes
    fn parse_color(&self, classes: &[String]) -> Option<TextColor> {
        for class in classes {
            if let Some(color_part) = class.strip_prefix("color-") {
                // Handle background colors
                if color_part.ends_with("-background") {
                    let base_color = &color_part[0..color_part.len() - 11]; // Remove "-background"
                    return match base_color {
                        "red" => Some(TextColor::RedBackground),
                        "blue" => Some(TextColor::BlueBackground),
                        "green" => Some(TextColor::GreenBackground),
                        "yellow" => Some(TextColor::YellowBackground),
                        "orange" => Some(TextColor::OrangeBackground),
                        "pink" => Some(TextColor::PinkBackground),
                        "purple" => Some(TextColor::PurpleBackground),
                        "brown" => Some(TextColor::BrownBackground),
                        "gray" => Some(TextColor::GrayBackground),
                        _ => None,
                    };
                } else {
                    // Regular colors
                    return match color_part {
                        "red" => Some(TextColor::Red),
                        "blue" => Some(TextColor::Blue),
                        "green" => Some(TextColor::Green),
                        "yellow" => Some(TextColor::Yellow),
                        "orange" => Some(TextColor::Orange),
                        "pink" => Some(TextColor::Pink),
                        "purple" => Some(TextColor::Purple),
                        "brown" => Some(TextColor::Brown),
                        "gray" => Some(TextColor::Gray),
                        _ => None,
                    };
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pandoc_types::definition::{Attr, Inline, MathType, Target};

    // Helper function to create basic Pandoc inline elements
    fn create_text_inline(text: &str) -> Vec<Inline> {
        vec![Inline::Str(text.to_string())]
    }

    #[test]
    fn test_convert_simple_text() {
        let converter = PandocTextConverter::new();
        let elements = create_text_inline("Hello world");

        let result = converter.convert(&elements).unwrap();

        assert_eq!(result.len(), 1);
        match &result[0] {
            RichText::Text { text, .. } => {
                assert_eq!(text.content, "Hello world");
            }
            _ => panic!("Expected Text variant"),
        }
    }

    #[test]
    fn test_convert_bold_text() {
        let converter = PandocTextConverter::new();
        let elements = vec![Inline::Strong(vec![Inline::Str("Bold text".to_string())])];

        let result = converter.convert(&elements).unwrap();

        assert_eq!(result.len(), 1);
        match &result[0] {
            RichText::Text {
                text, annotations, ..
            } => {
                assert_eq!(text.content, "Bold text");
                if let Some(ann) = annotations {
                    assert!(ann.bold);
                } else {
                    panic!("Expected annotations");
                }
            }
            _ => panic!("Expected Text variant"),
        }
    }

    #[test]
    fn test_convert_italic_text() {
        let converter = PandocTextConverter::new();
        let elements = vec![Inline::Emph(vec![Inline::Str("Italic text".to_string())])];

        let result = converter.convert(&elements).unwrap();

        assert_eq!(result.len(), 1);
        match &result[0] {
            RichText::Text {
                text, annotations, ..
            } => {
                assert_eq!(text.content, "Italic text");
                if let Some(ann) = annotations {
                    assert!(ann.italic);
                } else {
                    panic!("Expected annotations");
                }
            }
            _ => panic!("Expected Text variant"),
        }
    }

    #[test]
    fn test_convert_strikethrough_text() {
        let converter = PandocTextConverter::new();
        let elements = vec![Inline::Strikeout(vec![Inline::Str(
            "Strikethrough text".to_string(),
        )])];

        let result = converter.convert(&elements).unwrap();

        assert_eq!(result.len(), 1);
        match &result[0] {
            RichText::Text {
                text, annotations, ..
            } => {
                assert_eq!(text.content, "Strikethrough text");
                if let Some(ann) = annotations {
                    assert!(ann.strikethrough);
                } else {
                    panic!("Expected annotations");
                }
            }
            _ => panic!("Expected Text variant"),
        }
    }

    #[test]
    fn test_convert_code() {
        let converter = PandocTextConverter::new();
        let elements = vec![Inline::Code(Attr::default(), "code sample".to_string())];

        let result = converter.convert(&elements).unwrap();

        assert_eq!(result.len(), 1);
        match &result[0] {
            RichText::Text {
                text, annotations, ..
            } => {
                assert_eq!(text.content, "code sample");
                if let Some(ann) = annotations {
                    assert!(ann.code);
                } else {
                    panic!("Expected annotations");
                }
            }
            _ => panic!("Expected Text variant"),
        }
    }

    #[test]
    fn test_convert_math() {
        let converter = PandocTextConverter::new();
        let elements = vec![Inline::Math(MathType::InlineMath, "E=mc^2".to_string())];

        let result = converter.convert(&elements).unwrap();

        assert_eq!(result.len(), 1);
        match &result[0] {
            RichText::Equation { equation, .. } => {
                assert_eq!(equation.expression, "E=mc^2");
            }
            _ => panic!("Expected Equation variant"),
        }
    }

    #[test]
    fn test_convert_link() {
        let converter = PandocTextConverter::new();
        let elements = vec![Inline::Link(
            Attr::default(),
            vec![Inline::Str("Link text".to_string())],
            Target {
                url: "https://example.com".to_string(),
                title: "".to_string(),
            },
        )];

        let result = converter.convert(&elements).unwrap();

        assert_eq!(result.len(), 1);
        match &result[0] {
            RichText::Text { text, .. } => {
                assert_eq!(text.content, "Link text");
                assert!(text.link.is_some());
                if let Some(link) = &text.link {
                    assert_eq!(link.url, "https://example.com");
                }
            }
            _ => panic!("Expected Text variant"),
        }
    }

    #[test]
    fn test_convert_nested_formatting() {
        let converter = PandocTextConverter::new();
        let elements = vec![Inline::Strong(vec![
            Inline::Str("Bold ".to_string()),
            Inline::Emph(vec![Inline::Str("and italic".to_string())]),
            Inline::Str(" text".to_string()),
        ])];

        let result = converter.convert(&elements).unwrap();

        assert_eq!(result.len(), 3);

        // First part: "Bold "
        match &result[0] {
            RichText::Text {
                text, annotations, ..
            } => {
                assert_eq!(text.content, "Bold ");
                if let Some(ann) = annotations {
                    assert!(ann.bold);
                    assert!(!ann.italic);
                }
            }
            _ => panic!("Expected Text variant"),
        }

        // Second part: "and italic" (both bold and italic)
        match &result[1] {
            RichText::Text {
                text, annotations, ..
            } => {
                assert_eq!(text.content, "and italic");
                if let Some(ann) = annotations {
                    assert!(ann.bold);
                    assert!(ann.italic);
                }
            }
            _ => panic!("Expected Text variant"),
        }

        // Third part: " text" (just bold)
        match &result[2] {
            RichText::Text {
                text, annotations, ..
            } => {
                assert_eq!(text.content, " text");
                if let Some(ann) = annotations {
                    assert!(ann.bold);
                    assert!(!ann.italic);
                }
            }
            _ => panic!("Expected Text variant"),
        }
    }

    #[test]
    fn test_convert_color() {
        let converter = PandocTextConverter::new();

        // Create a span with a color class
        let mut attr = Attr::default();
        attr.classes.push("color-red".to_string());

        let elements = vec![Inline::Span(
            attr,
            vec![Inline::Str("Colored text".to_string())],
        )];

        let result = converter.convert(&elements).unwrap();

        assert_eq!(result.len(), 1);
        match &result[0] {
            RichText::Text {
                text, annotations, ..
            } => {
                assert_eq!(text.content, "Colored text");
                if let Some(ann) = annotations {
                    assert_eq!(ann.color, TextColor::Red);
                } else {
                    panic!("Expected annotations");
                }
            }
            _ => panic!("Expected Text variant"),
        }
    }

    #[test]
    fn test_convert_complex_formatting() {
        let converter = PandocTextConverter::new();

        // Create a complex element with multiple formatting types
        let mut color_attr = Attr::default();
        color_attr.classes.push("color-red".to_string());

        let elements = vec![Inline::Span(
            color_attr,
            vec![Inline::Strong(vec![
                Inline::Emph(vec![Inline::Str("Complex ".to_string())]),
                Inline::Str("formatting".to_string()),
            ])],
        )];

        let result = converter.convert(&elements).unwrap();

        // Should have two parts:
        // 1. "Complex " - bold, italic, red
        // 2. "formatting" - bold, red
        assert_eq!(result.len(), 2);

        match &result[0] {
            RichText::Text {
                text, annotations, ..
            } => {
                assert_eq!(text.content, "Complex ");
                if let Some(ann) = annotations {
                    assert!(ann.bold);
                    assert!(ann.italic);
                    assert_eq!(ann.color, TextColor::Red);
                }
            }
            _ => panic!("Expected Text variant"),
        }

        match &result[1] {
            RichText::Text {
                text, annotations, ..
            } => {
                assert_eq!(text.content, "formatting");
                if let Some(ann) = annotations {
                    assert!(ann.bold);
                    assert!(!ann.italic);
                    assert_eq!(ann.color, TextColor::Red);
                }
            }
            _ => panic!("Expected Text variant"),
        }
    }
}
