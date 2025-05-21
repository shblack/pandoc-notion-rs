use notion_client::objects::block::{Block as NotionBlock, BlockType, CodeValue, Language};
use notion_client::objects::rich_text::RichText;
use pandoc_types::definition::{Attr, Block as PandocBlock};
use std::error::Error;

use crate::p2n::pandoc_text::PandocTextConverter;

/// Builder for constructing Notion code blocks from Pandoc CodeBlock
pub struct NotionCodeBuilder {
    rich_text: Vec<RichText>,
    language: Language,
    caption: Vec<RichText>,
}

impl NotionCodeBuilder {
    /// Create a new NotionCodeBuilder with default values
    pub fn new() -> Self {
        Self {
            rich_text: Vec::new(),
            language: Language::PlainText,
            caption: Vec::new(),
        }
    }

    /// Set the rich text content
    pub fn rich_text(mut self, rich_text: Vec<RichText>) -> Self {
        self.rich_text = rich_text;
        self
    }

    /// Add a rich text element to the code
    pub fn add_rich_text(mut self, text: RichText) -> Self {
        self.rich_text.push(text);
        self
    }

    /// Set the programming language
    pub fn language(mut self, language: Language) -> Self {
        self.language = language;
        self
    }

    /// Set caption
    pub fn caption(mut self, caption: Vec<RichText>) -> Self {
        self.caption = caption;
        self
    }

    /// Build the Notion code block
    pub fn build(self) -> NotionBlock {
        let code_value = CodeValue {
            rich_text: self.rich_text,
            language: self.language,
            caption: self.caption,
        };

        NotionBlock {
            object: Some("block".to_string()),
            id: Some(String::new()), // Will be filled by Notion API
            parent: None,
            created_time: None,
            created_by: None,
            last_edited_time: None,
            last_edited_by: None,
            archived: Some(false),
            has_children: Some(false), // Code blocks don't have children
            block_type: BlockType::Code {
                code: code_value,
            },
        }
    }
}

/// Converter for transforming Pandoc code blocks to Notion code blocks
pub struct PandocCodeConverter {
    text_converter: PandocTextConverter,
}

impl Default for PandocCodeConverter {
    fn default() -> Self {
        Self::new()
    }
}

impl PandocCodeConverter {
    /// Create a new PandocCodeConverter
    pub fn new() -> Self {
        Self {
            text_converter: PandocTextConverter::default(),
        }
    }

    /// Convert a Pandoc CodeBlock to a Notion code block
    pub fn convert(&self, attr: &Attr, content: &str) -> Result<NotionBlock, Box<dyn Error>> {
        // Convert the string content to the expected rich text format
        let rich_text = self.text_converter.convert_plain_text(content)?;
        
        // Determine the language from the classes attribute
        let language = self.determine_language(attr);
        
        // Build and return the Notion code block
        let builder = NotionCodeBuilder::new()
            .rich_text(rich_text)
            .language(language);
        
        // TODO: Extract caption from attributes if supported
        
        Ok(builder.build())
    }
    
    /// Try to convert any Pandoc block to a Notion code block if it's a CodeBlock
    pub fn try_convert(
        &self,
        block: &PandocBlock,
        _parent_id: Option<String>, // Kept for API compatibility but not used
    ) -> Result<Option<NotionBlock>, Box<dyn Error>> {
        match block {
            PandocBlock::CodeBlock(attr, content) => {
                let result = self.convert(attr, content)?;
                Ok(Some(result))
            },
            _ => Ok(None),
        }
    }
    
    /// Determine the Notion language from Pandoc attributes
    fn determine_language(&self, attr: &Attr) -> Language {
        // Check if there's a language class
        if attr.classes.is_empty() {
            return Language::PlainText;
        }
        
        // Use the first class as the language identifier
        let language_id = attr.classes[0].to_lowercase();
        self.map_language_to_notion(&language_id)
    }
    
    /// Map Pandoc language identifier to Notion Language enum
    fn map_language_to_notion(&self, language: &str) -> Language {
        match language {
            "abap" => Language::Abap,
            "arduino" => Language::Arduino,
            "bash" | "sh" | "shell" | "zsh" => Language::Bash,
            "basic" => Language::Basic,
            "c" => Language::C,
            "clojure" | "clj" => Language::Clojure,
            "coffeescript" | "coffee" => Language::Coffeescript,
            "cpp" | "c++" => Language::CPlusPlus,
            "csharp" | "cs" | "c#" => Language::CSharp,
            "css" => Language::Css,
            "dart" => Language::Dart,
            "diff" => Language::Diff,
            "dockerfile" | "docker" => Language::Docker,
            "elixir" | "ex" => Language::Elixir,
            "elm" => Language::Elm,
            "erlang" | "erl" => Language::Erlang,
            "flow" => Language::Flow,
            "fortran" => Language::Fortran,
            "fsharp" | "f#" => Language::FSharp,
            "gherkin" => Language::Gherkin,
            "glsl" => Language::Glsl,
            "go" => Language::Go,
            "graphql" => Language::Graphql,
            "groovy" => Language::Groovy,
            "haskell" | "hs" => Language::Haskell,
            "html" => Language::Html,
            "java" => Language::Java,
            "javascript" | "js" => Language::Javascript,
            "json" => Language::Json,
            "julia" => Language::Julia,
            "kotlin" | "kt" => Language::Kotlin,
            "latex" | "tex" => Language::Latex,
            "less" => Language::Less,
            "lisp" => Language::Lisp,
            "livescript" | "ls" => Language::Livescript,
            "lua" => Language::Lua,
            "makefile" | "make" => Language::Makefile,
            "markdown" | "md" => Language::Markdown,
            "markup" => Language::Markup,
            "matlab" => Language::Matlab,
            "mermaid" => Language::Mermaid,
            "nix" => Language::Nix,
            "objectivec" | "objc" => Language::ObjectiveC,
            "ocaml" | "ml" => Language::Ocaml,
            "pascal" => Language::Pascal,
            "perl" | "pl" => Language::Perl,
            "php" => Language::Php,
            "text" | "plaintext" | "txt" => Language::PlainText,
            "powershell" | "ps" | "ps1" => Language::Powershell,
            "prolog" => Language::Prolog,
            "protobuf" | "proto" => Language::Protobuf,
            "python" | "py" => Language::Python,
            "r" => Language::R,
            "reason" => Language::Reason,
            "ruby" | "rb" => Language::Ruby,
            "rust" | "rs" => Language::Rust,
            "sass" => Language::Sass,
            "scala" => Language::Scala,
            "scheme" => Language::Scheme,
            "scss" => Language::Scss,
            "sql" => Language::Sql,
            "swift" => Language::Swift,
            "solidity" | "sol" => Language::Solidity,
            "typescript" | "ts" => Language::Typescript,
            "vbnet" => Language::VbNet,
            "verilog" => Language::Verilog,
            "vhdl" => Language::Vhdl,
            "visualbasic" | "vb" => Language::VisualBasic,
            "webassembly" | "wasm" => Language::Webassembly,
            "xml" => Language::Xml,
            "yaml" | "yml" => Language::Yaml,
            _ => Language::PlainText, // Default to plain text for unknown languages
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pandoc_types::definition::Inline;
    
    #[test]
    fn test_convert_simple_code_block() {
        let code_converter = PandocCodeConverter::new();
        
        // Create a simple code block with Python syntax
        let attr = Attr {
            identifier: String::new(),
            classes: vec!["python".to_string()],
            attributes: vec![],
        };
        let content = "def hello_world():\n    print('Hello, world!')";
        
        // Convert to Notion
        let result = code_converter.convert(&attr, content).unwrap();
        
        // Verify the result
        match &result.block_type {
            BlockType::Code { code } => {
                // Check content
                let combined_text: String = code.rich_text.iter()
                    .map(|rt| rt.plain_text().unwrap_or_default())
                    .collect();
                assert_eq!(combined_text, content);
                
                // Check language
                assert_eq!(code.language, Language::Python);
                
                // Check caption
                assert!(code.caption.is_empty());
            },
            _ => panic!("Expected a Code block")
        }
    }
    
    #[test]
    fn test_language_mapping() {
        let code_converter = PandocCodeConverter::new();
        
        // Test various language mappings
        assert_eq!(code_converter.map_language_to_notion("python"), Language::Python);
        assert_eq!(code_converter.map_language_to_notion("py"), Language::Python);
        assert_eq!(code_converter.map_language_to_notion("rust"), Language::Rust);
        assert_eq!(code_converter.map_language_to_notion("js"), Language::Javascript);
        assert_eq!(code_converter.map_language_to_notion("c++"), Language::CPlusPlus);
        assert_eq!(code_converter.map_language_to_notion("plaintext"), Language::PlainText);
        assert_eq!(code_converter.map_language_to_notion("unknown"), Language::PlainText); // Default
    }
    
    #[test]
    fn test_try_convert() {
        let code_converter = PandocCodeConverter::new();
        
        // Create a CodeBlock
        let attr = Attr {
            identifier: String::new(),
            classes: vec!["rust".to_string()],
            attributes: vec![],
        };
        let content = "fn main() { println!(\"Hello\"); }";
        let block = PandocBlock::CodeBlock(attr, content.to_string());
        
        // Try to convert it
        let result = code_converter.try_convert(&block, None).unwrap();
        
        // Should return Some(NotionBlock)
        assert!(result.is_some());
        
        // Create a non-CodeBlock block
        let non_code_block = PandocBlock::Para(vec![Inline::Str("Not code".to_string())]);
        
        // Try to convert it
        let result = code_converter.try_convert(&non_code_block, None).unwrap();
        
        // Should return None
        assert!(result.is_none());
    }
    
    #[test]
    fn test_builder() {
        // Test code block builder
        let builder = NotionCodeBuilder::new()
            .language(Language::Javascript)
            .add_rich_text(RichText::Text {
                text: notion_client::objects::rich_text::Text {
                    content: "console.log('test');".to_string(),
                    link: None,
                },
                annotations: None,
                plain_text: Some("console.log('test');".to_string()),
                href: None,
            });
        
        let block = builder.build();
        
        match &block.block_type {
            BlockType::Code { code } => {
                assert_eq!(code.language, Language::Javascript);
                assert_eq!(code.rich_text.len(), 1);
                assert_eq!(code.rich_text[0].plain_text().unwrap(), "console.log('test');");
            },
            _ => panic!("Expected Code block")
        }
    }
    
    #[test]
    fn test_determine_language() {
        let code_converter = PandocCodeConverter::new();
        
        // Test with a language class
        let attr_with_lang = Attr {
            identifier: String::new(),
            classes: vec!["python".to_string()],
            attributes: vec![],
        };
        assert_eq!(code_converter.determine_language(&attr_with_lang), Language::Python);
        
        // Test with multiple classes (should use first)
        let attr_multi = Attr {
            identifier: String::new(),
            classes: vec!["rust".to_string(), "highlighted".to_string()],
            attributes: vec![],
        };
        assert_eq!(code_converter.determine_language(&attr_multi), Language::Rust);
        
        // Test with no classes
        let attr_empty = Attr {
            identifier: String::new(),
            classes: vec![],
            attributes: vec![],
        };
        assert_eq!(code_converter.determine_language(&attr_empty), Language::PlainText);
    }
}