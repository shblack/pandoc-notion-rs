//! Converter between Notion content and various text formats
//!
//! This module provides a high-level interface for converting between Notion content
//! and other text formats using Pandoc as an intermediary.

use crate::n2p::{ConversionConfig, NotionToPandocVisitor};
use crate::notion_block_fetcher::{NotionBlockFetcher, create_block_fetcher};
use crate::notion_block_putter::{BlockPutterError, create_block_putter};
use crate::p2n::{PandocBlockVisitor, PandocToNotionVisitor};
use crate::text::processor::PandocProcessor;
use crate::text::{TextFormat, TextProcessingError, TextProcessor};
use log::{debug, info, error};
use notion_client::NotionClientError;
use notion_client::endpoints::Client as NotionClient;
use notion_client::objects::block::{Block as NotionBlock, BlockType};
use pandoc_types::definition::{Block as PandocBlock, Pandoc};
use std::fs;
use std::path::Path;

/// Represents possible errors during conversion
#[derive(Debug, thiserror::Error)]
pub enum ConversionError {
    #[error("Text processing error: {0}")]
    TextProcessing(#[from] TextProcessingError),

    #[error("Notion API error: {0}")]
    NotionApi(#[from] NotionClientError),

    #[error("Block putter error: {0}")]
    BlockPutterError(#[from] BlockPutterError),

    #[error("Invalid format: {0}")]
    InvalidFormat(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Other error: {0}")]
    Other(String),
}

/// Main converter that handles transformations between Notion and various formats
pub struct NotionConverter {
    /// Text processor for handling Pandoc operations
    processor: PandocProcessor,
    /// Notion API client
    notion_client: Option<notion_client::endpoints::Client>,
    /// Configuration for Notion to Pandoc conversion
    config: ConversionConfig,
}

impl NotionConverter {
    /// Create a NotionConverter with default settings
    pub fn new() -> Self {
        debug!("Creating new NotionConverter with default settings");
        Self {
            processor: PandocProcessor::new(),
            notion_client: None,
            config: ConversionConfig::default(),
        }
    }

    /// Create a converter with a custom Pandoc processor
    pub fn with_processor(processor: PandocProcessor) -> Self {
        Self {
            processor,
            notion_client: None,
            config: ConversionConfig::default(),
        }
    }

    /// Set whether to preserve Notion-specific attributes in Pandoc output
    pub fn with_preserve_attributes(mut self, preserve_attributes: bool) -> Self {
        self.config.preserve_attributes = preserve_attributes;
        self
    }

    /// Configure whether to escape special markdown characters in output text
    /// When false (default), excessive escaping of characters is removed
    pub fn with_escape_markdown(mut self, escape_markdown: bool) -> Self {
        self.config.escape_markdown = escape_markdown;
        self
    }

    /// Set the conversion configuration
    pub fn with_config(mut self, config: ConversionConfig) -> Self {
        self.config = config;
        self
    }

    /// Set Notion API client for this converter
    pub fn with_notion_client(mut self, client: notion_client::endpoints::Client) -> Self {
        self.notion_client = Some(client);
        self
    }

    /// Create a Notion API client with the given token
    pub fn configure_notion_client(&mut self, token: String) -> Result<(), ConversionError> {
        let client = notion_client::endpoints::Client::new(token, None).map_err(|e| e)?;

        self.notion_client = Some(client);
        info!("Successfully downloaded Notion content to file");
        Ok(())
    }

    /// Check if Notion client is configured
    fn ensure_notion_client(&self) -> Result<&notion_client::endpoints::Client, ConversionError> {
        self.notion_client
            .as_ref()
            .ok_or_else(|| ConversionError::Other("Notion client not configured".to_string()))
    }

    /// Debug function to print the structure of Notion blocks
    fn debug_print_notion_blocks(&self, blocks: &[NotionBlock], indent: usize) {
        let indent_str = " ".repeat(indent);

        debug!("{}Notion Blocks: {} items", indent_str, blocks.len());

        for (i, block) in blocks.iter().enumerate() {
            let block_type = format!("{:?}", block.block_type);
            debug!("{}Block #{}: Type: {}", indent_str, i, block_type);

            // Check if the block has children and print them
            let children = match &block.block_type {
                BlockType::Paragraph { paragraph } => &paragraph.children,
                BlockType::BulletedListItem { bulleted_list_item } => &bulleted_list_item.children,
                BlockType::NumberedListItem { numbered_list_item } => &numbered_list_item.children,
                BlockType::ToDo { to_do } => &to_do.children,
                BlockType::Quote { quote } => &quote.children,
                BlockType::Toggle { toggle } => &toggle.children,
                BlockType::Callout { callout: _ } => &None,
                _ => &None,
            };

            if let Some(children) = children {
                if !children.is_empty() {
                    debug!("{}  Has {} children:", indent_str, children.len());
                    self.debug_print_notion_blocks(children, indent + 4);
                }
            }
        }
    }

    /// Debug function to print the structure of Pandoc blocks
    fn debug_print_pandoc_blocks(&self, blocks: &[PandocBlock], indent: usize) {
        let indent_str = " ".repeat(indent);

        debug!("{}Pandoc Blocks: {} items", indent_str, blocks.len());

        for (i, block) in blocks.iter().enumerate() {
            match block {
                PandocBlock::Plain(inlines) => {
                    debug!(
                        "{}Block #{}: Plain with {} inlines",
                        indent_str,
                        i,
                        inlines.len()
                    );
                }
                PandocBlock::Para(inlines) => {
                    debug!(
                        "{}Block #{}: Para with {} inlines",
                        indent_str,
                        i,
                        inlines.len()
                    );
                }
                PandocBlock::LineBlock(lines) => {
                    debug!(
                        "{}Block #{}: LineBlock with {} lines",
                        indent_str,
                        i,
                        lines.len()
                    );
                }
                PandocBlock::CodeBlock(attr, text) => {
                    println!(
                        "{}Block #{}: CodeBlock '{}' ({} chars)",
                        indent_str,
                        i,
                        attr.classes.join(", "),
                        text.len()
                    );
                }
                PandocBlock::RawBlock(format, text) => {
                    println!(
                        "{}Block #{}: RawBlock format '{:?}' ({} chars)",
                        indent_str,
                        i,
                        format,
                        text.len()
                    );
                }
                PandocBlock::BlockQuote(blocks) => {
                    debug!(
                        "{}Block #{}: BlockQuote with {} blocks",
                        indent_str,
                        i,
                        blocks.len()
                    );
                    self.debug_print_pandoc_blocks(blocks, indent + 4);
                }
                PandocBlock::OrderedList(_attrs, items) => {
                    debug!(
                        "{}Block #{}: OrderedList with {} items",
                        indent_str,
                        i,
                        items.len()
                    );
                    for (j, item) in items.iter().enumerate() {
                        debug!("{}  Item #{} with {} blocks", indent_str, j, item.len());
                        self.debug_print_pandoc_blocks(item, indent + 8);
                    }
                }
                PandocBlock::BulletList(items) => {
                    debug!(
                        "{}Block #{}: BulletList with {} items",
                        indent_str,
                        i,
                        items.len()
                    );
                    for (j, item) in items.iter().enumerate() {
                        debug!("{}  Item #{} with {} blocks", indent_str, j, item.len());
                        self.debug_print_pandoc_blocks(item, indent + 8);
                    }
                }
                PandocBlock::DefinitionList(items) => {
                    debug!(
                        "{}Block #{}: Definition List with {} items",
                        indent_str,
                        i,
                        items.len()
                    );
                }
                PandocBlock::Header(level, _attr, inlines) => {
                    debug!(
                        "{}Block #{}: Header level {} with {} inlines",
                        indent_str,
                        i,
                        level,
                        inlines.len()
                    );
                }
                PandocBlock::HorizontalRule => {
                    debug!("{}Block #{}: HorizontalRule", indent_str, i);
                }
                PandocBlock::Table(_table) => {
                    debug!("{}Block #{}: Table", indent_str, i);
                }
                PandocBlock::Div(attr, blocks) => {
                    debug!(
                        "{}Block #{}: Div '{}' with {} blocks",
                        indent_str,
                        i,
                        attr.classes.join(", "),
                        blocks.len()
                    );
                    self.debug_print_pandoc_blocks(blocks, indent + 4);
                }
                PandocBlock::Null => {
                    debug!("{}Block #{}: Null", indent_str, i);
                }
                PandocBlock::Figure(_attr, _caption, blocks) => {
                    debug!(
                        "{}Block #{}: Figure with {} blocks",
                        indent_str,
                        i,
                        blocks.len()
                    );
                    self.debug_print_pandoc_blocks(blocks, indent + 4);
                }
            }
        }
    }

    /// Debug function to save data to a file
    fn debug_save_to_file<T: serde::Serialize>(
        &self,
        data: &T,
        filename: &str,
    ) -> Result<(), String> {
        let json = serde_json::to_string_pretty(data)
            .map_err(|e| format!("Failed to serialize: {}", e))?;

        fs::write(filename, json).map_err(|e| format!("Failed to write to file: {}", e))?;

        debug!("Saved debug data to {}", filename);
        Ok(())
    }

    /// Create a block fetcher with the current Notion client
    fn create_block_fetcher(&self) -> Result<NotionBlockFetcher, ConversionError> {
        let client = self.ensure_notion_client()?;

        // Clone the client for the block fetcher - using the same configuration
        let fetcher_client = client.clone();

        Ok(create_block_fetcher(fetcher_client))
    }

    /// Fetch blocks from Notion and convert to Pandoc AST
    pub async fn notion_blocks_to_pandoc(&self, block_id: &str) -> Result<Pandoc, ConversionError> {
        println!("\n===== FETCHING BLOCKS FROM NOTION =====");
        println!("Block ID: {}", block_id);

        // Create a block fetcher to recursively fetch all blocks including nested children
        let block_fetcher = self.create_block_fetcher()?;
        let blocks = block_fetcher
            .fetch_block_with_children(block_id)
            .await
            .map_err(|e| ConversionError::Other(format!("Block fetcher error: {}", e)))?;

        debug!("Fetched blocks from Notion (including children)");

        // Debug: Print and save the retrieved blocks
        self.debug_print_notion_blocks(&blocks, 0);
        let _ = self.debug_save_to_file(&blocks, "debug_notion_blocks.json");

        // Create a visitor to convert blocks to Pandoc
        let visitor = NotionToPandocVisitor::with_config(self.config.clone());

        // Process all blocks with the visitor using convert_blocks, which properly merges lists
        let pandoc_blocks = visitor.convert_blocks(&blocks);

        debug!("\n===== CONVERTED TO PANDOC BLOCKS =====");
        debug!("Converted to {} Pandoc blocks", pandoc_blocks.len());
        self.debug_print_pandoc_blocks(&pandoc_blocks, 0);

        // Create a Pandoc document with empty metadata
        let pandoc = Pandoc {
            meta: Default::default(),
            blocks: pandoc_blocks,
        };

        let _ = self.debug_save_to_file(&pandoc, "debug_pandoc_ast.json");

        Ok(pandoc)
    }

    /// Convert Notion blocks to text in the specified format
    pub async fn notion_blocks_to_text(
        &self,
        block_id: &str,
        format: TextFormat,
    ) -> Result<String, ConversionError> {
        println!("\n===== CONVERTING NOTION BLOCKS TO TEXT =====");
        println!("Block ID: {}, Format: {:?}", block_id, format);

        // First convert to Pandoc AST
        let pandoc = self.notion_blocks_to_pandoc(block_id).await?;

        // Then convert to the desired text format
        let mut text = self.processor.ast_to_text(&pandoc, format)?;

        // Remove markdown escaping if configured and format is markdown
        if format == TextFormat::Markdown && !self.config.escape_markdown {
            text = remove_markdown_escaping(text);
        }

        debug!("Converted to text, length: {} bytes", text.len());
        let _ = fs::write("debug_output_text.txt", &text);

        Ok(text)
    }

    /// Convert Notion page to a file in the specified format
    pub async fn notion_page_to_file<P: AsRef<Path>>(
        &self,
        page_id: &str,
        output_path: P,
        format: TextFormat,
    ) -> Result<(), ConversionError> {
        // Get the page content as Pandoc AST
        let pandoc = self.notion_blocks_to_pandoc(page_id).await?;

        // Write to file in the specified format
        self.processor
            .ast_to_file_with_format(&pandoc, output_path, format)?;

        Ok(())
    }

    /// Convert text in the specified format to Notion blocks
    pub fn text_to_notion_blocks(
        &self,
        text: &str,
        format: TextFormat,
    ) -> Result<Vec<NotionBlock>, ConversionError> {
        debug!("Converting text ({} bytes) to Notion blocks", text.len());
        // First convert text to Pandoc AST
        let pandoc = self.processor.text_to_ast(text, format)?;

        // Then convert to Notion blocks
        let visitor = PandocToNotionVisitor::new();

        let mut blocks = Vec::new();
        for block in &pandoc.blocks {
            // PandocToNotionVisitor.visit_block returns Result<Vec<NotionBlock>, Box<dyn Error>>
            match visitor.visit_block(block) {
                Ok(notion_blocks) => blocks.extend(notion_blocks),
                Err(e) => {
                    // Log or handle the error as needed
                    eprintln!("Failed to convert block: {}", e);
                }
            }
        }

        Ok(blocks)
    }

    /// Read a file in the specified format and convert to Notion blocks
    pub fn file_to_notion_blocks<P: AsRef<Path>>(
        &self,
        file_path: P,
        format: Option<TextFormat>,
    ) -> Result<Vec<NotionBlock>, ConversionError> {
        // Determine format from file extension if not provided
        let format = match format {
            Some(f) => f,
            None => {
                let path = file_path.as_ref();
                path.extension()
                    .and_then(|ext| ext.to_str())
                    .and_then(TextFormat::from_extension)
                    .unwrap_or(TextFormat::Markdown)
            }
        };

        // Convert file to AST
        let ast = self.processor.file_to_ast_with_format(file_path, format)?;

        // Convert AST to Notion blocks
        let visitor = PandocToNotionVisitor::new();

        let mut blocks = Vec::new();
        for block in &ast.blocks {
            // PandocToNotionVisitor.visit_block returns Result<Vec<NotionBlock>, Box<dyn Error>>
            match visitor.visit_block(block) {
                Ok(notion_blocks) => blocks.extend(notion_blocks),
                Err(e) => {
                    // Log or handle the error as needed
                    eprintln!("Failed to convert block: {}", e);
                }
            }
        }

        Ok(blocks)
    }

    /// Upload blocks to a Notion page
    ///
    /// Handles nested blocks by uploading them in batches in level-order traversal.
    /// This avoids Notion API limitations on deeply nested content.
    pub async fn upload_blocks_to_notion(
        &self,
        parent_id: &str,
        blocks: Vec<NotionBlock>,
    ) -> Result<(), ConversionError> {
        info!("Uploading {} blocks to Notion page {}", blocks.len(), parent_id);
        let client = self.ensure_notion_client()?;

        // If there are no blocks, return early
        if blocks.is_empty() {
            return Ok(());
        }

        // Process blocks in batches using level-order traversal
        self.upload_blocks_level_order(parent_id, blocks, client.clone())
            .await
    }

    /// Helper function to upload blocks in level-order traversal (breadth-first)
    /// This method is now a simple wrapper around NotionBlockPutter
    /// Implementation has been moved to the NotionBlockPutter module
    async fn upload_blocks_level_order(
        &self,
        parent_id: &str,
        blocks: Vec<NotionBlock>,
        client: NotionClient,
    ) -> Result<(), ConversionError> {
        // Use standard block putter with logging
        let block_putter = create_block_putter(client);

        block_putter
            .upload_blocks(parent_id, blocks)
            .await
            .map_err(ConversionError::BlockPutterError)
    }

    /// Convert a file to Notion and upload the blocks
    pub async fn file_to_notion<P: AsRef<Path>>(
        &self,
        file_path: P,
        parent_id: &str,
        format: Option<TextFormat>,
    ) -> Result<(), ConversionError> {
        debug!("Converting file {:?} to Notion blocks", file_path.as_ref());
        
        // Convert file to Notion blocks
        let blocks = self.file_to_notion_blocks(file_path, format)?;
        
        debug!("Generated {} Notion blocks, uploading to parent_id: {}", blocks.len(), parent_id);

        // Upload blocks to Notion
        self.upload_blocks_to_notion(parent_id, blocks).await?;

        info!("Successfully uploaded file to Notion");
        Ok(())
    }

    /// Download Notion content and convert to a file
    pub async fn notion_to_file<P: AsRef<Path>>(
        &self,
        block_id: &str,
        output_path: P,
        format: Option<TextFormat>,
    ) -> Result<(), ConversionError> {
        debug!("Downloading Notion content from block ID: {} to file: {:?}", block_id, output_path.as_ref());
        
        // Determine format from file extension if not provided
        let format = match format {
            Some(f) => f,
            None => {
                let path = output_path.as_ref();
                let detected_format = path.extension()
                    .and_then(|ext| ext.to_str())
                    .and_then(TextFormat::from_extension)
                    .unwrap_or(TextFormat::Markdown);
                debug!("Format not specified, detected format from extension: {:?}", detected_format);
                detected_format
            }
        };

        // Convert Notion content to file
        self.notion_page_to_file(block_id, output_path, format)
            .await?;
        
        debug!("Successfully converted Notion content to file");

        Ok(())
    }
}

/// Remove escaping from common markdown characters that Pandoc may have escaped
fn remove_markdown_escaping(text: String) -> String {
    // Replace common escaped markdown characters with their unescaped versions
    text.replace("\\*", "*")
        .replace("\\_", "_")
        .replace("\\[", "[")
        .replace("\\]", "]")
        .replace("\\(", "(")
        .replace("\\)", ")")
        .replace("\\#", "#")
        .replace("\\`", "`")
        .replace("\\~", "~")
        .replace("\\>", ">")
        .replace("\\-", "-")
        .replace("\\+", "+")
        .replace("\\=", "=")
        .replace("\\|", "|")
        .replace("\\{", "{")
        .replace("\\}", "}")
        .replace("\\!", "!")
        .replace("\\.", ".")
        .replace("\\<", "<")
}

/// Convenience function to create a new NotionConverter
pub fn create_converter() -> NotionConverter {
    NotionConverter::new()
}
