//! Pandoc-Notion: A library for converting between Notion content and Pandoc
//!
//! This crate provides tools to work with Notion's API data structures and
//! convert them to Pandoc's Abstract Syntax Tree (AST) for document processing.

// Main modules
pub mod n2p;
pub mod p2n;
pub mod test_utils;
pub mod text;

// Re-export key types and traits for convenient usage
pub use pandoc_types::definition::Pandoc;
pub use text::{TextFormat, TextProcessor, TextProcessingError};
pub use text::processor::PandocProcessor;

// Provide a convenience function to create a text processor
pub fn create_text_processor() -> PandocProcessor {
    text::processor::create_processor()
}

// This forces users to be explicit about which conversion path they're using
pub mod prelude {
    pub use pandoc_types::definition::{Pandoc, Block, Inline};
    pub use crate::text::{TextFormat, TextProcessor};
    pub use crate::create_text_processor;
}

// Version of the crate
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
