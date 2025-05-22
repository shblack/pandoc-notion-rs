//! Helper functions for Notion API testing
//!
//! This module provides utilities for creating and managing Notion pages
//! during integration tests.

use chrono::Utc;
use notion_client::endpoints::Client as NotionClient;
use notion_client::endpoints::pages::create::request::CreateAPageRequest;
use notion_client::endpoints::pages::update::request::UpdatePagePropertiesRequest;
use notion_client::objects::page::PageProperty;
use notion_client::objects::parent::Parent;
use notion_client::objects::rich_text::{RichText, Text};
use serde_json;
use std::collections::BTreeMap;
use std::env;

// Import the rate limiting function
use super::roundtrip_utils::respect_rate_limit;

/// Check if required Notion credentials are available in environment variables
#[allow(dead_code)]
pub fn notion_credentials_available() -> bool {
    env::var("NOTION_TOKEN").is_ok() && env::var("NOTION_TEST_PAGE_ID").is_ok()
}

/// Get Notion credentials from environment variables
#[allow(dead_code)]
pub fn get_notion_credentials() -> Result<(String, String), String> {
    let token = env::var("NOTION_TOKEN")
        .map_err(|_| "NOTION_TOKEN environment variable not set".to_string())?;

    let parent_page_id = env::var("NOTION_TEST_PAGE_ID")
        .map_err(|_| "NOTION_TEST_PAGE_ID environment variable not set".to_string())?;

    Ok((token, parent_page_id))
}

/// Generate a timestamped page title for testing
#[allow(dead_code)]
pub fn generate_test_page_title(prefix: &str) -> String {
    let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    format!("{} - {}", prefix, timestamp)
}

/// Create a child page under the specified parent page
#[allow(dead_code)]
pub async fn create_child_page(
    client: &NotionClient,
    parent_id: &str,
    title: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    // Create the page title as rich text
    let title_text = Text {
        content: title.to_string(),
        link: None,
    };

    let title_rich_text = RichText::Text {
        text: title_text,
        annotations: None,
        plain_text: None,
        href: None,
    };

    // Create the page properties (only title is required)
    let mut properties = BTreeMap::new();
    properties.insert(
        "title".to_string(),
        PageProperty::Title {
            id: None,
            title: vec![title_rich_text],
        },
    );

    // Create parent reference
    let parent = Parent::PageId {
        page_id: parent_id.to_string(),
    };

    // Create the page request
    let request = CreateAPageRequest {
        parent,
        properties,
        children: None,
        icon: None,
        cover: None,
    };

    // Send the request to create the page
    let page = client.pages.create_a_page(request).await?;
    
    // Respect API rate limits
    respect_rate_limit().await;

    // Convert the page to JSON value
    Ok(serde_json::to_value(page)?)
}

/// Archive a page
#[allow(dead_code)]
pub async fn archive_page(
    client: &NotionClient,
    page_id: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    // Create the update page request
    let request = UpdatePagePropertiesRequest {
        properties: BTreeMap::new(), // No property changes
        archived: Some(true),        // Archive the page
        icon: None,
        cover: None,
    };

    // Send the request to update the page
    let page = client
        .pages
        .update_page_properties(page_id, request)
        .await?;
    
    // Respect API rate limits
    respect_rate_limit().await;
    
    Ok(serde_json::to_value(page)?)
}

/// Create a Notion client from an API token
#[allow(dead_code)]
pub fn create_notion_client(token: &str) -> Result<NotionClient, Box<dyn std::error::Error>> {
    let client = NotionClient::new(token.to_string(), None)?;
    Ok(client)
}

/// Test setup helper: creates a test page and returns its ID
#[allow(dead_code)]
pub async fn setup_test_page(
    client: &NotionClient,
    parent_id: &str,
    test_name: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let page_title = generate_test_page_title(test_name);
    println!("Creating test page: {}", page_title);

    let page = create_child_page(client, parent_id, &page_title).await?;

    let page_id = page
        .get("id")
        .and_then(|id| id.as_str())
        .ok_or("Failed to extract page ID")?
        .to_string();

    println!("Created test page with ID: {}", page_id);
    
    // Note: We don't need to call respect_rate_limit() here as it's already called in create_child_page
    
    Ok(page_id)
}

/// Test teardown helper: archives a test page
#[allow(dead_code)]
pub async fn teardown_test_page(
    client: &NotionClient,
    page_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Archiving test page: {}", page_id);
    archive_page(client, page_id).await?;
    println!("Successfully archived test page");
    
    // Note: We don't need to call respect_rate_limit() here as it's already called in archive_page
    
    Ok(())
}
