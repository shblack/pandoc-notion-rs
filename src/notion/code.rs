use serde::{Deserialize, Serialize};
use crate::notion::text::RichTextObject;
use crate::notion::blocks::{Block, BlockContent};

/// Programming languages supported in Notion code blocks
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CodeLanguage {
    Abap,
    Arduino,
    Bash,
    Basic,
    C,
    Clojure,
    Coffeescript,
    #[serde(rename = "c++")]
    CPlusPlus,
    #[serde(rename = "c#")]
    CSharp,
    Css,
    Dart,
    Diff,
    Docker,
    Elixir,
    Elm,
    Erlang,
    Flow,
    Fortran,
    #[serde(rename = "f#")]
    FSharp,
    Gherkin,
    Glsl,
    Go,
    Graphql,
    Groovy,
    Haskell,
    Html,
    Java,
    Javascript,
    Json,
    Julia,
    Kotlin,
    Latex,
    Less,
    Lisp,
    Livescript,
    Lua,
    Makefile,
    Markdown,
    Markup,
    Matlab,
    Mermaid,
    Nix,
    #[serde(rename = "objective-c")]
    ObjectiveC,
    Ocaml,
    Pascal,
    Perl,
    Php,
    #[serde(rename = "plain text")]
    PlainText,
    Powershell,
    Prolog,
    Protobuf,
    Python,
    R,
    Reason,
    Ruby,
    Rust,
    Sass,
    Scala,
    Scheme,
    Scss,
    Shell,
    Sql,
    Swift,
    Typescript,
    #[serde(rename = "vb.net")]
    VbNet,
    Verilog,
    Vhdl,
    #[serde(rename = "visual basic")]
    VisualBasic,
    Webassembly,
    Xml,
    Yaml,
    #[serde(rename = "java/c/c++/c#")]
    JavaCFamily,
}

impl Default for CodeLanguage {
    fn default() -> Self {
        Self::PlainText
    }
}

/// Notion code block properties
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct CodeProperties {
    pub rich_text: Vec<RichTextObject>,
    pub language: CodeLanguage,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub caption: Vec<RichTextObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Block>>,
}

/// Helper function to create a code block
pub fn create_code(rich_text: Vec<RichTextObject>, language: CodeLanguage) -> Block {
    Block {
        object: None,
        id: None,
        parent: None,
        has_children: None,
        content: BlockContent::Code {
            code: CodeProperties {
                rich_text,
                language,
                caption: Vec::new(),
                children: None,
            },
        },
    }
}

/// Helper function to create a code block with a caption
pub fn create_code_with_caption(
    rich_text: Vec<RichTextObject>, 
    language: CodeLanguage,
    caption: Vec<RichTextObject>,
) -> Block {
    Block {
        object: None,
        id: None,
        parent: None,
        has_children: None,
        content: BlockContent::Code {
            code: CodeProperties {
                rich_text,
                language,
                caption,
                children: None,
            },
        },
    }
}

/// Helper function to create a code block with children
pub fn create_code_with_children(
    rich_text: Vec<RichTextObject>, 
    language: CodeLanguage,
    children: Vec<Block>,
) -> Block {
    Block {
        object: None,
        id: None,
        parent: None,
        has_children: None,
        content: BlockContent::Code {
            code: CodeProperties {
                rich_text,
                language,
                caption: Vec::new(),
                children: Some(children),
            },
        },
    }
}

/// Helper function to create a code block with both caption and children
pub fn create_code_with_caption_and_children(
    rich_text: Vec<RichTextObject>, 
    language: CodeLanguage,
    caption: Vec<RichTextObject>,
    children: Vec<Block>,
) -> Block {
    Block {
        object: None,
        id: None,
        parent: None,
        has_children: None,
        content: BlockContent::Code {
            code: CodeProperties {
                rich_text,
                language,
                caption,
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
    fn test_serialize_code() {
        let text = text::create_text("const hello = 'world';");
        let code = create_code(vec![text], CodeLanguage::Javascript);

        let json = to_value(&code).unwrap();
        assert_eq!(json["type"], "code");
        assert_eq!(json["code"]["rich_text"][0]["type"], "text");
        assert_eq!(json["code"]["rich_text"][0]["text"]["content"], "const hello = 'world';");
        assert_eq!(json["code"]["language"], "javascript");
        
        // Caption field might be skipped during serialization if empty
        let caption = &json["code"]["caption"];
        assert!(caption.is_null() || caption.as_array().map_or(false, |arr| arr.is_empty()));
    }

    #[test]
    fn test_serialize_code_with_caption() {
        let code_text = text::create_text("SELECT * FROM users;");
        let caption_text = text::create_text("SQL Query");
        let code = create_code_with_caption(
            vec![code_text], 
            CodeLanguage::Sql,
            vec![caption_text]
        );

        let json = to_value(&code).unwrap();
        assert_eq!(json["type"], "code");
        assert_eq!(json["code"]["rich_text"][0]["text"]["content"], "SELECT * FROM users;");
        assert_eq!(json["code"]["language"], "sql");
        assert_eq!(json["code"]["caption"][0]["text"]["content"], "SQL Query");
    }

    #[test]
    fn test_deserialize_code() {
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
            "type": "code",
            "code": {
                "rich_text": [{
                    "type": "text",
                    "text": {
                        "content": "def hello_world():\n    print('Hello, World!')",
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
                    "plain_text": "def hello_world():\n    print('Hello, World!')",
                    "href": null
                }],
                "language": "python",
                "caption": []
            }
        });

        let block: Block = from_value(json).unwrap();

        assert_eq!(block.id, Some("block-id".to_string()));
        assert_eq!(block.object, Some("block".to_string()));

        match block.content {
            BlockContent::Code { code } => {
                assert_eq!(code.rich_text.len(), 1);
                match &code.rich_text[0] {
                    text::RichTextObject::Text { text, .. } => {
                        assert_eq!(text.content, "def hello_world():\n    print('Hello, World!')");
                    },
                    _ => panic!("Expected Text variant"),
                }
                assert_eq!(code.language, CodeLanguage::Python);
                assert!(code.caption.is_empty());
            },
            _ => panic!("Expected Code variant"),
        }
    }
}