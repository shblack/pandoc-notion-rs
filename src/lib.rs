//! Pandoc-Notion: A library for converting between Notion content and Pandoc
//!
//! This crate provides tools to work with Notion's API data structures and
//! convert them to Pandoc's Abstract Syntax Tree (AST) for document processing.

// Main modules
pub mod converter;
pub mod n2p;
pub mod notion_block_fetcher;
pub mod notion_block_putter;
pub mod p2n;
pub mod test_utils;
pub mod text;

// Re-export key types and traits for convenient usage
pub use converter::{NotionConverter, ConversionError, create_converter};
pub use n2p::ConversionConfig;
pub use notion_block_fetcher::{NotionBlockFetcher, BlockFetcherConfig, create_block_fetcher, create_debug_block_fetcher};
pub use notion_block_putter::{NotionBlockPutter, BlockPutterConfig, create_block_putter, create_debug_block_putter};
pub use pandoc_types::definition::Pandoc;
pub use text::processor::PandocProcessor;
pub use text::{TextFormat, TextProcessingError, TextProcessor};

// Provide a convenience function to create a text processor
pub fn create_text_processor() -> PandocProcessor {
    text::processor::create_processor()
}

// This forces users to be explicit about which conversion path they're using
pub mod prelude {
    pub use crate::converter::{ConversionError, NotionConverter};
    pub use crate::create_converter;
    pub use crate::create_text_processor;
    pub use crate::ConversionConfig;
    pub use crate::notion_block_fetcher::{NotionBlockFetcher, BlockFetcherConfig, create_block_fetcher, create_debug_block_fetcher};
    pub use crate::notion_block_putter::{NotionBlockPutter, BlockPutterConfig, create_block_putter, create_debug_block_putter};
    pub use crate::text::{TextFormat, TextProcessor};
    pub use pandoc_types::definition::{Block, Inline, Pandoc};
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
