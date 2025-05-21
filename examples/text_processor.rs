//! Example demonstrating how to use the text processing functionality.
//!
//! This example shows:
//! - Converting between different text formats
//! - Working with Pandoc AST
//! - Working with files

use pandoc_notion::{create_text_processor, TextFormat};
use pandoc_notion::prelude::*;
use std::error::Error;
use std::path::Path;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    // 1. Create a text processor
    let text_processor = create_text_processor();
    
    // Check if Pandoc is available
    match text_processor.check_pandoc_availability() {
        Ok(version) => println!("Using {}", version),
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("This example requires Pandoc to be installed and available in your PATH");
            return Err(Box::new(e));
        }
    }
    
    // 2. Convert markdown to AST
    let markdown = "# Hello World\n\nThis is **bold** and *italic* text.\n\n## A List\n\n- Item 1\n- Item 2\n- Item 3\n";
    println!("\n--- Original Markdown ---\n{}", markdown);
    
    let ast = text_processor.text_to_ast(markdown, TextFormat::Markdown)?;
    
    // 3. Convert AST to different formats
    let html = text_processor.ast_to_text(&ast, TextFormat::Html)?;
    println!("\n--- Converted to HTML ---\n{}", html);
    
    let plain_text = text_processor.ast_to_text(&ast, TextFormat::PlainText)?;
    println!("\n--- Converted to Plain Text ---\n{}", plain_text);
    
    // 4. Direct format conversion
    let latex = text_processor.convert_text(
        markdown,
        TextFormat::Markdown,
        TextFormat::Latex
    )?;
    println!("\n--- Converted to LaTeX ---\n{}", latex);
    
    // 5. Working with temporary files
    let temp_dir = tempfile::tempdir()?;
    let md_file = temp_dir.path().join("example.md");
    let html_file = temp_dir.path().join("example.html");
    
    // Write markdown to file
    fs::write(&md_file, markdown)?;
    println!("\nWrote markdown to temporary file: {}", md_file.display());
    
    // Convert file
    text_processor.convert_file(&md_file, &html_file)?;
    println!("Converted to HTML file: {}", html_file.display());
    
    // Read the result
    let html_content = fs::read_to_string(&html_file)?;
    println!("\nHTML file content length: {} bytes", html_content.len());
    
    // 6. Roundtrip: file → AST → file
    let ast = text_processor.file_to_ast(&md_file)?;
    
    let rst_file = temp_dir.path().join("example.rst");
    text_processor.ast_to_file_with_format(&ast, &rst_file, TextFormat::Rst)?;
    println!("\nConverted to RST file via AST: {}", rst_file.display());
    
    // Clean up temp directory
    temp_dir.close()?;
    
    println!("\nAll operations completed successfully!");
    Ok(())
}