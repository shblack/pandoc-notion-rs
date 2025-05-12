# Block Object

A block object represents a piece of content within Notion. The API translates the headings, toggles, paragraphs, lists, media, and more that you can interact with in the Notion UI into different block type objects.

## API Response Format

When retrieving a block from the API, the response includes the following structure:

```json
{
  "object": "block",
  "id": "block-id",
  "parent": {
    "type": "page_id",
    "page_id": "parent-page-id"
  },
  "created_time": "2023-01-01T12:00:00.000Z",
  "last_edited_time": "2023-01-01T12:00:00.000Z",
  "created_by": {
    "object": "user",
    "id": "user-id"
  },
  "last_edited_by": {
    "object": "user",
    "id": "user-id"
  },
  "has_children": false,
  "archived": false,
  "in_trash": false,
  "type": "block_type",
  "block_type": {
    // Block type-specific content
  }
}
```

## API Request Format (Create/Update)

When creating or appending blocks, use this format:

```json
{
  "children": [
    {
      "block_type": {
        // Block type-specific content
      }
    }
  ]
}
```

When updating a block, use this format:

```json
{
  "block_type": {
    // Block type-specific content to update
  },
  "archived": false
}
```

**IMPORTANT**: Do NOT include the `"type": "block_type"` field at the top level in requests. The type is implicitly defined by the block type key (e.g., "paragraph", "heading_1", etc.).

## Block Properties

Fields marked with an * are available to integrations with any capabilities. Other properties require read content capabilities.

| Field | Type | Description | Example value |
|-------|------|-------------|---------------|
| object* | string | Always "block" | "block" |
| id* | string (UUIDv4) | Identifier for the block | "7af38973-3787-41b3-bd75-0ed3a1edfac9" |
| parent | object | Information about the block's parent | {"type": "block_id", "block_id": "7d50a184-5bbe-4d90-8f29-6bec57ed817b"} |
| type | string (enum) | Type of block | "paragraph" |
| created_time | string (ISO 8601) | Date and time when this block was created | "2020-03-17T19:10:04.968Z" |
| created_by | Partial User | User who created the block | {"object": "user", "id": "45ee8d13-687b-47ce-a5ca-6e2e45548c4b"} |
| last_edited_time | string (ISO 8601) | Date and time when this block was last updated | "2020-03-17T19:10:04.968Z" |
| last_edited_by | Partial User | User who last edited the block | {"object": "user", "id": "45ee8d13-687b-47ce-a5ca-6e2e45548c4b"} |
| archived | boolean | The archived status of the block | false |
| in_trash | boolean | Whether the block has been deleted | false |
| has_children | boolean | Whether or not the block has children blocks nested within it | true |
| {type} | block type object | An object containing type-specific block information | See block type-specific documentation |

## Block Types

The Notion API supports the following block types:

- bookmark
- breadcrumb
- bulleted_list_item
- callout
- child_database
- child_page
- column
- column_list
- divider
- embed
- equation
- file
- heading_1
- heading_2
- heading_3
- image
- link_preview
- link_to_page
- numbered_list_item
- paragraph
- pdf
- quote
- synced_block
- table
- table_of_contents
- table_row
- template
- to_do
- toggle
- video

Any unsupported block types appear with a type set to "unsupported".

## Block Types That Support Children

The following block types can contain nested blocks:

- Bulleted list item
- Callout
- Child database
- Child page
- Column
- Heading 1 (when is_toggleable: true)
- Heading 2 (when is_toggleable: true)
- Heading 3 (when is_toggleable: true)
- Numbered list item
- Paragraph
- Quote
- Synced block
- Table
- Template
- To do
- Toggle

## Example

### Complete Block Response

```json
{
  "object": "block",
  "id": "c02fc1d3-db8b-45c5-a222-27595b15aea7",
  "parent": {
    "type": "page_id",
    "page_id": "59833787-2cf9-4fdf-8782-e53db20768a5"
  },
  "created_time": "2022-03-01T19:05:00.000Z",
  "last_edited_time": "2022-07-06T19:41:00.000Z",
  "created_by": {
    "object": "user",
    "id": "ee5f0f84-409a-440f-983a-a5315961c6e4"
  },
  "last_edited_by": {
    "object": "user",
    "id": "ee5f0f84-409a-440f-983a-a5315961c6e4"
  },
  "has_children": false,
  "archived": false,
  "in_trash": false,
  "type": "heading_2",
  "heading_2": {
    "rich_text": [
      {
        "type": "text",
        "text": {
          "content": "Lacinato kale",
          "link": null
        },
        "annotations": {
          "bold": false,
          "italic": false,
          "strikethrough": false,
          "underline": false,
          "code": false,
          "color": "green"
        },
        "plain_text": "Lacinato kale",
        "href": null
      }
    ],
    "color": "default",
    "is_toggleable": false
  }
}
```

### Appending Multiple Blocks

```json
{
  "children": [
    {
      "paragraph": {
        "rich_text": [
          {
            "text": {
              "content": "First paragraph"
            }
          }
        ]
      }
    },
    {
      "heading_2": {
        "rich_text": [
          {
            "text": {
              "content": "Section heading"
            }
          }
        ]
      }
    },
    {
      "bulleted_list_item": {
        "rich_text": [
          {
            "text": {
              "content": "List item"
            }
          }
        ]
      }
    }
  ]
}
```

### Updating a Block

```json
{
  "paragraph": {
    "rich_text": [
      {
        "text": {
          "content": "Updated text content"
        }
      }
    ],
    "color": "blue_background"
  }
}
```