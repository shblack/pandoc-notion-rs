# Page Object

A Page object represents a single Notion page and contains its property values.

## API Response Format

When retrieving a page from the API, the response includes the following structure:

```json
{
  "object": "page",
  "id": "page-id",
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
  "cover": null,
  "icon": null,
  "parent": {
    "type": "database_id",
    "database_id": "database-id"
  },
  "archived": false,
  "in_trash": false,
  "properties": {...},
  "url": "https://www.notion.so/Page-Title-page-id",
  "public_url": null
}
```

## API Request Format (Create/Update)

When creating a new page, use this format:

```json
{
  "parent": {
    "type": "database_id",
    "database_id": "database-id"
  },
  "properties": {
    "Title": {
      "title": [
        {
          "text": {
            "content": "New page title"
          }
        }
      ]
    }
    // Additional properties based on the database schema
  },
  "children": [
    // Optional array of block objects to add as page content
  ]
}
```

When updating a page, use this format:

```json
{
  "properties": {
    "Title": {
      "title": [
        {
          "text": {
            "content": "Updated page title"
          }
        }
      ]
    }
    // Other properties to update
  },
  "archived": false,
  "icon": {
    "type": "emoji",
    "emoji": "🚀"
  }
  // Other fields to update
}
```

## Page Properties

Properties marked with an * are available to integrations with any capabilities. Other properties require read content capabilities to be returned from the Notion API.

| Property | Type | Description | Example value |
|----------|------|-------------|---------------|
| object* | string | Always "page" | "page" |
| id* | string (UUIDv4) | Unique identifier of the page | "45ee8d13-687b-47ce-a5ca-6e2e45548c4b" |
| created_time | string (ISO 8601) | Date and time when this page was created | "2020-03-17T19:10:04.968Z" |
| created_by | Partial User | User who created the page | {"object": "user", "id": "45ee8d13-687b-47ce-a5ca-6e2e45548c4b"} |
| last_edited_time | string (ISO 8601) | Date and time when this page was last updated | "2020-03-17T19:10:04.968Z" |
| last_edited_by | Partial User | User who last edited the page | {"object": "user", "id": "45ee8d13-687b-47ce-a5ca-6e2e45548c4b"} |
| archived | boolean | The archived status of the page | false |
| in_trash | boolean | Whether the page is in Trash | false |
| icon | File Object or Emoji object | Page icon (only type "external" is supported for File Object) | {"type": "emoji", "emoji": "🐞"} |
| cover | File object | Page cover image (only type "external" is supported) | {"type": "external", "external": {"url": "https://example.com/image.jpg"}} |
| properties | object | Property values of this page | See "Properties" section below |
| parent | object | Information about the page's parent | {"type": "database_id", "database_id": "d9824bdc-8445-4327-be8b-5b47500af6ce"} |
| url | string | The URL of the Notion page | "https://www.notion.so/Avocado-d093f1d200464ce78b36e58a3f0d8043" |
| public_url | string | The public page URL if published, otherwise null | "https://jm-testing.notion.site/p1-6df2c07bfc6b4c46815ad205d132e22d" |

## Properties

As of API version 2022-06-28, the `properties` object only contains the ID of each property in responses. In prior versions, it contained the values as well.

The structure of properties depends on the page's parent:
- If `parent.type` is "page_id" or "workspace", then the only valid property key is "title"
- If `parent.type` is "database_id", then the keys and values are determined by the database schema

### Property Structure

Each property in the `properties` object follows this pattern:
```json
"Property Name": {
  "id": "property_id",
  "type": "property_type",
  "property_type": {
    // Property-specific content
  }
}
```

Page content is available as blocks. The content can be read using retrieve block children and appended using append block children.

## Examples

### Example 1: Complete Page Response

```json
{
  "object": "page",
  "id": "be633bf1-dfa0-436d-b259-571129a590e5",
  "created_time": "2022-10-24T22:54:00.000Z",
  "last_edited_time": "2023-03-08T18:25:00.000Z",
  "created_by": {
    "object": "user",
    "id": "c2f20311-9e54-4d11-8c79-7398424ae41e"
  },
  "last_edited_by": {
    "object": "user",
    "id": "9188c6a5-7381-452f-b3dc-d4865aa89bdf"
  },
  "cover": null,
  "icon": {
    "type": "emoji",
    "emoji": "🐞"
  },
  "parent": {
    "type": "database_id",
    "database_id": "a1d8501e-1ac1-43e9-a6bd-ea9fe6c8822b"
  },
  "archived": true,
  "in_trash": true,
  "properties": {
    "Due date": {
      "id": "M%3BBw",
      "type": "date",
      "date": {
        "start": "2023-02-23",
        "end": null,
        "time_zone": null
      }
    },
    "Status": {
      "id": "Z%3ClH",
      "type": "status",
      "status": {
        "id": "86ddb6ec-0627-47f8-800d-b65afd28be13",
        "name": "Not started",
        "color": "default"
      }
    },
    "Title": {
      "id": "title",
      "type": "title",
      "title": [
        {
          "type": "text",
          "text": {
            "content": "Bug bash",
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
          "plain_text": "Bug bash",
          "href": null
        }
      ]
    }
  },
  "url": "https://www.notion.so/Bug-bash-be633bf1dfa0436db259571129a590e5",
  "public_url": "https://jm-testing.notion.site/p1-6df2c07bfc6b4c46815ad205d132e22d"
}
```

### Example 2: Creating a New Page in a Database

```json
{
  "parent": {
    "type": "database_id",
    "database_id": "a1d8501e-1ac1-43e9-a6bd-ea9fe6c8822b"
  },
  "properties": {
    "Title": {
      "title": [
        {
          "text": {
            "content": "New Task"
          }
        }
      ]
    },
    "Due date": {
      "date": {
        "start": "2023-05-15"
      }
    },
    "Status": {
      "status": {
        "name": "In progress"
      }
    }
  },
  "icon": {
    "type": "emoji",
    "emoji": "📝"
  }
}
```

### Example 3: Creating a New Page with Content

```json
{
  "parent": {
    "type": "page_id",
    "page_id": "b0668f48-8d66-4733-9d22-29ffab8d7c63"
  },
  "properties": {
    "title": [
      {
        "text": {
          "content": "New Subpage"
        }
      }
    ]
  },
  "children": [
    {
      "paragraph": {
        "rich_text": [
          {
            "text": {
              "content": "This is the first paragraph of my new page."
            }
          }
        ]
      }
    },
    {
      "heading_1": {
        "rich_text": [
          {
            "text": {
              "content": "Section Heading"
            }
          }
        ]
      }
    }
  ]
}
```