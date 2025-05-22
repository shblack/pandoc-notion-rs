use notion_client::objects::block::{
    Block as NotionBlock, BulletedListItemValue, CodeValue, DividerValue, HeadingsValue, NumberedListItemValue,
    ParagraphValue, QuoteValue, ToDoValue, ToggleValue,
};
use pandoc_types::definition::Block as PandocBlock;

/// Visitor trait for processing Notion blocks.
/// Each method corresponds to a different Notion block type.
pub trait NotionBlockVisitor {
    /// Visit a generic block and dispatch to the appropriate visitor method
    fn visit_block(&self, block: &NotionBlock) -> Vec<PandocBlock>;

    // Core block types
    fn visit_paragraph(&self, block: &NotionBlock, paragraph: &ParagraphValue) -> Vec<PandocBlock>;
    fn visit_heading_1(&self, block: &NotionBlock, heading: &HeadingsValue) -> Vec<PandocBlock>;
    fn visit_heading_2(&self, block: &NotionBlock, heading: &HeadingsValue) -> Vec<PandocBlock>;
    fn visit_heading_3(&self, block: &NotionBlock, heading: &HeadingsValue) -> Vec<PandocBlock>;
    fn visit_quote(&self, block: &NotionBlock, quote: &QuoteValue) -> Vec<PandocBlock>;
    fn visit_code(&self, block: &NotionBlock, code: &CodeValue) -> Vec<PandocBlock>;
    fn visit_bulleted_list_item(
        &self,
        block: &NotionBlock,
        item: &BulletedListItemValue,
    ) -> Vec<PandocBlock>;
    fn visit_numbered_list_item(
        &self,
        block: &NotionBlock,
        item: &NumberedListItemValue,
    ) -> Vec<PandocBlock>;
    fn visit_todo(&self, block: &NotionBlock, todo: &ToDoValue) -> Vec<PandocBlock>;
    fn visit_divider(&self, block: &NotionBlock, divider: &DividerValue) -> Vec<PandocBlock>;
    fn visit_toggle(&self, block: &NotionBlock, toggle: &ToggleValue) -> Vec<PandocBlock>;
    // Other block type visitors will be added as support expands

    // Handle unsupported blocks
    fn visit_unsupported(&self, block: &NotionBlock) -> Vec<PandocBlock>;

    /// Process all children of a block
    fn process_children(&self, children: &[NotionBlock]) -> Vec<PandocBlock>;

    /// Get children of a block based on block type
    fn get_children(&self, block: &NotionBlock) -> Vec<NotionBlock>;
}

/// Trait to make Notion blocks visitable
pub trait NotionBlockVisitable {
    /// Accept a visitor and return the processed result
    fn accept<V: NotionBlockVisitor>(&self, visitor: &V) -> Vec<PandocBlock>;
}

/// Implementation of Visitable for Notion blocks
impl NotionBlockVisitable for NotionBlock {
    fn accept<V: NotionBlockVisitor>(&self, visitor: &V) -> Vec<PandocBlock> {
        visitor.visit_block(self)
    }
}
