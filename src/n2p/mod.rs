//! Notion to Pandoc conversion utilities
//!
//! This module provides tools for converting Notion API data structures
//! to Pandoc document elements using the pandoc_types crate.
//! It enables the transformation of Notion's rich text and blocks
//! into Pandoc's Abstract Syntax Tree (AST) for document processing.
//!
//! ## Features
//!
//! - Convert Notion rich text objects to Pandoc inline elements
//! - Preserve text formatting (bold, italic, strikethrough, underline, code)
//! - Support equations as Pandoc math expressions
//! - Handle hyperlinks and URL references
//! - Convert Notion heading blocks to Pandoc headers
//! - Process nested block structures recursively using Visitor pattern
//! - Handle all standard Notion block types with proper conversion to Pandoc
//!
//! ## Architecture
//!
//! The conversion process uses the Visitor pattern to traverse Notion's block hierarchy,
//! mapping rich text and block types to their Pandoc equivalents while maintaining
//! formatting and semantic structure. The recursive traversal ensures that all nested
//! block children are properly processed.

/// Configuration for the Notion to Pandoc conversion process
#[derive(Debug, Clone)]
pub struct ConversionConfig {
    /// Whether to preserve Notion-specific attributes in Pandoc output
    /// When false, attributes are left blank (default)
    pub preserve_attributes: bool,
    /// Whether to escape special markdown characters in output
    /// When false, removes excessive escaping of characters (default)
    pub escape_markdown: bool,
}

impl Default for ConversionConfig {
    fn default() -> Self {
        Self {
            preserve_attributes: false,
            escape_markdown: false,  // Disabled by default
        }
    }
}

pub mod notion_block_visitor;
pub mod notion_code;
pub mod notion_heading;
pub mod notion_list;
pub mod notion_paragraph;
pub mod notion_quote;
pub mod notion_text;
pub mod visitor;

// Re-export key traits and implementations
pub use visitor::NotionBlockVisitor;
pub use notion_block_visitor::NotionToPandocVisitor;
