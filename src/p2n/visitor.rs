use notion_client::objects::block::Block as NotionBlock;
use pandoc_types::definition::{Attr, Block as PandocBlock, Inline};
use std::error::Error;

/// Visitor trait for processing Pandoc blocks.
/// This implementation focuses on the currently supported block types:
/// paragraphs, headings, lists, and inline text.
pub trait PandocBlockVisitor {
    /// Visit a generic block and dispatch to the appropriate visitor method
    fn visit_block(&self, block: &PandocBlock) -> Result<Vec<NotionBlock>, Box<dyn Error>>;

    // Core block types we support
    fn visit_paragraph(&self, inlines: &[Inline]) -> Result<Vec<NotionBlock>, Box<dyn Error>>;
    fn visit_header(
        &self,
        level: i32,
        attr: &Attr,
        inlines: &[Inline],
    ) -> Result<Vec<NotionBlock>, Box<dyn Error>>;
    fn visit_plain(&self, inlines: &[Inline]) -> Result<Vec<NotionBlock>, Box<dyn Error>>;

    // List block types
    fn visit_bullet_list(
        &self,
        items: &[Vec<PandocBlock>],
    ) -> Result<Vec<NotionBlock>, Box<dyn Error>>;
    fn visit_ordered_list(
        &self,
        attrs: &pandoc_types::definition::ListAttributes,
        items: &[Vec<PandocBlock>],
    ) -> Result<Vec<NotionBlock>, Box<dyn Error>>;

    fn visit_block_quote(&self, blocks: &[PandocBlock])
    -> Result<Vec<NotionBlock>, Box<dyn Error>>;

    // Code blocks
    fn visit_code_block(&self, attr: &Attr, content: &str)
    -> Result<Vec<NotionBlock>, Box<dyn Error>>;

    // Handle unsupported blocks
    fn visit_unsupported(&self, block: &PandocBlock) -> Result<Vec<NotionBlock>, Box<dyn Error>>;

    /// Process a list of blocks
    fn process_blocks(&self, blocks: &[PandocBlock]) -> Result<Vec<NotionBlock>, Box<dyn Error>>;
}

/// Trait to make Pandoc blocks visitable
pub trait PandocBlockVisitable {
    /// Accept a visitor and return the processed result
    fn accept<V: PandocBlockVisitor>(
        &self,
        visitor: &V,
    ) -> Result<Vec<NotionBlock>, Box<dyn Error>>;
}

/// Implementation of Visitable for Pandoc blocks
impl PandocBlockVisitable for PandocBlock {
    fn accept<V: PandocBlockVisitor>(
        &self,
        visitor: &V,
    ) -> Result<Vec<NotionBlock>, Box<dyn Error>> {
        visitor.visit_block(self)
    }
}
