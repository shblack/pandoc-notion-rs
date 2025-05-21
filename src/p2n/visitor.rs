use notion_client::objects::block::Block as NotionBlock;
use pandoc_types::definition::{Block as PandocBlock, Inline, Attr};
use std::error::Error;

/// Visitor trait for processing Pandoc blocks.
/// This implementation focuses on the currently supported block types:
/// paragraphs, headings, and inline text.
pub trait PandocBlockVisitor {
    /// Visit a generic block and dispatch to the appropriate visitor method
    fn visit_block(&self, block: &PandocBlock, parent_id: Option<String>) -> Result<Vec<NotionBlock>, Box<dyn Error>>;
    
    // Core block types we support
    fn visit_paragraph(&self, inlines: &[Inline], parent_id: Option<String>) -> Result<Vec<NotionBlock>, Box<dyn Error>>;
    fn visit_header(&self, level: i32, attr: &Attr, inlines: &[Inline], parent_id: Option<String>) -> Result<Vec<NotionBlock>, Box<dyn Error>>;
    fn visit_plain(&self, inlines: &[Inline], parent_id: Option<String>) -> Result<Vec<NotionBlock>, Box<dyn Error>>;
    
    // Handle unsupported blocks
    fn visit_unsupported(&self, block: &PandocBlock, parent_id: Option<String>) -> Result<Vec<NotionBlock>, Box<dyn Error>>;
    
    /// Process a list of blocks
    fn process_blocks(&self, blocks: &[PandocBlock], parent_id: Option<String>) -> Result<Vec<NotionBlock>, Box<dyn Error>>;
}

/// Trait to make Pandoc blocks visitable
pub trait PandocBlockVisitable {
    /// Accept a visitor and return the processed result
    fn accept<V: PandocBlockVisitor>(&self, visitor: &V, parent_id: Option<String>) -> Result<Vec<NotionBlock>, Box<dyn Error>>;
}

/// Implementation of Visitable for Pandoc blocks
impl PandocBlockVisitable for PandocBlock {
    fn accept<V: PandocBlockVisitor>(&self, visitor: &V, parent_id: Option<String>) -> Result<Vec<NotionBlock>, Box<dyn Error>> {
        visitor.visit_block(self, parent_id)
    }
}