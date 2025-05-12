# Heading Blocks

Notion supports three levels of headings: Heading 1, Heading 2, and Heading 3. Each heading type is represented by a different block object.

## API Response Format

When retrieving a heading block from the API, the response includes the following structure:

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
  "type": "heading_1",  // Can be heading_1, heading_2, or heading_3
  "heading_1": {        // The key matches the "type" value
    "rich_text": [...],
    "color": "default",
    "is_toggleable": false
  }
}
```

## API Request Format (Create/Update)

When creating or updating a heading block, use this format:

```json
{
  "heading_1": {  // Use heading_1, heading_2, or heading_3 as needed
    "rich_text": [...],
    "color": "default",
    "is_toggleable": false
  }
}
```

**IMPORTANT**: Do NOT include a `"type": "heading_X"` field at the top level in requests. The type is implicitly defined by the heading key used (heading_1, heading_2, or heading_3).

## Heading Properties

All heading blocks (heading_1, heading_2, and heading_3) support the same properties:

| Field | Type | Description | 
|-------|------|-------------|
| rich_text | array of rich text objects | The rich text of the heading |
| color | string (enum) | The color of the block |
| is_toggleable | boolean | Whether the heading is a toggle heading that can contain children |

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

### Example 1: Heading 1 Block Response

```json
{
  "object": "block",
  "id": "8b29ad0c-56b9-4b4b-a7e6-1d2d9a361d2a",
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
  "type": "heading_1",
  "heading_1": {
    "rich_text": [
      {
        "type": "text",
        "text": {
          "content": "Main Heading",
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
        "plain_text": "Main Heading",
        "href": null
      }
    ],
    "color": "default",
    "is_toggleable": false
  }
}
```

### Example 2: Creating a Heading 2 Block

```json
{
  "heading_2": {
    "rich_text": [
      {
        "text": {
          "content": "Section Heading"
        }
      }
    ],
    "color": "blue"
  }
}
```

### Example 3: Creating a Toggle Heading 3 Block

```json
{
  "heading_3": {
    "rich_text": [
      {
        "text": {
          "content": "Toggle Section"
        }
      }
    ],
    "color": "default",
    "is_toggleable": true
  }
}
```

### Example 4: Creating a Heading with Formatting

```json
{
  "heading_1": {
    "rich_text": [
      {
        "text": {
          "content": "Important "
        },
        "annotations": {
          "bold": true
        }
      },
      {
        "text": {
          "content": "Heading"
        },
        "annotations": {
          "italic": true
        }
      }
    ],
    "color": "red"
  }
}
```