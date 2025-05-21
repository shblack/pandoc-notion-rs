# pandoc-notion

A Rust library for converting [Notion](https://www.notion.so) content to [Pandoc](https://pandoc.org) document format.

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

## Overview

`pandoc-notion` provides a bridge between Notion's API and Pandoc's document processing capabilities. It allows you to convert Notion blocks and rich text into Pandoc's Abstract Syntax Tree (AST), enabling export to various document formats such as Markdown, HTML, PDF, LaTeX, and more.

## Features

- Convert Notion rich text to Pandoc inline elements
- Preserve text formatting (bold, italic, strikethrough, underline, inline code)
- Support for mathematical expressions
- Handle hyperlinks and URL references
- Proper handling of whitespace and special characters
- Conversion of Notion's text colors to CSS classes
- Text processing functionality for various formats (Markdown, HTML, Plain Text, LaTeX, etc.)
- File-based conversion between different document formats
- Direct manipulation of Pandoc's Abstract Syntax Tree (AST)
- Optional preservation of Notion-specific attributes (colors, block types, etc.)
- Comprehensive test suite ensuring robust conversion

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
pandoc-notion = "0.1.0"
```

## Usage

### Text Processing and Format Conversion

```rust
use pandoc_notion::{create_text_processor, TextFormat};
use pandoc_notion::prelude::*;

// Create a text processor
let text_processor = create_text_processor();

// Convert markdown text to Pandoc AST
let markdown = "# Hello World\n\nThis is **bold** and *italic* text.";
let ast = text_processor.text_to_ast(markdown, TextFormat::Markdown)?;

// Convert AST to HTML
let html = text_processor.ast_to_text(&ast, TextFormat::Html)?;

// Direct format conversion
let latex = text_processor.convert_text(
    markdown,
    TextFormat::Markdown,
    TextFormat::Latex
)?;

// File-based operations
text_processor.convert_file("input.md", "output.html")?;

// Working with explicit formats
text_processor.convert_file_with_format(
    "input.txt", 
    "output.rst", 
    TextFormat::PlainText, 
    TextFormat::Rst
)?;
```

### Converting Notion Rich Text to Pandoc Inline Elements

```rust
use pandoc_notion::notion::text::{create_text, Annotations};
use pandoc_notion::n2p::notion_text::NotionTextConverter;

// Create some Notion rich text
let text = create_text("Hello, world!");

// Convert to Pandoc inline elements
let inline_elements = NotionTextConverter::convert(&[text]);

// Use the inline elements in Pandoc document processing
```

### Configuring Attribute Preservation

By default, Notion-specific attributes (like colors, block types, etc.) are not preserved in the Pandoc output. You can enable attribute preservation:

```rust
use pandoc_notion::{create_converter, ConversionConfig};
use pandoc_notion::prelude::*;

// Create a converter with default settings (no attribute preservation)
let default_converter = create_converter();

// Create a converter with attribute preservation enabled
let converter = create_converter()
    .with_preserve_attributes(true);

// Alternatively, use the full configuration
let config = ConversionConfig {
    preserve_attributes: true,
    ..ConversionConfig::default()
};
let custom_converter = create_converter()
    .with_config(config);
```

### Converting Formatted Text

```rust
use pandoc_notion::notion::text::{create_formatted_text, Annotations};
use pandoc_notion::n2p::notion_text::NotionTextConverter;

// Create text with formatting
let mut annotations = Annotations::default();
annotations.bold = true;
annotations.italic = true;

let formatted_text = create_formatted_text(
    "Important note", 
    annotations,
    Some("https://example.com".to_string()) // Optional link
);

// Convert to Pandoc inline elements
let inline_elements = NotionTextConverter::convert(&[formatted_text]);
```

## Project Structure

- `src/notion/` - Definitions for Notion API data structures
- `src/n2p/` - Converters from Notion to Pandoc
  - `notion_text.rs` - Converts Notion rich text to Pandoc inline elements
- `src/text/` - Text processing functionality
  - `mod.rs` - Core text processing traits and types
  - `processor.rs` - Pandoc-based implementation of text processing

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- [Notion API Documentation](https://developers.notion.com/)
- [Pandoc-types Crate](https://crates.io/crates/pandoc-types)
- [Pandoc](https://pandoc.org/) - The universal document converter

## Requirements

- [Pandoc](https://pandoc.org/installing.html) must be installed and available in your PATH for text processing functionality