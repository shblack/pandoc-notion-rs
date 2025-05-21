//! Module for text-based input/output operations
//!
//! This module provides functionality for converting between text formats
//! and Pandoc AST, independent of Notion integration.

use pandoc_types::definition::Pandoc;
use std::path::Path;
use std::error::Error;
use std::fmt;

// Re-export submodules
pub mod processor;

/// Error type for text processing operations
#[derive(Debug)]
pub enum TextProcessingError {
    /// I/O error
    IoError(std::io::Error),
    /// Pandoc process error
    PandocError(String),
    /// JSON parsing error
    JsonError(serde_json::Error),
    /// Encoding error
    EncodingError(std::string::FromUtf8Error),
    /// Other errors
    Other(String),
}

impl fmt::Display for TextProcessingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "I/O error: {}", e),
            Self::PandocError(s) => write!(f, "Pandoc error: {}", s),
            Self::JsonError(e) => write!(f, "JSON error: {}", e),
            Self::EncodingError(e) => write!(f, "Encoding error: {}", e),
            Self::Other(s) => write!(f, "Error: {}", s),
        }
    }
}

impl Error for TextProcessingError {}

// Implement From for common error types
impl From<std::io::Error> for TextProcessingError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<serde_json::Error> for TextProcessingError {
    fn from(err: serde_json::Error) -> Self {
        Self::JsonError(err)
    }
}

impl From<std::string::FromUtf8Error> for TextProcessingError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Self::EncodingError(err)
    }
}

/// Supported text formats for conversion
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextFormat {
    /// Standard Markdown
    Markdown,
    /// CommonMark specification
    CommonMark,
    /// GitHub Flavored Markdown
    GithubMarkdown,
    /// Plain text
    PlainText,
    /// HTML
    Html,
    /// LaTeX
    Latex,
    /// ReStructuredText
    Rst,
    /// Org mode
    Org,
    /// Custom format (pass pandoc format string directly)
    Custom(&'static str),
}

impl TextFormat {
    /// Get the format string as expected by Pandoc
    pub fn as_pandoc_format(&self) -> &str {
        match self {
            Self::Markdown => "markdown",
            Self::CommonMark => "commonmark",
            Self::GithubMarkdown => "gfm",
            Self::PlainText => "plain",
            Self::Html => "html",
            Self::Latex => "latex",
            Self::Rst => "rst",
            Self::Org => "org",
            Self::Custom(fmt) => fmt,
        }
    }
    
    /// Try to determine format from file extension
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "md" => Some(Self::Markdown),
            "markdown" => Some(Self::Markdown),
            "txt" => Some(Self::PlainText),
            "html" | "htm" => Some(Self::Html),
            "tex" => Some(Self::Latex),
            "rst" => Some(Self::Rst),
            "org" => Some(Self::Org),
            _ => None,
        }
    }
}

/// Trait for text processing operations
pub trait TextProcessor {
    /// Convert text to Pandoc AST
    fn text_to_ast(&self, text: &str, format: TextFormat) -> Result<Pandoc, TextProcessingError>;
    
    /// Convert Pandoc AST to text
    fn ast_to_text(&self, ast: &Pandoc, format: TextFormat) -> Result<String, TextProcessingError>;
    
    /// Convert text from one format to another
    fn convert_text(
        &self,
        text: &str,
        from_format: TextFormat,
        to_format: TextFormat
    ) -> Result<String, TextProcessingError>;
    
    /// Convert a file to Pandoc AST
    fn file_to_ast<P: AsRef<Path>>(&self, path: P) -> Result<Pandoc, TextProcessingError> {
        // Default implementation tries to determine format from extension
        let path = path.as_ref();
        let format = path.extension()
            .and_then(|ext| ext.to_str())
            .and_then(TextFormat::from_extension)
            .unwrap_or(TextFormat::Markdown); // Default to markdown
            
        self.file_to_ast_with_format(path, format)
    }
    
    /// Convert a file to Pandoc AST with explicit format
    fn file_to_ast_with_format<P: AsRef<Path>>(
        &self,
        path: P,
        format: TextFormat
    ) -> Result<Pandoc, TextProcessingError>;
    
    /// Convert Pandoc AST to a file
    fn ast_to_file<P: AsRef<Path>>(&self, ast: &Pandoc, path: P) -> Result<(), TextProcessingError> {
        // Default implementation tries to determine format from extension
        let path = path.as_ref();
        let format = path.extension()
            .and_then(|ext| ext.to_str())
            .and_then(TextFormat::from_extension)
            .unwrap_or(TextFormat::Markdown); // Default to markdown
            
        self.ast_to_file_with_format(ast, path, format)
    }
    
    /// Convert Pandoc AST to a file with explicit format
    fn ast_to_file_with_format<P: AsRef<Path>>(
        &self,
        ast: &Pandoc,
        path: P,
        format: TextFormat
    ) -> Result<(), TextProcessingError>;
    
    /// Convert from one file format to another
    fn convert_file<P: AsRef<Path>, Q: AsRef<Path>>(
        &self,
        input_path: P,
        output_path: Q
    ) -> Result<(), TextProcessingError>;
    
    /// Convert from one file format to another with explicit formats
    fn convert_file_with_format<P: AsRef<Path>, Q: AsRef<Path>>(
        &self,
        input_path: P,
        output_path: Q,
        from_format: TextFormat,
        to_format: TextFormat
    ) -> Result<(), TextProcessingError>;
}