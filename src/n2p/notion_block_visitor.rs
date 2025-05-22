use crate::n2p::notion_code::try_convert_to_code;
use crate::n2p::notion_heading::try_convert_to_heading;
use crate::n2p::notion_list::{self, ListBuilder};
use crate::n2p::notion_paragraph::try_convert_to_paragraph;
use crate::n2p::notion_quote::{convert_notion_quote, QuoteBuilder};
use crate::n2p::visitor::NotionBlockVisitor;
use crate::n2p::ConversionConfig;
use notion_client::objects::block::{
    Block as NotionBlock, BlockType, BulletedListItemValue, CodeValue, HeadingsValue, 
    NumberedListItemValue, ParagraphValue, QuoteValue, ToDoValue,
};
use pandoc_types::definition::{Attr, Block as PandocBlock, Inline};

/// Concrete implementation of the NotionBlockVisitor for converting Notion blocks to Pandoc
pub struct NotionToPandocVisitor {
    config: ConversionConfig,
}

impl NotionToPandocVisitor {
    /// Create a new visitor with default configuration
    pub fn new() -> Self {
        Self {
            config: ConversionConfig::default(),
        }
    }
    
    /// Create a new visitor with specific configuration
    pub fn with_config(config: ConversionConfig) -> Self {
        Self { config }
    }

    /// Convert a list of Notion blocks to Pandoc blocks
    pub fn convert_blocks(&self, blocks: &[NotionBlock]) -> Vec<PandocBlock> {
        // First pass: process each block individually
        let raw_blocks = self.process_children(blocks);
        
        // Second pass: collect and merge list blocks
        let with_lists = ListBuilder::collect_document_lists(raw_blocks);
        
        // Third pass: process blockquotes to ensure proper nesting order
        QuoteBuilder::process_document_quotes(with_lists)
    }
}

impl NotionBlockVisitor for NotionToPandocVisitor {
    fn visit_block(&self, block: &NotionBlock) -> Vec<PandocBlock> {
        // Dispatch to specific visitor method based on block type
        // Currently only supporting paragraph and heading blocks
        match &block.block_type {
            BlockType::Paragraph { paragraph } => self.visit_paragraph(block, paragraph),
            BlockType::Heading1 { heading_1 } => self.visit_heading_1(block, heading_1),
            BlockType::Heading2 { heading_2 } => self.visit_heading_2(block, heading_2),
            BlockType::Heading3 { heading_3 } => self.visit_heading_3(block, heading_3),
            BlockType::Quote { quote } => self.visit_quote(block, quote),
            BlockType::Code { code } => self.visit_code(block, code),
            BlockType::BulletedListItem { bulleted_list_item } => {
                self.visit_bulleted_list_item(block, bulleted_list_item)
            }
            BlockType::NumberedListItem { numbered_list_item } => {
                self.visit_numbered_list_item(block, numbered_list_item)
            }
            BlockType::ToDo { to_do } => self.visit_todo(block, to_do),
            _ => self.visit_unsupported(block),
        }
    }

    fn visit_paragraph(&self, block: &NotionBlock, paragraph: &ParagraphValue) -> Vec<PandocBlock> {
        let mut result = Vec::new();

        // Use existing paragraph converter
        if let Some(pandoc_block) = try_convert_to_paragraph(block, &self.config) {
            result.push(pandoc_block);
        }

        // Process children if present
        if let Some(children) = &paragraph.children {
            if !children.is_empty() {
                // Use the visitor's process_children method to handle children
                result.extend(self.process_children(children));
            }
        }

        result
    }

    fn visit_heading_1(&self, block: &NotionBlock, _heading: &HeadingsValue) -> Vec<PandocBlock> {
        // Use existing heading converter
        if let Some(pandoc_block) = try_convert_to_heading(block, &self.config) {
            vec![pandoc_block]
        } else {
            vec![]
        }
    }

    fn visit_heading_2(&self, block: &NotionBlock, _heading: &HeadingsValue) -> Vec<PandocBlock> {
        // Use existing heading converter
        if let Some(pandoc_block) = try_convert_to_heading(block, &self.config) {
            vec![pandoc_block]
        } else {
            vec![]
        }
    }

    fn visit_heading_3(&self, block: &NotionBlock, _heading: &HeadingsValue) -> Vec<PandocBlock> {
        // Use existing heading converter
        if let Some(pandoc_block) = try_convert_to_heading(block, &self.config) {
            vec![pandoc_block]
        } else {
            vec![]
        }
    }

    fn visit_quote(&self, block: &NotionBlock, quote: &QuoteValue) -> Vec<PandocBlock> {
        let mut result = Vec::new();

        // Process children if present
        let children_blocks = if let Some(children) = &quote.children {
            if !children.is_empty() {
                // Process children separately to be included in the quote
                self.process_children(children)
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };

        // Convert the quote with its children
        if let Some(pandoc_block) = convert_notion_quote(block, &self.config, children_blocks) {
            result.push(pandoc_block);
        }

        result
    }

    fn visit_code(&self, block: &NotionBlock, _code: &CodeValue) -> Vec<PandocBlock> {
        let mut result = Vec::new();

        // Convert the code block
        if let Some(pandoc_block) = try_convert_to_code(block, &self.config) {
            result.push(pandoc_block);
        }

        result
    }

    // Implementation for other block types removed to simplify the code
    // Only paragraph and heading blocks are supported for now

    fn visit_bulleted_list_item(
        &self,
        block: &NotionBlock,
        _item: &BulletedListItemValue,
    ) -> Vec<PandocBlock> {
        // First, get the children
        let children = self.get_children(block);
        
        // Process children to get their Pandoc blocks
        let children_blocks = self.process_children(&children);
        
        // Convert this list item with its children
        if let Some(list_block) = notion_list::convert_notion_bulleted_list(block, &self.config, children_blocks) {
            vec![list_block]
        } else {
            self.visit_unsupported(block)
        }
    }

    fn visit_numbered_list_item(
        &self,
        block: &NotionBlock,
        _item: &NumberedListItemValue,
    ) -> Vec<PandocBlock> {
        // First, get the children
        let children = self.get_children(block);
        
        // Process children to get their Pandoc blocks
        let children_blocks = self.process_children(&children);
        
        // Convert this list item with its children
        if let Some(list_block) = notion_list::convert_notion_numbered_list(block, &self.config, children_blocks) {
            vec![list_block]
        } else {
            self.visit_unsupported(block)
        }
    }

    fn visit_todo(&self, block: &NotionBlock, _todo: &ToDoValue) -> Vec<PandocBlock> {
        // First, get the children
        let children = self.get_children(block);
        
        // Process children to get their Pandoc blocks
        let children_blocks = self.process_children(&children);
        
        // Convert this todo item with its children
        if let Some(todo_block) = notion_list::convert_notion_todo(block, &self.config, children_blocks) {
            vec![todo_block]
        } else {
            self.visit_unsupported(block)
        }
    }

    fn visit_unsupported(&self, block: &NotionBlock) -> Vec<PandocBlock> {
        // Create a comment indicating an unsupported block
        let block_type = format!("{:?}", block.block_type);
        let attr = if self.config.preserve_attributes {
            Attr {
                identifier: "".to_string(),
                classes: vec!["unsupported-block".to_string()],
                attributes: vec![("data-block-type".to_string(), block_type)],
            }
        } else {
            Attr::default()
        };

        vec![PandocBlock::Div(
            attr,
            vec![PandocBlock::Para(vec![Inline::Str(
                "Unsupported block type".to_string(),
            )])],
        )]
    }

    fn process_children(&self, children: &[NotionBlock]) -> Vec<PandocBlock> {
        let mut result = Vec::new();

        for child in children {
            let converted = self.visit_block(child);
            result.extend(converted);
        }

        result
    }

    fn get_children(&self, block: &NotionBlock) -> Vec<NotionBlock> {
        // Return children based on block type
        match &block.block_type {
            BlockType::Paragraph { paragraph } => paragraph.children.clone().unwrap_or_default(),
            BlockType::BulletedListItem { bulleted_list_item } => {
                bulleted_list_item.children.clone().unwrap_or_default()
            }
            BlockType::NumberedListItem { numbered_list_item } => {
                numbered_list_item.children.clone().unwrap_or_default()
            }
            BlockType::ToDo { to_do } => to_do.children.clone().unwrap_or_default(),
            BlockType::Quote { quote } => quote.children.clone().unwrap_or_default(),
            BlockType::Code { code: _ } => Vec::new(), // Code blocks don't have children in Notion API
            _ => Vec::new(),
        }
    }
}

/// Helper function to convert Pandoc inline elements to a string
#[allow(dead_code)]
fn text_to_string(inlines: &[Inline]) -> String {
    inlines
        .iter()
        .map(|inline| match inline {
            Inline::Str(s) => s.clone(),
            Inline::Space => " ".to_string(),
            Inline::SoftBreak => "\n".to_string(),
            Inline::LineBreak => "\n".to_string(),
            _ => "".to_string(), // Simplified handling for other inline types
        })
        .collect()
}
