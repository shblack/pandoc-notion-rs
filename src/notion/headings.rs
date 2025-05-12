use serde::{Deserialize, Serialize};
use crate::notion::text::RichTextObject;
use crate::notion::blocks::{Block, BlockColor, BlockContent};

/// Notion heading block properties
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct HeadingProperties {
    pub rich_text: Vec<RichTextObject>,
    #[serde(default)]
    pub color: BlockColor,
    #[serde(default)]
    pub is_toggleable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Block>>,
}

/// Helper function to create a heading 1 block
pub fn create_heading1(rich_text: Vec<RichTextObject>) -> Block {
    Block {
        object: None,
        id: None,
        parent: None,
        has_children: None,
        content: BlockContent::Heading1 {
            heading_1: HeadingProperties {
                rich_text,
                color: BlockColor::Default,
                is_toggleable: false,
                children: None,
            },
        },
    }
}

/// Helper function to create a heading 2 block
pub fn create_heading2(rich_text: Vec<RichTextObject>) -> Block {
    Block {
        object: None,
        id: None,
        parent: None,
        has_children: None,
        content: BlockContent::Heading2 {
            heading_2: HeadingProperties {
                rich_text,
                color: BlockColor::Default,
                is_toggleable: false,
                children: None,
            },
        },
    }
}

/// Helper function to create a heading 3 block
pub fn create_heading3(rich_text: Vec<RichTextObject>) -> Block {
    Block {
        object: None,
        id: None,
        parent: None,
        has_children: None,
        content: BlockContent::Heading3 {
            heading_3: HeadingProperties {
                rich_text,
                color: BlockColor::Default,
                is_toggleable: false,
                children: None,
            },
        },
    }
}

/// Helper function to create a toggleable heading 1 block with children
pub fn create_toggleable_heading1(rich_text: Vec<RichTextObject>, children: Vec<Block>) -> Block {
    Block {
        object: None,
        id: None,
        parent: None,
        has_children: None,
        content: BlockContent::Heading1 {
            heading_1: HeadingProperties {
                rich_text,
                color: BlockColor::Default,
                is_toggleable: true,
                children: Some(children),
            },
        },
    }
}

/// Helper function to create a toggleable heading 2 block with children
pub fn create_toggleable_heading2(rich_text: Vec<RichTextObject>, children: Vec<Block>) -> Block {
    Block {
        object: None,
        id: None,
        parent: None,
        has_children: None,
        content: BlockContent::Heading2 {
            heading_2: HeadingProperties {
                rich_text,
                color: BlockColor::Default,
                is_toggleable: true,
                children: Some(children),
            },
        },
    }
}

/// Helper function to create a toggleable heading 3 block with children
pub fn create_toggleable_heading3(rich_text: Vec<RichTextObject>, children: Vec<Block>) -> Block {
    Block {
        object: None,
        id: None,
        parent: None,
        has_children: None,
        content: BlockContent::Heading3 {
            heading_3: HeadingProperties {
                rich_text,
                color: BlockColor::Default,
                is_toggleable: true,
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
    fn test_serialize_headings() {
        // Test heading 1
        let text = text::create_text("Heading 1");
        let heading1 = create_heading1(vec![text]);

        let json = to_value(&heading1).unwrap();
        assert_eq!(json["type"], "heading_1");
        assert_eq!(json["heading_1"]["rich_text"][0]["type"], "text");
        assert_eq!(json["heading_1"]["rich_text"][0]["text"]["content"], "Heading 1");
        assert_eq!(json["heading_1"]["color"], "default");
        assert_eq!(json["heading_1"]["is_toggleable"], false);

        // Test heading 2
        let text = text::create_text("Heading 2");
        let heading2 = create_heading2(vec![text]);

        let json = to_value(&heading2).unwrap();
        assert_eq!(json["type"], "heading_2");
        assert_eq!(json["heading_2"]["rich_text"][0]["text"]["content"], "Heading 2");

        // Test heading 3
        let text = text::create_text("Heading 3");
        let heading3 = create_heading3(vec![text]);

        let json = to_value(&heading3).unwrap();
        assert_eq!(json["type"], "heading_3");
        assert_eq!(json["heading_3"]["rich_text"][0]["text"]["content"], "Heading 3");
    }

    #[test]
    fn test_deserialize_heading() {
        let json = json!({
            "object": "block",
            "id": "heading-block-id",
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
            "type": "heading_1",
            "heading_1": {
                "rich_text": [{
                    "type": "text",
                    "text": {
                        "content": "Main Heading",
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
                    "plain_text": "Main Heading",
                    "href": null
                }],
                "color": "default",
                "is_toggleable": false
            }
        });

        let block: Block = from_value(json).unwrap();

        assert_eq!(block.id, Some("heading-block-id".to_string()));
        assert_eq!(block.object, Some("block".to_string()));

        match block.content {
            BlockContent::Heading1 { heading_1 } => {
                assert_eq!(heading_1.rich_text.len(), 1);
                match &heading_1.rich_text[0] {
                    text::RichTextObject::Text { text, .. } => {
                        assert_eq!(text.content, "Main Heading");
                    },
                    _ => panic!("Expected Text variant"),
                }
                assert_eq!(heading_1.color, BlockColor::Default);
                assert_eq!(heading_1.is_toggleable, false);
            },
            _ => panic!("Expected Heading1 variant"),
        }
    }

    #[test]
    fn test_toggleable_heading() {
        let child_text = text::create_text("Child block");
        let child = crate::notion::paragraph::create_paragraph(vec![child_text]);

        let heading_text = text::create_text("Toggle Heading");
        let toggle_heading = create_toggleable_heading2(vec![heading_text], vec![child]);

        let json = to_value(&toggle_heading).unwrap();
        assert_eq!(json["type"], "heading_2");
        assert_eq!(json["heading_2"]["is_toggleable"], true);
        assert_eq!(json["heading_2"]["children"][0]["type"], "paragraph");
        assert_eq!(
            json["heading_2"]["children"][0]["paragraph"]["rich_text"][0]["text"]["content"],
            "Child block"
        );
    }
}