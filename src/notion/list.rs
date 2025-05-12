use serde::{Deserialize, Serialize};
use crate::notion::text::RichTextObject;
use crate::notion::blocks::{Block, BlockColor, BlockContent};

/// Notion bulleted list item properties
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct BulletedListItemProperties {
    pub rich_text: Vec<RichTextObject>,
    #[serde(default)]
    pub color: BlockColor,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Block>>,
}

/// Notion numbered list item properties
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct NumberedListItemProperties {
    pub rich_text: Vec<RichTextObject>,
    #[serde(default)]
    pub color: BlockColor,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Block>>,
}

/// Notion to-do list item properties
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct ToDoProperties {
    pub rich_text: Vec<RichTextObject>,
    #[serde(default)]
    pub checked: bool,
    #[serde(default)]
    pub color: BlockColor,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Block>>,
}

/// Helper function to create a bulleted list item
pub fn create_bulleted_list_item(rich_text: Vec<RichTextObject>) -> Block {
    Block {
        object: None,
        id: None,
        parent: None,
        has_children: None,
        content: BlockContent::BulletedListItem {
            bulleted_list_item: BulletedListItemProperties {
                rich_text,
                color: BlockColor::Default,
                children: None,
            },
        },
    }
}

/// Helper function to create a bulleted list item with children
pub fn create_bulleted_list_item_with_children(rich_text: Vec<RichTextObject>, children: Vec<Block>) -> Block {
    Block {
        object: None,
        id: None,
        parent: None,
        has_children: None,
        content: BlockContent::BulletedListItem {
            bulleted_list_item: BulletedListItemProperties {
                rich_text,
                color: BlockColor::Default,
                children: Some(children),
            },
        },
    }
}

/// Helper function to create a numbered list item
pub fn create_numbered_list_item(rich_text: Vec<RichTextObject>) -> Block {
    Block {
        object: None,
        id: None,
        parent: None,
        has_children: None,
        content: BlockContent::NumberedListItem {
            numbered_list_item: NumberedListItemProperties {
                rich_text,
                color: BlockColor::Default,
                children: None,
            },
        },
    }
}

/// Helper function to create a numbered list item with children
pub fn create_numbered_list_item_with_children(rich_text: Vec<RichTextObject>, children: Vec<Block>) -> Block {
    Block {
        object: None,
        id: None,
        parent: None,
        has_children: None,
        content: BlockContent::NumberedListItem {
            numbered_list_item: NumberedListItemProperties {
                rich_text,
                color: BlockColor::Default,
                children: Some(children),
            },
        },
    }
}

/// Helper function to create a to-do list item
pub fn create_to_do(rich_text: Vec<RichTextObject>, checked: bool) -> Block {
    Block {
        object: None,
        id: None,
        parent: None,
        has_children: None,
        content: BlockContent::ToDo {
            to_do: ToDoProperties {
                rich_text,
                checked,
                color: BlockColor::Default,
                children: None,
            },
        },
    }
}

/// Helper function to create a to-do list item with children
pub fn create_to_do_with_children(rich_text: Vec<RichTextObject>, checked: bool, children: Vec<Block>) -> Block {
    Block {
        object: None,
        id: None,
        parent: None,
        has_children: None,
        content: BlockContent::ToDo {
            to_do: ToDoProperties {
                rich_text,
                checked,
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
    fn test_serialize_bulleted_list_item() {
        let text = text::create_text("Lacinato kale");
        let item = create_bulleted_list_item(vec![text]);

        let json = to_value(&item).unwrap();
        assert_eq!(json["type"], "bulleted_list_item");
        assert_eq!(json["bulleted_list_item"]["rich_text"][0]["type"], "text");
        assert_eq!(json["bulleted_list_item"]["rich_text"][0]["text"]["content"], "Lacinato kale");
        assert_eq!(json["bulleted_list_item"]["color"], "default");
    }

    #[test]
    fn test_serialize_numbered_list_item() {
        let text = text::create_text("Finish reading the docs");
        let item = create_numbered_list_item(vec![text]);

        let json = to_value(&item).unwrap();
        assert_eq!(json["type"], "numbered_list_item");
        assert_eq!(json["numbered_list_item"]["rich_text"][0]["text"]["content"], "Finish reading the docs");
    }

    #[test]
    fn test_serialize_to_do() {
        let text = text::create_text("Finish Q3 goals");
        let item = create_to_do(vec![text], false);

        let json = to_value(&item).unwrap();
        assert_eq!(json["type"], "to_do");
        assert_eq!(json["to_do"]["rich_text"][0]["text"]["content"], "Finish Q3 goals");
        assert_eq!(json["to_do"]["checked"], false);
    }

    #[test]
    fn test_deserialize_bulleted_list_item() {
        let json = json!({
            "object": "block",
            "id": "c02fc1d3-db8b-45c5-a222-27595b15aea7",
            "parent": {
                "type": "page_id",
                "page_id": "59833787-2cf9-4fdf-8782-e53db20768a5"
            },
            "created_time": "2022-03-01T19:05:00.000Z",
            "last_edited_time": "2022-07-06T19:41:00.000Z",
            "has_children": true,
            "archived": false,
            "in_trash": false,
            "type": "bulleted_list_item",
            "bulleted_list_item": {
                "rich_text": [
                    {
                        "type": "text",
                        "text": {
                            "content": "Lacinato kale",
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
                        "plain_text": "Lacinato kale",
                        "href": null
                    }
                ],
                "color": "default"
            }
        });

        let block: Block = from_value(json).unwrap();
        assert_eq!(block.id, Some("c02fc1d3-db8b-45c5-a222-27595b15aea7".to_string()));
        assert_eq!(block.object, Some("block".to_string()));

        match &block.content {
            crate::notion::blocks::BlockContent::BulletedListItem { bulleted_list_item } => {
                assert_eq!(bulleted_list_item.rich_text.len(), 1);
                match &bulleted_list_item.rich_text[0] {
                    text::RichTextObject::Text { text, .. } => {
                        assert_eq!(text.content, "Lacinato kale");
                    },
                    _ => panic!("Expected Text variant"),
                }
                assert_eq!(bulleted_list_item.color, BlockColor::Default);
            },
            _ => panic!("Expected BulletedListItem variant"),
        }
    }

    #[test]
    fn test_nested_list_structure() {
        let nested_item_text = text::create_text("Nested list item");
        let nested_item = create_bulleted_list_item(vec![nested_item_text]);

        let nested_todo_text = text::create_text("Nested to-do item");
        let nested_todo = create_to_do(vec![nested_todo_text], false);

        let main_text = text::create_text("Main list item");
        let main_item = create_bulleted_list_item_with_children(
            vec![main_text], 
            vec![nested_item, nested_todo]
        );

        let json = to_value(&main_item).unwrap();
        assert_eq!(json["type"], "bulleted_list_item");
        assert_eq!(json["bulleted_list_item"]["children"][0]["type"], "bulleted_list_item");
        assert_eq!(json["bulleted_list_item"]["children"][1]["type"], "to_do");
        assert_eq!(
            json["bulleted_list_item"]["children"][0]["bulleted_list_item"]["rich_text"][0]["text"]["content"], 
            "Nested list item"
        );
    }
}