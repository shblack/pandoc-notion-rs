//! Pandoc-Notion: A library for converting between Notion content and Pandoc
//!
//! This crate provides tools to work with Notion's API data structures and
//! convert them to Pandoc's Abstract Syntax Tree (AST) for document processing.

mod converter;
mod n2p;
mod notion;
mod notion_block_fetcher;
mod notion_block_putter;
mod p2n;
mod test_utils;
mod text;

pub use converter::{NotionConverter, ConversionError, create_converter};
pub use n2p::ConversionConfig;
pub use text::{TextFormat, TextProcessingError, TextProcessor};

pub fn create_text_processor() -> impl TextProcessor {
    text::processor::create_processor()
}

pub mod prelude {
    pub use crate::converter::{ConversionError, NotionConverter};
    pub use crate::create_converter;
    pub use crate::create_text_processor;
    pub use crate::ConversionConfig;
    pub use crate::text::{TextFormat, TextProcessor};
}

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
