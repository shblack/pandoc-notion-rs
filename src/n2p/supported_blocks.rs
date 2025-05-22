//! Supported block types for Notion to Pandoc conversion
//!
//! This module defines an enum of all block types supported by the converter
//! and provides utilities for checking support status.

use notion_client::objects::block::{Block as NotionBlock, BlockType};

/// Enum representing all block types supported by the Notion to Pandoc converter
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportedBlockType {
    /// Paragraph block
    Paragraph,
    /// Level 1 heading
    Heading1,
    /// Level 2 heading
    Heading2,
    /// Level 3 heading
    Heading3,
    /// Quote block
    Quote,
    /// Code block
    Code,
    /// Bulleted list item
    BulletedListItem,
    /// Numbered list item
    NumberedListItem,
    /// To-do list item
    ToDo,
    /// Toggle block
    Toggle,
    /// Divider (horizontal rule)
    Divider,
    /// Callout block
    Callout,
    /// Image block
    Image,
    /// Table block
    Table,
    /// Table row block
    TableRow,
    /// File block
    File,
    /// Video block
    Video,
}

impl SupportedBlockType {
    /// Returns a list of all supported block types
    pub fn all() -> Vec<SupportedBlockType> {
        vec![
            SupportedBlockType::Paragraph,
            SupportedBlockType::Heading1,
            SupportedBlockType::Heading2,
            SupportedBlockType::Heading3,
            SupportedBlockType::Quote,
            SupportedBlockType::Code,
            SupportedBlockType::BulletedListItem,
            SupportedBlockType::NumberedListItem,
            SupportedBlockType::ToDo,
            SupportedBlockType::Toggle,
            SupportedBlockType::Divider,
            SupportedBlockType::Callout,
            SupportedBlockType::Image,
            SupportedBlockType::Table,
            SupportedBlockType::TableRow,
            SupportedBlockType::File,
            SupportedBlockType::Video,
        ]
    }

    /// Returns a list of currently implemented block types
    pub fn implemented() -> Vec<SupportedBlockType> {
        vec![
            SupportedBlockType::Paragraph,
            SupportedBlockType::Heading1,
            SupportedBlockType::Heading2,
            SupportedBlockType::Heading3,
            SupportedBlockType::Quote,
            SupportedBlockType::Code,
            SupportedBlockType::BulletedListItem,
            SupportedBlockType::NumberedListItem,
            SupportedBlockType::ToDo,
            SupportedBlockType::Divider,
            // Add more as they are implemented
        ]
    }

    /// Returns true if the given block type is implemented
    pub fn is_implemented(&self) -> bool {
        Self::implemented().contains(self)
    }
}

/// Extension trait for NotionBlock to check if it's a supported block type
pub trait SupportedBlock {
    /// Returns the SupportedBlockType if the block is supported, None otherwise
    fn supported_type(&self) -> Option<SupportedBlockType>;
    
    /// Returns true if the block type is supported
    fn is_supported(&self) -> bool;
    
    /// Returns true if the block type is implemented
    fn is_implemented(&self) -> bool;
}

impl SupportedBlock for NotionBlock {
    fn supported_type(&self) -> Option<SupportedBlockType> {
        match &self.block_type {
            BlockType::Paragraph { .. } => Some(SupportedBlockType::Paragraph),
            BlockType::Heading1 { .. } => Some(SupportedBlockType::Heading1),
            BlockType::Heading2 { .. } => Some(SupportedBlockType::Heading2),
            BlockType::Heading3 { .. } => Some(SupportedBlockType::Heading3),
            BlockType::Quote { .. } => Some(SupportedBlockType::Quote),
            BlockType::Code { .. } => Some(SupportedBlockType::Code),
            BlockType::BulletedListItem { .. } => Some(SupportedBlockType::BulletedListItem),
            BlockType::NumberedListItem { .. } => Some(SupportedBlockType::NumberedListItem),
            BlockType::ToDo { .. } => Some(SupportedBlockType::ToDo),
            BlockType::Toggle { .. } => Some(SupportedBlockType::Toggle),
            BlockType::Divider { .. } => Some(SupportedBlockType::Divider),
            BlockType::Callout { .. } => Some(SupportedBlockType::Callout),
            BlockType::Image { .. } => Some(SupportedBlockType::Image),
            BlockType::Table { .. } => Some(SupportedBlockType::Table),
            BlockType::TableRow { .. } => Some(SupportedBlockType::TableRow),
            BlockType::File { .. } => Some(SupportedBlockType::File),
            BlockType::Video { .. } => Some(SupportedBlockType::Video),
            _ => None,
        }
    }
    
    fn is_supported(&self) -> bool {
        self.supported_type().is_some()
    }
    
    fn is_implemented(&self) -> bool {
        if let Some(block_type) = self.supported_type() {
            block_type.is_implemented()
        } else {
            false
        }
    }
}