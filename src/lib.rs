//! Pandoc-Notion: A library for converting between Notion content and Pandoc
//!
//! This crate provides tools to work with Notion's API data structures and
//! convert them to Pandoc's Abstract Syntax Tree (AST) for document processing.

pub mod converter;
pub mod n2p;
pub mod notion;
pub mod notion_block_fetcher;
pub mod notion_block_putter;
pub mod p2n;
pub mod test_utils;
pub mod text;

pub use converter::{NotionConverter, ConversionError, create_converter};
pub use n2p::ConversionConfig;
pub use notion::toggleable::{ToggleableBlock, ToggleableBlockChildren};
pub use notion_block_fetcher::{NotionBlockFetcher, BlockFetcherConfig, create_block_fetcher, FetchResult};
pub use notion_block_putter::{NotionBlockPutter, BlockPutterConfig, create_block_putter};
pub use pandoc_types::definition::Pandoc;
pub use text::processor::PandocProcessor;
pub use text::{TextFormat, TextProcessingError, TextProcessor};

pub fn create_text_processor() -> PandocProcessor {
    text::processor::create_processor()
}

pub mod prelude {
    pub use crate::converter::{ConversionError, NotionConverter};
    pub use crate::create_converter;
    pub use crate::create_text_processor;
    pub use crate::ConversionConfig;
    pub use crate::notion::toggleable::{ToggleableBlock, ToggleableBlockChildren};
    pub use crate::notion_block_fetcher::{NotionBlockFetcher, BlockFetcherConfig, create_block_fetcher, FetchResult};
    pub use crate::notion_block_putter::{NotionBlockPutter, BlockPutterConfig, create_block_putter};
    pub use crate::text::{TextFormat, TextProcessor};
    pub use pandoc_types::definition::{Block, Inline, Pandoc};
}

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
