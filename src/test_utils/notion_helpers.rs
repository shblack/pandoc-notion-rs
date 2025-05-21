/// Helper functions for creating Notion blocks for testing
pub mod test {
    use notion_client::objects::block::{
        Block as NotionBlock, BlockType, BulletedListItemValue, HeadingsValue,
        NumberedListItemValue, ParagraphValue, QuoteValue, TextColor as BlockTextColor, ToDoValue,
    };
    use notion_client::objects::rich_text::{Annotations, RichText, Text, TextColor};

    /// Creates a simple text rich text element
    pub fn create_rich_text(content: &str) -> RichText {
        let text = Text {
            content: content.to_string(),
            link: None,
        };

        RichText::Text {
            text,
            annotations: None,
            plain_text: Some(content.to_string()),
            href: None,
        }
    }

    /// Creates a formatted rich text element with specified formatting
    pub fn create_formatted_rich_text(
        content: &str,
        bold: bool,
        italic: bool,
        strikethrough: bool,
        underline: bool,
        code: bool,
        color: Option<TextColor>,
    ) -> RichText {
        let text = Text {
            content: content.to_string(),
            link: None,
        };

        let annotations = Annotations {
            bold,
            italic,
            strikethrough,
            underline,
            code,
            color: color.unwrap_or(TextColor::Default),
        };

        RichText::Text {
            text,
            annotations: Some(annotations),
            plain_text: Some(content.to_string()),
            href: None,
        }
    }

    /// Creates a basic Notion block shell with the given ID
    fn create_block_shell(id: &str, has_children: bool) -> NotionBlock {
        NotionBlock {
            object: Some("block".to_string()),
            id: Some(id.to_string()),
            parent: None,
            created_time: None,
            last_edited_time: None,
            created_by: None,
            last_edited_by: None,
            has_children: Some(has_children),
            archived: Some(false),
            block_type: BlockType::Paragraph {
                paragraph: ParagraphValue {
                    rich_text: Vec::new(),
                    color: None,
                    children: None,
                },
            },
        }
    }

    /// Creates a heading block with the specified level and content
    pub fn create_heading_block(level: u8, content: &str) -> NotionBlock {
        let mut block = create_block_shell(&format!("heading_{}_id", level), false);

        block.block_type = match level {
            1 => BlockType::Heading1 {
                heading_1: HeadingsValue {
                    rich_text: vec![create_rich_text(content)],
                    color: None,
                    is_toggleable: Some(false),
                },
            },
            2 => BlockType::Heading2 {
                heading_2: HeadingsValue {
                    rich_text: vec![create_rich_text(content)],
                    color: None,
                    is_toggleable: Some(false),
                },
            },
            3 => BlockType::Heading3 {
                heading_3: HeadingsValue {
                    rich_text: vec![create_rich_text(content)],
                    color: None,
                    is_toggleable: Some(false),
                },
            },
            _ => panic!("Invalid heading level: {}", level),
        };

        block
    }

    /// Creates a heading with formatted text (bold, italic)
    pub fn create_heading_with_formatted_text(level: u8) -> NotionBlock {
        let mut block = create_block_shell(&format!("formatted_heading_{}_id", level), false);

        let rich_text = vec![
            create_rich_text("Formatted "),
            create_formatted_rich_text("bold", true, false, false, false, false, None),
            create_rich_text(" and "),
            create_formatted_rich_text("italic", false, true, false, false, false, None),
            create_rich_text(" heading"),
        ];

        block.block_type = match level {
            1 => BlockType::Heading1 {
                heading_1: HeadingsValue {
                    rich_text,
                    color: None,
                    is_toggleable: Some(false),
                },
            },
            2 => BlockType::Heading2 {
                heading_2: HeadingsValue {
                    rich_text,
                    color: None,
                    is_toggleable: Some(false),
                },
            },
            3 => BlockType::Heading3 {
                heading_3: HeadingsValue {
                    rich_text,
                    color: None,
                    is_toggleable: Some(false),
                },
            },
            _ => panic!("Invalid heading level: {}", level),
        };

        block
    }

    /// Creates a paragraph block with the specified content
    pub fn create_paragraph_block(content: &str, id: Option<&str>) -> NotionBlock {
        let id = id.unwrap_or("paragraph_id");
        let mut block = create_block_shell(id, false);

        block.block_type = BlockType::Paragraph {
            paragraph: ParagraphValue {
                rich_text: vec![create_rich_text(content)],
                color: None,
                children: None,
            },
        };

        block
    }

    /// Creates a heading with child blocks
    pub fn create_heading_with_children(level: u8) -> NotionBlock {
        let mut block = create_block_shell(&format!("heading_with_children_{}_id", level), true);

        // Note: We don't set children directly on HeadingsValue as it doesn't have that field
        // Instead, we'll need to handle children separately in the calling code

        block.block_type = match level {
            1 => BlockType::Heading1 {
                heading_1: HeadingsValue {
                    rich_text: vec![create_rich_text(&format!(
                        "Heading {} with children",
                        level
                    ))],
                    color: None,
                    is_toggleable: Some(true),
                },
            },
            2 => BlockType::Heading2 {
                heading_2: HeadingsValue {
                    rich_text: vec![create_rich_text(&format!(
                        "Heading {} with children",
                        level
                    ))],
                    color: None,
                    is_toggleable: Some(true),
                },
            },
            3 => BlockType::Heading3 {
                heading_3: HeadingsValue {
                    rich_text: vec![create_rich_text(&format!(
                        "Heading {} with children",
                        level
                    ))],
                    color: None,
                    is_toggleable: Some(true),
                },
            },
            _ => panic!("Invalid heading level: {}", level),
        };

        // Note: This doesn't actually attach the children to the block
        // The caller would need to handle this based on the actual API

        block
    }

    /// Creates a toggleable heading with explicit child blocks for testing
    /// Returns a vector where the first element is the heading and the rest are its children
    pub fn create_heading_with_child_blocks(
        level: u8,
        heading_content: &str,
        child_blocks: Vec<NotionBlock>,
    ) -> Vec<NotionBlock> {
        let mut blocks = Vec::new();
        
        // Create the heading block (toggleable)
        let mut heading = create_block_shell(&format!("heading_with_children_{}_id", level), true);
        
        heading.block_type = match level {
            1 => BlockType::Heading1 {
                heading_1: HeadingsValue {
                    rich_text: vec![create_rich_text(heading_content)],
                    color: None,
                    is_toggleable: Some(true),
                },
            },
            2 => BlockType::Heading2 {
                heading_2: HeadingsValue {
                    rich_text: vec![create_rich_text(heading_content)],
                    color: None,
                    is_toggleable: Some(true),
                },
            },
            3 => BlockType::Heading3 {
                heading_3: HeadingsValue {
                    rich_text: vec![create_rich_text(heading_content)],
                    color: None,
                    is_toggleable: Some(true),
                },
            },
            _ => panic!("Invalid heading level: {}", level),
        };
        
        // Add the heading and its children to the result
        blocks.push(heading);
        blocks.extend(child_blocks);
        
        blocks
    }

    /// Creates an empty heading of the specified level
    pub fn create_empty_heading(level: u8) -> NotionBlock {
        let mut block = create_block_shell(&format!("empty_heading_{}_id", level), false);

        block.block_type = match level {
            1 => BlockType::Heading1 {
                heading_1: HeadingsValue {
                    rich_text: vec![],
                    color: None,
                    is_toggleable: Some(false),
                },
            },
            2 => BlockType::Heading2 {
                heading_2: HeadingsValue {
                    rich_text: vec![],
                    color: None,
                    is_toggleable: Some(false),
                },
            },
            3 => BlockType::Heading3 {
                heading_3: HeadingsValue {
                    rich_text: vec![],
                    color: None,
                    is_toggleable: Some(false),
                },
            },
            _ => panic!("Invalid heading level: {}", level),
        };

        block
    }

    /// Creates a bulleted list item block
    pub fn create_bulleted_list_item(
        content: &str,
        color: Option<BlockTextColor>,
        children: Option<Vec<NotionBlock>>,
    ) -> NotionBlock {
        let mut block =
            create_block_shell(&format!("bulleted-list-{}", content), children.is_some());

        block.block_type = BlockType::BulletedListItem {
            bulleted_list_item: BulletedListItemValue {
                rich_text: vec![create_rich_text(content)],
                color: color.unwrap_or(BlockTextColor::Default),
                children,
            },
        };

        block
    }

    /// Creates a numbered list item block
    pub fn create_numbered_list_item(
        content: &str,
        color: Option<BlockTextColor>,
        children: Option<Vec<NotionBlock>>,
    ) -> NotionBlock {
        let mut block =
            create_block_shell(&format!("numbered-list-{}", content), children.is_some());

        block.block_type = BlockType::NumberedListItem {
            numbered_list_item: NumberedListItemValue {
                rich_text: vec![create_rich_text(content)],
                color: color.unwrap_or(BlockTextColor::Default),
                children,
            },
        };

        block
    }

    /// Creates a to-do list item block
    pub fn create_todo_item(
        content: &str,
        checked: bool,
        children: Option<Vec<NotionBlock>>,
    ) -> NotionBlock {
        let mut block = create_block_shell(&format!("todo-list-{}", content), children.is_some());

        block.block_type = BlockType::ToDo {
            to_do: ToDoValue {
                rich_text: vec![create_rich_text(content)],
                checked: Some(checked),
                color: None,
                children,
            },
        };

        block
    }

    /// Creates a quote block
    pub fn create_quote_block(content: &str, children: Option<Vec<NotionBlock>>) -> NotionBlock {
        let mut block = create_block_shell(&format!("quote-{}", content), children.is_some());

        block.block_type = BlockType::Quote {
            quote: QuoteValue {
                rich_text: vec![create_rich_text(content)],
                color: BlockTextColor::Default,
                children,
            },
        };

        block
    }

    /// Helper function to print a Notion block for debugging
    pub fn print_notion_block(block: &NotionBlock) {
        println!("  Block ID: {:?}", block.id);
        println!("  Has children: {:?}", block.has_children);

        match &block.block_type {
            BlockType::Heading1 { heading_1 } => {
                println!("  Type: Heading1");
                println!(
                    "  Text: {}",
                    heading_1
                        .rich_text
                        .iter()
                        .map(|rt| match rt {
                            RichText::Text { plain_text, .. } =>
                                plain_text.as_deref().unwrap_or(""),
                            _ => "",
                        })
                        .collect::<String>()
                );
                println!("  Is toggleable: {:?}", heading_1.is_toggleable);

                // Children would be handled separately
                if block.has_children == Some(true) {
                    println!("  Has children: true");
                }
            }
            BlockType::Heading2 { heading_2 } => {
                println!("  Type: Heading2");
                println!(
                    "  Text: {}",
                    heading_2
                        .rich_text
                        .iter()
                        .map(|rt| match rt {
                            RichText::Text { plain_text, .. } =>
                                plain_text.as_deref().unwrap_or(""),
                            _ => "",
                        })
                        .collect::<String>()
                );
                println!("  Is toggleable: {:?}", heading_2.is_toggleable);

                // Children would be handled separately
                if block.has_children == Some(true) {
                    println!("  Has children: true");
                }
            }
            BlockType::Heading3 { heading_3 } => {
                println!("  Type: Heading3");
                println!(
                    "  Text: {}",
                    heading_3
                        .rich_text
                        .iter()
                        .map(|rt| match rt {
                            RichText::Text { plain_text, .. } =>
                                plain_text.as_deref().unwrap_or(""),
                            _ => "",
                        })
                        .collect::<String>()
                );
                println!("  Is toggleable: {:?}", heading_3.is_toggleable);

                // Children would be handled separately
                if block.has_children == Some(true) {
                    println!("  Has children: true");
                }
            }
            BlockType::Paragraph { paragraph } => {
                println!("  Type: Paragraph");
                println!(
                    "  Text: {}",
                    paragraph
                        .rich_text
                        .iter()
                        .map(|rt| match rt {
                            RichText::Text { plain_text, .. } =>
                                plain_text.as_deref().unwrap_or(""),
                            _ => "",
                        })
                        .collect::<String>()
                );

                if let Some(children) = &paragraph.children {
                    println!("  Children count: {}", children.len());
                    for (i, child) in children.iter().enumerate() {
                        println!("  Child {}:", i);
                        print_notion_block(child);
                    }
                }
            }
            _ => println!("  Type: Other ({:?})", block.block_type),
        }
    }
}
