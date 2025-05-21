/// Helper functions for testing Pandoc-related functionality
pub mod test {
    use pandoc_types::definition::Inline;

    /// Extracts text from Pandoc inline elements
    /// 
    /// Concatenates all string content with appropriate spaces, handling Pandoc's 
    /// representation where text is often split across multiple Inline elements.
    /// 
    /// # Example
    /// ```
    /// use pandoc_notion::test_utils::pandoc_helpers::test::extract_text_from_inlines;
    /// use pandoc_types::definition::Inline;
    /// 
    /// let inlines = vec![Inline::Str("Hello".to_string()), Inline::Space, Inline::Str("world".to_string())];
    /// assert_eq!(extract_text_from_inlines(&inlines), "Hello world");
    /// ```
    pub fn extract_text_from_inlines(inlines: &[Inline]) -> String {
        let mut text = String::new();
        for inline in inlines {
            match inline {
                Inline::Str(s) => text.push_str(s),
                Inline::Space => text.push(' '),
                _ => {}
            }
        }
        text.trim().to_string()
    }

    /// Verifies that the extracted text from Pandoc inlines matches the expected text
    pub fn assert_inlines_text_eq(inlines: &[Inline], expected: &str) {
        let actual = extract_text_from_inlines(inlines);
        assert_eq!(actual, expected, 
            "Text extracted from inlines ({:?}) doesn't match expected text", inlines);
    }
}