//! Notion Block Putter
//!
//! This module provides functionality to upload blocks to Notion with proper handling of nested structures.
//! It works around the API limitation that prevents deeply nested content from being uploaded in a single request.

use log::debug;
use notion_client::endpoints::Client as NotionClient;
use notion_client::endpoints::blocks::append::request::AppendBlockChildrenRequest;
use notion_client::objects::block::{Block as NotionBlock, BlockType};
use std::collections::{HashMap, VecDeque};
use std::time::Duration;
use tokio::time::sleep;
use uuid::Uuid;

/// Error types for block uploading operations
#[derive(Debug, thiserror::Error)]
pub enum BlockPutterError {
    #[error("Notion API error: {0}")]
    NotionApi(#[from] notion_client::NotionClientError),
    
    #[error("Invalid parent ID")]
    InvalidParentId,
    
    #[error("Rate limit exceeded")]
    RateLimit,
    
    #[error("Other error: {0}")]
    Other(String),
}

/// Result type for block putter operations
pub type Result<T> = std::result::Result<T, BlockPutterError>;

/// Configuration for the block putter
#[derive(Debug, Clone)]
pub struct BlockPutterConfig {
    /// Delay between API calls to avoid rate limits (in milliseconds, default: 100)
    pub api_call_delay_ms: u64,
    
    /// Number of blocks to upload in a single request (default: 50, max: 100)
    pub batch_size: usize,
}

impl Default for BlockPutterConfig {
    fn default() -> Self {
        Self {
            api_call_delay_ms: 100,
            batch_size: 50,
        }
    }
}

/// Represents a block in the hierarchy with its children
#[allow(dead_code)]
struct BlockNode {
    /// The block data
    block: NotionBlock,
    
    /// Children of this block
    children: Vec<BlockNode>,
    
    /// Temporary ID for tracking
    temp_id: String,
    
    /// Notion ID once uploaded
    notion_id: Option<String>,
}

impl BlockNode {
    /// Create a new block node
    #[allow(dead_code)]
    fn new(mut block: NotionBlock) -> Self {
        // Extract children if any
        let extracted_children = NotionBlockPutter::extract_children(&mut block);
        
        // Generate a temporary ID if needed
        let temp_id = block.id.clone().unwrap_or_else(|| Uuid::new_v4().to_string());
        
        // Ensure the block has an ID
        block.id = Some(temp_id.clone());
        
        // Create the node with empty children initially
        let mut node = Self {
            block,
            children: Vec::new(),
            temp_id,
            notion_id: None,
        };
        
        // Process children recursively and add them
        if !extracted_children.is_empty() {
            node.children = extracted_children.into_iter()
                .map(BlockNode::new)
                .collect();
        }
        
        node
    }
    
    /// Build child nodes
    #[allow(dead_code)]
    fn with_child_nodes(mut self, blocks: Vec<NotionBlock>) -> Self {
        self.children = blocks.into_iter().map(BlockNode::new).collect();
        self
    }
}

/// NotionBlockPutter handles uploading blocks to Notion with proper handling of nested structures
pub struct NotionBlockPutter {
    /// Notion API client
    client: NotionClient,
    
    /// Configuration for the putter
    config: BlockPutterConfig,
}

impl NotionBlockPutter {
    /// Create a new block putter with the given client and default configuration
    pub fn new(client: NotionClient) -> Self {
        Self {
            client,
            config: BlockPutterConfig::default(),
        }
    }
    
    /// Create a new block putter with custom configuration
    pub fn with_config(client: NotionClient, config: BlockPutterConfig) -> Self {
        Self {
            client,
            config,
        }
    }
    
    /// Set the API call delay in milliseconds
    pub fn with_api_call_delay(mut self, delay_ms: u64) -> Self {
        self.config.api_call_delay_ms = delay_ms;
        self
    }
    
    /// Upload blocks to a Notion page with proper handling of nested structures
    pub async fn upload_blocks(&self, parent_id: &str, blocks: Vec<NotionBlock>) -> Result<()> {
        if blocks.is_empty() {
            return Ok(());
        }
        
        debug!("Uploading {} blocks to parent: {}", blocks.len(), parent_id);
        
        // Process blocks level by level
        self.upload_blocks_level_order(parent_id, blocks).await
    }
    
    /// Upload blocks level by level (breadth-first)
    async fn upload_blocks_level_order(&self, parent_id: &str, blocks: Vec<NotionBlock>) -> Result<()> {
        // Extract children and organize blocks by level
        let (processed_blocks, children_map) = self.preprocess_blocks(blocks);
        
        // Map to store temporary IDs to Notion IDs
        let mut id_map = HashMap::new();
        
        // Upload top-level blocks
        if !processed_blocks.is_empty() {
            let processed_blocks_clone = processed_blocks.clone();
            let top_level_ids = self.upload_block_batch(parent_id, processed_blocks).await?;
            
            // Store mappings
            for (i, id) in top_level_ids.iter().enumerate() {
                if let Some(block) = processed_blocks_clone.get(i) {
                    if let Some(temp_id) = &block.id {
                        id_map.insert(temp_id.clone(), id.clone());
                    }
                }
            }
        }
        
        // Now process each level in breadth-first order
        let mut queue = VecDeque::new();
        
        // Add all top-level blocks with children to the queue
        for (temp_id, notion_id) in &id_map {
            if let Some(children) = children_map.get(temp_id) {
                queue.push_back((notion_id.clone(), children.clone()));
            }
        }
        
        // Process the queue
        while let Some((parent_id, children)) = queue.pop_front() {
            // Upload this batch of children
            let child_ids = self.upload_block_batch(&parent_id, children.clone()).await?;
            
            // Store mappings and queue up any grandchildren
            for (i, id) in child_ids.iter().enumerate() {
                if let Some(child) = children.get(i) {
                    if let Some(temp_id) = &child.id {
                        id_map.insert(temp_id.clone(), id.clone());
                        
                        // Queue up any grandchildren
                        if let Some(grandchildren) = children_map.get(temp_id) {
                            queue.push_back((id.clone(), grandchildren.clone()));
                        }
                    }
                }
            }
            
            // Apply delay between batches if configured
            if self.config.api_call_delay_ms > 0 {
                sleep(Duration::from_millis(self.config.api_call_delay_ms)).await;
            }
        }
        
        Ok(())
    }
    
    /// Upload a batch of blocks to a parent
    async fn upload_block_batch(&self, parent_id: &str, blocks: Vec<NotionBlock>) -> Result<Vec<String>> {
        let mut result_ids = Vec::new();
        
        // Split into batches if needed (Notion API limit is 100 blocks per request)
        for batch in blocks.chunks(self.config.batch_size) {
            let request = AppendBlockChildrenRequest {
                children: batch.to_vec(),
                after: None,
            };
            
            debug!("Uploading batch of {} blocks to parent: {}", batch.len(), parent_id);
            
            // Send the request
            let response = self.client.blocks.append_block_children(parent_id, request)
                .await
                .map_err(BlockPutterError::NotionApi)?;
            
            // Log response for troubleshooting
            debug!("Notion API response: {:?}", response);
            
            // Extract block IDs from the response
            // Handle response.results as Vec<Block> directly (not Option<Vec<Block>>)
            for block in response.results {
                if let Some(id) = block.id {
                    result_ids.push(id);
                } else {
                    result_ids.push(String::new()); // Placeholder for blocks without IDs
                }
            }
            
            // Apply delay between batches if configured
            if self.config.api_call_delay_ms > 0 {
                sleep(Duration::from_millis(self.config.api_call_delay_ms)).await;
            }
        }
        
        Ok(result_ids)
    }
    
    /// Preprocess blocks to extract children and build a mapping structure
    fn preprocess_blocks(&self, blocks: Vec<NotionBlock>) -> (Vec<NotionBlock>, HashMap<String, Vec<NotionBlock>>) {
        let mut processed_blocks = Vec::new();
        let mut children_map = HashMap::new();
        
        for mut block in blocks {
            // Generate a temporary ID for this block if it doesn't have one
            let temp_id = block.id.clone().unwrap_or_else(|| Uuid::new_v4().to_string());
            block.id = Some(temp_id.clone());
            
            // Extract children if any
            let children = Self::extract_children(&mut block);
            
            // Store children in the map if there are any
            if !children.is_empty() {
                children_map.insert(temp_id.clone(), children);
            }
            
            // Add the block without its children
            processed_blocks.push(block);
        }
        
        (processed_blocks, children_map)
    }
    
    /// Extract children from a block based on its type
    fn extract_children(block: &mut NotionBlock) -> Vec<NotionBlock> {
        let children_opt = match &mut block.block_type {
            BlockType::Paragraph { paragraph } => paragraph.children.take(),
            BlockType::BulletedListItem { bulleted_list_item } => bulleted_list_item.children.take(),
            BlockType::NumberedListItem { numbered_list_item } => numbered_list_item.children.take(),
            BlockType::ToDo { to_do } => to_do.children.take(),
            BlockType::Toggle { toggle } => toggle.children.take(),
            BlockType::Quote { quote } => quote.children.take(),
            _ => None,
        };
        
        // Unwrap the Option or return an empty Vec
        children_opt.unwrap_or_else(Vec::new)
    }
}

/// Helper function to create a block putter with default configuration
pub fn create_block_putter(client: NotionClient) -> NotionBlockPutter {
    NotionBlockPutter::new(client)
        .with_api_call_delay(200) // Add a small delay to avoid rate limits
}