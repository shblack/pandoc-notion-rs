use serde::{Deserialize, Serialize};

/// Represents a color in Notion's API
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Color {
    Default,
    Gray,
    Brown,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    Pink,
    Red,
    GrayBackground,
    BrownBackground,
    OrangeBackground,
    YellowBackground,
    GreenBackground,
    BlueBackground,
    PurpleBackground,
    PinkBackground,
    RedBackground,
}

/// Represents a link in Notion's API
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Link {
    pub url: String,
}

/// Text content and optional link for rich text
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TextContent {
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Link>,
}

/// Text formatting annotations
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Annotations {
    pub bold: bool,
    pub italic: bool,
    pub strikethrough: bool,
    pub underline: bool,
    pub code: bool,
    pub color: Color,
}

impl Default for Annotations {
    fn default() -> Self {
        Self {
            bold: false,
            italic: false,
            strikethrough: false,
            underline: false,
            code: false,
            color: Color::Default,
        }
    }
}

/// Represents an equation in Notion's API
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Equation {
    pub expression: String,
}

/// A complete Notion rich text object as returned by the API
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum RichTextObject {
    /// Text rich text object
    Text {
        /// Text content
        text: TextContent,
        /// Formatting annotations
        #[serde(skip_serializing_if = "Option::is_none")]
        annotations: Option<Annotations>,
        /// Plain text representation (only in API responses)
        #[serde(skip_serializing_if = "Option::is_none")]
        plain_text: Option<String>,
        /// URL reference (only in API responses)
        #[serde(skip_serializing_if = "Option::is_none")]
        href: Option<String>,
    },
    /// Mention rich text object (placeholder)
    Mention {
        /// Mention content would go here
        #[serde(skip)]
        mention: (),
        /// Formatting annotations
        #[serde(skip_serializing_if = "Option::is_none")]
        annotations: Option<Annotations>,
        /// Plain text representation (only in API responses)
        #[serde(skip_serializing_if = "Option::is_none")]
        plain_text: Option<String>,
        /// URL reference (only in API responses)
        #[serde(skip_serializing_if = "Option::is_none")]
        href: Option<String>,
    },
    /// Equation rich text object
    Equation {
        /// Equation content
        equation: Equation,
        /// Formatting annotations
        #[serde(skip_serializing_if = "Option::is_none")]
        annotations: Option<Annotations>,
        /// Plain text representation (only in API responses)
        #[serde(skip_serializing_if = "Option::is_none")]
        plain_text: Option<String>,
        /// URL reference (only in API responses)
        #[serde(skip_serializing_if = "Option::is_none")]
        href: Option<String>,
    },
}

/// Trait to help with creating duplicates of RichTextObjects
/// This provides a more semantic interface than using .clone() directly
pub trait CloneTextObject {
    /// Creates a duplicate of self
    fn duplicate(&self) -> Self;
}

impl CloneTextObject for RichTextObject {
    /// Creates a duplicate of self
    fn duplicate(&self) -> Self {
        self.clone()
    }
}

/// A simplified builder for creating text-type rich text objects
pub fn create_text(content: &str) -> RichTextObject {
    RichTextObject::Text {
        text: TextContent {
            content: content.to_string(),
            link: None,
        },
        annotations: None,
        plain_text: None,
        href: None,
    }
}

/// Helper function to create a rich text object with formatting
pub fn create_formatted_text(
    content: &str,
    annotations: Annotations,
    link: Option<String>,
) -> RichTextObject {
    RichTextObject::Text {
        text: TextContent {
            content: content.to_string(),
            link: link.map(|url| Link { url }),
        },
        annotations: Some(annotations),
        plain_text: None,
        href: None,
    }
}

/// Helper function to create an equation rich text object
pub fn create_equation(expression: &str) -> RichTextObject {
    RichTextObject::Equation {
        equation: Equation {
            expression: expression.to_string(),
        },
        annotations: None,
        plain_text: None,
        href: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{from_value, json, to_value};

    #[test]
    fn test_serialize_simple_text() {
        let text = create_text("Example text");
        let json = to_value(&text).unwrap();

        assert_eq!(json["type"], "text");
        assert_eq!(json["text"]["content"], "Example text");
    }

    #[test]
    fn test_formatted_text_with_link() {
        let mut annotations = Annotations::default();
        annotations.bold = true;
        annotations.color = Color::Red;

        let text = create_formatted_text(
            "Example text",
            annotations,
            Some("https://example.com".to_string()),
        );

        let json = to_value(&text).unwrap();
        assert_eq!(json["type"], "text");
        assert_eq!(json["text"]["content"], "Example text");
        assert_eq!(json["text"]["link"]["url"], "https://example.com");
        assert_eq!(json["annotations"]["bold"], true);
        assert_eq!(json["annotations"]["color"], "red");
    }

    #[test]
    fn test_equation() {
        let equation = create_equation("e=mc^2");
        let json = to_value(&equation).unwrap();

        assert_eq!(json["type"], "equation");
        assert_eq!(json["equation"]["expression"], "e=mc^2");
    }

    #[test]
    fn test_deserialize_text() {
        let json = json!({
            "type": "text",
            "text": {
                "content": "Example text",
                "link": null
            },
            "annotations": {
                "bold": false,
                "italic": false,
                "strikethrough": false,
                "underline": false,
                "code": false,
                "color": "default"
            },
            "plain_text": "Example text",
            "href": null
        });

        let obj: RichTextObject = from_value(json).unwrap();

        match obj {
            RichTextObject::Text {
                text,
                annotations,
                plain_text,
                ..
            } => {
                assert_eq!(text.content, "Example text");
                assert!(text.link.is_none());
                assert!(annotations.is_some());
                assert_eq!(plain_text, Some("Example text".to_string()));
            }
            _ => panic!("Expected Text variant"),
        }
    }

    #[test]
    fn test_deserialize_equation() {
        let json = json!({
            "type": "equation",
            "equation": {
                "expression": "e=mc^2"
            },
            "annotations": {
                "bold": false,
                "italic": false,
                "strikethrough": false,
                "underline": false,
                "code": false,
                "color": "default"
            },
            "plain_text": "e=mc^2",
            "href": null
        });

        let obj: RichTextObject = from_value(json).unwrap();

        match obj {
            RichTextObject::Equation {
                equation,
                plain_text,
                ..
            } => {
                assert_eq!(equation.expression, "e=mc^2");
                assert_eq!(plain_text, Some("e=mc^2".to_string()));
            }
            _ => panic!("Expected Equation variant"),
        }
    }
}
