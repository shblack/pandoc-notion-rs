use crate::notion::toggleable::{ToggleableBlock, ToggleableBlockChildren};
use crate::n2p::notion_block_visitor::NotionToPandocVisitor;
use crate::n2p::ConversionConfig;
use crate::test_utils::notion_helpers::test as notion_test;
use notion_client::objects::block::{Block as NotionBlock, BlockType};
use pandoc_types::definition::{Block as PandocBlock};

#[cfg(test)]
mod tests {
    use super::*;
    
    // Helper function to create a test heading block with custom ID
    fn create_test_heading(level: u8, content: &str, is_toggleable: bool, has_children: bool) -> NotionBlock {
        let mut heading = if is_toggleable {
            notion_test::create_heading_with_children(level)
        } else {
            notion_test::create_heading_block(level, content)
        };
        
        // Override ID and has_children for test purposes
        heading.id = Some(format!("heading-{}-{}", level, if is_toggleable { "toggle" } else { "normal" }));
        heading.has_children = Some(has_children);
        
        heading
    }
    
    // Helper function to create a test paragraph block as a child
    fn create_test_paragraph(content: &str) -> NotionBlock {
        let mut paragraph = notion_test::create_paragraph_block(content, None);
        
        // Override ID for test purposes
        paragraph.id = Some(format!("paragraph-{}", content.replace(" ", "-")));
        
        paragraph
    }
    
    #[test]
    fn test_toggleable_block_trait() {
        // Non-toggleable heading
        let normal_heading = create_test_heading(1, "Normal Heading", false, false);
        assert!(!normal_heading.is_toggleable());
        assert_eq!(normal_heading.block_id(), Some("heading-1-normal"));
        assert!(!normal_heading.has_children());
        
        // Toggleable heading with children
        let toggleable_heading = create_test_heading(2, "Toggleable Heading", true, true);
        assert!(toggleable_heading.is_toggleable());
        assert_eq!(toggleable_heading.block_id(), Some("heading-2-toggle"));
        assert!(toggleable_heading.has_children());
    }
    
    #[test]
    fn test_toggleable_children_manager() {
        let mut manager = ToggleableBlockChildren::new();
        
        // Create a toggleable heading with children
        let heading = create_test_heading(1, "Toggleable Heading", true, true);
        let child1 = create_test_paragraph("Child paragraph 1");
        let child2 = create_test_paragraph("Child paragraph 2");
        
        // Add children to the manager
        let children = vec![child1, child2];
        let success = manager.add_children(&heading, children.clone());
        
        // Verify that the children were added
        assert!(success, "Children should be added successfully");
        assert!(manager.has_children_for(&heading));
        
        // Retrieve the children
        let retrieved_children = manager.get_children(&heading);
        assert!(retrieved_children.is_some());
        let retrieved_children = retrieved_children.unwrap();
        assert_eq!(retrieved_children.len(), 2);
        
        // Verify the content of the retrieved children
        match &retrieved_children[0].block_type {
            BlockType::Paragraph { paragraph } => {
                match &paragraph.rich_text[0] {
                    notion_client::objects::rich_text::RichText::Text { plain_text, .. } => {
                        assert_eq!(plain_text.as_ref().unwrap(), "Child paragraph 1");
                    },
                    _ => panic!("Expected Text rich text type"),
                }
            },
            _ => panic!("Expected paragraph block"),
        }
        
        // Non-toggleable heading
        let normal_heading = create_test_heading(1, "Normal Heading", false, false);
        assert!(!manager.add_children(&normal_heading, vec![create_test_paragraph("This won't be added")]));
        assert!(!manager.has_children_for(&normal_heading));
    }
    
    #[test]
    fn test_visitor_with_toggleable_headings() {
        // Create a toggleable heading with children
        let heading = create_test_heading(1, "Toggleable Heading", true, true);
        let child1 = create_test_paragraph("Child paragraph 1");
        let child2 = create_test_paragraph("Child paragraph 2");
        
        let _heading_id = heading.id.clone().unwrap();
        
        // Create manager and add children
        let mut manager = ToggleableBlockChildren::new();
        manager.add_children(&heading, vec![child1, child2]);
        
        // Create visitor with toggleable children support
        let visitor = NotionToPandocVisitor::with_toggleable_children(
            ConversionConfig::default(),
            manager
        );
        
        // Convert the blocks
        let result = visitor.convert_blocks(&[heading]);
        
        // Should produce a heading followed by two paragraphs
        assert_eq!(result.len(), 3, "Should have 3 blocks: 1 heading + 2 paragraphs");
        
        // First block should be a heading
        match &result[0] {
            PandocBlock::Header(level, _, _) => {
                assert_eq!(*level, 1, "Should be a level 1 heading");
            },
            _ => panic!("Expected Header block, got {:?}", result[0]),
        }
        
        // Second and third blocks should be paragraphs
        match &result[1] {
            PandocBlock::Para(_) => {},
            _ => panic!("Expected Para block, got {:?}", result[1]),
        }
        
        match &result[2] {
            PandocBlock::Para(_) => {},
            _ => panic!("Expected Para block, got {:?}", result[2]),
        }
    }
    
    #[test]
    fn test_nested_toggleable_headings() {
        // Create a top-level toggleable heading
        let top_heading = create_test_heading(1, "Top Heading", true, true);
        let _top_heading_id = top_heading.id.clone().unwrap();
        
        // Create a nested toggleable heading as a child of the top heading
        let nested_heading = create_test_heading(2, "Nested Heading", true, true);
        let _nested_heading_id = nested_heading.id.clone().unwrap();
        
        // Create a paragraph as a child of the nested heading
        let paragraph = create_test_paragraph("Deeply nested paragraph");
        
        // Create manager and add the hierarchy
        let mut manager = ToggleableBlockChildren::new();
        manager.add_children(&top_heading, vec![nested_heading.clone()]);
        manager.add_children(&nested_heading, vec![paragraph]);
        
        // Create visitor with toggleable children support
        let visitor = NotionToPandocVisitor::with_toggleable_children(
            ConversionConfig::default(),
            manager
        );
        
        // Convert the blocks
        let result = visitor.convert_blocks(&[top_heading]);
        
        // Should produce: 
        // 1. Top heading
        // 2. Nested heading
        // 3. Paragraph
        assert_eq!(result.len(), 3, "Should have 3 blocks in total");
        
        // First block should be the top heading
        match &result[0] {
            PandocBlock::Header(level, _, _) => {
                assert_eq!(*level, 1, "Should be a level 1 heading");
            },
            _ => panic!("Expected Header block, got {:?}", result[0]),
        }
        
        // Second block should be the nested heading
        match &result[1] {
            PandocBlock::Header(level, _, _) => {
                assert_eq!(*level, 2, "Should be a level 2 heading");
            },
            _ => panic!("Expected Header block, got {:?}", result[1]),
        }
        
        // Third block should be the paragraph
        match &result[2] {
            PandocBlock::Para(_) => {},
            _ => panic!("Expected Para block, got {:?}", result[2]),
        }
    }
}