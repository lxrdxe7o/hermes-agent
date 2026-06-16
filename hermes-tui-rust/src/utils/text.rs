//! Text module - Text utility functions
//!
//! This module provides utility functions for text manipulation,
use ratatui::text::{Line, Text};
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

/// Text wrapping configuration
#[derive(Debug, Clone)]
pub struct WrapConfig {
    /// Maximum width for wrapping
    pub width: u16,
    /// Whether to preserve newlines
    pub preserve_newlines: bool,
    /// Whether to wrap at word boundaries
    pub word_boundary: bool,
    /// Character to use for soft wraps
    pub soft_wrap_char: char,
}

impl Default for WrapConfig {
    fn default() -> Self {
        Self {
            width: 80,
            preserve_newlines: true,
            word_boundary: true,
            soft_wrap_char: '\n',
        }
    }
}

impl WrapConfig {
    /// Create a new wrap configuration
    #[must_use]
    pub fn new(width: u16) -> Self {
        Self {
            width,
            ..Default::default()
        }
    }

    /// Set whether to preserve newlines
    #[must_use]
    pub fn preserve_newlines(mut self, preserve: bool) -> Self {
        self.preserve_newlines = preserve;
        self
    }

    /// Set whether to wrap at word boundaries
    #[must_use]
    pub fn word_boundary(mut self, word_boundary: bool) -> Self {
        self.word_boundary = word_boundary;
        self
    }
}

/// Text wrapper for wrapping text to a specified width
#[derive(Debug, Default)]
pub struct TextWrapper {
    config: WrapConfig,
}

impl TextWrapper {
    /// Create a new text wrapper with default configuration
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new text wrapper with custom configuration
    #[must_use]
    pub fn with_config(config: WrapConfig) -> Self {
        Self { config }
    }

    /// Wrap text to the configured width
    /// Returns a vector of lines, each no longer than the configured width
    pub fn wrap(&self, text: &str) -> Vec<String> {
        if self.config.width == 0 {
            return vec![text.to_string()];
        }

        let width = self.config.width as usize;
        let mut lines = Vec::new();
        let mut current_line = String::new();
        let mut current_width = 0usize;

        for c in text.chars() {
            let char_width = c.width().unwrap_or(1);

            // Handle explicit newlines
            if c == '\n' && self.config.preserve_newlines {
                if !current_line.is_empty() {
                    lines.push(current_line);
                    current_line = String::new();
                    current_width = 0;
                }
                lines.push(String::new()); // Empty line for the newline
                continue;
            }

            // Check if adding this character would exceed the width
            if current_width + char_width > width {
                // Try to find a word boundary to break at
                if self.config.word_boundary && !current_line.is_empty() {
                    // Find the last space in current_line
                    if let Some(last_space) = current_line.rfind(char::is_whitespace) {
                        // Push everything up to and including the space
                        let before = current_line[..=last_space].to_string();
                        let after = current_line[last_space + 1..].to_string();
                        lines.push(before);
                        current_line = after.clone();
                        current_width = after.width();
                        // Re-check if we can add the current character
                        if current_width + char_width > width {
                            lines.push(current_line);
                            current_line = String::new();
                            current_width = 0;
                        }
                    } else {
                        // No space found, hard break
                        lines.push(current_line);
                        current_line = String::new();
                        current_width = 0;
                    }
                } else {
                    // Hard break
                    lines.push(current_line);
                    current_line = String::new();
                    current_width = 0;
                }
            }

            current_line.push(c);
            current_width += char_width;
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }

        lines
    }

    /// Wrap text and convert to ratatui Text
    #[must_use]
    pub fn wrap_to_text(&self, text: &str) -> Text<'_> {
        let lines: Vec<Line> = self.wrap(text).into_iter().map(Line::from).collect();
        Text::from(lines)
    }

    /// Get the wrap configuration
    #[must_use]
    pub fn config(&self) -> &WrapConfig {
        &self.config
    }

    /// Set the wrap configuration
    pub fn set_config(&mut self, config: WrapConfig) {
        self.config = config;
    }
}

/// Truncate text to a specified width
/// If text is longer than width, it will be truncated with an ellipsis
#[must_use]
pub fn truncate(text: &str, width: u16) -> String {
    if width == 0 {
        return String::new();
    }

    let width = width as usize;
    let text_width = text.width();

    if text_width <= width {
        return text.to_string();
    }

    // Reserve space for ellipsis
    let ellipsis = "...";
    let ellipsis_width = ellipsis.width();
    let available_width = width.saturating_sub(ellipsis_width);

    if available_width == 0 {
        return ellipsis.to_string();
    }

    let mut result = String::new();
    let mut current_width = 0usize;

    for c in text.chars() {
        let char_width = c.width().unwrap_or(1);
        if current_width + char_width > available_width {
            break;
        }
        result.push(c);
        current_width += char_width;
    }

    result + ellipsis
}

/// Truncate text from the left (for right-aligned text)
#[must_use]
pub fn truncate_left(text: &str, width: u16) -> String {
    if width == 0 {
        return String::new();
    }

    let width = width as usize;
    let text_width = text.width();

    if text_width <= width {
        return text.to_string();
    }

    let ellipsis = "...";
    let ellipsis_width = ellipsis.width();
    let available_width = width.saturating_sub(ellipsis_width);

    if available_width == 0 {
        return ellipsis.to_string();
    }

    let mut result = String::new();
    let mut current_width = 0usize;
    let chars: Vec<char> = text.chars().collect();

    // Start from the end and work backwards
    for c in chars.into_iter().rev() {
        let char_width = c.width().unwrap_or(1);
        if current_width + char_width > available_width {
            break;
        }
        result.insert(0, c);
        current_width += char_width;
    }

    format!("{ellipsis}{result}")
}

/// Truncate text from the middle (for center-aligned text)
#[must_use]
pub fn truncate_middle(text: &str, width: u16) -> String {
    if width == 0 {
        return String::new();
    }

    let width = width as usize;
    let text_width = text.width();

    if text_width <= width {
        return text.to_string();
    }

    let ellipsis = "...";
    let ellipsis_width = ellipsis.width();
    let available_width = width.saturating_sub(ellipsis_width);

    if available_width == 0 {
        return ellipsis.to_string();
    }

    // Split available width between left and right
    let left_width = available_width / 2;
    let right_width = available_width - left_width;

    let mut left = String::new();
    let mut left_current = 0usize;
    for c in text.chars() {
        let char_width = c.width().unwrap_or(1);
        if left_current + char_width > left_width {
            break;
        }
        left.push(c);
        left_current += char_width;
    }

    let mut right = String::new();
    let mut right_current = 0usize;
    let chars: Vec<char> = text.chars().collect();
    for c in chars.into_iter().rev() {
        let char_width = c.width().unwrap_or(1);
        if right_current + char_width > right_width {
            break;
        }
        right.insert(0, c);
        right_current += char_width;
    }

    left + ellipsis + &right
}

/// Pad text to a specified width
/// If text is shorter than width, it will be padded with spaces
#[must_use]
pub fn pad(text: &str, width: u16) -> String {
    let width = width as usize;
    let text_width = text.width();

    if text_width >= width {
        return text.to_string();
    }

    let padding = width.saturating_sub(text_width);
    format!("{}{}", text, " ".repeat(padding))
}

/// Pad text to a specified width, centered
#[must_use]
pub fn pad_center(text: &str, width: u16) -> String {
    let width = width as usize;
    let text_width = text.width();

    if text_width >= width {
        return text.to_string();
    }

    let padding = width.saturating_sub(text_width);
    let left_padding = padding / 2;
    let right_padding = padding - left_padding;

    format!(
        "{}{}{}",
        " ".repeat(left_padding),
        text,
        " ".repeat(right_padding)
    )
}

/// Pad text to a specified width, right-aligned
#[must_use]
pub fn pad_right(text: &str, width: u16) -> String {
    let width = width as usize;
    let text_width = text.width();

    if text_width >= width {
        return text.to_string();
    }

    let padding = width.saturating_sub(text_width);
    format!("{}{}", " ".repeat(padding), text)
}

/// Split text into lines at newlines
#[must_use]
pub fn split_lines(text: &str) -> Vec<&str> {
    text.split('\n').collect()
}

/// Join lines with newlines
#[must_use]
pub fn join_lines(lines: &[&str]) -> String {
    lines.join("\n")
}

/// Count the number of lines in text
#[must_use]
pub fn count_lines(text: &str) -> usize {
    text.lines().count()
}

/// Get the width of the longest line in text
#[must_use]
pub fn max_line_width(text: &str) -> usize {
    text.lines()
        .map(unicode_width::UnicodeWidthStr::width)
        .max()
        .unwrap_or(0)
}

/// Check if text contains only whitespace
#[must_use]
pub fn is_blank(text: &str) -> bool {
    text.trim().is_empty()
}

/// Check if text is a single line
#[must_use]
pub fn is_single_line(text: &str) -> bool {
    !text.contains('\n')
}

/// Remove trailing newlines from text
#[must_use]
pub fn trim_trailing_newlines(text: &str) -> String {
    text.trim_end_matches('\n').to_string()
}

/// Remove leading newlines from text
#[must_use]
pub fn trim_leading_newlines(text: &str) -> String {
    text.trim_start_matches('\n').to_string()
}

/// Escape special characters for display
#[must_use]
pub fn escape_special(text: &str) -> String {
    text.replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

/// Unescape special characters
#[must_use]
pub fn unescape_special(text: &str) -> String {
    text.replace("\\n", "\n")
        .replace("\\r", "\r")
        .replace("\\t", "\t")
}

/// Convert text to title case (first letter of each word capitalized)
#[must_use]
pub fn to_title_case(text: &str) -> String {
    text.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().chain(chars).collect(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

/// Convert text to sentence case (first letter capitalized)
#[must_use]
pub fn to_sentence_case(text: &str) -> String {
    let mut chars = text.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().chain(chars).collect(),
    }
}

/// Check if two strings are equal ignoring case
#[must_use]
pub fn eq_ignore_case(a: &str, b: &str) -> bool {
    a.eq_ignore_ascii_case(b)
}

/// Check if a string starts with a prefix (case-insensitive)
#[must_use]
pub fn starts_with_ignore_case(text: &str, prefix: &str) -> bool {
    text.to_lowercase().starts_with(&prefix.to_lowercase())
}

/// Check if a string ends with a suffix (case-insensitive)
#[must_use]
pub fn ends_with_ignore_case(text: &str, suffix: &str) -> bool {
    text.to_lowercase().ends_with(&suffix.to_lowercase())
}

/// Check if a string contains a substring (case-insensitive)
#[must_use]
pub fn contains_ignore_case(text: &str, substring: &str) -> bool {
    text.to_lowercase().contains(&substring.to_lowercase())
}

/// Replace all occurrences of a pattern with a replacement (case-insensitive)
#[must_use]
pub fn replace_ignore_case(text: &str, pattern: &str, replacement: &str) -> String {
    let pattern_lower = pattern.to_lowercase();
    let text_lower = text.to_lowercase();

    let mut result = String::new();
    let mut last_end = 0;

    while let Some(pos) = text_lower[last_end..].find(&pattern_lower) {
        let pos = pos + last_end;
        result.push_str(&text[last_end..pos]);
        result.push_str(replacement);
        last_end = pos + pattern.len();
    }

    result.push_str(&text[last_end..]);
    result
}

/// Count the number of words in text
#[must_use]
pub fn word_count(text: &str) -> usize {
    text.split_whitespace().count()
}

/// Count all characters in text (including whitespace)
#[must_use]
pub fn char_count(text: &str) -> usize {
    text.chars().count()
}

/// Get the first non-whitespace character's position
#[must_use]
pub fn first_non_whitespace(text: &str) -> Option<usize> {
    text.chars().position(|c| !c.is_whitespace())
}

/// Get the last non-whitespace character's position
#[must_use]
pub fn last_non_whitespace(text: &str) -> Option<usize> {
    text.chars()
        .rev()
        .position(|c| !c.is_whitespace())
        .map(|pos| text.len() - 1 - pos)
}

/// Trim whitespace from both ends
#[must_use]
pub fn trim(text: &str) -> String {
    text.trim().to_string()
}

/// Trim whitespace from the start
#[must_use]
pub fn trim_start(text: &str) -> String {
    text.trim_start().to_string()
}

/// Trim whitespace from the end
#[must_use]
pub fn trim_end(text: &str) -> String {
    text.trim_end().to_string()
}

/// Repeat a string n times
#[must_use]
pub fn repeat(text: &str, n: usize) -> String {
    text.repeat(n)
}

/// Join strings with a separator
#[must_use]
pub fn join(strings: &[&str], separator: &str) -> String {
    strings.join(separator)
}
/// Determine if input looks like a slash command.
#[must_use]
pub fn looks_like_slash_command(input: &str) -> bool {
    input.starts_with('/') && !input.contains('\n')
}

/// A completion request derived from user input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionRequest {
    /// Method name to send to the gateway.
    pub method: &'static str,
    /// Query text (`text` for slash, `word` for path).
    pub query: String,
    /// Character index in the input where replacement should start.
    pub replace_from: usize,
}

/// Compute the completion request (if any) for the current input.
///
/// Mirrors the behavior of the TypeScript TUI completion hook:
/// - `/...` triggers slash completion (except `/model`).
/// - Path-like tokens (`./`, `../`, `~/`, `/`, `@`, or `foo/`) trigger path completion.
#[must_use]
pub fn completion_request_for_input(input: &str) -> Option<CompletionRequest> {
    let is_slash = looks_like_slash_command(input);

    // `/model` uses the dedicated ModelPicker; skip slash completion there.
    if is_slash
        && input.starts_with("/model")
        && (input.len() == 6 || input.chars().nth(6).is_some_and(char::is_whitespace))
    {
        return None;
    }

    if is_slash {
        return Some(CompletionRequest {
            method: "complete.slash",
            query: input.to_string(),
            replace_from: 1,
        });
    }

    // Path completion: match the last path-like token before the end of input.
    // Mirrors ui-tui: tokens starting with ./, ../, ~/, /, @, or containing a slash.
    let word = last_path_token(input)?;
    let replace_from = input.len() - word.len();

    Some(CompletionRequest {
        method: "complete.path",
        query: word.to_string(),
        replace_from,
    })
}

/// Extract the last path-like token from `input`, or None if there isn't one.
fn last_path_token(input: &str) -> Option<&str> {
    // Find the start of the last whitespace-delimited token.
    let start = input
        .rfind(|c: char| c.is_whitespace() || c == '"' || c == '\'')
        .map_or(0, |i| i + 1);
    let word = &input[start..];
    if word.is_empty() {
        return None;
    }

    let is_path_prefix = word.starts_with("./")
        || word.starts_with("../")
        || word.starts_with("~/")
        || word.starts_with('/')
        || word.starts_with('@')
        || word.starts_with("./")
        || word.starts_with("../");
    let contains_slash = word.contains('/');
    let is_windows_abs = word.len() >= 3
        && word.as_bytes()[1] == b':'
        && (word.as_bytes()[2] == b'/' || word.as_bytes()[2] == b'\\');

    if is_path_prefix || contains_slash || is_windows_abs {
        Some(word)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_wrapper_basic() {
        let wrapper = TextWrapper::new();
        let text = "This is a long line of text that should be wrapped";
        let lines = wrapper.wrap(text);

        assert!(!lines.is_empty());
        for line in &lines {
            assert!(line.width() <= 80);
        }
    }

    #[test]
    fn test_text_wrapper_preserve_newlines() {
        let wrapper = TextWrapper::with_config(WrapConfig {
            preserve_newlines: false,
            ..Default::default()
        });
        let text = "Line 1\nLine 2\nLine 3";
        let lines = wrapper.wrap(text);

        // Without preserving newlines, should be fewer lines
        assert!(lines.len() < 3);
    }

    #[test]
    fn test_truncate() {
        let text = "This is a very long text that should be truncated";
        let truncated = truncate(text, 20);

        assert!(truncated.width() <= 20);
        assert!(truncated.ends_with("..."));
    }

    #[test]
    fn test_truncate_left() {
        let text = "This is a very long text";
        let truncated = truncate_left(text, 20);

        assert!(truncated.width() <= 20);
        assert!(truncated.starts_with("..."));
    }

    #[test]
    fn test_truncate_middle() {
        let text = "This is a very long text";
        let truncated = truncate_middle(text, 20);

        assert!(truncated.width() <= 20);
        assert!(truncated.contains("..."));
    }

    #[test]
    fn test_pad() {
        let text = "short";
        let padded = pad(text, 20);

        assert_eq!(padded.width(), 20);
        assert!(padded.starts_with("short"));
    }

    #[test]
    fn test_pad_center() {
        let text = "center";
        let padded = pad_center(text, 20);

        assert_eq!(padded.width(), 20);
        assert!(padded.contains("center"));
    }

    #[test]
    fn test_pad_right() {
        let text = "right";
        let padded = pad_right(text, 20);

        assert_eq!(padded.width(), 20);
        assert!(padded.ends_with("right"));
    }

    #[test]
    fn test_split_lines() {
        let text = "Line 1\nLine 2\nLine 3";
        let lines = split_lines(text);

        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "Line 1");
        assert_eq!(lines[1], "Line 2");
        assert_eq!(lines[2], "Line 3");
    }

    #[test]
    fn test_count_lines() {
        let text = "Line 1\nLine 2\nLine 3";
        let count = count_lines(text);

        assert_eq!(count, 3);
    }

    #[test]
    fn test_max_line_width() {
        let text = "Short\nThis is a much longer line\nMedium";
        let width = max_line_width(text);

        assert_eq!(width, "This is a much longer line".width());
    }

    #[test]
    fn test_is_blank() {
        assert!(is_blank(""));
        assert!(is_blank("   "));
        assert!(is_blank("\n\n"));
        assert!(!is_blank("hello"));
    }

    #[test]
    fn test_is_single_line() {
        assert!(is_single_line("single line"));
        assert!(!is_single_line("multi\nline"));
    }

    #[test]
    fn test_to_title_case() {
        assert_eq!(to_title_case("hello world"), "Hello World");
        assert_eq!(to_title_case("HELLO WORLD"), "HELLO WORLD");
    }

    #[test]
    fn test_to_sentence_case() {
        assert_eq!(to_sentence_case("hello world"), "Hello world");
        assert_eq!(to_sentence_case("HELLO WORLD"), "HELLO WORLD");
    }

    #[test]
    fn test_eq_ignore_case() {
        assert!(eq_ignore_case("Hello", "hello"));
        assert!(eq_ignore_case("HELLO", "hello"));
        assert!(!eq_ignore_case("Hello", "World"));
    }

    #[test]
    fn test_starts_with_ignore_case() {
        assert!(starts_with_ignore_case("Hello World", "hello"));
        assert!(!starts_with_ignore_case("Hello World", "world"));
    }

    #[test]
    fn test_ends_with_ignore_case() {
        assert!(ends_with_ignore_case("Hello World", "world"));
        assert!(!ends_with_ignore_case("Hello World", "hello"));
    }

    #[test]
    fn test_contains_ignore_case() {
        assert!(contains_ignore_case("Hello World", "hello"));
        assert!(contains_ignore_case("Hello World", "WORLD"));
        assert!(!contains_ignore_case("Hello World", "foo"));
    }

    #[test]
    fn test_word_count() {
        assert_eq!(word_count("hello world"), 2);
        assert_eq!(word_count("  hello   world  "), 2);
        assert_eq!(word_count(""), 0);
    }

    #[test]
    fn test_completion_request_slash() {
        let req = completion_request_for_input("/he").unwrap();
        assert_eq!(req.method, "complete.slash");
        assert_eq!(req.query, "/he");
        assert_eq!(req.replace_from, 1);
    }

    #[test]
    fn test_completion_request_skips_model() {
        assert!(completion_request_for_input("/model").is_none());
        assert!(completion_request_for_input("/model ").is_none());
    }

    #[test]
    fn test_completion_request_path() {
        let req = completion_request_for_input("read ./src").unwrap();
        assert_eq!(req.method, "complete.path");
        assert_eq!(req.query, "./src");
        assert_eq!(req.replace_from, 5);
    }

    #[test]
    fn test_completion_request_no_trigger() {
        assert!(completion_request_for_input("hello").is_none());
        assert!(completion_request_for_input("").is_none());
    }
    #[test]
    fn test_char_count() {
        assert_eq!(char_count("hello world"), 11);
        assert_eq!(char_count("  hello   world  "), 17);
        assert_eq!(char_count(""), 0);
    }
}
