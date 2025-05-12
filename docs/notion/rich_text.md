# Rich Text Object

Rich text objects represent the formatting and content of text in Notion. They're used across many block types to represent formatted text content.

## API Response Format

When retrieving rich text from the API, each rich text object follows this structure:

```json
{
  "type": "text",
  "text": {
    "content": "Example text",
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
  "plain_text": "Example text",
  "href": null
}
```

## API Request Format (Create/Update)

When creating or updating rich text, use this format:

```json
{
  "text": {
    "content": "Example text",
    "link": {
      "url": "https://example.com"
    }
  },
  "annotations": {
    "bold": true,
    "italic": false,
    "strikethrough": false,
    "underline": false,
    "code": false,
    "color": "red"
  }
}
```

The simplest form can be:

```json
{
  "text": {
    "content": "Simple text"
  }
}
```

## Rich Text Types

Rich text objects can have different types:

| Type | Description |
|------|-------------|
| text | Plain text content, optionally with a link |
| mention | Mentions of users, pages, databases, or dates |
| equation | Mathematical expressions in LaTeX format |

## Rich Text Properties

### Text Objects

| Field | Type | Description |
|-------|------|-------------|
| content | string | The text content |
| link | object or null | Optional link data (if the text is a link) |

### Mention Objects

| Field | Type | Description |
|-------|------|-------------|
| type | string | The type of mention (user, page, database, date, etc.) |
| [type] | object | Object with type-specific information |

### Equation Objects

| Field | Type | Description |
|-------|------|-------------|
| expression | string | The LaTeX string of the equation |

### Annotations

All rich text objects can have these annotation properties:

| Field | Type | Description |
|-------|------|-------------|
| bold | boolean | Whether the text is bold |
| italic | boolean | Whether the text is italic |
| strikethrough | boolean | Whether the text has a strikethrough |
| underline | boolean | Whether the text is underlined |
| code | boolean | Whether the text is inline code |
| color | string | The color of the text |

### Available Colors

The `color` property can be set to any of the following values:
- "default"
- "gray"
- "brown"
- "orange"
- "yellow"
- "green"
- "blue"
- "purple"
- "pink"
- "red"
- "gray_background"
- "brown_background"
- "orange_background"
- "yellow_background"
- "green_background"
- "blue_background"
- "purple_background"
- "pink_background"
- "red_background"

## Examples

### Example 1: Basic Text with Formatting

```json
{
  "text": {
    "content": "Bold and italic text"
  },
  "annotations": {
    "bold": true,
    "italic": true
  }
}
```

### Example 2: Text with Link

```json
{
  "text": {
    "content": "Visit Notion",
    "link": {
      "url": "https://notion.so"
    }
  }
}
```

### Example 3: Mention of a User

```json
{
  "type": "mention",
  "mention": {
    "type": "user",
    "user": {
      "id": "7335a82f-1793-4320-a57a-62257dfbe2cf"
    }
  }
}
```

### Example 4: Date Mention

```json
{
  "type": "mention",
  "mention": {
    "type": "date",
    "date": {
      "start": "2023-05-01",
      "end": null,
      "time_zone": null
    }
  }
}
```

### Example 5: Equation

```json
{
  "type": "equation",
  "equation": {
    "expression": "E = mc^2"
  }
}
```

### Example 6: Multiple Rich Text Objects in a Block

When creating a block with multiple rich text segments, provide an array:

```json
{
  "paragraph": {
    "rich_text": [
      {
        "text": {
          "content": "This is "
        }
      },
      {
        "text": {
          "content": "bold",
          "link": null
        },
        "annotations": {
          "bold": true
        }
      },
      {
        "text": {
          "content": " and this is "
        }
      },
      {
        "text": {
          "content": "italic",
          "link": null
        },
        "annotations": {
          "italic": true
        }
      }
    ]
  }
}
```