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

pub mod notion_block_visitor;
pub mod notion_heading;
pub mod notion_list;
pub mod notion_paragraph;
pub mod notion_quote;
pub mod notion_text;
pub mod visitor;
