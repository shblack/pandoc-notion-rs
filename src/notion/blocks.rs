use serde::{Serialize, Deserialize};

/// Block colors in Notion's API
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BlockColor {
    Default,
    Gray,
    Brown,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    Pink,
    Red,
    GrayBackground,
    BrownBackground,
    OrangeBackground,
    YellowBackground,
    GreenBackground,
    BlueBackground,
    PurpleBackground,
    PinkBackground,
    RedBackground,
}

impl Default for BlockColor {
    fn default() -> Self {
        Self::Default
    }
}

/// Parent types for blocks
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum BlockParent {
    #[serde(rename = "page_id")]
    Page { page_id: String },
    #[serde(rename = "workspace")]
    Workspace { workspace: bool },
    #[serde(rename = "block_id")]
    Block { block_id: String },
}

/// Minimal user information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserReference {
    pub object: String,
    pub id: String,
}

/// Block object as returned by the API
///
/// Note: We deliberately ignore several server-side metadata fields that are returned in the API response
/// but not relevant for our purposes: "created_time", "last_edited_time", "created_by", "last_edited_by",
/// "archived", and "in_trash".
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Block {
    /// The block type identification fields - only present on API responses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<BlockParent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_children: Option<bool>,

    /// Block content - this determines the block type
    #[serde(flatten)]
    pub content: BlockContent,
}

/// The content of a block, determining its type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum BlockContent {
    /// Paragraph block
    #[serde(rename = "paragraph")]
    Paragraph {
        /// Paragraph properties
        paragraph: crate::notion::paragraph::ParagraphProperties,
    },
    /// Heading 1 block
    #[serde(rename = "heading_1")]
    Heading1 {
        /// Heading 1 properties
        heading_1: crate::notion::headings::HeadingProperties,
    },
    /// Heading 2 block
    #[serde(rename = "heading_2")]
    Heading2 {
        /// Heading 2 properties
        heading_2: crate::notion::headings::HeadingProperties,
    },
    /// Heading 3 block
    #[serde(rename = "heading_3")]
    Heading3 {
        /// Heading 3 properties
        heading_3: crate::notion::headings::HeadingProperties,
    },
    /// Quote block
    #[serde(rename = "quote")]
    Quote {
        /// Quote properties
        quote: crate::notion::quote::QuoteProperties,
    },
    /// Bulleted list item block
    #[serde(rename = "bulleted_list_item")]
    BulletedListItem {
        /// Bulleted list item properties
        bulleted_list_item: crate::notion::list::BulletedListItemProperties,
    },
    /// Numbered list item block
    #[serde(rename = "numbered_list_item")]
    NumberedListItem {
        /// Numbered list item properties
        numbered_list_item: crate::notion::list::NumberedListItemProperties,
    },
    /// To-do list item block
    #[serde(rename = "to_do")]
    ToDo {
        /// To-do properties
        to_do: crate::notion::list::ToDoProperties,
    },
    /// Code block
    #[serde(rename = "code")]
    Code {
        /// Code properties
        code: crate::notion::code::CodeProperties,
    },
    // Other block types can be added here
}

// Re-export helper functions from paragraph.rs
pub use crate::notion::paragraph::{create_paragraph, create_paragraph_with_children};

// Re-export helper functions from headings.rs
pub use crate::notion::headings::{
    create_heading1, create_heading2, create_heading3, create_toggleable_heading1,
    create_toggleable_heading2, create_toggleable_heading3,
};

// Re-export helper functions from quote.rs
pub use crate::notion::quote::{create_quote, create_quote_with_children};

// Re-export helper functions from code.rs
pub use crate::notion::code::{
    create_code, create_code_with_caption, create_code_with_caption_and_children,
    create_code_with_children,
};

#[cfg(test)]
mod tests {}
