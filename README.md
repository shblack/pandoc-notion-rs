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
- Comprehensive test suite ensuring robust conversion

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
pandoc-notion = "0.1.0"
```

## Usage

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