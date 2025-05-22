//! Pandoc to Notion conversion utilities
//!
//! This module provides tools for converting Pandoc document elements
//! to Notion API data structures using the official notion-client types.
//! It enables the transformation of Pandoc's Abstract Syntax Tree (AST)
//! into Notion's rich text objects for API integration.
//!
//! ## Features
//!
//! - Convert Pandoc inline elements to Notion rich text objects
//! - Preserve text formatting (bold, italic, strikethrough, underline, code)
//! - Support mathematical expressions as Notion equations
//! - Handle hyperlinks and URL references
//! - Preserve whitespace and line breaks
//! - Convert Pandoc spans with CSS classes to Notion text colors
//! - Support for paragraphs and headings with visitor pattern
//!
//! ## Architecture
//!
//! The conversion process uses a non-recursive approach to handle nested elements,
//! avoiding circular references. A TextBuilder helps construct Notion rich text
//! objects (from notion-client crate) with the appropriate formatting and attributes.
//!
//! The visitor pattern is implemented to traverse Pandoc block elements and convert
//! them to appropriate Notion blocks.

pub mod pandoc_block_visitor;
pub mod pandoc_code;
pub mod pandoc_divider;
pub mod pandoc_heading;
pub mod pandoc_list;
pub mod pandoc_paragraph;
pub mod pandoc_quote;
pub mod pandoc_text;
pub mod visitor;

// Re-export key traits and implementations
pub use visitor::PandocBlockVisitor;
pub use pandoc_block_visitor::PandocToNotionVisitor;
