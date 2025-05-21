# Notion API Client Documentation

A comprehensive guide for AI systems and developers to work with the `notion-client` Rust library.

## Table of Contents

- [1. Introduction](#1-introduction)
- [2. Client Initialization](#2-client-initialization)
- [3. Core Object Types](#3-core-object-types)
  - [3.1 Page](#31-page)
  - [3.2 Database](#32-database)
  - [3.3 Block](#33-block)
  - [3.4 Page Properties](#34-page-properties)
  - [3.5 Rich Text](#35-rich-text)
  - [3.6 Parent](#36-parent)
- [4. API Endpoints](#4-api-endpoints)
  - [4.1 Pages API](#41-pages-api)
  - [4.2 Databases API](#42-databases-api)
  - [4.3 Blocks API](#43-blocks-api)
  - [4.4 Users API](#44-users-api)
  - [4.5 Search API](#45-search-api)
  - [4.6 Comments API](#46-comments-api)
- [5. Filters and Queries](#5-filters-and-queries)
  - [5.1 Filters](#51-filters)
  - [5.2 Sort Options](#52-sort-options)
  - [5.3 Pagination](#53-pagination)
- [6. Usage Examples](#6-usage-examples)

## 1. Introduction

`notion-client` is a Rust client library for the Notion API that supports all major endpoints for working with databases, pages, blocks, users, comments, and search. This document provides a comprehensive reference for AI systems and developers working with this library.

## 2. Client Initialization

```rust
use notion_client::endpoints::Client;

// Initialize with your Notion API token
let client = Client::new("your_notion_token".to_string(), None);
let client = client.unwrap();  // Handle potential errors in production code

// You can also customize the reqwest client if needed
let custom_builder = reqwest::ClientBuilder::new().timeout(std::time::Duration::from_secs(30));
let client = Client::new("your_notion_token".to_string(), Some(custom_builder));
```

## 3. Core Object Types

### 3.1 Page

A Page represents a Notion page with properties, content, and metadata.

```rust
pub struct Page {
    pub id: String,                            // Unique identifier
    pub created_time: DateTime<Utc>,           // Creation timestamp
    pub created_by: User,                      // Creator information
    pub last_edited_time: DateTime<Utc>,       // Last edit timestamp
    pub last_edited_by: User,                  // Last editor information
    pub archived: bool,                        // Whether page is archived
    pub icon: Option<Icon>,                    // Page icon (emoji or file)
    pub cover: Option<File>,                   // Page cover image
    pub properties: HashMap<String, PageProperty>, // Page properties
    pub parent: Parent,                        // Parent container
    pub url: String,                           // URL to the page
    pub public_url: Option<String>,            // Public URL if shared
}
```

### 3.2 Database

A Database is a collection of Pages with a defined schema.

```rust
pub struct Database {
    pub id: String,                            // Unique identifier
    pub created_time: DateTime<Utc>,           // Creation timestamp
    pub created_by: User,                      // Creator information
    pub last_edited_time: DateTime<Utc>,       // Last edit timestamp
    pub last_edited_by: User,                  // Last editor information
    pub title: Vec<RichText>,                  // Database title
    pub description: Vec<RichText>,            // Database description
    pub icon: Option<Icon>,                    // Database icon
    pub cover: Option<File>,                   // Database cover image
    pub properties: HashMap<String, DatabaseProperty>, // Schema properties
    pub parent: Parent,                        // Parent container
    pub url: String,                           // URL to the database
    pub archived: bool,                        // Whether database is archived
    pub is_inline: bool,                       // Whether displayed inline
}
```

### 3.3 Block

Blocks are the basic content units in Notion pages.

```rust
pub enum Block {
    Bookmark { ... },                         // External link bookmark
    Breadcrumb { ... },                       // Breadcrumb navigation
    BulletedListItem { ... },                 // Bulleted list item
    Callout { ... },                          // Callout/admonition block
    ChildDatabase { ... },                    // Embedded database
    ChildPage { ... },                        // Link to child page
    Code { ... },                             // Code block with syntax highlighting
    Column { ... },                           // Column container
    ColumnList { ... },                       // Multi-column container
    Divider { ... },                          // Horizontal divider
    Embed { ... },                            // Embedded content
    Equation { ... },                         // Math equation
    File { ... },                             // File attachment
    Heading1 { ... },                         // H1 heading
    Heading2 { ... },                         // H2 heading
    Heading3 { ... },                         // H3 heading
    Image { ... },                            // Image
    LinkPreview { ... },                      // Preview of a linked resource
    LinkToPage { ... },                       // Link to another page
    NumberedListItem { ... },                 // Numbered list item
    Paragraph { ... },                        // Text paragraph
    Pdf { ... },                              // PDF file
    Quote { ... },                            // Block quote
    SyncedBlock { ... },                      // Synced content block
    Table { ... },                            // Table
    TableOfContents { ... },                  // Auto-generated TOC
    TableRow { ... },                         // Table row
    Template { ... },                         // Reusable template
    ToDo { ... },                             // To-do item with checkbox
    Toggle { ... },                           // Collapsible toggle
    Unsupported { ... },                      // Unsupported block type
    Video { ... },                            // Video
}
```

### 3.4 Page Properties

Properties define the schema for Pages and Database entries.

```rust
pub enum PageProperty {
    Checkbox { id: Option<String>, checkbox: bool },
    CreatedBy { id: Option<String>, created_by: User },
    CreatedTime { id: Option<String>, created_time: DateTime<Utc> },
    Date { id: Option<String>, date: Option<DatePropertyValue> },
    Email { id: Option<String>, email: Option<String> },
    Files { id: Option<String>, files: Vec<FilePropertyValue> },
    Formula { id: Option<String>, formula: Option<FormulaPropertyValue> },
    LastEditedBy { id: Option<String>, last_edited_by: User },
    LastEditedTime { id: Option<String>, last_edited_time: Option<DateTime<Utc>> },
    MultiSelect { id: Option<String>, multi_select: Vec<SelectPropertyValue> },
    Number { id: Option<String>, number: Option<Number> },
    People { id: Option<String>, people: Vec<User> },
    PhoneNumber { id: Option<String>, phone_number: Option<String> },
    Relation { id: Option<String>, relation: Vec<RelationPropertyValue>, has_more: Option<bool> },
    Rollup { id: Option<String>, rollup: Option<RollupPropertyValue> },
    RichText { id: Option<String>, rich_text: Vec<RichText> },
    Select { id: Option<String>, select: Option<SelectPropertyValue> },
    Status { id: Option<String>, status: Option<SelectPropertyValue> },
    Title { id: Option<String>, title: Vec<RichText> },
    Url { id: Option<String>, url: Option<String> },
    UniqueID { id: Option<String>, unique_id: Option<UniqueIDPropertyValue> },
    Verification { id: Option<String>, verification: Option<VerificationPropertyValue> },
    Button { id: Option<String> },
}
```

### 3.5 Rich Text

Rich Text represents formatted text content.

```rust
pub struct RichText {
    pub plain_text: String,                    // Plain text content
    pub href: Option<String>,                  // Optional hyperlink
    pub annotations: Annotations,              // Text formatting
    pub type_: RichTextType,                   // Text type
}

pub enum RichTextType {
    Text { content: String, link: Option<Link> },
    Mention { ... },                          // @mentions
    Equation { expression: String },          // Inline math
}

pub struct Annotations {
    pub bold: bool,                           // Bold formatting
    pub italic: bool,                         // Italic formatting
    pub strikethrough: bool,                  // Strikethrough formatting
    pub underline: bool,                      // Underline formatting
    pub code: bool,                           // Inline code formatting
    pub color: Color,                         // Text color
}
```

### 3.6 Parent

Parent defines what contains a Notion object.

```rust
pub enum Parent {
    DatabaseId { database_id: String },       // Parent is a database
    PageId { page_id: String },               // Parent is a page
    Workspace,                                // Parent is the workspace
    BlockId { block_id: String },             // Parent is a block
}
```

## 4. API Endpoints

### 4.1 Pages API

#### Retrieve a Page

```rust
// Get page properties
async fn retrieve_page(
    client: &Client,
    page_id: &str,
) -> Result<Page, NotionClientError> {
    // Field-based access, not method-based
    client.pages.retrieve_a_page(page_id, None).await
}
```

#### Create a Page

```rust
async fn create_page(
    client: &Client,
    parent: Parent,                           // Parent container (database or page)
    properties: BTreeMap<String, PageProperty>, // Page properties matching parent schema
    children: Option<Vec<Block>>,             // Page content blocks (optional)
    icon: Option<Icon>,                       // Page icon (optional)
    cover: Option<File>,                      // Page cover (optional)
) -> Result<Page, NotionClientError> {
        let request = CreateAPageRequest {
            parent,
            properties,
            children,
            icon,
            cover,
        };
    
        // Field-based access, not method-based
        client.pages.create_a_page(request).await
    }
```

#### Update Page Properties

```rust
async fn update_page(
    client: &Client,
    page_id: &str,
    properties: BTreeMap<String, PageProperty>, // Properties to update
    icon: Option<Icon>,                       // New icon (optional)
    cover: Option<File>,                      // New cover (optional)
    archived: Option<bool>,                   // Archive status (optional)
) -> Result<Page, NotionClientError> {
        let request = UpdatePagePropertiesRequest {
            properties,
            icon,
            cover,
            archived,
        };
    
        client.pages.update_page_properties(page_id, request).await
    }
```

### 4.2 Databases API

#### Retrieve a Database

```rust
async fn retrieve_database(
    client: &Client,
    database_id: &str,
) -> Result<Database, NotionClientError> {
    // Field-based access, not method-based  
    client.databases.retrieve_a_database(database_id).await
}
```

#### Query a Database

```rust
async fn query_database(
    client: &Client,
    database_id: &str,
    filter: Option<Filter>,                   // Filter criteria
    sorts: Option<Vec<Sort>>,                 // Sort order
    start_cursor: Option<String>,             // Pagination cursor
    page_size: Option<u32>,                   // Results per page
) -> Result<QueryDatabaseResponse, NotionClientError> {
    let request = QueryDatabaseRequest {
        filter,
        sorts,
        start_cursor,
        page_size,
    };
    
    client.databases.query_a_database(database_id, request).await
}
```

#### Create a Database

```rust
async fn create_database(
    client: &Client,
    parent: Parent,                           // Parent page
    title: Vec<RichText>,                     // Database title
    properties: BTreeMap<String, PropertySchema>, // Database property schema
    icon: Option<Icon>,                       // Database icon (optional)
    cover: Option<File>,                      // Database cover (optional)
    is_inline: Option<bool>,                  // Whether the database is inline (optional)
) -> Result<Database, NotionClientError> {
        let request = CreateDatabaseRequest {
            parent,
            title,
            properties,
            icon,
            cover,
            is_inline,
        };
    
        // Field-based access, not method-based
        client.databases.create_a_database(request).await
    }
```

#### Update a Database

```rust
async fn update_database(
    client: &Client,
    database_id: &str,
    title: Option<Vec<RichText>>,             // New database title (optional)
    properties: Option<BTreeMap<String, PropertySchema>>, // New property schema (optional)
    icon: Option<Icon>,                       // New database icon (optional)
    cover: Option<File>,                      // New database cover (optional)
    is_inline: Option<bool>,                  // New inline status (optional)
) -> Result<Database, NotionClientError> {
        let request = UpdateDatabaseRequest {
            title,
            properties,
            icon,
            cover,
            is_inline,
        };
    
        // Field-based access, not method-based
        client.databases.update_a_database(database_id, request).await
    }
```

### 4.3 Blocks API

#### Retrieve a Block

```rust
async fn retrieve_block(
    client: &Client,
    block_id: &str,
) -> Result<Block, NotionClientError> {
    // Field-based access, not method-based
    client.blocks.retrieve_a_block(block_id).await
}
```

#### Retrieve Block Children

```rust
async fn retrieve_block_children(
    client: &Client,
    block_id: &str,
    start_cursor: Option<String>,            // Pagination cursor
    page_size: Option<u32>,                  // Results per page
) -> Result<BlockChildrenResponse, NotionClientError> {
    client.blocks.retrieve_block_children(block_id, start_cursor, page_size).await
}
```

#### Append Block Children

```rust
async fn append_block_children(
    client: &Client,
    block_id: &str,
    children: Vec<Block>,                    // Blocks to append
) -> Result<AppendBlockChildrenResponse, NotionClientError> {
    let request = AppendBlockChildrenRequest { children };
    client.blocks.append_block_children(block_id, request).await
}
```

#### Update a Block

```rust
async fn update_block(
    client: &Client,
    block_id: &str,
    block: Block,                           // Updated block content
) -> Result<Block, NotionClientError> {
    client.blocks.update_a_block(block_id, block).await
}
```

#### Delete a Block

```rust
async fn delete_block(
    client: &Client,
    block_id: &str,
) -> Result<Block, NotionClientError> {
    // Field-based access, not method-based
    client.blocks.delete_a_block(block_id).await
}
```

### 4.4 Users API

#### List All Users

```rust
async fn list_all_users(
    client: &Client,
    start_cursor: Option<String>,           // Pagination cursor
    page_size: Option<u32>,                 // Results per page (default 100)
) -> Result<ListUsersResponse, NotionClientError> {
    // Field-based access, not method-based
    client.users.list_all_users(start_cursor, page_size).await
}
```

#### Retrieve a User

```rust
async fn retrieve_user(
    client: &Client,
    user_id: &str,
) -> Result<User, NotionClientError> {
    // Field-based access, not method-based
    client.users.retrieve_a_user(user_id).await
}
```

#### Get Current User (Me)

```rust
async fn get_my_user(
    client: &Client,
) -> Result<User, NotionClientError> {
    client.users.retrieve_my_user().await
}
```

### 4.5 Search API

```rust
async fn search(
    client: &Client,
    query: Option<String>,                 // Search term
    filter: Option<SearchFilter>,          // Filter by object type
    sort: Option<SearchSort>,              // Sort criteria
    start_cursor: Option<String>,          // Pagination cursor
    page_size: Option<u32>,                // Results per page
) -> Result<SearchResponse, NotionClientError> {
    let request = SearchRequest {
        query,
        filter,
        sort,
        start_cursor,
        page_size,
    };
    
    client.search.search(request).await
}
```

### 4.6 Comments API

#### Create a Comment

```rust
async fn create_comment(
    client: &Client,
    parent: CommentParent,                  // Parent (page ID or discussion ID)
    rich_text: Vec<RichText>,               // Comment content
) -> Result<Comment, NotionClientError> {
        let request = CreateCommentRequest {
            parent,
            rich_text,
        };
    
        // Field-based access, not method-based
        client.comments.create_a_comment(request).await
    }
```

#### List Comments

```rust
async fn list_comments(
    client: &Client,
    block_id: &str,
    start_cursor: Option<String>,         // Pagination cursor
    page_size: Option<u32>,               // Results per page
) -> Result<ListCommentsResponse, NotionClientError> {
    let request = ListCommentsRequest {
        block_id: block_id.to_string(),
        start_cursor,
        page_size,
    };
    
    client.comments.list_comments(request).await
}
```

## 5. Filters and Queries

### 5.1 Filters

Filters are used when querying databases.

```rust
pub enum Filter {
    Value { filter_type: FilterType },       // Single filter condition
    And { and: Vec<FilterType> },            // All conditions must match
    Or { or: Vec<FilterType> },              // Any condition may match
}

pub enum FilterType {
    Property {                               // Filter on page property
        property: String,                    // Property name
        condition: PropertyCondition,        // Condition to test
    },
    Timestamp {                              // Filter on timestamp
        timestamp: Timestamp,                // Type of timestamp
        condition: TimestampCondition,       // Condition to test
    },
}
```

#### Property Conditions

Comprehensive list of all filter conditions by property type:

```rust
pub enum PropertyCondition {
    Checkbox(CheckBoxCondition),             // For checkbox properties
    Date(DateCondition),                     // For date properties
    Files(FilesCondition),                   // For file properties
    Formula(FormulaCondition),               // For formula properties
    MultiSelect(MultiSelectCondition),       // For multi-select properties
    Number(NumberCondition),                 // For number properties
    People(PeopleCondition),                 // For people properties
    Relation(RelationCondition),             // For relation properties
    RichText(RichTextCondition),             // For text properties
    Rollup(Box<RollupCondition>),            // For rollup properties
    Select(SelectCondition),                 // For select properties
    Status(StatusCondition),                 // For status properties
    Timestamp(TimestampCondition),           // For created/edited time
    ID(IDCondition),                         // For ID properties
}
```

#### Text Conditions

```rust
pub enum RichTextCondition {
    Contains(String),                        // Text contains string
    DoesNotContain(String),                  // Text doesn't contain string
    DoesNotEqual(String),                    // Text is not exactly string
    EndsWith(String),                        // Text ends with string
    Equals(String),                          // Text equals string exactly
    IsEmpty,                                 // Text is empty
    IsNotEmpty,                              // Text is not empty
    StartsWith(String),                      // Text starts with string
}
```

#### Number Conditions

```rust
pub enum NumberCondition {
    DoesNotEqual(Number),                    // Number ≠ value
    Equals(Number),                          // Number = value
    GreaterThan(Number),                     // Number > value
    GreaterThanOrEqualTo(Number),            // Number ≥ value
    IsEmpty,                                 // Number is empty/null
    IsNotEmpty,                              // Number is not empty
    LessThanOrEqualTo(Number),               // Number ≤ value
    LessThan(Number),                        // Number < value
}
```

#### Date Conditions

```rust
pub enum DateCondition {
    After(DateTime<Utc>),                    // Date > value
    Before(DateTime<Utc>),                   // Date < value
    Equals(DateTime<Utc>),                   // Date = value
    IsEmpty,                                 // Date is empty/null
    IsNotEmpty,                              // Date is not empty
    NextMonth,                               // Date is in next month
    NextWeek,                                // Date is in next week
    NextYear,                                // Date is in next year
    OnOrAfter(DateTime<Utc>),                // Date ≥ value
    OnOrBefore(DateTime<Utc>),               // Date ≤ value
    PastMonth,                               // Date is in past month
    PastWeek,                                // Date is in past week
    PastYear,                                // Date is in past year
    ThisWeek,                                // Date is this week
}
```

#### Select Conditions

```rust
pub enum SelectCondition {
    Equals(String),                          // Option matches exactly
    DoesNotEqual(String),                    // Option doesn't match
    IsEmpty,                                 // No option selected
    IsNotEmpty,                              // Has option selected
}
```

#### Multi-Select Conditions

```rust
pub enum MultiSelectCondition {
    Contains(String),                        // Contains option
    DoesNotContain(String),                  // Doesn't contain option
    IsEmpty,                                 // No options selected
    IsNotEmpty,                              // Has options selected
}
```

### 5.2 Sort Options

```rust
pub enum Sort {
    Property {                               // Sort by page property
        property: String,                    // Property name
        direction: SortDirection,            // Ascending or descending
    },
    Timestamp {                              // Sort by timestamp
        timestamp: Timestamp,                // Created or last edited time
        direction: SortDirection,            // Ascending or descending
    },
}

pub enum SortDirection {
    Ascending,                              // A → Z, 1 → 9, oldest → newest
    Descending,                             // Z → A, 9 → 1, newest → oldest
}
```

### 5.3 Pagination

Most endpoints support pagination with `start_cursor` and `page_size` parameters:

```rust
// First page (default page size is typically 100)
let first_page = client.databases
    .query_a_database(database_id, request)
    .await?;

// If more results exist
if first_page.has_more {
    // Use the next_cursor from the first page to get the next page
    let next_page_request = QueryDatabaseRequest {
        start_cursor: first_page.next_cursor,
        ..Default::default()
    };
    
    let second_page = client.databases
        .query_a_database(database_id, next_page_request)
        .await?;
}
```

## 6. Usage Examples

### Querying a Database with Filters

```rust
use notion_client::endpoints::{
    databases::query::request::{
        Filter, FilterType, PropertyCondition, RichTextCondition, QueryDatabaseRequest,
        Sort, SortDirection, Timestamp
    },
    Client,
};

async fn query_tasks(client: &Client, database_id: &str) -> Result<(), NotionClientError> {
    // Create a filter for high priority tasks
    let filter = Filter::And {
        and: vec![
            FilterType::Property {
                property: "Status".to_string(),
                condition: PropertyCondition::Select(
                    SelectCondition::Equals("In Progress".to_string())
                ),
            },
            FilterType::Property {
                property: "Priority".to_string(),
                condition: PropertyCondition::Select(
                    SelectCondition::Equals("High".to_string())
                ),
            },
        ],
    };
    
    // Sort by due date
    let sorts = vec![Sort::Property {
        property: "Due Date".to_string(),
        direction: SortDirection::Ascending,
    }];
    
    let request = QueryDatabaseRequest {
        filter: Some(filter),
        sorts: Some(sorts),
        page_size: Some(25),
        ..Default::default()
    };
    
    let response = client.databases.query_a_database(database_id, request).await?;
    
    for page in response.results {
        println!("Task: {:?}", page);
    }
    
    Ok(())
}
```

### Creating a New Page

```rust
use std::collections::BTreeMap;
use notion_client::endpoints::{
    pages::create::request::CreateAPageRequest,
    Client,
};
use notion_client::objects::{
    page::PageProperty,
    parent::Parent,
    rich_text::{RichText, RichTextType, Annotations},
};

async fn create_task(client: &Client, database_id: &str) -> Result<(), NotionClientError> {
    // Set up the parent reference to the database
    let parent = Parent::Database {
        database_id: database_id.to_string(),
    };
    
    // Create property values matching the database schema
    let mut properties = BTreeMap::new();
    
    // Title property
    properties.insert(
        "Name".to_string(),
        PageProperty::Title {
            id: None,
            title: vec![RichText {
                plain_text: "New Task".to_string(),
                href: None,
                annotations: Annotations::default(),
                type_: RichTextType::Text {
                    content: "New Task".to_string(),
                    link: None,
                },
            }],
        },
    );
    
    // Status property
    properties.insert(
        "Status".to_string(),
        PageProperty::Select {
            id: None,
            select: Some(SelectPropertyValue {
                id: None,
                name: Some("To Do".to_string()),
                color: None,
            }),
        },
    );
    
    // Create the request
    let request = CreateAPageRequest {
        parent,
        properties,
        ..Default::default()
    };
    
    // Send the request
    let new_page = client.pages.create_a_page(request).await?;
    println!("Created new page with ID: {}", new_page.id);
    
    Ok(())
}
```

### Adding Content Blocks to a Page

```rust
use notion_client::endpoints::{
    blocks::append::request::AppendBlockChildrenRequest,
    Client,
};
use notion_client::objects::block::Block;

async fn add_content_to_page(client: &Client, page_id: &str) -> Result<(), NotionClientError> {
    // Create blocks to add
    let blocks = vec![
        Block::Heading1 {
            heading_1: HeadingBlockContent {
                rich_text: vec![RichText {
                    plain_text: "Project Overview".to_string(),
                    href: None,
                    annotations: Annotations::default(),
                    type_: RichTextType::Text {
                        content: "Project Overview".to_string(),
                        link: None,
                    },
                }],
                color: Color::Default,
                is_toggleable: false,
            },
            id: "".to_string(),
            created_time: None,
            created_by: None,
            last_edited_time: None,
            last_edited_by: None,
            has_children: false,
            archived: false,
        },
        Block::Paragraph {
            paragraph: ParagraphBlockContent {
                rich_text: vec![RichText {
                    plain_text: "This is a description of the project.".to_string(),
                    href: None,
                    annotations: Annotations::default(),
                    type_: RichTextType::Text {
                        content: "This is a description of the project.".to_string(),
                        link: None,
                    },
                }],
                color: Color::Default,
            },
            id: "".to_string(),
            created_time: None,
            created_by: None,
            last_edited_time: None,
            last_edited_by: None,
            has_children: false,
            archived: false,
        },
    ];
    
    let request = AppendBlockChildrenRequest { children: blocks };
    
    client.blocks.append_block_children(page_id, request).await?;
    println!("Added blocks to page {}", page_id);
    
    Ok(())
}
```

This documentation provides a comprehensive overview of the Notion API client library and serves as a reference for AI systems and developers working with the Notion API through this Rust client.