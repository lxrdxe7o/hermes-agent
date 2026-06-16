//! Syntax Highlighting Module
//!
//! This module provides syntax highlighting for code blocks in chat messages
//! using the syntect library.

use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};
use syntect::{
    easy::HighlightLines,
    highlighting::{Style as SyntectStyle, ThemeSet},
    parsing::SyntaxSet,
    util::LinesWithEndings,
};

/// Syntax highlighter for code blocks
pub struct SyntaxHighlighter {
    /// Syntax set for parsing code
    syntax_set: SyntaxSet,
    /// Theme set for coloring
    theme_set: ThemeSet,
    /// Current theme name
    theme_name: String,
}

impl Default for SyntaxHighlighter {
    fn default() -> Self {
        Self::new()
    }
}

impl SyntaxHighlighter {
    /// Create a new syntax highlighter with default theme
    #[must_use]
    pub fn new() -> Self {
        let syntax_set = SyntaxSet::load_defaults_newlines();
        let theme_set = ThemeSet::load_defaults();

        Self {
            syntax_set,
            theme_set,
            theme_name: "base16-ocean.dark".to_string(),
        }
    }

    /// Create a new syntax highlighter with a specific theme
    pub fn with_theme(theme_name: impl Into<String>) -> Self {
        let mut highlighter = Self::new();
        highlighter.set_theme(theme_name);
        highlighter
    }

    /// Set the syntax highlighting theme
    pub fn set_theme(&mut self, theme_name: impl Into<String>) {
        self.theme_name = theme_name.into();
    }

    /// Get available themes
    pub fn available_themes(&self) -> Vec<&str> {
        self.theme_set
            .themes
            .keys()
            .map(std::string::String::as_str)
            .collect()
    }

    /// Highlight a code block and return styled lines
    pub fn highlight(&self, code: &str, language: Option<&str>) -> Vec<Line<'static>> {
        // Get the theme
        let theme = self
            .theme_set
            .themes
            .get(&self.theme_name)
            .unwrap_or_else(|| self.theme_set.themes.get("base16-ocean.dark").unwrap());

        // Determine syntax based on language hint
        let syntax = if let Some(lang) = language {
            self.syntax_set
                .find_syntax_for_file(lang)
                .ok()
                .or_else(|| Some(self.syntax_set.find_syntax_by_token(lang)))
                .unwrap_or(Some(self.syntax_set.find_syntax_plain_text()))
        } else {
            Some(self.syntax_set.find_syntax_plain_text())
        };

        let syntax = syntax.unwrap();
        let mut h = HighlightLines::new(syntax, theme);

        // Process each line
        let lines = LinesWithEndings::from(code);
        let mut styled_lines = Vec::new();

        for line in lines {
            let ranges: Vec<(SyntectStyle, &str)> =
                h.highlight_line(line, &self.syntax_set).unwrap_or_default();
            let mut spans = Vec::new();

            for (style, text) in ranges {
                let fg_color = style.foreground;
                let color = convert_syntect_color(fg_color);

                spans.push(Span::styled(text.to_string(), Style::new().fg(color)));
            }

            styled_lines.push(Line::from(spans));
        }

        styled_lines
    }
}

/// Convert syntect highlighting Color to ratatui Color
fn convert_syntect_color(color: syntect::highlighting::Color) -> Color {
    // syntect Color is a struct with r, g, b, a fields as u8
    Color::Rgb(color.r, color.g, color.b)
}

/// Extract code blocks from markdown text
/// Returns a vector of (language, code) tuples
#[must_use]
pub fn extract_code_blocks(text: &str) -> Vec<(Option<String>, String)> {
    let mut code_blocks = Vec::new();
    let mut lines = text.lines();

    while let Some(line) = lines.next() {
        // Check for fenced code blocks (```lang or ```)
        if line.trim().starts_with("```") {
            let lang = if line.trim().len() > 3 {
                Some(line.trim()[3..].trim().to_string())
            } else {
                None
            };

            let mut code_lines = Vec::new();
            for next_line in lines.by_ref() {
                if next_line.trim().starts_with("```") {
                    break;
                }
                code_lines.push(next_line);
            }

            code_blocks.push((lang, code_lines.join("\n")));
        }
    }

    code_blocks
}

/// Check if a line is a fenced code block start
#[must_use]
pub fn is_code_block_start(line: &str) -> bool {
    line.trim().starts_with("```")
}

/// Check if a line is a fenced code block end
#[must_use]
pub fn is_code_block_end(line: &str) -> bool {
    line.trim().starts_with("```")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_syntax_highlighter_new() {
        let highlighter = SyntaxHighlighter::new();
        assert!(!highlighter.available_themes().is_empty());
    }

    #[test]
    fn test_extract_code_blocks() {
        let text = r#"
Some text before

```python
def hello():
    print("world")
```

More text

```rust
fn main() {
    println!("hello");
}
```
"#;

        let blocks = extract_code_blocks(text);
        assert_eq!(blocks.len(), 2);

        assert_eq!(blocks[0].0, Some("python".to_string()));
        assert!(blocks[0].1.contains("def hello():"));

        assert_eq!(blocks[1].0, Some("rust".to_string()));
        assert!(blocks[1].1.contains("fn main()"));
    }

    #[test]
    fn test_highlight_basic() {
        let highlighter = SyntaxHighlighter::new();
        let code = "fn main() {\n    println!(\"hello\");\n}";

        let lines = highlighter.highlight(code, Some("rust"));
        assert!(!lines.is_empty());
        assert_eq!(lines.len(), 3);
    }

    #[test]
    fn test_is_code_block_start() {
        assert!(is_code_block_start("```python"));
        assert!(is_code_block_start("```"));
        assert!(!is_code_block_start("not a code block"));
    }
}
