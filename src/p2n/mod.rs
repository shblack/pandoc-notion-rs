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
//!
//! ## Architecture
//!
//! The conversion process uses a non-recursive approach to handle nested elements,
//! avoiding circular references. A TextBuilder helps construct Notion rich text 
//! objects (from notion-client crate) with the appropriate formatting and attributes.

pub mod pandoc_text;
