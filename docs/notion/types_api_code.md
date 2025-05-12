# Code Block

Code blocks in Notion allow you to display formatted code with syntax highlighting for a variety of programming languages.

## API Response Format

When retrieving a code block from the API, the response includes the following structure:

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
  "type": "code",
  "code": {
    "rich_text": [...],
    "caption": [...],
    "language": "javascript"
  }
}
```

## API Request Format (Create/Update)

When creating or updating a code block, use this format:

```json
{
  "code": {
    "rich_text": [
      {
        "text": {
          "content": "const hello = 'world';"
        }
      }
    ],
    "caption": [
      {
        "text": {
          "content": "Example code"
        }
      }
    ],
    "language": "javascript"
  }
}
```

**IMPORTANT**: Do NOT include a `"type": "code"` field at the top level in requests. The type is implicitly defined by the `code` key.

## Code Block Properties

| Field | Type | Description |
|-------|------|-------------|
| rich_text | array of rich text objects | The code content to display |
| caption | array of rich text objects | Optional caption for the code block |
| language | string (enum) | The programming language for syntax highlighting |

### Supported Languages

The `language` property can be set to any of the following values:

- abap
- arduino
- bash
- basic
- c
- clojure
- coffeescript
- c++
- c#
- css
- dart
- diff
- docker
- elixir
- elm
- erlang
- flow
- fortran
- f#
- gherkin
- glsl
- go
- graphql
- groovy
- haskell
- html
- java
- javascript
- json
- julia
- kotlin
- latex
- less
- lisp
- livescript
- lua
- makefile
- markdown
- markup
- matlab
- mermaid
- nix
- objective-c
- ocaml
- pascal
- perl
- php
- plain text
- powershell
- prolog
- protobuf
- python
- r
- reason
- ruby
- rust
- sass
- scala
- scheme
- scss
- shell
- sql
- swift
- typescript
- vb.net
- verilog
- vhdl
- visual basic
- webassembly
- xml
- yaml
- java/c/c++/c#

## Examples

### Example 1: Complete Code Block Response

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
  "type": "code",
  "code": {
    "caption": [],
    "rich_text": [
      {
        "type": "text",
        "text": {
          "content": "const a = 3"
        }
      }
    ],
    "language": "javascript"
  }
}
```

### Example 2: Creating a Python Code Block

```json
{
  "code": {
    "rich_text": [
      {
        "text": {
          "content": "def hello_world():\n    print('Hello, World!')\n\nhello_world()"
        }
      }
    ],
    "language": "python"
  }
}
```

### Example 3: Creating a Code Block with Caption

```json
{
  "code": {
    "rich_text": [
      {
        "text": {
          "content": "SELECT * FROM users WHERE active = true;"
        }
      }
    ],
    "caption": [
      {
        "text": {
          "content": "Query to find active users"
        }
      }
    ],
    "language": "sql"
  }
}
```