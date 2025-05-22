// Import the fixtures module
#[path = "fixtures/roundtrip_utils.rs"]
mod roundtrip_utils;

// Use the imported module
use roundtrip_utils::run_markdown_roundtrip;

#[test]
fn test_two_level_nested_list() {
    // Test a simple two-level nested list
    let markdown = r#"
- First level item
  - Second level item
- Another first level item
"#;

    let result = run_markdown_roundtrip(markdown)
        .expect("Roundtrip conversion should succeed");

    result.print_report();
    assert!(
        result.is_identical(),
        "Two-level nested list should roundtrip without changes"
    );
}

#[test]
fn test_three_level_nested_list() {
    // Test a three-level nested list
    let markdown = r#"
- First level
  - Second level
    - Third level
- Another first level
"#;

    let result = run_markdown_roundtrip(markdown)
        .expect("Roundtrip conversion should succeed");

    result.print_report();
    assert!(
        result.is_identical(),
        "Three-level nested list should roundtrip without changes"
    );
}

#[test]
fn test_mixed_list_types() {
    // Test mixed bullet and numbered lists
    let markdown = r#"
- Bullet item
  1. Numbered subitem
     - Bullet sub-subitem
  2. Another numbered subitem
- Another bullet item
"#;

    let result = run_markdown_roundtrip(markdown)
        .expect("Roundtrip conversion should succeed");

    result.print_report();
    assert!(
        result.is_identical(),
        "Mixed list types should roundtrip without changes"
    );
}

#[test]
fn test_two_level_blockquote() {
    // Test a two-level nested blockquote
    let markdown = r#"
> First level quote
>
> > Second level quote
>
> Back to first level
"#;

    let result = run_markdown_roundtrip(markdown)
        .expect("Roundtrip conversion should succeed");

    result.print_report();
    assert!(
        result.is_identical(),
        "Two-level blockquote should roundtrip without changes"
    );
}

#[test]
fn test_three_level_blockquote() {
    // Test a three-level nested blockquote
    let markdown = r#"
> First level
>
> > Second level
> >
> > > Third level
> >
> > Back to second
>
> Back to first
"#;

    let result = run_markdown_roundtrip(markdown)
        .expect("Roundtrip conversion should succeed");

    result.print_report();
    assert!(
        result.is_identical(),
        "Three-level blockquote should roundtrip without changes"
    );
}

#[test]
fn test_numbered_list_with_nested_bullet_and_todo() {
    // Test a numbered list with nested bullet and to-do lists
    let markdown = r#"
1. First numbered item
   - Nested bullet
   - [ ] Nested unchecked to-do
   - [x] Nested checked to-do
2. Second numbered item
   - [ ] Another to-do
     - Nested bullet inside to-do
"#;

    let result = run_markdown_roundtrip(markdown)
        .expect("Roundtrip conversion should succeed");

    result.print_report();
    assert!(
        result.is_identical(),
        "Numbered list with nested bullet and to-do should roundtrip without changes"
    );
}

#[test]
fn test_bullet_list_with_nested_numbered_and_todo() {
    // Test a bullet list with nested numbered and to-do lists
    let markdown = r#"
- First bullet item
  1. Nested numbered
  2. Another numbered
     - [ ] Unchecked to-do
     - [x] Checked to-do
- Second bullet item
  - [ ] Nested to-do
    1. Numbered inside to-do
"#;

    let result = run_markdown_roundtrip(markdown)
        .expect("Roundtrip conversion should succeed");

    result.print_report();
    assert!(
        result.is_identical(),
        "Bullet list with nested numbered and to-do should roundtrip without changes"
    );
}

#[test]
fn test_todo_list_with_nested_bullet_and_numbered() {
    // Test a to-do list with nested bullet and numbered lists
    let markdown = r#"
- [ ] First unchecked to-do
  - Nested bullet
  - Another bullet
    1. Numbered inside bullet
- [x] Checked to-do
  1. Numbered inside to-do
     - Bullet inside numbered
"#;

    let result = run_markdown_roundtrip(markdown)
        .expect("Roundtrip conversion should succeed");

    result.print_report();
    assert!(
        result.is_identical(),
        "To-do list with nested bullet and numbered should roundtrip without changes"
    );
}
