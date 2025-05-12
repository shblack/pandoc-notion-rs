use serde::{Deserialize, Serialize};
use crate::notion::text::RichTextObject;
use crate::notion::blocks::{Block, BlockColor, BlockContent};

/// Notion paragraph block properties
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct ParagraphProperties {
    pub rich_text: Vec<RichTextObject>,
    #[serde(default)]
    pub color: BlockColor,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Block>>,
}

/// Helper function to create a paragraph block
pub fn create_paragraph(rich_text: Vec<RichTextObject>) -> Block {
    Block {
        object: None,
        id: None,
        parent: None,
        has_children: None,
        content: BlockContent::Paragraph {
            paragraph: ParagraphProperties {
                rich_text,
                color: BlockColor::Default,
                children: None,
            },
        },
    }
}

/// Helper function to create a paragraph block with children
pub fn create_paragraph_with_children(rich_text: Vec<RichTextObject>, children: Vec<Block>) -> Block {
    Block {
        object: None,
        id: None,
        parent: None,
        has_children: None,
        content: BlockContent::Paragraph {
            paragraph: ParagraphProperties {
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
    use serde_json::{json, to_value, from_value};

    #[test]
    fn test_serialize_paragraph() {
        let text = text::create_text("Example paragraph");
        let paragraph = create_paragraph(vec![text]);

        let json = to_value(&paragraph).unwrap();
        assert_eq!(json["type"], "paragraph");
        assert_eq!(json["paragraph"]["rich_text"][0]["type"], "text");
        assert_eq!(json["paragraph"]["rich_text"][0]["text"]["content"], "Example paragraph");
        assert_eq!(json["paragraph"]["color"], "default");
    }

    #[test]
    fn test_deserialize_paragraph() {
        let json = json!({
            "object": "block",
            "id": "block-id",
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
            "type": "paragraph",
            "paragraph": {
                "rich_text": [{
                    "type": "text",
                    "text": {
                        "content": "Example paragraph",
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
                    "plain_text": "Example paragraph",
                    "href": null
                }],
                "color": "default"
            }
        });

        let block: Block = from_value(json).unwrap();

        assert_eq!(block.id, Some("block-id".to_string()));
        assert_eq!(block.object, Some("block".to_string()));

        match block.content {
            BlockContent::Paragraph { paragraph } => {
                assert_eq!(paragraph.rich_text.len(), 1);
                match &paragraph.rich_text[0] {
                    text::RichTextObject::Text { text, .. } => {
                        assert_eq!(text.content, "Example paragraph");
                    },
                    _ => panic!("Expected Text variant"),
                }
                assert_eq!(paragraph.color, BlockColor::Default);
            },
            _ => panic!("Expected Paragraph variant"),
        }
    }

    #[test]
    fn test_nested_blocks() {
        let child_text = text::create_text("Child paragraph");
        let child = create_paragraph(vec![child_text]);

        let parent_text = text::create_text("Parent paragraph");
        let parent = create_paragraph_with_children(vec![parent_text], vec![child]);

        let json = to_value(&parent).unwrap();
        assert_eq!(json["paragraph"]["children"][0]["type"], "paragraph");
        assert_eq!(
            json["paragraph"]["children"][0]["paragraph"]["rich_text"][0]["text"]["content"], 
            "Child paragraph"
        );
    }
}