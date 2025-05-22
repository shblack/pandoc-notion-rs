//! Notion Block Fetcher
//!
//! This module provides functionality to recursively fetch blocks and their children
//! from the Notion API, ensuring nested content like lists and blockquotes are properly
//! preserved during conversion.

use log::{debug, warn};
use notion_client::endpoints::Client as NotionClient;
use notion_client::objects::block::{Block as NotionBlock, BlockType};
use std::collections::VecDeque;
use std::time::Duration;
use tokio::time::sleep;
use crate::notion::toggleable::{ToggleableBlock, ToggleableBlockChildren};

/// Error types for block fetching operations
/// Error type for the block fetcher
#[derive(Debug, thiserror::Error)]
#[allow(dead_code)]
pub enum BlockFetcherError {
    #[error("Notion API error: {0}")]
    NotionApi(#[from] notion_client::NotionClientError),
    
    #[error("Missing block ID")]
    #[allow(dead_code)]
    MissingBlockId,
    
    #[error("Rate limit exceeded")]
    #[allow(dead_code)]
    RateLimit,
    
    #[error("Other error: {0}")]
    #[allow(dead_code)]
    Other(String),
}

/// Result type for block fetcher operations
pub type Result<T> = std::result::Result<T, BlockFetcherError>;

/// Configuration for the block fetcher
#[derive(Debug, Clone)]
pub struct BlockFetcherConfig {
    /// Maximum recursion depth (default: 10)
    pub max_depth: usize,
    
    /// Delay between API calls to avoid rate limits (in milliseconds, default: 0)
    pub api_call_delay_ms: u64,
    
    /// Whether to fetch all blocks at once (breadth-first) or depth-first
    pub breadth_first: bool,
}

impl Default for BlockFetcherConfig {
    fn default() -> Self {
        Self {
            max_depth: 10,
            api_call_delay_ms: 0,
            breadth_first: false,
        }
    }
}

/// NotionBlockFetcher handles recursive fetching of blocks and their children
pub struct NotionBlockFetcher {
    /// Notion API client
    client: NotionClient,
    
    /// Configuration for the fetcher
    config: BlockFetcherConfig,
}

impl NotionBlockFetcher {
    /// Create a new block fetcher with the given client and default configuration
    pub fn new(client: NotionClient) -> Self {
        debug!("Creating new NotionBlockFetcher with default configuration");
        Self {
            client,
            config: BlockFetcherConfig::default(),
        }
    }
    
    /// Create a new block fetcher with custom configuration
    #[allow(dead_code)]
    pub fn with_config(client: NotionClient, config: BlockFetcherConfig) -> Self {
        Self {
            client,
            config,
        }
    }
    
    /// Set the maximum recursion depth
    #[allow(dead_code)]
    pub fn with_max_depth(mut self, max_depth: usize) -> Self {
        self.config.max_depth = max_depth;
        self
    }
    
    /// Set the API call delay in milliseconds
    pub fn with_api_call_delay(mut self, delay_ms: u64) -> Self {
        self.config.api_call_delay_ms = delay_ms;
        self
    }
    

    
    /// Fetch a block and all its children recursively
    pub async fn fetch_block_with_children(&self, block_id: &str) -> Result<FetchResult> {
        debug!("Fetching blocks for parent: {}", block_id);
        
        // Fetch the top-level blocks
        let top_level_blocks = self.fetch_blocks(block_id).await?;
        
        let mut toggleable_children = ToggleableBlockChildren::new();
        
        let blocks = if self.config.breadth_first {
            self.fetch_children_breadth_first(top_level_blocks, &mut toggleable_children).await?
        } else {
            self.fetch_children_depth_first(top_level_blocks, 0, &mut toggleable_children).await?
        };
        
        Ok(FetchResult {
            blocks,
            toggleable_children,
        })
    }
    
    /// Fetch blocks from the Notion API for a specific parent
    async fn fetch_blocks(&self, block_id: &str) -> Result<Vec<NotionBlock>> {
        // Apply delay if configured to avoid rate limits
        if self.config.api_call_delay_ms > 0 {
            sleep(Duration::from_millis(self.config.api_call_delay_ms)).await;
        }
        
        let response = self.client.blocks.retrieve_block_children(block_id, None, None)
            .await
            .map_err(BlockFetcherError::NotionApi)?;
        
        debug!("Fetched {} child blocks", response.results.len());
        
        Ok(response.results)
    }
    
    /// Depth-first traversal to fetch all children
    async fn fetch_children_depth_first(
        &self, 
        blocks: Vec<NotionBlock>, 
        depth: usize,
        toggleable_children: &mut ToggleableBlockChildren
    ) -> Result<Vec<NotionBlock>> {
        // Stop if we've reached the maximum depth
        if depth >= self.config.max_depth {
            debug!("Reached maximum depth ({}), stopping recursion", self.config.max_depth);
            return Ok(blocks);
        }
        
        let mut result = Vec::new();
        
        for mut block in blocks {
            // Special handling for toggleable blocks
            if block.is_toggleable() && block.has_children() {
                if let Some(id) = block.block_id() {
                    debug!("Found toggleable block with children: {}", id);
                    
                    // Fetch children
                    let children = self.fetch_blocks(id).await?;
                    
                    // Process children recursively
                    let processed_children = Box::pin(
                        self.fetch_children_depth_first(children, depth + 1, toggleable_children)
                    ).await?;
                    
                    // For toggle blocks, attach children directly to the block
                    if let BlockType::Toggle { .. } = &mut block.block_type {
                        self.attach_children_to_block(&mut block, processed_children.clone());
                    }
                    
                    // Also store in the manager (useful for toggleable headings)
                    toggleable_children.add_children(&block, processed_children);
                }
            }
            // Regular handling for non-toggleable blocks with children
            else if self.block_has_children(&block) {
                if let Some(id) = block.id.as_ref() {
                    // Fetch children
                    debug!("Block of type {:?} has children, fetching recursively...", block.block_type);
                    
                    let children = self.fetch_blocks(id).await?;
                    
                    // Recursively process children - use Box::pin to handle recursion in async fn
                    let processed_children = Box::pin(
                        self.fetch_children_depth_first(children, depth + 1, toggleable_children)
                    ).await?;
                    
                    // Attach children to the block
                    self.attach_children_to_block(&mut block, processed_children);
                }
            }
            
            result.push(block);
        }
        
        Ok(result)
    }
    
    /// Breadth-first traversal to fetch all children
    async fn fetch_children_breadth_first(
        &self, 
        blocks: Vec<NotionBlock>,
        toggleable_children: &mut ToggleableBlockChildren
    ) -> Result<Vec<NotionBlock>> {
        // Create a map to track blocks by ID
        let mut blocks_map = std::collections::HashMap::new();
        
        // Keep track of parent-child relationships separately
        let mut parent_child_map = std::collections::HashMap::new();
        
        // Track toggleable blocks for later processing
        let mut toggleable_block_ids = Vec::new();
        
        // Initialize the queue with the top-level blocks
        let mut queue = VecDeque::new();
        for block in blocks {
            if let Some(id) = block.id.clone() {
                let has_children = self.block_has_children(&block);
                
                // Check if this is a toggleable block
                if block.is_toggleable() && has_children {
                    toggleable_block_ids.push(id.clone());
                }
                
                blocks_map.insert(id.clone(), block);
                if has_children {
                    queue.push_back((id.clone(), 0));
                }
            }
        }
        
        // Process the queue
        while let Some((block_id, depth)) = queue.pop_front() {
            // Skip if we've reached the maximum depth
            if depth >= self.config.max_depth {
                continue;
            }
            
            // Fetch children
            let children = self.fetch_blocks(&block_id).await?;
            
            // Track child IDs for this parent
            let mut child_ids = Vec::new();
            
            // Process each child
            for child in children {
                if let Some(child_id) = child.id.clone() {
                    child_ids.push(child_id.clone());
                    
                    // Add the child to the blocks map
                    let has_children = self.block_has_children(&child);
                    
                    // Check if this is a toggleable block
                    if child.is_toggleable() && has_children {
                        toggleable_block_ids.push(child_id.clone());
                    }
                    
                    blocks_map.insert(child_id.clone(), child);
                    
                    // If the child has children, add it to the queue
                    if has_children {
                        queue.push_back((child_id.clone(), depth + 1));
                    }
                }
            }
            
            // Store the parent-child relationship
            parent_child_map.insert(block_id, child_ids);
        }
        
        // First collect all child IDs to determine top-level blocks
        let all_child_ids: std::collections::HashSet<_> = parent_child_map
            .values()
            .flat_map(|ids| ids.iter().cloned())
            .collect();
            
        // Create a clone of the blocks_map to use when building parent-child relationships
        let blocks_map_clone = blocks_map.clone();
        
        // Now build the tree structure
        for (parent_id, child_ids) in &parent_child_map {
            if let Some(parent) = blocks_map.get_mut(parent_id) {
                let mut parent_children = Vec::new();
                for child_id in child_ids {
                    if let Some(child) = blocks_map_clone.get(child_id) {
                        parent_children.push(child.clone());
                    }
                }
                
                // Special handling for toggleable blocks
                if parent.is_toggleable() {
                    // For toggle blocks, use their native children field
                    if let BlockType::Toggle { .. } = &parent.block_type {
                        // Children are already attached by attach_children_to_block below
                    }
                    
                    // Store in toggleable children manager (for toggleable headings and as backup)
                    toggleable_children.add_children(parent, parent_children.clone());
                }
                
                // Regular handling for all blocks with children
                self.attach_children_to_block(parent, parent_children);
            }
        }
        
        // Return all the top-level blocks
        Ok(blocks_map
            .into_iter()
            .filter(|(id, _)| !all_child_ids.contains(id))
            .map(|(_, block)| block)
            .collect())
    }
    
    /// Check if a block has children
    fn block_has_children(&self, block: &NotionBlock) -> bool {
        block.has_children.unwrap_or(false)
    }
    
    /// Get the children of a block (if any)
    fn _get_block_children<'a>(&self, block: &'a NotionBlock) -> Option<&'a Vec<NotionBlock>> {
        match &block.block_type {
            BlockType::Paragraph { paragraph } => paragraph.children.as_ref(),
            BlockType::BulletedListItem { bulleted_list_item } => bulleted_list_item.children.as_ref(),
            BlockType::NumberedListItem { numbered_list_item } => numbered_list_item.children.as_ref(),
            BlockType::ToDo { to_do } => to_do.children.as_ref(),
            BlockType::Toggle { toggle } => toggle.children.as_ref(),
            BlockType::Quote { quote } => quote.children.as_ref(),
            _ => None,
        }
    }
    
    /// Attach children to a block based on its type
    fn attach_children_to_block(&self, block: &mut NotionBlock, children: Vec<NotionBlock>) {
        if children.is_empty() {
            return;
        }
        
        match &mut block.block_type {
            BlockType::Paragraph { paragraph } => {
                paragraph.children = Some(children);
            },
            BlockType::BulletedListItem { bulleted_list_item } => {
                bulleted_list_item.children = Some(children);
            },
            BlockType::NumberedListItem { numbered_list_item } => {
                numbered_list_item.children = Some(children);
            },
            BlockType::ToDo { to_do } => {
                to_do.children = Some(children);
            },
            BlockType::Toggle { toggle } => {
                toggle.children = Some(children);
            },
            BlockType::Quote { quote } => {
                quote.children = Some(children);
            },
            _ => {
                warn!("Cannot attach children to block type {:?}", block.block_type);
            }
        }
    }
}

/// Result of a block fetch operation
pub struct FetchResult {
    /// The blocks fetched
    pub blocks: Vec<NotionBlock>,
    
    /// Children of toggleable blocks
    pub toggleable_children: ToggleableBlockChildren,
}

/// Helper function to create a block fetcher with default configuration and rate limiting
pub fn create_block_fetcher(client: NotionClient) -> NotionBlockFetcher {
    NotionBlockFetcher::new(client)
        .with_api_call_delay(200) // Add a small delay to avoid rate limits
}