# Pandoc Inline Text Representation

This documentation explains how Pandoc represents inline text elements in its Abstract Syntax Tree (AST). Understanding this structure is essential for developing applications that need to interact with Pandoc's JSON format, such as filters, converters, or custom text processors.

## Table of Contents

- [Introduction](#introduction)
- [AST Structure](#ast-structure)
- [Basic Text Elements](#basic-text-elements)
  - [Plain Text (Str)](#plain-text-str)
  - [Whitespace (Space)](#whitespace-space)
- [Text Formatting](#text-formatting)
  - [Bold Text (Strong)](#bold-text-strong)
  - [Italic Text (Emph)](#italic-text-emph)
  - [Strikethrough Text (Strikeout)](#strikethrough-text-strikeout)
  - [Underlined Text (RawInline)](#underlined-text-rawinline)
  - [Inline Code (Code)](#inline-code-code)
- [Mathematical Notation](#mathematical-notation)
  - [Inline Math](#inline-math)
  - [Display Math](#display-math)
- [Links and URLs](#links-and-urls)
  - [Hyperlinks (Link)](#hyperlinks-link)
  - [URLs](#urls)
- [Advanced Formatting](#advanced-formatting)
  - [Combining Multiple Formats](#combining-multiple-formats)
  - [Spans with Attributes](#spans-with-attributes)
- [Working with the AST](#working-with-the-ast)
  - [Converting to AST](#converting-to-ast)
  - [Processing the AST](#processing-the-ast)

## Introduction

Pandoc represents documents as an Abstract Syntax Tree (AST) in JSON format. This representation breaks down text into structured elements that capture both content and formatting. For inline text (text within a paragraph or other block element), Pandoc uses a system of nested elements to represent different formatting features.

## AST Structure

Each inline element in the Pandoc AST follows a common structure:

```
{
  "t": "ElementType",
  "c": content
}
```

Where:
- `"t"` specifies the element type (e.g., "Str", "Emph", "Strong")
- `"c"` contains the element's content, which varies by type

## Basic Text Elements

### Plain Text (Str)

Plain text is represented using the `Str` type.

**Markdown:**
```
Hello world
```

**AST Representation:**
```json
{
  "t": "Str",
  "c": "Hello"
},
{
  "t": "Space"
},
{
  "t": "Str",
  "c": "world"
}
```

### Whitespace (Space)

Spaces between words are represented as separate `Space` elements.

**Markdown:**
```
word1 word2
```

**AST Representation:**
```json
{
  "t": "Str",
  "c": "word1"
},
{
  "t": "Space"
},
{
  "t": "Str",
  "c": "word2"
}
```

## Text Formatting

### Bold Text (Strong)

Bold text is represented with the `Strong` type. The `c` field contains an array of inline elements that should be formatted as bold.

**Markdown:**
```
This is **bold text**.
```

**AST Representation:**
```json
{
  "t": "Str",
  "c": "This"
},
{
  "t": "Space"
},
{
  "t": "Str",
  "c": "is"
},
{
  "t": "Space"
},
{
  "t": "Strong",
  "c": [
    {
      "t": "Str",
      "c": "bold"
    },
    {
      "t": "Space"
    },
    {
      "t": "Str",
      "c": "text"
    }
  ]
},
{
  "t": "Str",
  "c": "."
}
```

### Italic Text (Emph)

Italic text uses the `Emph` (emphasis) type.

**Markdown:**
```
This is *italic text*.
```

**AST Representation:**
```json
{
  "t": "Str",
  "c": "This"
},
{
  "t": "Space"
},
{
  "t": "Str",
  "c": "is"
},
{
  "t": "Space"
},
{
  "t": "Emph",
  "c": [
    {
      "t": "Str",
      "c": "italic"
    },
    {
      "t": "Space"
    },
    {
      "t": "Str",
      "c": "text"
    }
  ]
},
{
  "t": "Str",
  "c": "."
}
```

### Strikethrough Text (Strikeout)

Strikethrough text uses the `Strikeout` type.

**Markdown:**
```
This is ~~struck through~~ text.
```

**AST Representation:**
```json
{
  "t": "Str",
  "c": "This"
},
{
  "t": "Space"
},
{
  "t": "Str",
  "c": "is"
},
{
  "t": "Space"
},
{
  "t": "Strikeout",
  "c": [
    {
      "t": "Str",
      "c": "struck"
    },
    {
      "t": "Space"
    },
    {
      "t": "Str",
      "c": "through"
    }
  ]
},
{
  "t": "Space"
},
{
  "t": "Str",
  "c": "text."
}
```

### Underlined Text (RawInline)

Since Markdown doesn't natively support underlining, Pandoc represents underlined text using `RawInline` elements with HTML tags.

**Markdown/HTML:**
```
This is <u>underlined</u> text.
```

**AST Representation:**
```json
{
  "t": "Str",
  "c": "This"
},
{
  "t": "Space"
},
{
  "t": "Str",
  "c": "is"
},
{
  "t": "Space"
},
{
  "t": "RawInline",
  "c": [
    "html",
    "<u>"
  ]
},
{
  "t": "Str",
  "c": "underlined"
},
{
  "t": "RawInline",
  "c": [
    "html",
    "</u>"
  ]
},
{
  "t": "Space"
},
{
  "t": "Str",
  "c": "text."
}
```

### Inline Code (Code)

Inline code is represented with the `Code` type.

**Markdown:**
```
Use the `print()` function.
```

**AST Representation:**
```json
{
  "t": "Str",
  "c": "Use"
},
{
  "t": "Space"
},
{
  "t": "Str",
  "c": "the"
},
{
  "t": "Space"
},
{
  "t": "Code",
  "c": [
    ["", [], []],
    "print()"
  ]
},
{
  "t": "Space"
},
{
  "t": "Str",
  "c": "function."
}
```

The first element in the `Code` element's content array is an attributes array consisting of:
1. An identifier (empty string if none provided)
2. A list of classes (empty array if none provided)
3. A list of key-value pairs (empty array if none provided)

## Mathematical Notation

### Inline Math

Inline math is represented with the `Math` type, with the `InlineMath` subtype.

**Markdown:**
```
Einstein's equation: $E=mc^2$
```

**AST Representation:**
```json
{
  "t": "Str",
  "c": "Einstein's"
},
{
  "t": "Space"
},
{
  "t": "Str",
  "c": "equation:"
},
{
  "t": "Space"
},
{
  "t": "Math",
  "c": [
    {
      "t": "InlineMath"
    },
    "E=mc^2"
  ]
}
```

### Display Math

Display math (math on its own line) is represented with the `Math` type, with the `DisplayMath` subtype.

**Markdown:**
```
$$\int_{a}^{b} f(x) dx$$
```

**AST Representation:**
```json
{
  "t": "Math",
  "c": [
    {
      "t": "DisplayMath"
    },
    "\\int_{a}^{b} f(x) dx"
  ]
}
```

## Links and URLs

### Hyperlinks (Link)

Links are represented with the `Link` type.

**Markdown:**
```
Visit [Pandoc's website](https://pandoc.org).
```

**AST Representation:**
```json
{
  "t": "Str",
  "c": "Visit"
},
{
  "t": "Space"
},
{
  "t": "Link",
  "c": [
    ["", [], []],
    [
      {
        "t": "Str",
        "c": "Pandoc's"
      },
      {
        "t": "Space"
      },
      {
        "t": "Str",
        "c": "website"
      }
    ],
    [
      "https://pandoc.org",
      ""
    ]
  ]
},
{
  "t": "Str",
  "c": "."
}
```

The `Link` element's content array consists of:
1. Attributes (identifier, classes, key-value pairs)
2. An array of inline elements (the link text)
3. An array containing the URL and title (if any)

### URLs

Automatic URL linking in Markdown is also represented with the `Link` type.

**Markdown:**
```
Visit https://pandoc.org for more information.
```

**AST Representation:**
```json
{
  "t": "Str",
  "c": "Visit"
},
{
  "t": "Space"
},
{
  "t": "Link",
  "c": [
    ["", [], []],
    [
      {
        "t": "Str",
        "c": "https://pandoc.org"
      }
    ],
    [
      "https://pandoc.org",
      ""
    ]
  ]
},
{
  "t": "Space"
},
{
  "t": "Str",
  "c": "for"
},
{
  "t": "Space"
},
{
  "t": "Str",
  "c": "more"
},
{
  "t": "Space"
},
{
  "t": "Str",
  "c": "information."
}
```

## Advanced Formatting

### Combining Multiple Formats

Text with multiple formatting features is represented as nested elements.

**Markdown:**
```
This is ***bold and italic*** text.
```

**AST Representation:**
```json
{
  "t": "Str",
  "c": "This"
},
{
  "t": "Space"
},
{
  "t": "Str",
  "c": "is"
},
{
  "t": "Space"
},
{
  "t": "Strong",
  "c": [
    {
      "t": "Emph",
      "c": [
        {
          "t": "Str",
          "c": "bold"
        },
        {
          "t": "Space"
        },
        {
          "t": "Str",
          "c": "and"
        },
        {
          "t": "Space"
        },
        {
          "t": "Str",
          "c": "italic"
        }
      ]
    }
  ]
},
{
  "t": "Space"
},
{
  "t": "Str",
  "c": "text."
}
```

### Spans with Attributes

Custom styled text using spans with attributes.

**Markdown:**
```
[This is a span with attributes]{.custom-class #custom-id key="value"}
```

**AST Representation:**
```json
{
  "t": "Span",
  "c": [
    ["custom-id", ["custom-class"], [["key", "value"]]],
    [
      {
        "t": "Str",
        "c": "This"
      },
      {
        "t": "Space"
      },
      {
        "t": "Str",
        "c": "is"
      },
      {
        "t": "Space"
      },
      {
        "t": "Str",
        "c": "a"
      },
      {
        "t": "Space"
      },
      {
        "t": "Str",
        "c": "span"
      },
      {
        "t": "Space"
      },
      {
        "t": "Str",
        "c": "with"
      },
      {
        "t": "Space"
      },
      {
        "t": "Str",
        "c": "attributes"
      }
    ]
  ]
}
```

## Working with the AST

### Converting to AST

To convert Markdown to Pandoc's AST format, you can use the Pandoc command-line tool:

```bash
pandoc -f markdown -t json input.md -o output.json
```

In Python, you can use the `pypandoc` library:

```python
import json
import pypandoc

# Convert markdown to JSON AST
markdown_text = "This is **bold** and *italic* text."
ast_json = pypandoc.convert_text(markdown_text, 'json', format='md')

# Parse the JSON
ast = json.loads(ast_json)

# Now you can work with the AST
```

### Processing the AST

The hierarchical structure of the AST allows for programmatic traversal and transformation. Here's a simple example in JavaScript that counts the number of emphasized (italic) text elements:

```javascript
function countEmphElements(ast) {
  let count = 0;
  
  function traverse(node) {
    if (typeof node === 'object' && node !== null) {
      if (node.t === 'Emph') {
        count++;
      }
      
      if (Array.isArray(node.c)) {
        node.c.forEach(child => traverse(child));
      } else if (typeof node.c === 'object' && node.c !== null) {
        traverse(node.c);
      }
      
      if (Array.isArray(node)) {
        node.forEach(item => traverse(item));
      }
    }
  }
  
  traverse(ast);
  return count;
}
```

In Rust, you might use pattern matching to process different element types:

```rust
fn process_inline_element(element: &InlineElement) {
    match element {
        InlineElement::Str(content) => println!("String: {}", content),
        InlineElement::Emph(elements) => {
            println!("Emphasized text:");
            for elem in elements {
                process_inline_element(elem);
            }
        },
        InlineElement::Strong(elements) => {
            println!("Strong text:");
            for elem in elements {
                process_inline_element(elem);
            }
        },
        // Handle other element types
        _ => println!("Other element type"),
    }
}
```

Understanding the structure of Pandoc's AST is crucial for developing tools that transform or analyze document content, allowing for rich functionality when working with formatted text.