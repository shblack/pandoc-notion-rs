//! Toggleable block handling
//! 
//! This module provides a trait and utilities for working with toggleable blocks,
//! such as Notion toggleable headings that can contain children.

use notion_client::objects::block::Block as NotionBlock;
use std::collections::HashMap;

mod toggleable_block;
#[cfg(test)]
mod toggleable_heading_test;

pub use toggleable_block::ToggleableBlock;

/// Manages children for toggleable blocks
#[derive(Debug, Default, Clone)]
pub struct ToggleableBlockChildren {
    /// Map of block IDs to their children
    children_map: HashMap<String, Vec<NotionBlock>>,
}

impl ToggleableBlockChildren {
    /// Create a new empty manager
    pub fn new() -> Self {
        Self {
            children_map: HashMap::new(),
        }
    }
    
    /// Add children for a toggleable block
    pub fn add_children(&mut self, block: &impl ToggleableBlock, children: Vec<NotionBlock>) -> bool {
        if block.is_toggleable() && block.has_children() {
            if let Some(id) = block.block_id() {
                self.children_map.insert(id.to_string(), children);
                return true;
            }
        }
        false
    }
    
    /// Get children for a toggleable block
    pub fn get_children(&self, block: &impl ToggleableBlock) -> Option<&Vec<NotionBlock>> {
        if block.is_toggleable() {
            if let Some(id) = block.block_id() {
                return self.children_map.get(id);
            }
        }
        None
    }
    
    /// Check if the manager has children for a block
    pub fn has_children_for(&self, block: &impl ToggleableBlock) -> bool {
        self.get_children(block).is_some()
    }
    
    /// Get a reference to the underlying map
    pub fn map(&self) -> &HashMap<String, Vec<NotionBlock>> {
        &self.children_map
    }
    
    /// Take ownership of the internal map
    pub fn into_map(self) -> HashMap<String, Vec<NotionBlock>> {
        self.children_map
    }
    
    /// Create from an existing map
    pub fn from_map(map: HashMap<String, Vec<NotionBlock>>) -> Self {
        Self {
            children_map: map,
        }
    }
}