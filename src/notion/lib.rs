// Export modules
pub mod blocks;
pub mod code;
pub mod headings;
pub mod list;
pub mod paragraph;
pub mod quote;
pub mod text;

// Re-export commonly used types for convenience
pub use blocks::{Block, BlockColor, BlockContent};
pub use headings::{
    HeadingProperties, create_heading1, create_heading2, create_heading3,
    create_toggleable_heading1, create_toggleable_heading2, create_toggleable_heading3,
};
pub use list::{
    BulletedListItemProperties, NumberedListItemProperties, ToDoProperties,
    create_bulleted_list_item, create_bulleted_list_item_with_children, create_numbered_list_item,
    create_numbered_list_item_with_children, create_to_do, create_to_do_with_children,
};
pub use paragraph::{ParagraphProperties, create_paragraph, create_paragraph_with_children};
pub use quote::{QuoteProperties, create_quote, create_quote_with_children};
pub use code::{CodeLanguage, CodeProperties, create_code, create_code_with_caption, 
    create_code_with_children, create_code_with_caption_and_children};
pub use text::{Annotations, CloneTextObject, Color, Equation, Link, RichTextObject, TextContent};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
