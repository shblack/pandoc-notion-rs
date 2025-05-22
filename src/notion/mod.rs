//! Notion API related utilities and extensions
//!
//! This module provides utilities and extensions for working with the Notion API,
//! such as handling specialized block types and providing additional functionality
//! beyond what the base Notion API client offers.

pub mod toggleable;

// Re-export key types from toggleable
pub use toggleable::{ToggleableBlock, ToggleableBlockChildren};