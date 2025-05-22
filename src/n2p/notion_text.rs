use notion_client::objects::rich_text::{Annotations, RichText, TextColor};
use pandoc_types::definition::{Attr, Inline, MathType, Target};

/// Converts Notion API rich text objects to Pandoc inline elements
pub struct NotionTextConverter;

impl NotionTextConverter {
    /// Convert a Vec of Notion API RichText objects to Pandoc Inline elements
    pub fn convert(rich_texts: &[RichText]) -> Vec<Inline> {
        let mut result = Vec::new();

        for rich_text in rich_texts.iter() {
            let inline_elements = Self::convert_single_rich_text(rich_text);
            result.extend(inline_elements);
        }

        result
    }

    /// Convert a single RichText object to Pandoc Inline elements
    fn convert_single_rich_text(rich_text: &RichText) -> Vec<Inline> {
        // Handle empty case
        if let RichText::None = rich_text {
            return vec![];
        }

        // Extract the plain text
        let plain_text = rich_text.plain_text().unwrap_or_default();

        // Start with the basic content as inline elements
        let content = match rich_text {
            RichText::Text { text, href, .. } => {
                let inline_elements = Self::text_to_inline(&plain_text);

                // If there's a link, wrap the inline elements in a Link
                if let Some(link) = &text.link {
                    vec![Inline::Link(
                        Attr::default(),
                        inline_elements,
                        Target {
                            url: link.url.clone(),
                            title: String::new(),
                        },
                    )]
                } else if let Some(url) = href {
                    // If there's an href but no explicit link in the text
                    vec![Inline::Link(
                        Attr::default(),
                        inline_elements,
                        Target {
                            url: url.clone(),
                            title: String::new(),
                        },
                    )]
                } else {
                    inline_elements
                }
            }
            RichText::Equation { equation, .. } => {
                vec![Inline::Math(
                    MathType::InlineMath,
                    equation.expression.clone(),
                )]
            }
            RichText::Mention { href, .. } => {
                // For mentions, use the plain text representation
                let inline_elements = Self::text_to_inline(&plain_text);

                // If there's a URL in href, make it a link
                if let Some(url) = href {
                    vec![Inline::Link(
                        Attr::default(),
                        inline_elements,
                        Target {
                            url: url.clone(),
                            title: String::new(),
                        },
                    )]
                } else {
                    inline_elements
                }
            }
            RichText::None => vec![], // This case is already handled above
        };

        // Extract and apply annotations
        let annotations = match rich_text {
            RichText::Text { annotations, .. } => annotations.clone().unwrap_or_default(),
            RichText::Equation { annotations, .. } => annotations.clone(),
            RichText::Mention { annotations, .. } => annotations.clone(),
            RichText::None => Annotations::default(),
        };

        Self::apply_annotations(content, &annotations)
    }

    /// Convert text string to inline elements, properly handling spaces
    fn text_to_inline(text: &str) -> Vec<Inline> {
        if text.is_empty() {
            return vec![];
        }

        let mut result = Vec::new();
        let mut current_word = String::new();

        for c in text.chars() {
            match c {
                // Horizontal whitespace becomes Space
                ' ' | '\t' => {
                    if !current_word.is_empty() {
                        result.push(Inline::Str(current_word));
                        current_word = String::new();
                    }
                    result.push(Inline::Space);
                }
                // Newlines become LineBreak to ensure proper line separation
                // This ensures each newline in Notion content creates a visible line break
                // in the output document rather than soft wrapping
                '\n' => {
                    if !current_word.is_empty() {
                        result.push(Inline::Str(current_word));
                        current_word = String::new();
                    }
                    result.push(Inline::LineBreak);
                }
                // All other characters are part of text
                _ => {
                    current_word.push(c);
                }
            }
        }

        // Add the final word if there is one
        if !current_word.is_empty() {
            result.push(Inline::Str(current_word));
        }

        result
    }

    /// Apply text annotations to a list of inline elements
    fn apply_annotations(inlines: Vec<Inline>, annotations: &Annotations) -> Vec<Inline> {
        let mut result = inlines;

        // Apply formatting in a specific order

        // Apply underline
        if annotations.underline {
            // Pandoc doesn't have native underline, use a Span with a class
            let mut attr = Attr::default();
            attr.classes.push("underline".to_string());
            result = vec![Inline::Span(attr, result)];
        }

        // Apply strikethrough
        if annotations.strikethrough {
            result = vec![Inline::Strikeout(result)];
        }

        // Apply italic
        if annotations.italic {
            result = vec![Inline::Emph(result)];
        }

        // Apply bold
        if annotations.bold {
            result = vec![Inline::Strong(result)];
        }

        // Apply code (which collapses all other formatting)
        if annotations.code {
            // For code, we need to convert the inlines to a string
            let code_text = Self::inlines_to_string(&result);
            result = vec![Inline::Code(Attr::default(), code_text)];
        }

        // Apply color last (outermost)
        if annotations.color != TextColor::Default {
            // Convert color to a class name (e.g., "red" or "red-background")
            let color_class = match annotations.color {
                TextColor::Default => return result, // Skip if default
                TextColor::Gray => "color-gray",
                TextColor::Brown => "color-brown",
                TextColor::Orange => "color-orange",
                TextColor::Yellow => "color-yellow",
                TextColor::Green => "color-green",
                TextColor::Blue => "color-blue",
                TextColor::Purple => "color-purple",
                TextColor::Pink => "color-pink",
                TextColor::Red => "color-red",
                TextColor::GrayBackground => "color-gray-background",
                TextColor::BrownBackground => "color-brown-background",
                TextColor::OrangeBackground => "color-orange-background",
                TextColor::YellowBackground => "color-yellow-background",
                TextColor::GreenBackground => "color-green-background",
                TextColor::BlueBackground => "color-blue-background",
                TextColor::PurpleBackground => "color-purple-background",
                TextColor::PinkBackground => "color-pink-background",
                TextColor::RedBackground => "color-red-background",
            };

            let mut attr = Attr::default();
            attr.classes.push(color_class.to_string());
            result = vec![Inline::Span(attr, result)];
        }

        result
    }

    /// Convert a list of inline elements to a string (for code blocks)
    fn inlines_to_string(inlines: &[Inline]) -> String {
        let mut result = String::new();

        for inline in inlines {
            match inline {
                Inline::Str(s) => result.push_str(s),
                Inline::Space => result.push(' '),
                Inline::SoftBreak => result.push('\n'), // SoftBreak is rendered as a newline just like LineBreak
                Inline::LineBreak => result.push('\n'),
                Inline::Emph(children)
                | Inline::Strong(children)
                | Inline::Strikeout(children)
                | Inline::Span(_, children)
                | Inline::Link(_, children, _) => {
                    result.push_str(&Self::inlines_to_string(children));
                }
                // Handle other inline types as needed
                _ => {}
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use notion_client::objects::rich_text::{Annotations, Equation, Link, Text};

    // Helper function to create a Text rich text object
    fn create_text_rich_text(
        content: &str,
        annotations: Option<Annotations>,
        link: Option<Link>,
    ) -> RichText {
        let text = Text {
            content: content.to_string(),
            link: link,
        };

        RichText::Text {
            text,
            annotations,
            plain_text: Some(content.to_string()),
            href: None,
        }
    }

    // Helper function to create an Equation rich text object
    fn create_equation_rich_text(expression: &str) -> RichText {
        let equation = Equation {
            expression: expression.to_string(),
        };

        RichText::Equation {
            equation,
            annotations: Annotations::default(),
            plain_text: expression.to_string(),
            href: None,
        }
    }

    #[test]
    fn test_convert_plain_text() {
        // Create a simple text object
        let rich_text = create_text_rich_text("Hello world", None, None);

        // Convert it to Pandoc inline elements
        let result = NotionTextConverter::convert_single_rich_text(&rich_text);

        // Expected result: Str("Hello") + Space + Str("world")
        assert_eq!(result.len(), 3);
        assert!(matches!(result[0], Inline::Str(ref s) if s == "Hello"));
        assert!(matches!(result[1], Inline::Space));
        assert!(matches!(result[2], Inline::Str(ref s) if s == "world"));
    }

    #[test]
    fn test_convert_bold_text() {
        // Create a bold text object
        let mut annotations = Annotations::default();
        annotations.bold = true;

        let rich_text = create_text_rich_text("Bold text", Some(annotations), None);

        // Convert it to Pandoc inline elements
        let result = NotionTextConverter::convert_single_rich_text(&rich_text);

        // Result should be a Strong element containing the text
        assert_eq!(result.len(), 1);
        assert!(matches!(result[0], Inline::Strong(_)));

        if let Inline::Strong(inner) = &result[0] {
            assert_eq!(inner.len(), 3); // Str("Bold") + Space + Str("text")
        } else {
            panic!("Expected Strong element");
        }
    }

    #[test]
    fn test_convert_italic_text() {
        // Create an italic text object
        let mut annotations = Annotations::default();
        annotations.italic = true;

        let rich_text = create_text_rich_text("Italic text", Some(annotations), None);

        // Convert it to Pandoc inline elements
        let result = NotionTextConverter::convert_single_rich_text(&rich_text);

        // Result should be an Emph element containing the text
        assert_eq!(result.len(), 1);
        assert!(matches!(result[0], Inline::Emph(_)));
    }

    #[test]
    fn test_convert_strikethrough_text() {
        // Create a strikethrough text object
        let mut annotations = Annotations::default();
        annotations.strikethrough = true;

        let rich_text = create_text_rich_text("Strikethrough text", Some(annotations), None);

        // Convert it to Pandoc inline elements
        let result = NotionTextConverter::convert_single_rich_text(&rich_text);

        // Result should be a Strikeout element containing the text
        assert_eq!(result.len(), 1);
        assert!(matches!(result[0], Inline::Strikeout(_)));
    }

    #[test]
    fn test_convert_underline_text() {
        // Create an underlined text object
        let mut annotations = Annotations::default();
        annotations.underline = true;

        let rich_text = create_text_rich_text("Underlined text", Some(annotations), None);

        // Convert it to Pandoc inline elements
        let result = NotionTextConverter::convert_single_rich_text(&rich_text);

        // Result should be a Span with "underline" class
        assert_eq!(result.len(), 1);
        assert!(matches!(result[0], Inline::Span(_, _)));

        if let Inline::Span(attr, _) = &result[0] {
            assert!(attr.classes.contains(&"underline".to_string()));
        } else {
            panic!("Expected Span element");
        }
    }

    #[test]
    fn test_convert_code_text() {
        // Create a code text object
        let mut annotations = Annotations::default();
        annotations.code = true;

        let rich_text = create_text_rich_text("code sample", Some(annotations), None);

        // Convert it to Pandoc inline elements
        let result = NotionTextConverter::convert_single_rich_text(&rich_text);

        // Result should be a Code element
        assert_eq!(result.len(), 1);
        assert!(matches!(result[0], Inline::Code(_, _)));

        if let Inline::Code(_, content) = &result[0] {
            assert_eq!(content, "code sample");
        } else {
            panic!("Expected Code element");
        }
    }

    #[test]
    fn test_convert_colored_text() {
        // Create a colored text object
        let mut annotations = Annotations::default();
        annotations.color = TextColor::Red;

        let rich_text = create_text_rich_text("Colored text", Some(annotations), None);

        // Convert it to Pandoc inline elements
        let result = NotionTextConverter::convert_single_rich_text(&rich_text);

        // Result should be a Span with appropriate color class
        assert_eq!(result.len(), 1);
        assert!(matches!(result[0], Inline::Span(_, _)));

        if let Inline::Span(attr, _) = &result[0] {
            assert!(attr.classes.contains(&"color-red".to_string()));
        } else {
            panic!("Expected Span element");
        }
    }

    #[test]
    fn test_convert_link() {
        // Create a text with link
        let link = Link {
            url: "https://example.com".to_string(),
        };

        let rich_text = create_text_rich_text("Link text", None, Some(link));

        // Convert it to Pandoc inline elements
        let result = NotionTextConverter::convert_single_rich_text(&rich_text);

        // Result should be a Link element
        assert_eq!(result.len(), 1);
        assert!(matches!(result[0], Inline::Link(_, _, _)));

        if let Inline::Link(_, content, target) = &result[0] {
            assert_eq!(target.url, "https://example.com");
            assert_eq!(content.len(), 3); // Str("Link") + Space + Str("text")
        } else {
            panic!("Expected Link element");
        }
    }

    #[test]
    fn test_convert_equation() {
        // Create an equation
        let rich_text = create_equation_rich_text("E=mc^2");

        // Convert it to Pandoc inline elements
        let result = NotionTextConverter::convert_single_rich_text(&rich_text);

        // Result should be a Math element
        assert_eq!(result.len(), 1);
        assert!(matches!(result[0], Inline::Math(_, _)));

        if let Inline::Math(math_type, content) = &result[0] {
            assert!(matches!(math_type, MathType::InlineMath));
            assert_eq!(content, "E=mc^2");
        } else {
            panic!("Expected Math element");
        }
    }

    #[test]
    fn test_convert_multiple_rich_texts() {
        // Create multiple rich text objects
        let text1 = create_text_rich_text("Hello", None, None);

        let mut annotations = Annotations::default();
        annotations.bold = true;
        let text2 = create_text_rich_text("bold", Some(annotations), None);

        let rich_texts = vec![text1, text2];

        // Convert the array to Pandoc inline elements
        let result = NotionTextConverter::convert(&rich_texts);

        // Expected: Str("Hello") + Inline::Strong with "bold"
        assert!(result.len() > 0);

        // Check the first element is "Hello"
        assert!(matches!(result[0], Inline::Str(ref s) if s == "Hello"));

        // Check for the bold text (might be preceded by a space depending on implementation)
        let bold_index = result
            .iter()
            .position(|inline| matches!(inline, Inline::Strong(_)));
        assert!(bold_index.is_some());
    }

    #[test]
    fn test_convert_none_rich_text() {
        // Create a None rich text
        let rich_text = RichText::None;

        // Convert it to Pandoc inline elements
        let result = NotionTextConverter::convert_single_rich_text(&rich_text);

        // Result should be empty
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_text_to_inline() {
        // Test basic text
        let result = NotionTextConverter::text_to_inline("Hello world");
        assert_eq!(result.len(), 3);

        // Test with newlines
        let result = NotionTextConverter::text_to_inline("Hello\nworld");
        assert_eq!(result.len(), 3);
        assert!(matches!(result[1], Inline::SoftBreak));

        // Test with multiple spaces
        let result = NotionTextConverter::text_to_inline("Hello  world");
        assert_eq!(result.len(), 4);
        assert!(matches!(result[1], Inline::Space));
        assert!(matches!(result[2], Inline::Space));
    }

    #[test]
    fn test_inlines_to_string() {
        // Create a simple inline structure
        let inlines = vec![
            Inline::Str("Hello".to_string()),
            Inline::Space,
            Inline::Str("world".to_string()),
        ];

        // Convert back to string
        let result = NotionTextConverter::inlines_to_string(&inlines);

        // Check result
        assert_eq!(result, "Hello world");
    }

    #[test]
    fn test_newline_to_linebreak() {
        // Create rich text with newlines
        let rich_text = create_text_rich_text("Line 1\nLine 2\nLine 3", None, None);
        
        // Convert to Pandoc inlines
        let result = NotionTextConverter::convert(&[rich_text]);
        
        // Expected structure should include LineBreak elements
        // We expect at least 5 elements: "Line 1", LineBreak, "Line 2", LineBreak, "Line 3"
        assert!(result.len() >= 5);
        
        // Find LineBreak elements and verify they exist
        let line_breaks = result.iter().filter(|inline| 
            matches!(inline, Inline::LineBreak)
        ).count();
        
        // Should have at least 2 line breaks
        assert_eq!(line_breaks, 2, "Expected 2 LineBreak elements");
    }
}
