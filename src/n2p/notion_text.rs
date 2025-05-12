use crate::notion::text::{Annotations, RichTextObject};
use pandoc_types::definition::{Attr, Inline, MathType, Target};

/// Converts Notion rich text objects to Pandoc inline elements
pub struct NotionTextConverter;

impl NotionTextConverter {
    /// Convert a Vec of Notion RichTextObject objects to Pandoc Inline elements
    pub fn convert(rich_texts: &[RichTextObject]) -> Vec<Inline> {
        let mut result = Vec::new();

        for (i, rich_text) in rich_texts.iter().enumerate() {
            let mut inline_elements = Self::convert_single_rich_text(rich_text);

            // Add a space between elements if needed
            if i < rich_texts.len() - 1 {
                // Extract plain text for space checks
                let current_text = Self::get_plain_text(rich_text);
                let next_text = Self::get_plain_text(&rich_texts[i + 1]);
                
                if !current_text.ends_with(' ') && !next_text.starts_with(' ') {
                    inline_elements.push(Inline::Space);
                }
            }

            result.extend(inline_elements);
        }

        result
    }

    /// Helper to extract plain text from RichTextObject
    fn get_plain_text(rich_text: &RichTextObject) -> String {
        match rich_text {
            RichTextObject::Text { plain_text, text, .. } => {
                plain_text.clone().unwrap_or_else(|| text.content.clone())
            }
            RichTextObject::Equation { plain_text, equation, .. } => {
                plain_text.clone().unwrap_or_else(|| equation.expression.clone())
            }
            RichTextObject::Mention { plain_text, .. } => {
                plain_text.clone().unwrap_or_default()
            }
        }
    }

    /// Convert a single RichTextObject to Pandoc Inline elements
    fn convert_single_rich_text(rich_text: &RichTextObject) -> Vec<Inline> {
        // Start with the basic content as inline elements
        let content = match rich_text {
            RichTextObject::Text { text, href, .. } => {
                let inline_elements = Self::text_to_inline(&text.content);

                // If there's a link, wrap the inline elements in a Link
                if let Some(link) = &text.link {
                    vec![Inline::Link(
                        Attr::default(),
                        inline_elements,
                        Target { 
                            url: link.url.clone(), 
                            title: String::new() 
                        },
                    )]
                } else if let Some(url) = href {
                    // If there's an href but no explicit link
                    vec![Inline::Link(
                        Attr::default(),
                        inline_elements,
                        Target { 
                            url: url.clone(), 
                            title: String::new() 
                        },
                    )]
                } else {
                    inline_elements
                }
            }
            RichTextObject::Equation { equation, .. } => {
                vec![Inline::Math(
                    MathType::InlineMath,
                    equation.expression.clone(),
                )]
            }
            RichTextObject::Mention { plain_text, href, .. } => {
                // For mentions, just use the plain text representation
                let display_text = plain_text.clone().unwrap_or_default();
                let inline_elements = Self::text_to_inline(&display_text);
                
                // If there's a URL in href, make it a link
                if let Some(url) = href {
                    vec![Inline::Link(
                        Attr::default(),
                        inline_elements,
                        Target { 
                            url: url.clone(), 
                            title: String::new() 
                        },
                    )]
                } else {
                    inline_elements
                }
            }
        };

        // Apply annotations to the content
        Self::apply_annotations(content, Self::get_annotations(rich_text))
    }

    /// Helper to extract annotations from rich text object
    fn get_annotations(rich_text: &RichTextObject) -> Annotations {
        match rich_text {
            RichTextObject::Text { annotations, .. } |
            RichTextObject::Equation { annotations, .. } |
            RichTextObject::Mention { annotations, .. } => {
                annotations.clone().unwrap_or_default()
            }
        }
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
                },
                // Newlines become LineBreak
                '\n' => {
                    if !current_word.is_empty() {
                        result.push(Inline::Str(current_word));
                        current_word = String::new();
                    }
                    result.push(Inline::LineBreak);
                },
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
    fn apply_annotations(inlines: Vec<Inline>, annotations: Annotations) -> Vec<Inline> {
        let mut result = inlines;

        // Apply formatting in a specific order with color and code applied last
        
        // Apply basic formatting
        if annotations.underline {
            // Pandoc doesn't have native underline, use a Span with a class
            let mut attr = Attr::default();
            attr.classes.push("underline".to_string());
            result = vec![Inline::Span(attr, result)];
        }

        if annotations.strikethrough {
            result = vec![Inline::Strikeout(result)];
        }

        if annotations.italic {
            result = vec![Inline::Emph(result)];
        }

        if annotations.bold {
            result = vec![Inline::Strong(result)];
        }

        // Apply code (which collapses formatting)
        if annotations.code {
            // For code, we need to convert the inlines to a string
            let code_text = Self::inlines_to_string(&result);
            result = vec![Inline::Code(Attr::default(), code_text)];
        }

        // Apply color last (outermost)
        if annotations.color != crate::notion::text::Color::Default {
            // Convert color name to a class name (e.g., "red" or "red-background")
            let color_name = format!("{:?}", annotations.color);
            // Convert from CamelCase to kebab-case
            let color_class = if color_name.ends_with("Background") {
                let base_color = &color_name[0..color_name.len() - 10]; // Remove "Background"
                format!("color-{}-background", base_color.to_lowercase())
            } else {
                format!("color-{}", color_name.to_lowercase())
            };
            
            let mut attr = Attr::default();
            attr.classes.push(color_class);
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
                Inline::SoftBreak => result.push('\n'),
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
    use crate::notion::text::{create_text, create_formatted_text, create_equation};

    #[test]
    fn test_text_to_inline_empty() {
        let result = NotionTextConverter::text_to_inline("");
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_text_to_inline_simple() {
        let result = NotionTextConverter::text_to_inline("hello world");
        assert_eq!(result.len(), 3);
        assert!(matches!(result[0], Inline::Str(ref s) if s == "hello"));
        assert!(matches!(result[1], Inline::Space));
        assert!(matches!(result[2], Inline::Str(ref s) if s == "world"));
    }

    #[test]
    fn test_text_to_inline_multiple_spaces() {
        let result = NotionTextConverter::text_to_inline("hello  world");
        assert_eq!(result.len(), 4);
        assert!(matches!(result[0], Inline::Str(ref s) if s == "hello"));
        assert!(matches!(result[1], Inline::Space));
        assert!(matches!(result[2], Inline::Space));
        assert!(matches!(result[3], Inline::Str(ref s) if s == "world"));
    }

    #[test]
    fn test_text_to_inline_leading_trailing_spaces() {
        let result = NotionTextConverter::text_to_inline("  hello world  ");
        assert_eq!(result.len(), 7);
        assert!(matches!(result[0], Inline::Space));
        assert!(matches!(result[1], Inline::Space));
        assert!(matches!(result[2], Inline::Str(ref s) if s == "hello"));
        assert!(matches!(result[3], Inline::Space));
        assert!(matches!(result[4], Inline::Str(ref s) if s == "world"));
        assert!(matches!(result[5], Inline::Space));
        assert!(matches!(result[6], Inline::Space));
    }

    #[test]
    fn test_apply_annotations_bold() {
        let inlines = vec![Inline::Str("test".to_string())];
        let mut annotations = Annotations::default();
        annotations.bold = true;
        
        let result = NotionTextConverter::apply_annotations(inlines, annotations);
        assert_eq!(result.len(), 1);
        assert!(matches!(result[0], Inline::Strong(_)));
        
        if let Inline::Strong(content) = &result[0] {
            assert_eq!(content.len(), 1);
            assert!(matches!(content[0], Inline::Str(ref s) if s == "test"));
        }
    }

    #[test]
    fn test_apply_annotations_combined() {
        let inlines = vec![Inline::Str("test".to_string())];
        let mut annotations = Annotations::default();
        annotations.bold = true;
        annotations.italic = true;
        
        let result = NotionTextConverter::apply_annotations(inlines, annotations);
        assert_eq!(result.len(), 1);
        
        // Test that both bold and italic are applied
        let text = NotionTextConverter::inlines_to_string(&result);
        assert_eq!(text, "test");
        
        // Check nesting structure based on our ordering: bold(italic(text))
        assert!(matches!(result[0], Inline::Strong(_)));
        
        if let Inline::Strong(content) = &result[0] {
            assert_eq!(content.len(), 1);
            assert!(matches!(content[0], Inline::Emph(_)));
        }
    }

    #[test]
    fn test_convert_single_rich_text_text() {
        let rich_text = create_text("Hello");
        
        let result = NotionTextConverter::convert_single_rich_text(&rich_text);
        assert_eq!(result.len(), 1);
        assert!(matches!(result[0], Inline::Str(ref s) if s == "Hello"));
    }

    #[test]
    fn test_convert_single_rich_text_link() {
        let mut annotations = Annotations::default();
        annotations.bold = true;
        
        let rich_text = create_formatted_text(
            "Link", 
            annotations, 
            Some("https://example.com".to_string())
        );
        
        let result = NotionTextConverter::convert_single_rich_text(&rich_text);
        assert_eq!(result.len(), 1);
        assert!(matches!(result[0], Inline::Strong(_)));
        
        if let Inline::Strong(content) = &result[0] {
            assert_eq!(content.len(), 1);
            assert!(matches!(content[0], Inline::Link(_, _, _)));
        }
    }

    #[test]
    fn test_convert_single_rich_text_equation() {
        let rich_text = create_equation("E=mc^2");
        
        let result = NotionTextConverter::convert_single_rich_text(&rich_text);
        assert_eq!(result.len(), 1);
        assert!(matches!(result[0], Inline::Math(MathType::InlineMath, ref expr) if expr == "E=mc^2"));
    }

    #[test]
    fn test_convert_rich_texts_with_spaces() {
        let text1 = create_text("Hello");
        let text2 = create_text("world");
        
        let result = NotionTextConverter::convert(&[text1, text2]);
        assert_eq!(result.len(), 3);
        assert!(matches!(result[0], Inline::Str(ref s) if s == "Hello"));
        assert!(matches!(result[1], Inline::Space));
        assert!(matches!(result[2], Inline::Str(ref s) if s == "world"));
    }
    
    // Edge case tests
    
    #[test]
    fn test_whitespace_variations() {
        // Test with tabs
        let text_with_tabs = "hello\tworld";
        let inlines = NotionTextConverter::text_to_inline(text_with_tabs);
        let text = NotionTextConverter::inlines_to_string(&inlines);
        assert_eq!(text, "hello world"); // tab converted to space
        
        // Test with newlines
        let text_with_newlines = "hello\nworld";
        let inlines = NotionTextConverter::text_to_inline(text_with_newlines);
        let text = NotionTextConverter::inlines_to_string(&inlines);
        assert_eq!(text, "hello\nworld"); // newline preserved as LineBreak
        
        // Test with multiple consecutive spaces
        let text_with_many_spaces = NotionTextConverter::text_to_inline("hello      world");
        assert_eq!(text_with_many_spaces.len(), 8);
        assert!(matches!(text_with_many_spaces[0], Inline::Str(ref s) if s == "hello"));
        for i in 1..7 {
            assert!(matches!(text_with_many_spaces[i], Inline::Space));
        }
        assert!(matches!(text_with_many_spaces[7], Inline::Str(ref s) if s == "world"));
    }
    
    #[test]
    fn test_unicode_and_special_characters() {
        // Test with emojis
        let text_with_emoji = NotionTextConverter::text_to_inline("Hello 😊 World");
        assert_eq!(text_with_emoji.len(), 5);
        assert!(matches!(text_with_emoji[0], Inline::Str(ref s) if s == "Hello"));
        assert!(matches!(text_with_emoji[1], Inline::Space));
        assert!(matches!(text_with_emoji[2], Inline::Str(ref s) if s == "😊"));
        assert!(matches!(text_with_emoji[3], Inline::Space));
        assert!(matches!(text_with_emoji[4], Inline::Str(ref s) if s == "World"));
        
        // Test with non-Latin characters
        let text_with_non_latin = NotionTextConverter::text_to_inline("こんにちは 世界");
        assert_eq!(text_with_non_latin.len(), 3);
        assert!(matches!(text_with_non_latin[0], Inline::Str(ref s) if s == "こんにちは"));
        assert!(matches!(text_with_non_latin[1], Inline::Space));
        assert!(matches!(text_with_non_latin[2], Inline::Str(ref s) if s == "世界"));
        
        // Test with special Unicode characters
        let text_with_special = NotionTextConverter::text_to_inline("em—dash and «quotes»");
        assert!(text_with_special.len() > 0);
        // Check that the content is preserved
        let joined = NotionTextConverter::inlines_to_string(&text_with_special);
        assert_eq!(joined, "em—dash and «quotes»");
    }
    
    #[test]
    fn test_empty_and_null_content() {
        // Test with empty rich text array
        let result = NotionTextConverter::convert(&[]);
        assert_eq!(result.len(), 0);
        
        // Test with rich text that has empty content
        let empty_text = create_text("");
        let result = NotionTextConverter::convert_single_rich_text(&empty_text);
        assert_eq!(result.len(), 0);
    }
    
    #[test]
    fn test_deeply_nested_formatting() {
        // Test code formatting (which collapses all other formatting)
        let mut annotations = Annotations::default();
        annotations.bold = true;
        annotations.italic = true;
        annotations.code = true;
        
        let rich_text = create_formatted_text(
            "Nested", 
            annotations, 
            None
        );
        
        let result = NotionTextConverter::convert_single_rich_text(&rich_text);
        
        // Verify we have the expected result
        assert_eq!(result.len(), 1);
        
        // Code formatting should be applied (which means all other formatting is collapsed)
        assert!(matches!(result[0], Inline::Code(_, _)));
        
        if let Inline::Code(_, text) = &result[0] {
            assert_eq!(text, "Nested");
        }
    }
    
    #[test]
    fn test_math_expressions() {
        // Test inline math
        let inline_math = create_equation("E=mc^2");
        let result = NotionTextConverter::convert_single_rich_text(&inline_math);
        assert_eq!(result.len(), 1);
        assert!(matches!(result[0], Inline::Math(MathType::InlineMath, ref expr) if expr == "E=mc^2"));
        
        // Test complex math expression
        let complex_math = create_equation(r"\int_{a}^{b} f(x) \, dx = F(b) - F(a)");
        let result = NotionTextConverter::convert_single_rich_text(&complex_math);
        assert_eq!(result.len(), 1);
        assert!(matches!(result[0], Inline::Math(MathType::InlineMath, ref expr) if expr == r"\int_{a}^{b} f(x) \, dx = F(b) - F(a)"));
    }
    
    #[test]
    fn test_text_with_colors() {
        // Create text with color
        let mut annotations = Annotations::default();
        annotations.color = crate::notion::text::Color::Red;
        
        let colored_text = create_formatted_text("Colored Text", annotations, None);
        let result = NotionTextConverter::convert_single_rich_text(&colored_text);
        
        // Verify color is applied as a Span with appropriate class
        assert_eq!(result.len(), 1);
        assert!(matches!(result[0], Inline::Span(_, _)));
        
        if let Inline::Span(attr, _) = &result[0] {
            assert!(attr.classes.contains(&"color-red".to_string()));
        }
        
        // Ensure text is preserved
        let text = NotionTextConverter::inlines_to_string(&result);
        assert_eq!(text, "Colored Text");
        
        // Test background color
        let mut annotations = Annotations::default();
        annotations.color = crate::notion::text::Color::RedBackground;
        annotations.bold = true; // Add some formatting
        
        let bg_colored_text = create_formatted_text("Background Color", annotations, None);
        let result = NotionTextConverter::convert_single_rich_text(&bg_colored_text);
        
        // Should have span for color and bold formatting
        assert!(matches!(result[0], Inline::Span(_, _)));
        
        if let Inline::Span(attr, content) = &result[0] {
            assert!(attr.classes.contains(&"color-red-background".to_string()));
            // Bold formatting should be inside the span
            assert_eq!(content.len(), 1);
            assert!(matches!(content[0], Inline::Strong(_)));
        }
        
        // And preserve the text
        let text = NotionTextConverter::inlines_to_string(&result);
        assert_eq!(text, "Background Color");
    }
    
    #[test]
    fn test_complex_formatting_with_color() {
        // Create text with multiple formatting types and color
        let mut annotations = Annotations::default();
        annotations.bold = true;
        annotations.italic = true;
        annotations.underline = true;
        annotations.color = crate::notion::text::Color::Blue;
        
        let complex_text = create_formatted_text("Complex Formatting", annotations, None);
        let result = NotionTextConverter::convert_single_rich_text(&complex_text);
        
        // Verify all formatting is applied
        assert_eq!(result.len(), 1);
        
        // Color should be applied as a span and be the outermost layer
        assert!(matches!(result[0], Inline::Span(_, _)));
        
        if let Inline::Span(attr, content) = &result[0] {
            // Check color class
            assert!(attr.classes.contains(&"color-blue".to_string()));
            
            // Verify nested formatting is applied (order depends on implementation)
            assert_eq!(content.len(), 1);
            
            // Most important: verify the text is preserved through all formatting layers
            let text = NotionTextConverter::inlines_to_string(&result);
            assert_eq!(text, "Complex Formatting");
        }
    }
}