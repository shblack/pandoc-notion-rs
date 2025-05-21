use crate::p2n::pandoc_code::PandocCodeConverter;
use crate::p2n::pandoc_heading::PandocHeadingConverter;
use crate::p2n::pandoc_list::PandocListConverter;
use crate::p2n::pandoc_paragraph::PandocParagraphConverter;
use crate::p2n::pandoc_quote::PandocQuoteConverter;
use crate::p2n::visitor::PandocBlockVisitor;
use notion_client::objects::block::{Block as NotionBlock, BlockType, ParagraphValue};
use notion_client::objects::rich_text::RichText;
use pandoc_types::definition::{Attr, Block as PandocBlock, Inline, ListAttributes};
use std::error::Error;

/// Concrete implementation of the PandocBlockVisitor for converting Pandoc blocks to Notion
pub struct PandocToNotionVisitor {
    paragraph_converter: PandocParagraphConverter,
    heading_converter: PandocHeadingConverter,
    list_converter: PandocListConverter,
    quote_converter: PandocQuoteConverter,
    code_converter: PandocCodeConverter,
}

impl PandocToNotionVisitor {
    /// Create a new visitor
    pub fn new() -> Self {
        Self {
            paragraph_converter: PandocParagraphConverter::new(),
            heading_converter: PandocHeadingConverter::new(),
            list_converter: PandocListConverter::new(),
            quote_converter: PandocQuoteConverter::new(),
            code_converter: PandocCodeConverter::new(),
        }
    }

    /// Convert a list of Pandoc blocks to Notion blocks
    pub fn convert_blocks(
        &self,
        blocks: &[PandocBlock],
    ) -> Result<Vec<NotionBlock>, Box<dyn Error>> {
        self.process_blocks(blocks)
    }
}

impl Default for PandocToNotionVisitor {
    fn default() -> Self {
        Self::new()
    }
}

impl PandocBlockVisitor for PandocToNotionVisitor {
    fn visit_block(&self, block: &PandocBlock) -> Result<Vec<NotionBlock>, Box<dyn Error>> {
        // Dispatch to specific visitor method based on block type
        match block {
            PandocBlock::Para(inlines) => self.visit_paragraph(inlines),
            PandocBlock::Plain(inlines) => self.visit_plain(inlines),
            PandocBlock::Header(level, attr, inlines) => self.visit_header(*level, attr, inlines),
            PandocBlock::BlockQuote(blocks) => self.visit_block_quote(blocks),
            PandocBlock::BulletList(items) => self.visit_bullet_list(items),
            PandocBlock::OrderedList(attrs, items) => self.visit_ordered_list(attrs, items),
            PandocBlock::CodeBlock(attr, content) => self.visit_code_block(attr, content),
            _ => self.visit_unsupported(block),
        }
    }

    fn visit_paragraph(&self, inlines: &[Inline]) -> Result<Vec<NotionBlock>, Box<dyn Error>> {
        // Use the existing paragraph converter
        let para_block = PandocBlock::Para(inlines.to_vec());
        match self.paragraph_converter.convert(&para_block, None)? {
            Some(block) => Ok(vec![block]),
            None => Ok(vec![]),
        }
    }

    fn visit_header(
        &self,
        level: i32,
        attr: &Attr,
        inlines: &[Inline],
    ) -> Result<Vec<NotionBlock>, Box<dyn Error>> {
        // Use the existing header converter
        let header_block = PandocBlock::Header(level, attr.clone(), inlines.to_vec());
        match self.heading_converter.convert(&header_block, None)? {
            Some(block) => Ok(vec![block]),
            None => Ok(vec![]),
        }
    }
    fn visit_plain(&self, inlines: &[Inline]) -> Result<Vec<NotionBlock>, Box<dyn Error>> {
        // Convert Plain to Paragraph by default
        // Plain is essentially the same as Para but used in different contexts
        self.visit_paragraph(inlines)
    }

    fn visit_bullet_list(
        &self,
        items: &[Vec<PandocBlock>],
    ) -> Result<Vec<NotionBlock>, Box<dyn Error>> {
        let mut result = Vec::new();

        for item in items {
            if let Some(first_block) = item.first() {
                // First check if this might be a todo item
                if let Some(todo_block) = self.list_converter.try_convert_todo_item(first_block)? {
                    // Process any nested blocks as children
                    let nested_blocks = self.list_converter.extract_nested_blocks(item);
                    let mut children = Vec::new();

                    // Process each nested block using the visitor
                    for nested_block in nested_blocks {
                        let nested_result = self.visit_block(nested_block)?;
                        children.extend(nested_result);
                    }

                    // Add children to the todo item
                    let todo_with_children = self
                        .list_converter
                        .add_children_to_block(todo_block, children)?;

                    result.push(todo_with_children);
                    continue;
                }

                // If not a to-do item, convert as a regular bulleted list item
                let bullet_block = self.list_converter.convert_bullet_list_item(first_block)?;

                // Process any nested blocks as children
                let nested_blocks = self.list_converter.extract_nested_blocks(item);
                let mut children = Vec::new();

                // Process each nested block using the visitor
                for nested_block in nested_blocks {
                    let nested_result = self.visit_block(nested_block)?;
                    children.extend(nested_result);
                }

                // Add children to the bullet item
                let bullet_with_children = self
                    .list_converter
                    .add_children_to_block(bullet_block, children)?;

                result.push(bullet_with_children);
            }
        }

        Ok(result)
    }

    fn visit_ordered_list(
        &self,
        attrs: &ListAttributes,
        items: &[Vec<PandocBlock>],
    ) -> Result<Vec<NotionBlock>, Box<dyn Error>> {
        let mut result = Vec::new();

        for item in items {
            if let Some(first_block) = item.first() {
                // First check if this might be a todo item (same approach as bullet list)
                if let Some(todo_block) = self.list_converter.try_convert_todo_item(first_block)? {
                    // Process any nested blocks as children
                    let nested_blocks = self.list_converter.extract_nested_blocks(item);
                    let mut children = Vec::new();

                    // Process each nested block using the visitor
                    for nested_block in nested_blocks {
                        let nested_result = self.visit_block(nested_block)?;
                        children.extend(nested_result);
                    }

                    // Add children to the todo item
                    let todo_with_children = self
                        .list_converter
                        .add_children_to_block(todo_block, children)?;

                    result.push(todo_with_children);
                    continue;
                }

                // If not a to-do item, convert as a regular ordered list item
                let ordered_block = self
                    .list_converter
                    .convert_ordered_list_item(first_block, attrs)?;

                // Process any nested blocks as children
                let nested_blocks = self.list_converter.extract_nested_blocks(item);
                let mut children = Vec::new();

                // Process each nested block using the visitor
                for nested_block in nested_blocks {
                    let nested_result = self.visit_block(nested_block)?;
                    children.extend(nested_result);
                }

                // Add children to the ordered item
                let ordered_with_children = self
                    .list_converter
                    .add_children_to_block(ordered_block, children)?;

                result.push(ordered_with_children);
            }
        }

        Ok(result)
    }

    fn visit_block_quote(
        &self,
        blocks: &[PandocBlock],
    ) -> Result<Vec<NotionBlock>, Box<dyn Error>> {
        // Convert the block quote using our quote converter
        let quote_block = self.quote_converter.convert(blocks)?;
        
        Ok(vec![quote_block])
    }

    fn visit_code_block(
        &self,
        attr: &Attr,
        content: &str,
    ) -> Result<Vec<NotionBlock>, Box<dyn Error>> {
        // Convert the code block using our code converter
        let code_block = self.code_converter.convert(attr, content)?;
        
        Ok(vec![code_block])
    }

    fn visit_unsupported(&self, block: &PandocBlock) -> Result<Vec<NotionBlock>, Box<dyn Error>> {
        // Create a paragraph block with a message about unsupported block
        let block_type = format!("{:?}", block);
        let message = format!("Unsupported Pandoc block type: {}", block_type);
        let rich_text = vec![RichText::Text {
            text: notion_client::objects::rich_text::Text {
                content: message.clone(),
                link: None,
            },
            annotations: None,
            plain_text: Some(message),
            href: None,
        }];

        let paragraph_value = ParagraphValue {
            rich_text,
            color: None,
            children: None,
        };

        let notion_block = NotionBlock {
            object: Some("block".to_string()),
            id: Some(String::new()),
            parent: None,
            created_time: None,
            created_by: None,
            last_edited_time: None,
            last_edited_by: None,
            archived: Some(false),
            has_children: Some(false),
            block_type: BlockType::Paragraph {
                paragraph: paragraph_value,
            },
        };

        Ok(vec![notion_block])
    }

    fn process_blocks(&self, blocks: &[PandocBlock]) -> Result<Vec<NotionBlock>, Box<dyn Error>> {
        let mut result = Vec::new();

        for block in blocks {
            let converted = self.visit_block(block)?;
            result.extend(converted);
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_paragraph() {
        let visitor = PandocToNotionVisitor::new();

        // Create a simple paragraph
        let paragraph = PandocBlock::Para(vec![Inline::Str("Test paragraph".to_string())]);

        // Convert using visitor
        let result = visitor.visit_block(&paragraph).unwrap();

        // Should produce a single block
        assert_eq!(result.len(), 1);

        // Verify it's a paragraph
        match &result[0].block_type {
            BlockType::Paragraph { paragraph } => {
                assert_eq!(paragraph.rich_text.len(), 1);
                assert_eq!(
                    paragraph.rich_text[0].plain_text().unwrap(),
                    "Test paragraph"
                );
            }
            _ => panic!("Expected Paragraph block type"),
        }
    }

    #[test]
    fn test_convert_header() {
        let visitor = PandocToNotionVisitor::new();

        // Create a simple header
        let header = PandocBlock::Header(
            2,
            Attr::default(),
            vec![Inline::Str("Test heading".to_string())],
        );

        // Convert using visitor
        let result = visitor.visit_block(&header).unwrap();

        // Should produce a single block
        assert_eq!(result.len(), 1);

        // Verify it's a heading with the correct level
        match &result[0].block_type {
            BlockType::Heading2 { heading_2 } => {
                assert_eq!(heading_2.rich_text.len(), 1);
                assert_eq!(heading_2.rich_text[0].plain_text().unwrap(), "Test heading");
            }
            _ => panic!("Expected Heading2 block type"),
        }
    }

    #[test]
    fn test_convert_plain() {
        let visitor = PandocToNotionVisitor::new();

        // Create a Plain block
        let plain = PandocBlock::Plain(vec![Inline::Str("Plain text".to_string())]);

        // Convert using visitor
        let result = visitor.visit_block(&plain).unwrap();

        // Should produce a single block
        assert_eq!(result.len(), 1);

        // Verify it's converted to a paragraph
        match &result[0].block_type {
            BlockType::Paragraph { paragraph } => {
                assert_eq!(paragraph.rich_text.len(), 1);
                assert_eq!(paragraph.rich_text[0].plain_text().unwrap(), "Plain text");
            }
            _ => panic!("Expected Paragraph block type"),
        }
    }

    #[test]
    fn test_convert_unsupported() {
        let visitor = PandocToNotionVisitor::new();

        // Create an unsupported block type (HorizontalRule)
        let unsupported_block = PandocBlock::HorizontalRule;

        // Convert using visitor
        let result = visitor.visit_block(&unsupported_block).unwrap();

        // Should produce a single block
        assert_eq!(result.len(), 1);

        // Verify it's converted to a paragraph with an unsupported message
        match &result[0].block_type {
            BlockType::Paragraph { paragraph } => {
                assert_eq!(paragraph.rich_text.len(), 1);
                let text = paragraph.rich_text[0].plain_text().unwrap();
                assert!(text.contains("Unsupported Pandoc block type"));
                assert!(text.contains("HorizontalRule"));
            }
            _ => panic!("Expected Paragraph block type"),
        }
    }

    #[test]
    fn test_convert_blocks() {
        let visitor = PandocToNotionVisitor::new();

        // Create a list of blocks
        let blocks = vec![
            PandocBlock::Header(
                1,
                Attr::default(),
                vec![Inline::Str("Document Title".to_string())],
            ),
            PandocBlock::Para(vec![Inline::Str("First paragraph".to_string())]),
            PandocBlock::Header(
                2,
                Attr::default(),
                vec![Inline::Str("Section Heading".to_string())],
            ),
            PandocBlock::Para(vec![Inline::Str("Second paragraph".to_string())]),
        ];

        // Convert all blocks
        let result = visitor.convert_blocks(&blocks).unwrap();

        // Should have 4 blocks
        assert_eq!(result.len(), 4);

        // Check each block type in order
        match &result[0].block_type {
            BlockType::Heading1 { heading_1 } => {
                assert_eq!(
                    heading_1.rich_text[0].plain_text().unwrap(),
                    "Document Title"
                );
            }
            _ => panic!("Expected Heading1 block type"),
        }

        match &result[1].block_type {
            BlockType::Paragraph { paragraph } => {
                assert_eq!(
                    paragraph.rich_text[0].plain_text().unwrap(),
                    "First paragraph"
                );
            }
            _ => panic!("Expected Paragraph block type"),
        }

        match &result[2].block_type {
            BlockType::Heading2 { heading_2 } => {
                assert_eq!(
                    heading_2.rich_text[0].plain_text().unwrap(),
                    "Section Heading"
                );
            }
            _ => panic!("Expected Heading2 block type"),
        }

        match &result[3].block_type {
            BlockType::Paragraph { paragraph } => {
                assert_eq!(
                    paragraph.rich_text[0].plain_text().unwrap(),
                    "Second paragraph"
                );
            }
            _ => panic!("Expected Paragraph block type"),
        }
    }

    #[test]
    fn test_convert_bullet_list() {
        let visitor = PandocToNotionVisitor::new();

        // Create a simple bullet list
        let item1 = vec![PandocBlock::Plain(vec![Inline::Str(
            "First bullet".to_string(),
        )])];
        let item2 = vec![PandocBlock::Plain(vec![Inline::Str(
            "Second bullet".to_string(),
        )])];
        let bullet_list = PandocBlock::BulletList(vec![item1, item2]);

        // Convert using visitor
        let result = visitor.visit_block(&bullet_list).unwrap();

        // Should produce two blocks (two bullet list items)
        assert_eq!(result.len(), 2);

        // Verify first bullet item
        match &result[0].block_type {
            BlockType::BulletedListItem { bulleted_list_item } => {
                assert_eq!(bulleted_list_item.rich_text.len(), 1);
                assert_eq!(
                    bulleted_list_item.rich_text[0].plain_text().unwrap(),
                    "First bullet"
                );
            }
            _ => panic!("Expected BulletedListItem block type"),
        }

        // Verify second bullet item
        match &result[1].block_type {
            BlockType::BulletedListItem { bulleted_list_item } => {
                assert_eq!(bulleted_list_item.rich_text.len(), 1);
                assert_eq!(
                    bulleted_list_item.rich_text[0].plain_text().unwrap(),
                    "Second bullet"
                );
            }
            _ => panic!("Expected BulletedListItem block type"),
        }
    }

    #[test]
    fn test_convert_ordered_list() {
        let visitor = PandocToNotionVisitor::new();

        // Create a simple ordered list
        let attrs = pandoc_types::definition::ListAttributes {
            start_number: 1,
            style: pandoc_types::definition::ListNumberStyle::Decimal,
            delim: pandoc_types::definition::ListNumberDelim::Period,
        };
        let item1 = vec![PandocBlock::Plain(vec![Inline::Str(
            "First numbered".to_string(),
        )])];
        let item2 = vec![PandocBlock::Plain(vec![Inline::Str(
            "Second numbered".to_string(),
        )])];
        let ordered_list = PandocBlock::OrderedList(attrs, vec![item1, item2]);

        // Convert using visitor
        let result = visitor.visit_block(&ordered_list).unwrap();

        // Should produce two blocks (two numbered list items)
        assert_eq!(result.len(), 2);

        // Verify first numbered item
        match &result[0].block_type {
            BlockType::NumberedListItem { numbered_list_item } => {
                assert_eq!(numbered_list_item.rich_text.len(), 1);
                assert_eq!(
                    numbered_list_item.rich_text[0].plain_text().unwrap(),
                    "First numbered"
                );
            }
            _ => panic!("Expected NumberedListItem block type"),
        }

        // Verify second numbered item
        match &result[1].block_type {
            BlockType::NumberedListItem { numbered_list_item } => {
                assert_eq!(numbered_list_item.rich_text.len(), 1);
                assert_eq!(
                    numbered_list_item.rich_text[0].plain_text().unwrap(),
                    "Second numbered"
                );
            }
            _ => panic!("Expected NumberedListItem block type"),
        }
    }

    #[test]
    fn test_convert_todo_list() {
        let visitor = PandocToNotionVisitor::new();

        // Create an unchecked todo item inside a bullet list
        let unchecked_item = vec![PandocBlock::Plain(vec![
            Inline::Str("☐".to_string()),
            Inline::Space,
            Inline::Str("Unchecked task".to_string()),
        ])];
        let unchecked_list = PandocBlock::BulletList(vec![unchecked_item]);

        // Convert using visitor
        let result = visitor.visit_block(&unchecked_list).unwrap();

        // Should produce one block
        assert_eq!(result.len(), 1);

        // Verify it's a todo block (unchecked)
        match &result[0].block_type {
            BlockType::ToDo { to_do } => {
                assert_eq!(to_do.rich_text.len(), 1);
                assert_eq!(to_do.rich_text[0].plain_text().unwrap(), "Unchecked task");
                assert_eq!(to_do.checked, Some(false));
            }
            _other => {
                panic!("Expected ToDo block type");
            }
        }

        // Create a checked todo item inside a bullet list
        let checked_item = vec![PandocBlock::Plain(vec![
            Inline::Str("☒".to_string()),
            Inline::Space,
            Inline::Str("Checked task".to_string()),
        ])];
        let checked_list = PandocBlock::BulletList(vec![checked_item]);

        // Convert using visitor
        let result = visitor.visit_block(&checked_list).unwrap();

        // Should produce one block
        assert_eq!(result.len(), 1);

        // Verify it's a todo block (checked)
        match &result[0].block_type {
            BlockType::ToDo { to_do } => {
                assert_eq!(to_do.rich_text.len(), 1);
                assert_eq!(to_do.rich_text[0].plain_text().unwrap(), "Checked task");
                assert_eq!(to_do.checked, Some(true));
            }
            _other => {
                panic!("Expected ToDo block type");
            }
        }
    }

    #[test]
    fn test_convert_nested_lists() {
        let visitor = PandocToNotionVisitor::new();

        // Create a nested list structure: ordered list with a bullet list as a sub-item
        let nested_item1 = vec![PandocBlock::Plain(vec![Inline::Str(
            "Nested bullet 1".to_string(),
        )])];
        let nested_item2 = vec![PandocBlock::Plain(vec![Inline::Str(
            "Nested bullet 2".to_string(),
        )])];
        let nested_bullet_list = PandocBlock::BulletList(vec![nested_item1, nested_item2]);

        // Parent ordered list with two items, the second containing the nested bullet list
        let ord_item1 = vec![PandocBlock::Plain(vec![Inline::Str(
            "Numbered item 1".to_string(),
        )])];
        let ord_item2 = vec![
            PandocBlock::Plain(vec![Inline::Str("Numbered item 2".to_string())]),
            nested_bullet_list,
        ];

        let attrs = pandoc_types::definition::ListAttributes {
            start_number: 1,
            style: pandoc_types::definition::ListNumberStyle::Decimal,
            delim: pandoc_types::definition::ListNumberDelim::Period,
        };
        let ordered_list = PandocBlock::OrderedList(attrs, vec![ord_item1, ord_item2]);

        // Convert using visitor
        let result = visitor.visit_block(&ordered_list).unwrap();

        // Should produce 2 top-level blocks (two numbered list items)
        assert_eq!(result.len(), 2);

        // Verify first numbered item
        match &result[0].block_type {
            BlockType::NumberedListItem { numbered_list_item } => {
                assert_eq!(numbered_list_item.rich_text.len(), 1);
                assert_eq!(
                    numbered_list_item.rich_text[0].plain_text().unwrap(),
                    "Numbered item 1"
                );
                assert!(
                    numbered_list_item.children.is_none()
                        || numbered_list_item.children.as_ref().unwrap().is_empty()
                );
            }
            _ => panic!("Expected NumberedListItem block type"),
        }

        // Verify second numbered item with children
        match &result[1].block_type {
            BlockType::NumberedListItem { numbered_list_item } => {
                assert_eq!(numbered_list_item.rich_text.len(), 1);
                assert_eq!(
                    numbered_list_item.rich_text[0].plain_text().unwrap(),
                    "Numbered item 2"
                );

                // Should have nested children
                assert!(numbered_list_item.children.is_some());
                let children = numbered_list_item.children.as_ref().unwrap();
                assert_eq!(children.len(), 2); // Two bullet items

                // Check first nested bullet
                match &children[0].block_type {
                    BlockType::BulletedListItem { bulleted_list_item } => {
                        assert_eq!(bulleted_list_item.rich_text.len(), 1);
                        assert_eq!(
                            bulleted_list_item.rich_text[0].plain_text().unwrap(),
                            "Nested bullet 1"
                        );
                    }
                    _ => panic!("Expected BulletedListItem for first child"),
                }

                // Check second nested bullet
                match &children[1].block_type {
                    BlockType::BulletedListItem { bulleted_list_item } => {
                        assert_eq!(bulleted_list_item.rich_text.len(), 1);
                        assert_eq!(
                            bulleted_list_item.rich_text[0].plain_text().unwrap(),
                            "Nested bullet 2"
                        );
                    }
                    _ => panic!("Expected BulletedListItem for second child"),
                }
            }
            _ => panic!("Expected NumberedListItem block type"),
        }
    }
}
