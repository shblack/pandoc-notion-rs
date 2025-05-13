//! Pandoc-Notion: A library for converting between Notion content and Pandoc
//!
//! This crate provides tools to work with Notion's API data structures and
//! convert them to Pandoc's Abstract Syntax Tree (AST) for document processing.

// Main modules
pub mod n2p;
pub mod p2n;

// Re-export only the essential types for convenience

// Version of the crate
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
