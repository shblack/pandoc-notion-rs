//! Tests for Pandoc to Notion text conversion
//! 
//! This module contains more direct tests for the pandoc_text converter without the
//! complex recursive formatting that was causing issues in the main test module.

use crate::notion::text::{Color, RichTextObject};
use crate::p2n::pandoc_text::TextBuilder;

/// Test that the TextBuilder can append and commit text properly
#[test]
fn test_text_builder_basics() {
    let mut builder = TextBuilder::new();
    builder.append_text("Hello");
    builder.append_text(" world");
    
    let result = builder.build().unwrap();
    assert_eq!(result.len(), 1);
    
    match &result[0] {
        RichTextObject::Text { text, .. } => {
            assert_eq!(text.content, "Hello world");
        },
        _ => panic!("Expected Text variant"),
    }
}

/// Test the handling of newlines in the text builder
#[test]
fn test_text_builder_newlines() {
    let mut builder = TextBuilder::new();
    builder.append_text("Line 1");
    builder.append_text("\n");
    builder.append_text("Line 2");
    
    let result = builder.build().unwrap();
    assert_eq!(result.len(), 1);
    
    match &result[0] {
        RichTextObject::Text { text, .. } => {
            assert_eq!(text.content, "Line 1\nLine 2");
        },
        _ => panic!("Expected Text variant"),
    }
}

/// Test setting bold annotation on text
#[test]
fn test_text_builder_bold() {
    let mut builder = TextBuilder::new();
    
    // Update annotations using the public method
    builder.update_annotation(|annotations| {
        annotations.bold = true;
    });
    
    builder.append_text("Bold text");
    
    let result = builder.build().unwrap();
    assert_eq!(result.len(), 1);
    
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

/// Test setting italic annotation on text
#[test]
fn test_text_builder_italic() {
    let mut builder = TextBuilder::new();
    
    // Update annotations using the public method
    builder.update_annotation(|annotations| {
        annotations.italic = true;
    });
    
    builder.append_text("Italic text");
    
    let result = builder.build().unwrap();
    assert_eq!(result.len(), 1);
    
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

/// Test setting strikethrough annotation on text
#[test]
fn test_text_builder_strikethrough() {
    let mut builder = TextBuilder::new();
    
    // Update annotations using the public method
    builder.update_annotation(|annotations| {
        annotations.strikethrough = true;
    });
    
    builder.append_text("Strikethrough text");
    
    let result = builder.build().unwrap();
    assert_eq!(result.len(), 1);
    
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

/// Test setting code formatting annotation on text
#[test]
fn test_text_builder_code() {
    let mut builder = TextBuilder::new();
    
    // Update annotations using the public method
    builder.update_annotation(|annotations| {
        annotations.code = true;
    });
    
    builder.append_text("Code example");
    
    let result = builder.build().unwrap();
    assert_eq!(result.len(), 1);
    
    match &result[0] {
        RichTextObject::Text { text, annotations, .. } => {
            assert_eq!(text.content, "Code example");
            if let Some(ann) = annotations {
                assert!(ann.code);
            } else {
                panic!("Expected annotations");
            }
        },
        _ => panic!("Expected Text variant"),
    }
}

/// Test setting color annotation on text
#[test]
fn test_text_builder_color() {
    let mut builder = TextBuilder::new();
    
    // Update annotations using the public method
    builder.update_annotation(|annotations| {
        annotations.color = Color::Red;
    });
    
    builder.append_text("Red text");
    
    let result = builder.build().unwrap();
    assert_eq!(result.len(), 1);
    
    match &result[0] {
        RichTextObject::Text { text, annotations, .. } => {
            assert_eq!(text.content, "Red text");
            if let Some(ann) = annotations {
                assert_eq!(ann.color, Color::Red);
            } else {
                panic!("Expected annotations");
            }
        },
        _ => panic!("Expected Text variant"),
    }
}

/// Test setting background color annotation on text
#[test]
fn test_text_builder_background_color() {
    let mut builder = TextBuilder::new();
    
    // Update annotations using the public method
    builder.update_annotation(|annotations| {
        annotations.color = Color::BlueBackground;
    });
    
    builder.append_text("Blue background text");
    
    let result = builder.build().unwrap();
    assert_eq!(result.len(), 1);
    
    match &result[0] {
        RichTextObject::Text { text, annotations, .. } => {
            assert_eq!(text.content, "Blue background text");
            if let Some(ann) = annotations {
                assert_eq!(ann.color, Color::BlueBackground);
            } else {
                panic!("Expected annotations");
            }
        },
        _ => panic!("Expected Text variant"),
    }
}

/// Test setting underline annotation on text
#[test]
fn test_text_builder_underline() {
    let mut builder = TextBuilder::new();
    
    // Update annotations using the public method
    builder.update_annotation(|annotations| {
        annotations.underline = true;
    });
    
    builder.append_text("Underlined text");
    
    let result = builder.build().unwrap();
    assert_eq!(result.len(), 1);
    
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

/// Test creation of equation rich text
#[test]
fn test_text_builder_equation() {
    let mut builder = TextBuilder::new();
    
    // Create an equation
    builder.commit_equation("E=mc^2").unwrap();
    
    let result = builder.build().unwrap();
    assert_eq!(result.len(), 1);
    
    match &result[0] {
        RichTextObject::Equation { equation, .. } => {
            assert_eq!(equation.expression, "E=mc^2");
        },
        _ => panic!("Expected Equation variant"),
    }
}

/// Test text with link
#[test]
fn test_text_builder_link() {
    let mut builder = TextBuilder::new();
    
    // Set link URL
    builder.set_link("https://example.com");
    
    // Add link text
    builder.append_text("Link text");
    
    let result = builder.build().unwrap();
    assert_eq!(result.len(), 1);
    
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

/// Test multiple rich text objects
#[test]
fn test_text_builder_multiple_objects() {
    let mut builder = TextBuilder::new();
    
    // First text object
    builder.append_text("First part");
    builder.commit_current_text().unwrap();
    
    // Second text object with different formatting
    builder.update_annotation(|annotations| {
        annotations.bold = true;
    });
    builder.append_text("Second part");
    
    let result = builder.build().unwrap();
    assert_eq!(result.len(), 2);
    
    // Check first object
    match &result[0] {
        RichTextObject::Text { text, .. } => {
            assert_eq!(text.content, "First part");
        },
        _ => panic!("Expected Text variant"),
    }
    
    // Check second object
    match &result[1] {
        RichTextObject::Text { text, annotations, .. } => {
            assert_eq!(text.content, "Second part");
            if let Some(ann) = annotations {
                assert!(ann.bold);
            } else {
                panic!("Expected annotations");
            }
        },
        _ => panic!("Expected Text variant"),
    }
}

/// Test handling of unicode characters
#[test]
fn test_text_builder_unicode() {
    let mut builder = TextBuilder::new();
    builder.append_text("Unicode: 你好, こんにちは, 안녕하세요, Привет, 😊");
    
    let result = builder.build().unwrap();
    assert_eq!(result.len(), 1);
    
    match &result[0] {
        RichTextObject::Text { text, .. } => {
            assert_eq!(text.content, "Unicode: 你好, こんにちは, 안녕하세요, Привет, 😊");
        },
        _ => panic!("Expected Text variant"),
    }
}

/// Test combination of multiple annotations
#[test]
fn test_text_builder_combined_formatting() {
    let mut builder = TextBuilder::new();
    
    // Apply multiple annotations
    builder.update_annotation(|annotations| {
        annotations.bold = true;
        annotations.italic = true;
        annotations.underline = true;
    });
    
    builder.append_text("Bold, italic and underlined");
    
    let result = builder.build().unwrap();
    assert_eq!(result.len(), 1);
    
    match &result[0] {
        RichTextObject::Text { text, annotations, .. } => {
            assert_eq!(text.content, "Bold, italic and underlined");
            if let Some(ann) = annotations {
                assert!(ann.bold);
                assert!(ann.italic);
                assert!(ann.underline);
            } else {
                panic!("Expected annotations");
            }
        },
        _ => panic!("Expected Text variant"),
    }
}