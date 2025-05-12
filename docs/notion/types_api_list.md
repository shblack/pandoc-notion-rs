# List Block Types

Notion provides three types of list blocks:
- Bulleted list items
- Numbered list items
- To-do list items

Each type has specific properties and formatting options.

## Bulleted List Item

### API Response Format

When retrieving a bulleted list item from the API, the response includes the following structure:

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
  "type": "bulleted_list_item",
  "bulleted_list_item": {
    "rich_text": [...],
    "color": "default",
    "children": [...]
  }
}
```

### API Request Format (Create/Update)

When creating or updating a bulleted list item, use this format:

```json
{
  "bulleted_list_item": {
    "rich_text": [
      {
        "text": {
          "content": "List item content"
        }
      }
    ],
    "color": "default"
  }
}
```

**IMPORTANT**: Do NOT include a `"type": "bulleted_list_item"` field at the top level in requests. The type is implicitly defined by the `bulleted_list_item` key.

### Properties

| Field | Type | Description |
|-------|------|-------------|
| rich_text | array of rich text objects | The rich text content in the bulleted list item |
| color | string (enum) | The color of the block |
| children | array of block objects | Optional nested child blocks |

## Numbered List Item

### API Response Format

When retrieving a numbered list item from the API, the response includes the following structure:

```json
{
  "object": "block",
  "id": "block-id",
  "type": "numbered_list_item",
  "numbered_list_item": {
    "rich_text": [...],
    "color": "default",
    "children": [...]
  }
  // Other standard block properties
}
```

### API Request Format (Create/Update)

When creating or updating a numbered list item, use this format:

```json
{
  "numbered_list_item": {
    "rich_text": [
      {
        "text": {
          "content": "List item content"
        }
      }
    ],
    "color": "default"
  }
}
```

### Properties

| Field | Type | Description |
|-------|------|-------------|
| rich_text | array of rich text objects | The rich text displayed in the numbered list item |
| color | string (enum) | The color of the block |
| children | array of block objects | Optional nested child blocks |

## To-Do List Item

### API Response Format

When retrieving a to-do list item from the API, the response includes the following structure:

```json
{
  "object": "block",
  "id": "block-id",
  "type": "to_do",
  "to_do": {
    "rich_text": [...],
    "checked": false,
    "color": "default",
    "children": [...]
  }
  // Other standard block properties
}
```

### API Request Format (Create/Update)

When creating or updating a to-do list item, use this format:

```json
{
  "to_do": {
    "rich_text": [
      {
        "text": {
          "content": "To-do item content"
        }
      }
    ],
    "checked": false,
    "color": "default"
  }
}
```

### Properties

| Field | Type | Description |
|-------|------|-------------|
| rich_text | array of rich text objects | The rich text displayed in the to-do item |
| checked | boolean | Whether the to-do item is checked |
| color | string (enum) | The color of the block |
| children | array of block objects | Optional nested child blocks |

## Available Colors

The `color` property for all list types can be set to any of the following values:
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

### Example 1: Complete Bulleted List Item Response

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
  "has_children": true,
  "archived": false,
  "in_trash": false,
  "type": "bulleted_list_item",
  "bulleted_list_item": {
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
          "color": "default"
        },
        "plain_text": "Lacinato kale",
        "href": null
      }
    ],
    "color": "default",
    "children": [
      {
        "type": "paragraph",
        "paragraph": {
          "rich_text": [
            {
              "type": "text",
              "text": {
                "content": "Child paragraph",
                "link": null
              }
            }
          ],
          "color": "default"
        }
      }
    ]
  }
}
```

### Example 2: Creating a Numbered List Item

```json
{
  "numbered_list_item": {
    "rich_text": [
      {
        "text": {
          "content": "Finish reading the docs"
        }
      }
    ],
    "color": "default"
  }
}
```

### Example 3: Creating a To-Do List Item

```json
{
  "to_do": {
    "rich_text": [
      {
        "text": {
          "content": "Finish Q3 goals"
        }
      }
    ],
    "checked": false,
    "color": "default"
  }
}
```

### Example 4: Creating a Nested List Structure

```json
{
  "bulleted_list_item": {
    "rich_text": [
      {
        "text": {
          "content": "Main list item"
        }
      }
    ],
    "children": [
      {
        "bulleted_list_item": {
          "rich_text": [
            {
              "text": {
                "content": "Nested list item"
              }
            }
          ]
        }
      },
      {
        "to_do": {
          "rich_text": [
            {
              "text": {
                "content": "Nested to-do item"
              }
            }
          ],
          "checked": false
        }
      }
    ]
  }
}
```