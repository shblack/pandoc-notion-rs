use crate::p2n::pandoc_heading::PandocHeadingConverter;
use crate::p2n::pandoc_paragraph::PandocParagraphConverter;
use crate::p2n::visitor::PandocBlockVisitor;
use notion_client::objects::block::{Block as NotionBlock, BlockType, ParagraphValue};
use notion_client::objects::rich_text::RichText;
use pandoc_types::definition::{Attr, Block as PandocBlock, Inline};
use std::error::Error;

/// Concrete implementation of the PandocBlockVisitor for converting Pandoc blocks to Notion
pub struct PandocToNotionVisitor {
    paragraph_converter: PandocParagraphConverter,
    heading_converter: PandocHeadingConverter,
}

impl PandocToNotionVisitor {
    /// Create a new visitor
    pub fn new() -> Self {
        Self {
            paragraph_converter: PandocParagraphConverter::new(),
            heading_converter: PandocHeadingConverter::new(),
        }
    }

    /// Convert a list of Pandoc blocks to Notion blocks
    pub fn convert_blocks(
        &self,
        blocks: &[PandocBlock],
        parent_id: Option<String>,
    ) -> Result<Vec<NotionBlock>, Box<dyn Error>> {
        self.process_blocks(blocks, parent_id)
    }
}

impl Default for PandocToNotionVisitor {
    fn default() -> Self {
        Self::new()
    }
}

impl PandocBlockVisitor for PandocToNotionVisitor {
    fn visit_block(
        &self,
        block: &PandocBlock,
        parent_id: Option<String>,
    ) -> Result<Vec<NotionBlock>, Box<dyn Error>> {
        // Dispatch to specific visitor method based on block type
        match block {
            PandocBlock::Para(inlines) => self.visit_paragraph(inlines, parent_id),
            PandocBlock::Plain(inlines) => self.visit_plain(inlines, parent_id),
            PandocBlock::Header(level, attr, inlines) => {
                self.visit_header(*level, attr, inlines, parent_id)
            }
            _ => self.visit_unsupported(block, parent_id),
        }
    }

    fn visit_paragraph(
        &self,
        inlines: &[Inline],
        parent_id: Option<String>,
    ) -> Result<Vec<NotionBlock>, Box<dyn Error>> {
        // Use the existing paragraph converter
        let para_block = PandocBlock::Para(inlines.to_vec());
        match self.paragraph_converter.convert(&para_block, parent_id)? {
            Some(block) => Ok(vec![block]),
            None => Ok(vec![]),
        }
    }

    fn visit_header(
        &self,
        level: i32,
        attr: &Attr,
        inlines: &[Inline],
        parent_id: Option<String>,
    ) -> Result<Vec<NotionBlock>, Box<dyn Error>> {
        // Use the existing header converter
        let header_block = PandocBlock::Header(level, attr.clone(), inlines.to_vec());
        match self.heading_converter.convert(&header_block, parent_id)? {
            Some(block) => Ok(vec![block]),
            None => Ok(vec![]),
        }
    }

    fn visit_plain(
        &self,
        inlines: &[Inline],
        parent_id: Option<String>,
    ) -> Result<Vec<NotionBlock>, Box<dyn Error>> {
        // Convert Plain to Paragraph by default
        // Plain is essentially the same as Para but used in different contexts
        self.visit_paragraph(inlines, parent_id)
    }

    fn visit_unsupported(
        &self,
        block: &PandocBlock,
        parent_id: Option<String>,
    ) -> Result<Vec<NotionBlock>, Box<dyn Error>> {
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

        // Create parent if specified
        let parent = parent_id.map(|id| {
            use notion_client::objects::parent::Parent;
            Parent::PageId { page_id: id }
        });

        let notion_block = NotionBlock {
            object: Some("block".to_string()),
            id: Some(String::new()),
            parent,
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

    fn process_blocks(
        &self,
        blocks: &[PandocBlock],
        parent_id: Option<String>,
    ) -> Result<Vec<NotionBlock>, Box<dyn Error>> {
        let mut result = Vec::new();

        for block in blocks {
            let converted = self.visit_block(block, parent_id.clone())?;
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
        let result = visitor.visit_block(&paragraph, None).unwrap();

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
        let result = visitor.visit_block(&header, None).unwrap();

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
        let result = visitor.visit_block(&plain, None).unwrap();

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

        // Create an unsupported block type (Code block)
        let code_block = PandocBlock::CodeBlock(
            Attr::default(),
            "function test() { return true; }".to_string(),
        );

        // Convert using visitor
        let result = visitor.visit_block(&code_block, None).unwrap();

        // Should produce a single block
        assert_eq!(result.len(), 1);

        // Verify it's converted to a paragraph with an unsupported message
        match &result[0].block_type {
            BlockType::Paragraph { paragraph } => {
                assert_eq!(paragraph.rich_text.len(), 1);
                let text = paragraph.rich_text[0].plain_text().unwrap();
                assert!(text.contains("Unsupported Pandoc block type"));
                assert!(text.contains("CodeBlock"));
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
        let result = visitor.convert_blocks(&blocks, None).unwrap();

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
}
