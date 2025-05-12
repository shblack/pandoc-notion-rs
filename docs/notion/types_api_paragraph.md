# Paragraph Block

Paragraph blocks are the most common content blocks in Notion. This document details how to work with paragraphs through the Notion API.

## API Response Format

When retrieving a paragraph block from the API, the response will include the following structure:

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
  "type": "paragraph",
  "paragraph": {
    "rich_text": [...],
    "color": "default"
  }
}
```

## API Request Format (Create/Update)

When creating or updating a paragraph block, use this format:

```json
{
  "paragraph": {
    "rich_text": [...],
    "color": "default"
  }
}
```

IMPORTANT: Do NOT include the `"type": "paragraph"` field at the top level in requests. The type is implicitly defined by the `paragraph` key.

When adding a paragraph as a child block, wrap it in a children array:

```json
{
  "children": [
    {
      "paragraph": {
        "rich_text": [
          {
            "text": {"content": "I'm a paragraph."}
          }
        ]
      }
    }
  ]
}
```

## Paragraph Properties

Paragraph block objects contain the following information within the `paragraph` property:

| Field     | Type                   | Description                                |
|-----------|------------------------|--------------------------------------------|
| rich_text | array of rich text objects | The rich text displayed in the paragraph. |
| color     | string (enum)          | The color of the block. Default is "default". |
| children  | array of block objects | The nested child blocks (if any) of the paragraph. |

### Available Colors

The `color` property can be set to any of the following values:
- "default"
- "gray", "gray_background"
- "brown", "brown_background"
- "orange", "orange_background"
- "yellow", "yellow_background"
- "green", "green_background"
- "blue", "blue_background"
- "purple", "purple_background"
- "pink", "pink_background"
- "red", "red_background"

## Examples

### Example 1: API Response for a Basic Paragraph

```json
{
  "object": "block",
  "id": "16cabc1e-edcd-8165-9e47-ddcd77401df9",
  "parent": {"type": "page_id", "page_id": "16cabc1e-edcd-81f1-ae27-de4d47c5c0c2"},
  "created_time": "2024-12-30T17:21:00.000Z",
  "last_edited_time": "2024-12-30T17:21:00.000Z",
  "has_children": false,
  "archived": false,
  "in_trash": false,
  "type": "paragraph",
  "paragraph": {
    "rich_text": [
      {
        "type": "text",
        "text": {"content": "I'm a paragraph.", "link": null},
        "annotations": {
          "bold": false,
          "italic": false,
          "strikethrough": false,
          "underline": false,
          "code": false,
          "color": "default"
        },
        "plain_text": "I'm a paragraph.",
        "href": null
      }
    ],
    "color": "default"
  }
}
```

### Example 2: Creating a Simple Paragraph

```json
{
  "paragraph": {
    "rich_text": [
      {
        "text": {
          "content": "I'm a paragraph."
        }
      }
    ]
  }
}
```

### Example 3: Updating a Paragraph with Formatting

```json
{
  "paragraph": {
    "rich_text": [
      {
        "text": {
          "content": "I'm an updated paragraph."
        },
        "annotations": {
          "bold": true,
          "color": "red_background"
        }
      }
    ]
  }
}
```

### Example 4: Paragraph with a Mention

```json
{
  "paragraph": {
    "rich_text": [
      {
        "type": "mention",
        "mention": {
          "type": "date",
          "date": {
            "start": "2023-03-01",
            "end": null,
            "time_zone": null
          }
        },
        "annotations": {
          "bold": false,
          "italic": false,
          "strikethrough": false,
          "underline": false,
          "code": false,
          "color": "default"
        },
        "plain_text": "2023-03-01",
        "href": null
      },
      {
        "type": "text",
        "text": {
          "content": " is the date",
          "link": null
        },
        "annotations": {
          "bold": false,
          "italic": false,
          "strikethrough": false,
          "underline": false,
          "code": false,
          "color": "default"
        },
        "plain_text": " is the date",
        "href": null
      }
    ],
    "color": "default"
  }
}
```