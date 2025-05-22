use crate::n2p::ConversionConfig;
use crate::n2p::notion_code::try_convert_to_code;
use crate::n2p::notion_divider;
use crate::n2p::notion_heading::try_convert_to_heading;
use crate::n2p::notion_list::{self, ListBuilder};
use crate::n2p::notion_paragraph::try_convert_to_paragraph;
use crate::n2p::notion_quote::{QuoteBuilder, convert_notion_quote};
use crate::n2p::notion_toggle::try_convert_to_toggle;
use crate::n2p::visitor::NotionBlockVisitor;
use crate::notion::toggleable::{ToggleableBlock, ToggleableBlockChildren};
use log;
use notion_client::objects::block::{
    Block as NotionBlock, BlockType, BulletedListItemValue, CodeValue, DividerValue, HeadingsValue,
    NumberedListItemValue, ParagraphValue, QuoteValue, ToDoValue, ToggleValue,
};
use pandoc_types::definition::{Attr, Block as PandocBlock, Inline};

/// Concrete implementation of the NotionBlockVisitor for converting Notion blocks to Pandoc
pub struct NotionToPandocVisitor {
    config: ConversionConfig,
    toggleable_children: Option<ToggleableBlockChildren>,
}

impl NotionToPandocVisitor {
    /// Create a new visitor with default configuration
    pub fn new() -> Self {
        Self {
            config: ConversionConfig::default(),
            toggleable_children: None,
        }
    }

    /// Create a new visitor with specific configuration
    pub fn with_config(config: ConversionConfig) -> Self {
        Self {
            config,
            toggleable_children: None,
        }
    }

    /// Create a new visitor with toggleable children support
    pub fn with_toggleable_children(
        config: ConversionConfig,
        toggleable_children: ToggleableBlockChildren,
    ) -> Self {
        Self {
            config,
            toggleable_children: Some(toggleable_children),
        }
    }

    /// Convert a list of Notion blocks to Pandoc blocks
    pub fn convert_blocks(&self, blocks: &[NotionBlock]) -> Vec<PandocBlock> {
        // Process blocks with full document-level processing
        self.process_children(blocks)
    }

    /// Helper method to process any block and its children using a closure
    /// This is the standard approach for all block types that can have children.
    /// It handles:
    /// 1. Converting the block itself via the provided closure
    /// 2. Processing any children the block might have (from any source)
    fn process_block_with_children<F>(
        &self,
        block: &NotionBlock,
        process_block: F,
    ) -> Vec<PandocBlock>
    where
        F: FnOnce(&NotionBlock) -> Option<PandocBlock>,
    {
        let mut result = Vec::new();

        // Process the block itself
        if let Some(pandoc_block) = process_block(block) {
            result.push(pandoc_block);
        }

        // Process children if they exist (either in native field or toggleable_children manager)
        let children = self.get_children(block);
        if !children.is_empty() {
            log::debug!("Processing children of block: {:?}", block.block_type);
            result.extend(self.process_children(&children));
        }

        result
    }
    
    /// Helper method to process list items with a specific converter function
    fn process_list_item<F>(&self, block: &NotionBlock, converter: F) -> Vec<PandocBlock>
    where
        F: FnOnce(&NotionBlock, &ConversionConfig, Vec<PandocBlock>) -> Option<PandocBlock>,
    {
        // Get the children
        let children = self.get_children(block);
        
        // Process children to get their Pandoc blocks
        let children_blocks = self.process_children(&children);
        
        // Convert this list item with its children
        if let Some(list_block) = converter(block, &self.config, children_blocks) {
            vec![list_block]
        } else {
            self.visit_unsupported(block)
        }
    }
}

impl NotionBlockVisitor for NotionToPandocVisitor {
    fn visit_block(&self, block: &NotionBlock) -> Vec<PandocBlock> {
        // Dispatch to specific visitor method based on block type
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
            BlockType::Divider { divider } => self.visit_divider(block, divider),
            BlockType::Toggle { toggle } => self.visit_toggle(block, toggle),
            _ => self.visit_unsupported(block),
        }
    }

    fn visit_paragraph(
        &self,
        block: &NotionBlock,
        _paragraph: &ParagraphValue,
    ) -> Vec<PandocBlock> {
        self.process_block_with_children(block, |b| try_convert_to_paragraph(b, &self.config))
    }

    fn visit_heading_3(&self, block: &NotionBlock, _heading: &HeadingsValue) -> Vec<PandocBlock> {
        self.process_block_with_children(block, |b| try_convert_to_heading(b, &self.config))
    }

    fn visit_heading_1(&self, block: &NotionBlock, _heading: &HeadingsValue) -> Vec<PandocBlock> {
        self.process_block_with_children(block, |b| try_convert_to_heading(b, &self.config))
    }

    fn visit_heading_2(&self, block: &NotionBlock, _heading: &HeadingsValue) -> Vec<PandocBlock> {
        self.process_block_with_children(block, |b| try_convert_to_heading(b, &self.config))
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
        self.process_list_item(block, notion_list::convert_notion_bulleted_list)
    }

    fn visit_numbered_list_item(
        &self,
        block: &NotionBlock,
        _item: &NumberedListItemValue,
    ) -> Vec<PandocBlock> {
        self.process_list_item(block, notion_list::convert_notion_numbered_list)
    }

    fn visit_todo(&self, block: &NotionBlock, _todo: &ToDoValue) -> Vec<PandocBlock> {
        self.process_list_item(block, notion_list::convert_notion_todo)
    }

    fn visit_divider(&self, block: &NotionBlock, _divider: &DividerValue) -> Vec<PandocBlock> {
        // Dividers don't have children, so just convert the block directly
        if let Some(divider_block) = notion_divider::convert_notion_divider(block, &self.config) {
            vec![divider_block]
        } else {
            self.visit_unsupported(block)
        }
    }

    fn visit_toggle(&self, block: &NotionBlock, _toggle: &ToggleValue) -> Vec<PandocBlock> {
        let mut result = Vec::new();
        
        // Get children using the standard method
        let children = self.get_children(block);
        
        // Process children to get their Pandoc blocks if there are any
        let processed_children_blocks = if !children.is_empty() {
            log::debug!("Processing children of toggle block");
            self.process_children(&children)
        } else {
            Vec::new()
        };

        // If we're not rendering toggles as divs, handle differently
        if !self.config.render_toggle_div {
            // First convert the toggle itself (will be just text or a blank line)
            if let Some(toggle_block) = try_convert_to_toggle(block, &self.config) {
                result.push(toggle_block);
            }
            
            // Then add all the children blocks directly
            result.extend(processed_children_blocks);
        } else {
            // Rendering as divs - children go inside the div
            if let Some(mut pandoc_block) = try_convert_to_toggle(block, &self.config) {
                if let PandocBlock::Div(_, ref mut content) = pandoc_block {
                    // Add children inside the div
                    content.extend(processed_children_blocks);
                }
                result.push(pandoc_block);
            }
        }

        result
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
        // Step 1: Process each block individually
        let mut raw_blocks = Vec::new();
        for child in children {
            let converted = self.visit_block(child);
            raw_blocks.extend(converted);
        }
        
        // Step 2: Collect and merge list blocks
        let with_lists = ListBuilder::collect_document_lists(raw_blocks);
        
        // Step 3: Process blockquotes to ensure proper nesting order
        QuoteBuilder::process_document_quotes(with_lists)
    }

    /// Gets children for any block type, using a consistent approach:
    /// 1. First checks the block's native children field
    /// 2. If empty, falls back to the toggleable_children manager (for toggleable headings)
    fn get_children(&self, block: &NotionBlock) -> Vec<NotionBlock> {
        // First try to get children from the block's native children field based on block type
        let native_children = match &block.block_type {
            BlockType::Paragraph { paragraph } => paragraph.children.clone().unwrap_or_default(),
            BlockType::BulletedListItem { bulleted_list_item } => {
                bulleted_list_item.children.clone().unwrap_or_default()
            }
            BlockType::NumberedListItem { numbered_list_item } => {
                numbered_list_item.children.clone().unwrap_or_default()
            }
            BlockType::ToDo { to_do } => to_do.children.clone().unwrap_or_default(),
            BlockType::Quote { quote } => quote.children.clone().unwrap_or_default(),
            BlockType::Toggle { toggle } => toggle.children.clone().unwrap_or_default(),
            BlockType::Code { code: _ } => Vec::new(),
            _ => Vec::new(),
        };

        // If native children is empty, check if this is a toggleable heading block
        // with toggleable children
        if native_children.is_empty() && block.is_toggleable() {
            if let Some(ref toggleable_children) = self.toggleable_children {
                if let Some(children) = toggleable_children.get_children(block) {
                    return children.clone();
                }
            }
        }

        native_children
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
