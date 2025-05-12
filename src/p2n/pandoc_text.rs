//! Converter for Pandoc inline elements to Notion rich text
//!
//! This module provides the functionality to convert Pandoc's inline elements
//! (such as text, formatting, equations, and links) into Notion's rich text objects.
//! The conversion preserves formatting attributes and handles nested structures.

use crate::notion::text::{Annotations, Color, Equation, Link, RichTextObject, TextContent};
use pandoc_types::definition::{Inline, Target};
use std::error::Error;
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

/// Error types that can occur during the Pandoc to Notion conversion process
#[derive(Debug)]
pub enum ConversionError {
    /// An element type that the converter doesn't know how to handle
    UnsupportedElement(String),
    /// Data that doesn't conform to the expected format
    InvalidFormat(String),
}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConversionError::UnsupportedElement(msg) => write!(f, "Unsupported element: {}", msg),
            ConversionError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
        }
    }
}

impl Error for ConversionError {}

/// Configuration options for the Pandoc to Notion converter
///
/// These options control how whitespace and formatting are handled
/// during the conversion process.
#[derive(Debug, Clone)]
pub struct ConversionConfig {
    /// If true, convert newlines to line breaks rather than spaces
    pub preserve_line_breaks: bool,
    /// If true, maintain all whitespace in the original text
    pub preserve_whitespace: bool,
    /// If true, collapse multiple consecutive whitespace into a single space
    pub collapse_whitespace: bool,
}

impl Default for ConversionConfig {
    fn default() -> Self {
        Self {
            preserve_line_breaks: true,
            preserve_whitespace: true,
            collapse_whitespace: false,
        }
    }
}

/// Builder for constructing Notion rich text objects
///
/// TextBuilder accumulates text content and formatting attributes, then
/// produces properly formatted Notion rich text objects. It handles
/// annotations like bold, italic, and links, as well as special content
/// like equations.
pub struct TextBuilder {
    current_text: String,
    annotations: Annotations,
    link: Option<Link>,
    rich_texts: Vec<RichTextObject>,
}

// Testing methods
#[cfg(test)]
impl TextBuilder {
    /// Get a reference to the current annotations (for testing)
    pub fn get_annotations(&self) -> &Annotations {
        &self.annotations
    }
    
    /// Set annotations directly (for testing)
    pub fn set_annotations(&mut self, annotations: Annotations) {
        self.annotations = annotations;
    }
    
    /// Update a specific annotation (for testing)
    pub fn update_annotation<F>(&mut self, f: F) 
    where F: FnOnce(&mut Annotations) {
        f(&mut self.annotations);
    }
}

impl TextBuilder {
    /// Create a new empty TextBuilder with default annotations
    pub fn new() -> Self {
        Self {
            current_text: String::new(),
            annotations: Annotations::default(),
            link: None,
            rich_texts: Vec::new(),
        }
    }

    /// Finalize the current text buffer with its formatting
    ///
    /// This creates a new rich text object with the current annotations
    /// and adds it to the internal list of rich text objects.
    pub fn commit_current_text(&mut self) -> Result<(), ConversionError> {
        if !self.current_text.is_empty() {
            let text_content = TextContent {
                content: self.current_text.clone(),
                link: self.link.clone(),
            };

            let rich_text = RichTextObject::Text {
                text: text_content,
                annotations: Some(self.annotations),
                plain_text: Some(self.current_text.clone()),
                href: self.link.as_ref().map(|link| link.url.clone()),
            };

            self.rich_texts.push(rich_text);
            self.current_text.clear();
            self.link = None;
        }
        Ok(())
    }

    /// Create a math equation rich text object
    ///
    /// This creates a Notion equation object with the given LaTeX expression.
    pub fn commit_equation(&mut self, expression: &str) -> Result<(), ConversionError> {
        // First commit any current text
        self.commit_current_text()?;

        let equation = Equation {
            expression: expression.to_string(),
        };

        let rich_text = RichTextObject::Equation {
            equation,
            annotations: Some(self.annotations),
            plain_text: Some(expression.to_string()),
            href: None,
        };

        self.rich_texts.push(rich_text);
        Ok(())
    }

    /// Create a new builder with modified annotations
    ///
    /// This method is used for handling nested formatting (e.g., bold inside italic).
    /// It creates a new builder with the same base annotations, then applies
    /// the provided update function to modify those annotations.
    pub fn apply_formatting<F>(&mut self, mut update_annotations: F) -> Result<TextBuilder, ConversionError> 
    where F: FnMut(&mut Annotations) {
        // Commit any current text before creating a nested builder
        self.commit_current_text()?;
        
        // Create a new builder with the same annotations
        let mut nested_builder = TextBuilder::new();
        nested_builder.annotations = self.annotations;
        
        // Apply the annotation update
        update_annotations(&mut nested_builder.annotations);
        
        Ok(nested_builder)
    }

    /// Add text to the current accumulation buffer
    ///
    /// This appends the given text to the current text buffer without committing it.
    pub fn append_text(&mut self, text: &str) {
        self.current_text.push_str(text);
    }

    /// Finish building and return all accumulated rich text objects
    ///
    /// This commits any remaining text in the buffer and returns the
    /// complete list of rich text objects that have been built.
    pub fn build(mut self) -> Result<Vec<RichTextObject>, ConversionError> {
        self.commit_current_text()?;
        Ok(self.rich_texts)
    }

    /// Set a hyperlink URL for the current text
    ///
    /// This sets a link that will be applied to the next committed text.
    pub fn set_link(&mut self, url: &str) {
        self.link = Some(Link { url: url.to_string() });
    }
}

/// Handler for processing specific types of Pandoc inline elements
///
/// This trait implements the Chain of Responsibility pattern, where each handler
/// knows how to convert a specific type of Pandoc element to Notion format.
pub trait PandocElementHandler {
    /// Check if this handler can process the given element type
    fn can_handle(&self, element: &Inline) -> bool;
    
    /// Process the element and update the text builder accordingly
    fn handle(&self, element: &Inline, builder: &mut TextBuilder) -> Result<(), ConversionError>;
}

/// Main converter for Pandoc inline elements to Notion rich text
///
/// This struct orchestrates the conversion process by dispatching elements
/// to the appropriate handlers and managing the overall conversion flow.
pub struct PandocTextConverter {
    /// Chain of handlers for different element types
    handlers: Vec<Box<dyn PandocElementHandler>>,
    /// Configuration for the conversion process
    config: ConversionConfig,
}

impl PandocTextConverter {
    pub fn new() -> Self {
        let config = ConversionConfig::default();
        let converter_rc = Rc::new(RefCell::new(None));
        
        // Create the converter instance with empty handlers first
        let instance = Self {
            handlers: Vec::new(),
            config: config.clone(),
        };
        
        // Store the instance in the RefCell
        let handlers = instance.create_default_handlers(converter_rc.clone());
        
        // Create a new instance with the handlers
        let result = Self {
            handlers,
            config,
        };
        
        // Store the real converter in the RefCell for handlers to use
        if let Ok(mut opt_conv) = converter_rc.try_borrow_mut() {
            *opt_conv = Some(result.clone());
        }
        
        // Return the fully initialized converter
        result
    }
    
    pub fn with_config(config: ConversionConfig) -> Self {
        let mut converter = Self::new();
        converter.config = config;
        converter
    }
    
    // Create the default set of element handlers
    fn create_default_handlers(&self, converter_rc: Rc<RefCell<Option<Self>>>) -> Vec<Box<dyn PandocElementHandler>> {
        vec![
            Box::new(StrHandler),
            Box::new(SpaceHandler),
            Box::new(BreakHandler::new(self.config.clone())),
            Box::new(StrongHandler::new(converter_rc.clone())),
            Box::new(EmphHandler::new(converter_rc.clone())),
            Box::new(StrikeoutHandler::new(converter_rc.clone())),
            Box::new(CodeHandler::new(converter_rc.clone())),
            Box::new(MathHandler),
            Box::new(LinkHandler::new(converter_rc.clone())),
            Box::new(SpanHandler::new(converter_rc.clone())),
        ]
    }
    
    /// Convert a sequence of Pandoc inline elements to Notion rich text objects
    ///
    /// This is the main entry point for the conversion process.
    pub fn convert(&self, elements: &[Inline]) -> Result<Vec<RichTextObject>, ConversionError> {
        let mut builder = TextBuilder::new();
        self.convert_content(elements, &mut builder)?;
        builder.build()
    }
    
    /// Convert content elements into a specific builder
    ///
    /// This is used for nested conversions, such as processing the content of 
    /// a formatted element (bold, italic, etc.) or a link.
    pub fn convert_content(&self, elements: &[Inline], builder: &mut TextBuilder) -> Result<(), ConversionError> {
        for element in elements {
            let mut handled = false;
            
            for handler in &self.handlers {
                if handler.can_handle(element) {
                    handler.handle(element, builder)?;
                    handled = true;
                    break;
                }
            }
            
            if !handled {
                return Err(ConversionError::UnsupportedElement(format!("No handler for element: {:?}", element)));
            }
        }
        
        Ok(())
    }
}

/// Allow the converter to be cloned (needed for recursive handling)
impl Clone for PandocTextConverter {
    fn clone(&self) -> Self {
        Self {
            handlers: Vec::new(), // Handlers will be recreated in create_default_handlers
            config: self.config.clone(),
        }
    }
}

// Handler implementations for different Pandoc elements

/// Handler for plain text string elements
pub struct StrHandler;

impl PandocElementHandler for StrHandler {
    fn can_handle(&self, element: &Inline) -> bool {
        matches!(element, Inline::Str(_))
    }

    fn handle(&self, element: &Inline, builder: &mut TextBuilder) -> Result<(), ConversionError> {
        if let Inline::Str(text) = element {
            builder.append_text(text);
            Ok(())
        } else {
            Err(ConversionError::UnsupportedElement("Not a Str element".to_string()))
        }
    }
}

/// Handler for space elements (converts to Notion space)
pub struct SpaceHandler;

impl PandocElementHandler for SpaceHandler {
    fn can_handle(&self, element: &Inline) -> bool {
        matches!(element, Inline::Space)
    }

    fn handle(&self, element: &Inline, builder: &mut TextBuilder) -> Result<(), ConversionError> {
        if matches!(element, Inline::Space) {
            builder.append_text(" ");
            Ok(())
        } else {
            Err(ConversionError::UnsupportedElement("Not a Space element".to_string()))
        }
    }
}

/// Handler for line break and soft break elements
///
/// This handler converts Pandoc's line breaks to appropriate
/// Notion text, based on the configuration settings.
pub struct BreakHandler {
    config: ConversionConfig,
}

impl BreakHandler {
    pub fn new(config: ConversionConfig) -> Self {
        Self { config }
    }
}

impl PandocElementHandler for BreakHandler {
    fn can_handle(&self, element: &Inline) -> bool {
        matches!(element, Inline::LineBreak | Inline::SoftBreak)
    }

    fn handle(&self, element: &Inline, builder: &mut TextBuilder) -> Result<(), ConversionError> {
        if self.can_handle(element) {
            if self.config.preserve_line_breaks {
                builder.append_text("\n");
            } else {
                builder.append_text(" ");
            }
            Ok(())
        } else {
            Err(ConversionError::UnsupportedElement("Not a break element".to_string()))
        }
    }
}

/// Handler for strong (bold) text elements
///
/// This handler applies bold formatting to the contained content.
pub struct StrongHandler {
    converter: Rc<RefCell<Option<PandocTextConverter>>>,
}

impl StrongHandler {
    pub fn new(converter: Rc<RefCell<Option<PandocTextConverter>>>) -> Self {
        Self { converter }
    }
}

impl PandocElementHandler for StrongHandler {
    fn can_handle(&self, element: &Inline) -> bool {
        matches!(element, Inline::Strong(_))
    }

    fn handle(&self, element: &Inline, builder: &mut TextBuilder) -> Result<(), ConversionError> {
        if let Inline::Strong(content) = element {
            // Create a nested builder with bold formatting
            let mut nested_builder = builder.apply_formatting(|annotations| {
                annotations.bold = true;
            })?;
            
            // Convert the nested content
            if let Ok(ref_conv) = self.converter.try_borrow() {
                if let Some(converter) = ref_conv.as_ref() {
                    converter.convert_content(content, &mut nested_builder)?;
                } else {
                    // If no converter available, just add the raw text
                    for elem in content {
                        if let Inline::Str(text) = elem {
                            nested_builder.append_text(text);
                        } else if let Inline::Space = elem {
                            nested_builder.append_text(" ");
                        }
                    }
                }
            } else {
                // Fallback if we can't borrow the converter
                for elem in content {
                    if let Inline::Str(text) = elem {
                        nested_builder.append_text(text);
                    } else if let Inline::Space = elem {
                        nested_builder.append_text(" ");
                    }
                }
            }
            
            // Add the nested builder's content to the main builder
            let nested_rich_texts = nested_builder.build()?;
            builder.rich_texts.extend(nested_rich_texts);
            
            Ok(())
        } else {
            Err(ConversionError::UnsupportedElement("Not a Strong element".to_string()))
        }
    }
}

/// Handler for emphasis (italic) text elements
///
/// This handler applies italic formatting to the contained content.
pub struct EmphHandler {
    converter: Rc<RefCell<Option<PandocTextConverter>>>,
}

impl EmphHandler {
    pub fn new(converter: Rc<RefCell<Option<PandocTextConverter>>>) -> Self {
        Self { converter }
    }
}

impl PandocElementHandler for EmphHandler {
    fn can_handle(&self, element: &Inline) -> bool {
        matches!(element, Inline::Emph(_))
    }

    fn handle(&self, element: &Inline, builder: &mut TextBuilder) -> Result<(), ConversionError> {
        if let Inline::Emph(content) = element {
            // Create a nested builder with italic formatting
            let mut nested_builder = builder.apply_formatting(|annotations| {
                annotations.italic = true;
            })?;
            
            // Convert the nested content
            if let Ok(ref_conv) = self.converter.try_borrow() {
                if let Some(converter) = ref_conv.as_ref() {
                    converter.convert_content(content, &mut nested_builder)?;
                } else {
                    // If no converter available, just add the raw text
                    for elem in content {
                        if let Inline::Str(text) = elem {
                            nested_builder.append_text(text);
                        } else if let Inline::Space = elem {
                            nested_builder.append_text(" ");
                        }
                    }
                }
            } else {
                // Fallback if we can't borrow the converter
                for elem in content {
                    if let Inline::Str(text) = elem {
                        nested_builder.append_text(text);
                    } else if let Inline::Space = elem {
                        nested_builder.append_text(" ");
                    }
                }
            }
            
            // Add the nested builder's content to the main builder
            let nested_rich_texts = nested_builder.build()?;
            builder.rich_texts.extend(nested_rich_texts);
            
            Ok(())
        } else {
            Err(ConversionError::UnsupportedElement("Not an Emph element".to_string()))
        }
    }
}

/// Handler for strikeout (strikethrough) text elements
///
/// This handler applies strikethrough formatting to the contained content.
pub struct StrikeoutHandler {
    converter: Rc<RefCell<Option<PandocTextConverter>>>,
}

impl StrikeoutHandler {
    pub fn new(converter: Rc<RefCell<Option<PandocTextConverter>>>) -> Self {
        Self { converter }
    }
}

impl PandocElementHandler for StrikeoutHandler {
    fn can_handle(&self, element: &Inline) -> bool {
        matches!(element, Inline::Strikeout(_))
    }

    fn handle(&self, element: &Inline, builder: &mut TextBuilder) -> Result<(), ConversionError> {
        if let Inline::Strikeout(content) = element {
            // Create a nested builder with strikethrough formatting
            let mut nested_builder = builder.apply_formatting(|annotations| {
                annotations.strikethrough = true;
            })?;
            
            // Convert the nested content
            if let Ok(ref_conv) = self.converter.try_borrow() {
                if let Some(converter) = ref_conv.as_ref() {
                    converter.convert_content(content, &mut nested_builder)?;
                } else {
                    // If no converter available, just add the raw text
                    for elem in content {
                        if let Inline::Str(text) = elem {
                            nested_builder.append_text(text);
                        } else if let Inline::Space = elem {
                            nested_builder.append_text(" ");
                        }
                    }
                }
            } else {
                // Fallback if we can't borrow the converter
                for elem in content {
                    if let Inline::Str(text) = elem {
                        nested_builder.append_text(text);
                    } else if let Inline::Space = elem {
                        nested_builder.append_text(" ");
                    }
                }
            }
            
            // Add the nested builder's content to the main builder
            let nested_rich_texts = nested_builder.build()?;
            builder.rich_texts.extend(nested_rich_texts);
            
            Ok(())
        } else {
            Err(ConversionError::UnsupportedElement("Not a Strikeout element".to_string()))
        }
    }
}

/// Handler for inline code elements
///
/// This handler applies code (monospace) formatting to the text.
pub struct CodeHandler {
    converter: Rc<RefCell<Option<PandocTextConverter>>>,
}

impl CodeHandler {
    pub fn new(converter: Rc<RefCell<Option<PandocTextConverter>>>) -> Self {
        Self { converter }
    }
}

impl PandocElementHandler for CodeHandler {
    fn can_handle(&self, element: &Inline) -> bool {
        matches!(element, Inline::Code(_, _))
    }

    fn handle(&self, element: &Inline, builder: &mut TextBuilder) -> Result<(), ConversionError> {
        if let Inline::Code(_, text) = element {
            // Create a nested builder with code formatting
            let mut nested_builder = builder.apply_formatting(|annotations| {
                annotations.code = true;
            })?;
            
            // Add the code text directly
            nested_builder.append_text(text);
            
            // Add the nested builder's content to the main builder
            let nested_rich_texts = nested_builder.build()?;
            builder.rich_texts.extend(nested_rich_texts);
            
            Ok(())
        } else {
            Err(ConversionError::UnsupportedElement("Not a Code element".to_string()))
        }
    }
}

/// Handler for mathematical expressions
///
/// This handler converts Pandoc math elements to Notion equation objects.
pub struct MathHandler;

impl PandocElementHandler for MathHandler {
    fn can_handle(&self, element: &Inline) -> bool {
        matches!(element, Inline::Math(_, _))
    }

    fn handle(&self, element: &Inline, builder: &mut TextBuilder) -> Result<(), ConversionError> {
        if let Inline::Math(_, expression) = element {
            // Create an equation rich text object
            builder.commit_equation(expression)?;
            Ok(())
        } else {
            Err(ConversionError::UnsupportedElement("Not a Math element".to_string()))
        }
    }
}

/// Handler for hyperlink elements
///
/// This handler creates Notion text with link attributes.
pub struct LinkHandler {
    converter: Rc<RefCell<Option<PandocTextConverter>>>,
}

impl LinkHandler {
    pub fn new(converter: Rc<RefCell<Option<PandocTextConverter>>>) -> Self {
        Self { converter }
    }
}

impl PandocElementHandler for LinkHandler {
    fn can_handle(&self, element: &Inline) -> bool {
        matches!(element, Inline::Link(_, _, _))
    }

    fn handle(&self, element: &Inline, builder: &mut TextBuilder) -> Result<(), ConversionError> {
        if let Inline::Link(_, content, Target { url, .. }) = element {
            // First commit any existing text
            builder.commit_current_text()?;
            
            // Create a nested builder with the link URL
            let mut nested_builder = TextBuilder::new();
            nested_builder.annotations = builder.annotations;
            nested_builder.set_link(url);
            
            // Convert the link content
            if let Ok(ref_conv) = self.converter.try_borrow() {
                if let Some(converter) = ref_conv.as_ref() {
                    converter.convert_content(content, &mut nested_builder)?;
                } else {
                    // If no converter available, just add the raw text
                    for elem in content {
                        if let Inline::Str(text) = elem {
                            nested_builder.append_text(text);
                        } else if let Inline::Space = elem {
                            nested_builder.append_text(" ");
                        }
                    }
                }
            } else {
                // Fallback if we can't borrow the converter
                for elem in content {
                    if let Inline::Str(text) = elem {
                        nested_builder.append_text(text);
                    } else if let Inline::Space = elem {
                        nested_builder.append_text(" ");
                    }
                }
            }
            
            // Add the nested builder's content to the main builder
            let nested_rich_texts = nested_builder.build()?;
            builder.rich_texts.extend(nested_rich_texts);
            
            Ok(())
        } else {
            Err(ConversionError::UnsupportedElement("Not a Link element".to_string()))
        }
    }
}

/// Handler for span elements with class attributes
///
/// This handler processes spans with special class attributes,
/// converting them to Notion's colors and underline formatting.
pub struct SpanHandler {
    converter: Rc<RefCell<Option<PandocTextConverter>>>,
}

impl SpanHandler {
    pub fn new(converter: Rc<RefCell<Option<PandocTextConverter>>>) -> Self {
        Self { converter }
    }
    
    /// Extract a Notion color from CSS class names
    ///
    /// This parses color classes in the format "color-red" or "color-blue-background"
    /// and converts them to the appropriate Notion color enum values.
    fn parse_color(&self, classes: &[String]) -> Option<Color> {
        for class in classes {
            if class == "underline" {
                // This is an underline span, not a color
                return None;
            }
            
            if class.starts_with("color-") {
                let color_part = &class["color-".len()..];
                
                // Handle background colors
                if color_part.ends_with("-background") {
                    let base_color = &color_part[0..color_part.len() - 11]; // Remove "-background"
                    return match base_color {
                        "red" => Some(Color::RedBackground),
                        "blue" => Some(Color::BlueBackground),
                        "green" => Some(Color::GreenBackground),
                        "yellow" => Some(Color::YellowBackground),
                        "orange" => Some(Color::OrangeBackground),
                        "pink" => Some(Color::PinkBackground),
                        "purple" => Some(Color::PurpleBackground),
                        "brown" => Some(Color::BrownBackground),
                        "gray" => Some(Color::GrayBackground),
                        _ => None,
                    };
                } else {
                    // Regular colors
                    return match color_part {
                        "red" => Some(Color::Red),
                        "blue" => Some(Color::Blue),
                        "green" => Some(Color::Green),
                        "yellow" => Some(Color::Yellow),
                        "orange" => Some(Color::Orange),
                        "pink" => Some(Color::Pink),
                        "purple" => Some(Color::Purple),
                        "brown" => Some(Color::Brown),
                        "gray" => Some(Color::Gray),
                        _ => None,
                    };
                }
            }
        }
        None
    }
    
    /// Check if the span has the "underline" class
    ///
    /// Pandoc doesn't have a native underline format, so underlined text
    /// is typically represented as a span with an "underline" class.
    fn is_underline(&self, classes: &[String]) -> bool {
        classes.iter().any(|class| class == "underline")
    }
}

impl PandocElementHandler for SpanHandler {
    fn can_handle(&self, element: &Inline) -> bool {
        matches!(element, Inline::Span(_, _))
    }

    fn handle(&self, element: &Inline, builder: &mut TextBuilder) -> Result<(), ConversionError> {
        if let Inline::Span(attr, content) = element {
            // Create a nested builder with the appropriate formatting
            let mut nested_builder = builder.apply_formatting(|annotations| {
                // Check if this is an underline span
                if self.is_underline(&attr.classes) {
                    annotations.underline = true;
                }
                
                // Check for color styling
                if let Some(color) = self.parse_color(&attr.classes) {
                    annotations.color = color;
                }
            })?;
            
            // Convert the nested content
            if let Ok(ref_conv) = self.converter.try_borrow() {
                if let Some(converter) = ref_conv.as_ref() {
                    converter.convert_content(content, &mut nested_builder)?;
                } else {
                    // If no converter available, just add the raw text
                    for elem in content {
                        if let Inline::Str(text) = elem {
                            nested_builder.append_text(text);
                        } else if let Inline::Space = elem {
                            nested_builder.append_text(" ");
                        }
                    }
                }
            } else {
                // Fallback if we can't borrow the converter
                for elem in content {
                    if let Inline::Str(text) = elem {
                        nested_builder.append_text(text);
                    } else if let Inline::Space = elem {
                        nested_builder.append_text(" ");
                    }
                }
            }
            
            // Add the nested builder's content to the main builder
            let nested_rich_texts = nested_builder.build()?;
            builder.rich_texts.extend(nested_rich_texts);
            
            Ok(())
        } else {
            Err(ConversionError::UnsupportedElement("Not a Span element".to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::notion::text::{Color, RichTextObject};
    use pandoc_types::definition::{Attr, Inline, MathType, Target};

    // Helper function to create basic Pandoc inline elements
    fn create_text_inline(text: &str) -> Vec<Inline> {
        vec![Inline::Str(text.to_string())]
    }

    #[test]
    fn test_convert_simple_text() {
        let converter = PandocTextConverter::new();
        let pandoc_elements = create_text_inline("Hello world");
        
        let result = converter.convert(&pandoc_elements).unwrap();
        assert_eq!(result.len(), 1);
        
        // Verify the content is preserved
        match &result[0] {
            RichTextObject::Text { text, .. } => {
                assert_eq!(text.content, "Hello world");
                assert!(text.link.is_none());
            },
            _ => panic!("Expected Text variant"),
        }
    }

    #[test]
    fn test_convert_spaces() {
        let converter = PandocTextConverter::new();
        let pandoc_elements = vec![
            Inline::Str("Hello".to_string()),
            Inline::Space,
            Inline::Str("world".to_string()),
        ];
        
        let result = converter.convert(&pandoc_elements).unwrap();
        assert_eq!(result.len(), 1);
        
        // Verify spaces are preserved
        match &result[0] {
            RichTextObject::Text { text, .. } => {
                assert_eq!(text.content, "Hello world");
            },
            _ => panic!("Expected Text variant"),
        }
    }

    #[test]
    fn test_convert_multiple_spaces() {
        let converter = PandocTextConverter::new();
        let pandoc_elements = vec![
            Inline::Str("Hello".to_string()),
            Inline::Space,
            Inline::Space,
            Inline::Str("world".to_string()),
        ];
        
        let result = converter.convert(&pandoc_elements).unwrap();
        assert_eq!(result.len(), 1);
        
        // Verify multiple spaces are preserved
        match &result[0] {
            RichTextObject::Text { text, .. } => {
                assert_eq!(text.content, "Hello  world");
            },
            _ => panic!("Expected Text variant"),
        }
    }
    
    #[test]
    fn test_convert_line_breaks() {
        let converter = PandocTextConverter::new();
        let pandoc_elements = vec![
            Inline::Str("Hello".to_string()),
            Inline::LineBreak,
            Inline::Str("world".to_string()),
        ];
        
        let result = converter.convert(&pandoc_elements).unwrap();
        assert_eq!(result.len(), 1);
        
        // Verify line breaks are preserved
        match &result[0] {
            RichTextObject::Text { text, .. } => {
                assert_eq!(text.content, "Hello\nworld");
            },
            _ => panic!("Expected Text variant"),
        }
    }
    
    #[test]
    fn test_convert_bold_text() {
        let converter = PandocTextConverter::new();
        let pandoc_elements = vec![
            Inline::Strong(vec![Inline::Str("Bold text".to_string())]),
        ];
        
        let result = converter.convert(&pandoc_elements).unwrap();
        assert_eq!(result.len(), 1);
        
        // Verify bold formatting is applied
        match &result[0] {
            RichTextObject::Text { text, annotations, .. } => {
                assert_eq!(text.content, "Bold text");
                if let Some(ann) = annotations {
                    assert!(ann.bold);
                } else {
                    panic!("Expected annotations");
                }
            },
            _ => panic!("Expected Text variant"),
        }
    }
    
    #[test]
    fn test_convert_italic_text() {
        let converter = PandocTextConverter::new();
        let pandoc_elements = vec![
            Inline::Emph(vec![Inline::Str("Italic text".to_string())]),
        ];
        
        let result = converter.convert(&pandoc_elements).unwrap();
        assert_eq!(result.len(), 1);
        
        // Verify italic formatting is applied
        match &result[0] {
            RichTextObject::Text { text, annotations, .. } => {
                assert_eq!(text.content, "Italic text");
                if let Some(ann) = annotations {
                    assert!(ann.italic);
                } else {
                    panic!("Expected annotations");
                }
            },
            _ => panic!("Expected Text variant"),
        }
    }
    
    #[test]
    fn test_convert_strikethrough_text() {
        let converter = PandocTextConverter::new();
        let pandoc_elements = vec![
            Inline::Strikeout(vec![Inline::Str("Strikethrough text".to_string())]),
        ];
        
        let result = converter.convert(&pandoc_elements).unwrap();
        assert_eq!(result.len(), 1);
        
        // Verify strikethrough formatting is applied
        match &result[0] {
            RichTextObject::Text { text, annotations, .. } => {
                assert_eq!(text.content, "Strikethrough text");
                if let Some(ann) = annotations {
                    assert!(ann.strikethrough);
                } else {
                    panic!("Expected annotations");
                }
            },
            _ => panic!("Expected Text variant"),
        }
    }
    
    #[test]
    fn test_convert_code() {
        let converter = PandocTextConverter::new();
        let pandoc_elements = vec![
            Inline::Code(Attr::default(), "code example".to_string()),
        ];
        
        let result = converter.convert(&pandoc_elements).unwrap();
        assert_eq!(result.len(), 1);
        
        // Verify code formatting is applied
        match &result[0] {
            RichTextObject::Text { text, annotations, .. } => {
                assert_eq!(text.content, "code example");
                if let Some(ann) = annotations {
                    assert!(ann.code);
                } else {
                    panic!("Expected annotations");
                }
            },
            _ => panic!("Expected Text variant"),
        }
    }
    
    #[test]
    fn test_convert_math() {
        let converter = PandocTextConverter::new();
        let pandoc_elements = vec![
            Inline::Math(MathType::InlineMath, "E=mc^2".to_string()),
        ];
        
        let result = converter.convert(&pandoc_elements).unwrap();
        assert_eq!(result.len(), 1);
        
        // Verify it's an equation
        match &result[0] {
            RichTextObject::Equation { equation, .. } => {
                assert_eq!(equation.expression, "E=mc^2");
            },
            _ => panic!("Expected Equation variant"),
        }
    }
    
    #[test]
    fn test_convert_link() {
        let converter = PandocTextConverter::new();
        let pandoc_elements = vec![
            Inline::Link(
                Attr::default(),
                vec![Inline::Str("Link text".to_string())],
                Target {
                    url: "https://example.com".to_string(),
                    title: String::new(),
                },
            ),
        ];
        
        let result = converter.convert(&pandoc_elements).unwrap();
        assert_eq!(result.len(), 1);
        
        // Verify link is properly set
        match &result[0] {
            RichTextObject::Text { text, href, .. } => {
                assert_eq!(text.content, "Link text");
                assert!(text.link.is_some());
                if let Some(link) = &text.link {
                    assert_eq!(link.url, "https://example.com");
                }
                assert_eq!(href, &Some("https://example.com".to_string()));
            },
            _ => panic!("Expected Text variant with link"),
        }
    }
    
    #[test]
    fn test_convert_nested_formatting() {
        let converter = PandocTextConverter::new();
        let pandoc_elements = vec![
            Inline::Strong(vec![
                Inline::Emph(vec![
                    Inline::Str("Bold and italic".to_string())
                ])
            ]),
        ];
        
        let result = converter.convert(&pandoc_elements).unwrap();
        assert_eq!(result.len(), 1);
        
        // Verify nested formatting is preserved
        match &result[0] {
            RichTextObject::Text { text, annotations, .. } => {
                assert_eq!(text.content, "Bold and italic");
                if let Some(ann) = annotations {
                    assert!(ann.bold);
                    assert!(ann.italic);
                } else {
                    panic!("Expected annotations");
                }
            },
            _ => panic!("Expected Text variant"),
        }
    }
    
    #[test]
    fn test_convert_color() {
        let converter = PandocTextConverter::new();
        
        // Create a span with a color class
        let mut attr = Attr::default();
        attr.classes.push("color-red".to_string());
        
        let pandoc_elements = vec![
            Inline::Span(
                attr,
                vec![Inline::Str("Colored text".to_string())]
            ),
        ];
        
        let result = converter.convert(&pandoc_elements).unwrap();
        assert_eq!(result.len(), 1);
        
        // Verify color is applied
        match &result[0] {
            RichTextObject::Text { text, annotations, .. } => {
                assert_eq!(text.content, "Colored text");
                if let Some(ann) = annotations {
                    assert_eq!(ann.color, Color::Red);
                } else {
                    panic!("Expected annotations");
                }
            },
            _ => panic!("Expected Text variant"),
        }
    }
    
    #[test]
    fn test_convert_background_color() {
        let converter = PandocTextConverter::new();
        
        // Create a span with a background color class
        let mut attr = Attr::default();
        attr.classes.push("color-blue-background".to_string());
        
        let pandoc_elements = vec![
            Inline::Span(
                attr,
                vec![Inline::Str("Background colored text".to_string())]
            ),
        ];
        
        let result = converter.convert(&pandoc_elements).unwrap();
        assert_eq!(result.len(), 1);
        
        // Verify background color is applied
        match &result[0] {
            RichTextObject::Text { text, annotations, .. } => {
                assert_eq!(text.content, "Background colored text");
                if let Some(ann) = annotations {
                    assert_eq!(ann.color, Color::BlueBackground);
                } else {
                    panic!("Expected annotations");
                }
            },
            _ => panic!("Expected Text variant"),
        }
    }
    
    #[test]
    fn test_convert_underline() {
        let converter = PandocTextConverter::new();
        
        // Create a span with underline class
        let mut attr = Attr::default();
        attr.classes.push("underline".to_string());
        
        let pandoc_elements = vec![
            Inline::Span(
                attr,
                vec![Inline::Str("Underlined text".to_string())]
            ),
        ];
        
        let result = converter.convert(&pandoc_elements).unwrap();
        assert_eq!(result.len(), 1);
        
        // Verify underline is applied
        match &result[0] {
            RichTextObject::Text { text, annotations, .. } => {
                assert_eq!(text.content, "Underlined text");
                if let Some(ann) = annotations {
                    assert!(ann.underline);
                } else {
                    panic!("Expected annotations");
                }
            },
            _ => panic!("Expected Text variant"),
        }
    }
    
    #[test]
    fn test_convert_complex_formatting() {
        let converter = PandocTextConverter::new();
        
        // Create a complex element with multiple formatting types
        let mut color_attr = Attr::default();
        color_attr.classes.push("color-red".to_string());
        
        let pandoc_elements = vec![
            Inline::Span(
                color_attr,
                vec![
                    Inline::Strong(vec![
                        Inline::Emph(vec![
                            Inline::Str("Complex ".to_string()),
                        ]),
                        Inline::Str("formatting".to_string()),
                    ]),
                ]
            ),
        ];
        
        let result = converter.convert(&pandoc_elements).unwrap();
        
        // Verify all formatting is correctly applied
        // Note: In this case, we might get multiple rich text objects due to
        // the way the formatting is applied across different text segments
        assert!(result.len() >= 1);
        
        // Ensure all content is present
        let all_text = result.iter().map(|rt| match rt {
            RichTextObject::Text { text, .. } => text.content.clone(),
            RichTextObject::Equation { equation, .. } => equation.expression.clone(),
            _ => String::new(),
        }).collect::<Vec<String>>().join("");
        
        assert!(all_text.contains("Complex "));
        assert!(all_text.contains("formatting"));
    }
    
    #[test]
    fn test_convert_unicode_characters() {
        let converter = PandocTextConverter::new();
        let pandoc_elements = vec![
            Inline::Str("Unicode: 你好, こんにちは, 안녕하세요, Привет, 😊".to_string()),
        ];
        
        let result = converter.convert(&pandoc_elements).unwrap();
        assert_eq!(result.len(), 1);
        
        // Verify unicode characters are preserved
        match &result[0] {
            RichTextObject::Text { text, .. } => {
                assert_eq!(text.content, "Unicode: 你好, こんにちは, 안녕하세요, Привет, 😊");
            },
            _ => panic!("Expected Text variant"),
        }
    }
    
    #[test]
    fn test_convert_empty_content() {
        let converter = PandocTextConverter::new();
        let pandoc_elements: Vec<Inline> = vec![];
        
        let result = converter.convert(&pandoc_elements).unwrap();
        assert_eq!(result.len(), 0);
    }
}