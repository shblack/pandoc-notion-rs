use crate::notion::blocks::{Block, BlockColor, BlockContent};
use crate::notion::text::RichTextObject;
use serde::{Deserialize, Serialize};

/// Notion quote block properties
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct QuoteProperties {
    pub rich_text: Vec<RichTextObject>,
    #[serde(default)]
    pub color: BlockColor,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Block>>,
}

/// Helper function to create a quote block
pub fn create_quote(rich_text: Vec<RichTextObject>) -> Block {
    Block {
        object: None,
        id: None,
        parent: None,
        has_children: None,
        content: BlockContent::Quote {
            quote: QuoteProperties {
                rich_text,
                color: BlockColor::Default,
                children: None,
            },
        },
    }
}

/// Helper function to create a quote block with children
pub fn create_quote_with_children(rich_text: Vec<RichTextObject>, children: Vec<Block>) -> Block {
    Block {
        object: None,
        id: None,
        parent: None,
        has_children: None,
        content: BlockContent::Quote {
            quote: QuoteProperties {
                rich_text,
                color: BlockColor::Default,
                children: Some(children),
            },
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::notion::text;
    use serde_json::{from_value, json, to_value};

    #[test]
    fn test_serialize_quote() {
        // Test quote
        let text = text::create_text("To be or not to be...");
        let quote = create_quote(vec![text.clone()]);

        let json = to_value(&quote).unwrap();
        assert_eq!(json["type"], "quote");
        assert_eq!(json["quote"]["rich_text"][0]["type"], "text");
        assert_eq!(
            json["quote"]["rich_text"][0]["text"]["content"],
            "To be or not to be..."
        );
        assert_eq!(json["quote"]["color"], "default");

        // Test quote with children
        let child_text = text::create_text("Child block");
        let child = crate::notion::paragraph::create_paragraph(vec![child_text]);
        let quote_with_children = create_quote_with_children(vec![text], vec![child]);

        let json = to_value(&quote_with_children).unwrap();
        assert_eq!(json["quote"]["children"][0]["type"], "paragraph");
    }

    #[test]
    fn test_deserialize_quote() {
        let json = json!({
            "object": "block",
            "id": "quote-block-id",
            "parent": {
                "type": "page_id",
                "page_id": "parent-page-id"
            },
            "created_time": "2023-01-01T12:00:00.000Z",
            "last_edited_time": "2023-01-01T12:00:00.000Z",
            "created_by": {
                "object": "user",
                "id": "user-id"
            },
            "last_edited_by": {
                "object": "user",
                "id": "user-id"
            },
            "has_children": false,
            "archived": false,
            "in_trash": false,
            "type": "quote",
            "quote": {
                "rich_text": [{
                    "type": "text",
                    "text": {
                        "content": "To be or not to be...",
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
                    "plain_text": "To be or not to be...",
                    "href": null
                }],
                "color": "default"
            }
        });

        let block: Block = from_value(json).unwrap();

        assert_eq!(block.id, Some("quote-block-id".to_string()));
        assert_eq!(block.object, Some("block".to_string()));

        match block.content {
            BlockContent::Quote { quote } => {
                assert_eq!(quote.rich_text.len(), 1);
                match &quote.rich_text[0] {
                    text::RichTextObject::Text { text, .. } => {
                        assert_eq!(text.content, "To be or not to be...");
                    }
                    _ => panic!("Expected Text variant"),
                }
                assert_eq!(quote.color, BlockColor::Default);
            }
            _ => panic!("Expected Quote variant"),
        }
    }
}
