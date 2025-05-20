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
//!
//! ## Architecture
//!
//! The conversion process maps Notion's rich text structure and block types
//! to their Pandoc equivalents, maintaining formatting and semantic structure.

pub mod notion_text;
