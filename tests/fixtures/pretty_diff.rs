//! Pretty diff utilities for test output
//!
//! This module provides formatted diff utilities for comparing strings
//! with human-readable, colorized output.

#![allow(dead_code, unused_imports, unused_variables)]

use similar::{ChangeTag, TextDiff};
use std::fmt::Write;

// ANSI color codes for prettier console output
const RESET: &str = "\x1b[0m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
#[allow(dead_code)]
const BLUE: &str = "\x1b[34m";
#[allow(dead_code)]
const YELLOW: &str = "\x1b[33m";
#[allow(dead_code)]
const BOLD: &str = "\x1b[1m";
#[allow(dead_code)]
const ITALIC: &str = "\x1b[3m";
const BG_RED: &str = "\x1b[41m";
const BG_GREEN: &str = "\x1b[42m";

/// Color formatting options for diff output
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ColorFormat {
    /// No color formatting (plain text)
    None,
    /// ANSI color formatting for terminal output
    Ansi,
    /// HTML color formatting
    Html,
}

impl Default for ColorFormat {
    fn default() -> Self {
        ColorFormat::Ansi
    }
}

/// Diff display style
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum DiffStyle {
    /// Unified diff (line-by-line with +/- prefixes)
    Unified,
    /// Side-by-side diff showing both versions in parallel columns
    SideBySide,
    /// Character-level diff highlighting individual character changes
    Character,
    /// Summary only showing statistics about changes
    Summary,
}

impl Default for DiffStyle {
    fn default() -> Self {
        DiffStyle::Unified
    }
}

/// Configuration for diff display
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DiffConfig {
    /// Color formatting
    pub color: ColorFormat,
    /// Diff display style
    pub style: DiffStyle,
    /// Context lines to show around changes (for unified diffs)
    pub context_lines: usize,
    /// Terminal width (for side-by-side diffs)
    pub width: usize,
}

impl Default for DiffConfig {
    fn default() -> Self {
        Self {
            color: ColorFormat::default(),
            style: DiffStyle::default(),
            context_lines: 3,
            width: 120,
        }
    }
}

/// Generate a diff between two strings
pub fn diff_strings(left: &str, right: &str, config: &DiffConfig) -> String {
    match config.style {
        DiffStyle::Unified => unified_diff(left, right, config),
        DiffStyle::SideBySide => side_by_side_diff(left, right, config),
        DiffStyle::Character => character_diff(left, right, config),
        DiffStyle::Summary => summary_diff(left, right, config),
    }
}

/// Format a unified diff (like git diff but more readable)
fn unified_diff(left: &str, right: &str, config: &DiffConfig) -> String {
    let diff = TextDiff::from_lines(left, right);
    let mut result = String::new();

    for change in diff.iter_all_changes() {
        match change.tag() {
            ChangeTag::Equal => {
                writeln!(result, " {}", change.value()).unwrap();
            }
            ChangeTag::Delete => match config.color {
                ColorFormat::None => {
                    writeln!(result, "-{}", change.value()).unwrap();
                }
                ColorFormat::Ansi => {
                    writeln!(result, "{}-{}{}", RED, change.value(), RESET).unwrap();
                }
                ColorFormat::Html => {
                    writeln!(
                        result,
                        "<span style=\"color: red\">-{}</span>",
                        change.value()
                    )
                    .unwrap();
                }
            },
            ChangeTag::Insert => match config.color {
                ColorFormat::None => {
                    writeln!(result, "+{}", change.value()).unwrap();
                }
                ColorFormat::Ansi => {
                    writeln!(result, "{}+{}{}", GREEN, change.value(), RESET).unwrap();
                }
                ColorFormat::Html => {
                    writeln!(
                        result,
                        "<span style=\"color: green\">+{}</span>",
                        change.value()
                    )
                    .unwrap();
                }
            },
        }
    }

    result
}

/// Format a side-by-side diff with colorization
fn side_by_side_diff(left: &str, right: &str, config: &DiffConfig) -> String {
    let diff = TextDiff::from_lines(left, right);
    let mut result = String::new();

    // Calculate column width
    let half_width = (config.width - 3) / 2; // 3 characters for the separator

    // Helper function to format a line to fit in the column
    let format_line = |line: &str, width: usize| -> String {
        let trimmed = line.trim_end();
        if trimmed.chars().count() > width {
            let truncated: String = trimmed.chars().take(width).collect();
            truncated
        } else {
            format!("{:<width$}", trimmed, width = width)
        }
    };

    // Header with separator line
    writeln!(
        result,
        "{:─<width$} │ {:─<width$}",
        "",
        "",
        width = half_width
    )
    .unwrap();

    // Group changes to align corresponding lines
    let mut left_lines = Vec::new();
    let mut right_lines = Vec::new();

    for change in diff.iter_all_changes() {
        match change.tag() {
            ChangeTag::Equal => {
                // Output any accumulated changes
                print_side_by_side_lines(
                    &mut result,
                    &left_lines,
                    &right_lines,
                    half_width,
                    config,
                );
                left_lines.clear();
                right_lines.clear();

                // Print the equal line
                let formatted = format_line(change.value(), half_width);
                match config.color {
                    ColorFormat::None => {
                        writeln!(result, "{} │ {}", formatted, formatted).unwrap();
                    }
                    ColorFormat::Ansi => {
                        writeln!(result, "{} │ {}", formatted, formatted).unwrap();
                    }
                    ColorFormat::Html => {
                        writeln!(
                            result,
                            "<div><span>{}</span> │ <span>{}</span></div>",
                            formatted, formatted
                        )
                        .unwrap();
                    }
                }
            }
            ChangeTag::Delete => {
                left_lines.push(change.value().to_string());
            }
            ChangeTag::Insert => {
                right_lines.push(change.value().to_string());
            }
        }
    }

    // Handle any remaining changes
    print_side_by_side_lines(&mut result, &left_lines, &right_lines, half_width, config);

    result
}

/// Helper function to print side-by-side lines with proper colorization
fn print_side_by_side_lines(
    result: &mut String,
    left_lines: &[String],
    right_lines: &[String],
    half_width: usize,
    config: &DiffConfig,
) {
    let max_lines = std::cmp::max(left_lines.len(), right_lines.len());

    for i in 0..max_lines {
        let left = if i < left_lines.len() {
            format_line(&left_lines[i], half_width)
        } else {
            " ".repeat(half_width)
        };

        let right = if i < right_lines.len() {
            format_line(&right_lines[i], half_width)
        } else {
            " ".repeat(half_width)
        };

        match config.color {
            ColorFormat::None => {
                writeln!(result, "{} │ {}", left, right).unwrap();
            }
            ColorFormat::Ansi => {
                if i < left_lines.len() {
                    write!(result, "{}{}{}", RED, left, RESET).unwrap();
                } else {
                    write!(result, "{}", left).unwrap();
                }

                write!(result, " │ ").unwrap();

                if i < right_lines.len() {
                    writeln!(result, "{}{}{}", GREEN, right, RESET).unwrap();
                } else {
                    writeln!(result, "{}", right).unwrap();
                }
            }
            ColorFormat::Html => {
                if i < left_lines.len() {
                    write!(result, "<span style=\"color: red\">{}</span>", left).unwrap();
                } else {
                    write!(result, "{}", left).unwrap();
                }

                write!(result, " │ ").unwrap();

                if i < right_lines.len() {
                    writeln!(result, "<span style=\"color: green\">{}</span>", right).unwrap();
                } else {
                    writeln!(result, "{}", right).unwrap();
                }
            }
        }
    }
}

/// Format a line to fit in a column
fn format_line(line: &str, width: usize) -> String {
    let trimmed = line.trim_end();
    if trimmed.chars().count() > width {
        let truncated: String = trimmed.chars().take(width).collect();
        truncated
    } else {
        format!("{:<width$}", trimmed, width = width)
    }
}

/// Format a character-level diff highlighting individual character changes
fn character_diff(left: &str, right: &str, config: &DiffConfig) -> String {
    let diff = TextDiff::from_chars(left, right);
    let mut result = String::new();

    for change in diff.iter_all_changes() {
        match change.tag() {
            ChangeTag::Equal => {
                write!(result, "{}", change.value()).unwrap();
            }
            ChangeTag::Delete => match config.color {
                ColorFormat::None => {
                    write!(result, "[-{}]", change.value()).unwrap();
                }
                ColorFormat::Ansi => {
                    write!(result, "{}{}{}", BG_RED, change.value(), RESET).unwrap();
                }
                ColorFormat::Html => {
                    write!(
                        result,
                        "<span style=\"background-color: #ffcccc\">{}</span>",
                        change.value()
                    )
                    .unwrap();
                }
            },
            ChangeTag::Insert => match config.color {
                ColorFormat::None => {
                    write!(result, "[+{}]", change.value()).unwrap();
                }
                ColorFormat::Ansi => {
                    write!(result, "{}{}{}", BG_GREEN, change.value(), RESET).unwrap();
                }
                ColorFormat::Html => {
                    write!(
                        result,
                        "<span style=\"background-color: #ccffcc\">{}</span>",
                        change.value()
                    )
                    .unwrap();
                }
            },
        }
    }

    result
}

/// Format a summary diff showing only statistics

fn summary_diff(left: &str, right: &str, config: &DiffConfig) -> String {
    let diff = TextDiff::from_lines(left, right);

    let mut equal_count = 0;
    let mut added_count = 0;
    let mut removed_count = 0;

    for change in diff.iter_all_changes() {
        match change.tag() {
            ChangeTag::Equal => equal_count += 1,
            ChangeTag::Delete => removed_count += 1,
            ChangeTag::Insert => added_count += 1,
        }
    }

    let total = equal_count + added_count + removed_count;
    let changed = added_count + removed_count;
    let change_percentage = if total > 0 {
        (changed as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    let mut result = String::new();
    writeln!(result, "Diff Summary:").unwrap();
    writeln!(result, "  Total lines: {}", total).unwrap();
    writeln!(result, "  Added lines: {}", added_count).unwrap();
    writeln!(result, "  Removed lines: {}", removed_count).unwrap();
    writeln!(result, "  Unchanged lines: {}", equal_count).unwrap();
    writeln!(result, "  Change percentage: {:.2}%", change_percentage).unwrap();

    result
}

/// Print a diff between two strings to stdout
pub fn print_diff(left: &str, right: &str) {
    let config = DiffConfig::default();
    let diff = diff_strings(left, right, &config);
    print!("{}", diff);
}

/// Print a unified diff to stdout
pub fn print_unified_diff(left: &str, right: &str) {
    let mut config = DiffConfig::default();
    config.style = DiffStyle::Unified;
    let diff = diff_strings(left, right, &config);
    print!("{}", diff);
}

/// Print a side-by-side diff to stdout
pub fn print_side_by_side_diff(left: &str, right: &str) {
    let mut config = DiffConfig::default();
    config.style = DiffStyle::SideBySide;
    let diff = diff_strings(left, right, &config);
    print!("{}", diff);
}

/// Print a character-level diff to stdout
pub fn print_char_diff(left: &str, right: &str) {
    let mut config = DiffConfig::default();
    config.style = DiffStyle::Character;
    let diff = diff_strings(left, right, &config);
    print!("{}", diff);
}

/// Print a diff summary to stdout
pub fn print_diff_summary(left: &str, right: &str) {
    let mut config = DiffConfig::default();
    config.style = DiffStyle::Summary;
    let diff = diff_strings(left, right, &config);
    print!("{}", diff);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_diff() {
        let left = "line1\nline2\nline3\n";
        let right = "line1\nmodified line2\nline3\n";

        let mut config = DiffConfig::default();
        config.style = DiffStyle::Unified;

        let diff = diff_strings(left, right, &config);
        assert!(diff.contains("line1"));
        assert!(diff.contains("modified line2"));
    }

    #[test]
    fn test_side_by_side_diff() {
        let left = "line1\nline2\nline3\n";
        let right = "line1\nmodified line2\nline3\n";

        let mut config = DiffConfig::default();
        config.style = DiffStyle::SideBySide;

        let diff = diff_strings(left, right, &config);
        assert!(diff.contains("line1"));
        assert!(diff.contains("line2"));
        assert!(diff.contains("modified line2"));
    }

    #[test]
    fn test_summary_diff() {
        let left = "line1\nline2\nline3\n";
        let right = "line1\nmodified line2\nline3\n";

        let mut config = DiffConfig::default();
        config.style = DiffStyle::Summary;

        let diff = summary_diff(left, right, &config);
        assert!(diff.contains("Total lines"));
        assert!(diff.contains("Added lines"));
        assert!(diff.contains("Removed lines"));
    }
}
