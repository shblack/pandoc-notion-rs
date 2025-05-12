// Notion API data structures and utilities
// This module contains types and functions to work with Notion's API objects

// Export submodules
pub mod blocks;
pub mod code;
pub mod headings;
pub mod list;
pub mod paragraph;
pub mod quote;
pub mod text;

// Re-export commonly used types for convenience
pub use blocks::{Block, BlockColor, BlockContent, BlockParent};
pub use code::{CodeLanguage, CodeProperties};
pub use headings::HeadingProperties;
pub use list::{BulletedListItemProperties, NumberedListItemProperties, ToDoProperties};
pub use paragraph::ParagraphProperties;
pub use quote::QuoteProperties;
pub use text::{Annotations, CloneTextObject, Color, Equation, Link, RichTextObject, TextContent};