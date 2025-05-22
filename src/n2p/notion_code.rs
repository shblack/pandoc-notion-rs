use crate::n2p::ConversionConfig;
use notion_client::objects::block::{Block as NotionBlock, BlockType, CodeValue, Language};
use pandoc_types::definition::{Attr, Block as PandocBlock};

/// Builder for converting Notion code blocks to Pandoc code blocks
pub struct CodeBuilder {
    language: String,
    content: String,
    identifier: String,
    classes: Vec<String>,
    attributes: Vec<(String, String)>,
}

impl CodeBuilder {
    /// Create a new CodeBuilder with default values
    pub fn new() -> Self {
        Self {
            language: String::new(),
            content: String::new(),
            identifier: String::new(),
            classes: Vec::new(),
            attributes: Vec::new(),
        }
    }

    /// Set the programming language for the code block
    pub fn language(mut self, language: &str) -> Self {
        self.language = language.to_string();
        self.classes = vec![language.to_lowercase()];
        self
    }

    /// Set the code content
    pub fn content(mut self, content: &str) -> Self {
        self.content = content.to_string();
        self
    }

    /// Set an attribute for the code block
    pub fn attribute(mut self, key: &str, value: &str) -> Self {
        self.attributes.push((key.to_string(), value.to_string()));
        self
    }

    /// Set the identifier for the code block
    pub fn identifier(mut self, id: &str) -> Self {
        self.identifier = id.to_string();
        self
    }

    /// Build the Pandoc CodeBlock
    pub fn build(self, config: &ConversionConfig) -> PandocBlock {
        let attr = if config.preserve_attributes {
            Attr {
                identifier: self.identifier,
                classes: self.classes,
                attributes: self.attributes,
            }
        } else {
            // Empty attributes when not preserving, but always keep language class
            Attr {
                identifier: String::new(),
                classes: self.classes,
                attributes: Vec::new(),
            }
        };
        PandocBlock::CodeBlock(attr, self.content)
    }
}

/// Convert a Notion code block to a Pandoc code block
pub fn convert_notion_code(block: &NotionBlock, config: &ConversionConfig) -> Option<Vec<PandocBlock>> {
    match &block.block_type {
        BlockType::Code { code } => {
            let code_block = build_code_from_notion(code, config);
            Some(vec![code_block])
        }
        _ => None,
    }
}

/// Helper function to build a code block from Notion code data
fn build_code_from_notion(code: &CodeValue, config: &ConversionConfig) -> PandocBlock {
    // Convert rich text to plain text for code content
    let content = code.rich_text.iter()
        .map(|rt| rt.plain_text().unwrap_or_default())
        .collect::<String>();

    // Get language from Notion code block
    let language_str = convert_notion_language(&code.language);

    // Build CodeBlock
    CodeBuilder::new()
        .language(language_str)
        .content(&content)
        .build(config)
}

/// Convert Notion language enum to Pandoc language string
fn convert_notion_language(language: &Language) -> &str {
    match language {
        Language::Abap => "abap",
        Language::Arduino => "arduino",
        Language::Bash => "bash",
        Language::Basic => "basic",
        Language::C => "c",
        Language::Clojure => "clojure",
        Language::Coffeescript => "coffeescript",
        Language::CPlusPlus => "cpp",
        Language::CSharp => "csharp",
        Language::Css => "css",
        Language::Dart => "dart",
        Language::Diff => "diff",
        Language::Docker => "dockerfile",
        Language::Elixir => "elixir",
        Language::Elm => "elm",
        Language::Erlang => "erlang",
        Language::Flow => "flow",
        Language::Fortran => "fortran",
        Language::FSharp => "fsharp",
        Language::Gherkin => "gherkin",
        Language::Glsl => "glsl",
        Language::Go => "go",
        Language::Graphql => "graphql",
        Language::Groovy => "groovy",
        Language::Haskell => "haskell",
        Language::Html => "html",
        Language::Java => "java",
        Language::Javascript => "javascript",
        Language::Json => "json",
        Language::Julia => "julia",
        Language::Kotlin => "kotlin",
        Language::Latex => "latex",
        Language::Less => "less",
        Language::Lisp => "lisp",
        Language::Livescript => "livescript",
        Language::Lua => "lua",
        Language::Makefile => "makefile",
        Language::Markdown => "markdown",
        Language::Markup => "markup",
        Language::Matlab => "matlab",
        Language::Mermaid => "mermaid",
        Language::Nix => "nix",
        Language::ObjectiveC => "objectivec",
        Language::Ocaml => "ocaml",
        Language::Pascal => "pascal",
        Language::Perl => "perl",
        Language::Php => "php",
        Language::PlainText => "text",
        Language::Powershell => "powershell",
        Language::Prolog => "prolog",
        Language::Protobuf => "protobuf",
        Language::Python => "python",
        Language::R => "r",
        Language::Reason => "reason",
        Language::Ruby => "ruby",
        Language::Rust => "rust",
        Language::Sass => "sass",
        Language::Scala => "scala",
        Language::Scheme => "scheme",
        Language::Scss => "scss",
        Language::Shell => "shell",
        Language::Sql => "sql",
        Language::Swift => "swift",
        Language::Solidity => "solidity",
        Language::Typescript => "typescript",
        Language::VbNet => "vbnet",
        Language::Verilog => "verilog",
        Language::Vhdl => "vhdl",
        Language::VisualBasic => "vb",
        Language::Webassembly => "wasm",
        Language::Xml => "xml",
        Language::Yaml => "yaml",
        Language::JavaOrCOrCPlusPlusOrCSharp => "java", // Default to java
    }
}

/// Convenience function to directly convert any block to a code block if it is one
pub fn try_convert_to_code(block: &NotionBlock, config: &ConversionConfig) -> Option<PandocBlock> {
    convert_notion_code(block, config).map(|blocks| blocks[0].clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::n2p::ConversionConfig;
    use notion_client::objects::rich_text::{RichText, Text};

    // Test configuration for all tests
    fn test_config() -> ConversionConfig {
        ConversionConfig { 
            preserve_attributes: true,
            escape_markdown: false,
            render_toggle_div: false,
        }
    }

    // Helper function to create a rich text element for testing
    fn create_rich_text(content: &str) -> RichText {
        let text = Text {
            content: content.to_string(),
            link: None,
        };

        RichText::Text {
            text,
            annotations: None,
            plain_text: Some(content.to_string()),
            href: None,
        }
    }

    // Helper function to create a code block for testing
    fn create_code_block(content: &str, language: Language) -> NotionBlock {
        NotionBlock {
            object: Some("block".to_string()),
            id: Some("test_code_block_id".to_string()),
            parent: None,
            created_time: None,
            last_edited_time: None,
            created_by: None,
            last_edited_by: None,
            has_children: Some(false),
            archived: Some(false),
            block_type: BlockType::Code {
                code: CodeValue {
                    rich_text: vec![create_rich_text(content)],
                    language,
                    caption: vec![],
                },
            },
        }
    }

    #[test]
    fn test_convert_notion_code() {
        // Create a simple Notion code block with Rust code
        let content = "fn main() {\n    println!(\"Hello, world!\");\n}";
        let code_block = create_code_block(content, Language::Rust);

        // Convert to Pandoc
        let result = convert_notion_code(&code_block, &test_config());
        assert!(result.is_some());

        let blocks = result.unwrap();
        assert_eq!(blocks.len(), 1);

        // Verify the content and structure of the Pandoc block
        match &blocks[0] {
            PandocBlock::CodeBlock(attr, code_content) => {
                // Check code content
                assert_eq!(code_content, content);
                
                // Check language
                assert!(attr.classes.contains(&"rust".to_string()));
                
                // Check identifier and attributes
                assert_eq!(attr.identifier, "");
                assert!(attr.attributes.is_empty());
            }
            _ => panic!("Expected CodeBlock, got something else"),
        }
    }

    #[test]
    fn test_try_convert_to_code() {
        // Create a simple Notion code block with Python code
        let content = "def hello():\n    print('Hello, world!')";
        let code_block = create_code_block(content, Language::Python);

        // Convert using convenience function
        let result = try_convert_to_code(&code_block, &test_config());
        assert!(result.is_some());

        // Verify the content and structure of the Pandoc block
        match result.unwrap() {
            PandocBlock::CodeBlock(attr, code_content) => {
                // Check code content
                assert_eq!(code_content, content);
                
                // Check language
                assert!(attr.classes.contains(&"python".to_string()));
            }
            _ => panic!("Expected CodeBlock, got something else"),
        }
    }

    #[test]
    fn test_language_mapping() {
        // Test a few representative languages to ensure proper mapping
        assert_eq!(convert_notion_language(&Language::Javascript), "javascript");
        assert_eq!(convert_notion_language(&Language::Rust), "rust");
        assert_eq!(convert_notion_language(&Language::CPlusPlus), "cpp");
        assert_eq!(convert_notion_language(&Language::PlainText), "text");
        assert_eq!(convert_notion_language(&Language::JavaOrCOrCPlusPlusOrCSharp), "java");
    }

    #[test]
    fn test_code_builder() {
        // Test creating a code block using the builder pattern
        let builder = CodeBuilder::new()
            .language("python")
            .content("print('Hello')")
            .identifier("test-code")
            .attribute("data-line", "2");

        let block = builder.build(&test_config());

        match block {
            PandocBlock::CodeBlock(attr, content) => {
                assert_eq!(content, "print('Hello')");
                assert_eq!(attr.identifier, "test-code");
                assert_eq!(attr.classes, vec!["python"]);
                assert_eq!(attr.attributes, vec![("data-line".to_string(), "2".to_string())]);
            }
            _ => panic!("Expected CodeBlock, got something else"),
        }
    }

    #[test]
    fn test_non_code_block() {
        // Create a paragraph block (not a code block)
        let paragraph = NotionBlock {
            object: Some("block".to_string()),
            id: Some("paragraph_id".to_string()),
            parent: None,
            created_time: None,
            last_edited_time: None,
            created_by: None,
            last_edited_by: None,
            has_children: Some(false),
            archived: Some(false),
            block_type: BlockType::Paragraph {
                paragraph: notion_client::objects::block::ParagraphValue {
                    rich_text: vec![create_rich_text("Not a code block")],
                    color: None,
                    children: None,
                },
            },
        };

        // Try to convert it
        let result = try_convert_to_code(&paragraph, &test_config());
        
        // Should return None since it's not a code block
        assert!(result.is_none());
    }
}