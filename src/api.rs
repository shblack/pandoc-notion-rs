//! High-level API for Notion-Pandoc integration
//!
//! This module provides a clean, high-level API for common operations with Notion content,
//! hiding the implementation details and making it easy to integrate Notion with text-based workflows.

use crate::{
    converter::{ConversionError, NotionConverter},
    create_converter, create_text_processor,
    notion_block_fetcher::BlockFetcherConfig,
    notion_block_putter::BlockPutterConfig,
    text::{TextFormat, TextProcessingError},
    ConversionConfig,
};
use notion_client::endpoints::Client as NotionClient;
use notion_client::NotionClientError;
use std::error::Error;
use std::fmt;
use std::path::Path;
use thiserror::Error;

/// Error type for API operations
#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Notion API error: {0}")]
    NotionApi(#[from] NotionClientError),

    #[error("Conversion error: {0}")]
    Conversion(#[from] ConversionError),

    #[error("Text processing error: {0}")]
    TextProcessing(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Other error: {0}")]
    Other(String),
}

impl From<TextProcessingError> for ApiError {
    fn from(err: TextProcessingError) -> Self {
        Self::TextProcessing(err.to_string())
    }
}

/// Client for high-level Notion operations
pub struct NotionClient2 {
    converter: NotionConverter,
    token: String,
}

impl NotionClient2 {
    /// Create a new NotionClient2 with the provided API token
    pub fn new(token: impl Into<String>) -> Result<Self, ApiError> {
        let token = token.into();
        let client = NotionClient::new(token.clone(), None)
            .map_err(ApiError::NotionApi)?;

        let converter = create_converter()
            .with_notion_client(client);

        Ok(Self {
            converter,
            token,
        })
    }

    /// Configure the client with custom options
    pub fn with_config(mut self, config: ClientConfig) -> Self {
        // Apply conversion config
        if let Some(preserve_attributes) = config.preserve_attributes {
            let conversion_config = ConversionConfig {
                preserve_attributes,
                escape_markdown: config.escape_markdown.unwrap_or(true),
            };
            self.converter = self.converter.with_config(conversion_config);
        }

        self
    }

    /// Get the content of a Notion page as text
    pub async fn get_page_content(
        &self,
        page_id: &str,
        format: TextFormat,
    ) -> Result<String, ApiError> {
        self.converter
            .notion_blocks_to_text(page_id, format)
            .await
            .map_err(ApiError::Conversion)
    }

    /// Get the content of a Notion page as Markdown
    pub async fn get_page_as_markdown(&self, page_id: &str) -> Result<String, ApiError> {
        self.get_page_content(page_id, TextFormat::Markdown).await
    }

    /// Get the content of a Notion page as plain text
    pub async fn get_page_as_plain_text(&self, page_id: &str) -> Result<String, ApiError> {
        self.get_page_content(page_id, TextFormat::PlainText).await
    }

    /// Get the content of a Notion page as HTML
    pub async fn get_page_as_html(&self, page_id: &str) -> Result<String, ApiError> {
        self.get_page_content(page_id, TextFormat::Html).await
    }

    /// Save the content of a Notion page to a file
    pub async fn save_page_to_file<P: AsRef<Path>>(
        &self,
        page_id: &str,
        file_path: P,
    ) -> Result<(), ApiError> {
        self.converter
            .notion_page_to_file(page_id, file_path)
            .await
            .map_err(ApiError::Conversion)
    }

    /// Upload content from a file to a Notion page
    pub async fn upload_file_to_page<P: AsRef<Path>>(
        &self,
        file_path: P,
        parent_id: &str,
    ) -> Result<(), ApiError> {
        self.converter
            .file_to_notion(file_path, parent_id)
            .await
            .map_err(ApiError::Conversion)
    }

    /// Convert text content to Notion blocks and upload to a page
    pub async fn upload_text_to_page(
        &self,
        text: &str,
        format: TextFormat,
        parent_id: &str,
    ) -> Result<(), ApiError> {
        let blocks = self.converter
            .text_to_notion_blocks(text, format)
            .map_err(ApiError::Conversion)?;

        self.converter
            .upload_blocks_to_notion(&blocks, parent_id)
            .await
            .map_err(ApiError::Conversion)
    }
}

/// Configuration options for the NotionClient2
pub struct ClientConfig {
    /// Whether to preserve Notion-specific attributes (colors, etc.)
    pub preserve_attributes: Option<bool>,
    /// Whether to escape markdown characters in output
    pub escape_markdown: Option<bool>,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            preserve_attributes: None,
            escape_markdown: None,
        }
    }
}

/// Create a new NotionClient2 with the provided API token
pub fn create_client(token: impl Into<String>) -> Result<NotionClient2, ApiError> {
    NotionClient2::new(token)
}

/// Get the content of a Notion page as text (convenience function)
pub async fn get_page_content(
    token: impl Into<String>,
    page_id: &str,
    format: TextFormat,
) -> Result<String, ApiError> {
    let client = create_client(token)?;
    client.get_page_content(page_id, format).await
}

/// Get the content of a Notion page as Markdown (convenience function)
pub async fn get_page_as_markdown(
    token: impl Into<String>,
    page_id: &str,
) -> Result<String, ApiError> {
    let client = create_client(token)?;
    client.get_page_as_markdown(page_id).await
}

/// Get the content of a Notion page as plain text (convenience function)
pub async fn get_page_as_plain_text(
    token: impl Into<String>,
    page_id: &str,
) -> Result<String, ApiError> {
    let client = create_client(token)?;
    client.get_page_as_plain_text(page_id).await
}

/// Save the content of a Notion page to a file (convenience function)
pub async fn save_page_to_file<P: AsRef<Path>>(
    token: impl Into<String>,
    page_id: &str,
    file_path: P,
) -> Result<(), ApiError> {
    let client = create_client(token)?;
    client.save_page_to_file(page_id, file_path).await
}