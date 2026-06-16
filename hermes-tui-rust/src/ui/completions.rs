//! Completions popup component for TUI
//!
//! This module provides a popup dialog for autocomplete suggestions
//! (such as slash commands, paths, etc.) displayed above the input composer.

use crate::protocol::types::CompletionItem;
use crate::state::config::ChatColorsRgb;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, List, ListItem},
    Frame,
};

#[derive(Debug, Clone)]
pub struct CompletionPopup {
    /// Autocomplete items
    items: Vec<CompletionItem>,
    /// Visibility state
    visible: bool,
    /// Currently selected index
    selected_index: usize,
    /// Start position to replace in input
    replace_from: Option<usize>,
    /// UI colors from configuration
    colors: ChatColorsRgb,
}

impl CompletionPopup {
    /// Create a new completion popup instance
    #[must_use]
    pub fn new(colors: ChatColorsRgb) -> Self {
        Self {
            items: Vec::new(),
            visible: false,
            selected_index: 0,
            replace_from: None,
            colors,
        }
    }

    /// Show the completion popup with items
    pub fn show(&mut self, items: Vec<CompletionItem>, replace_from: Option<usize>) {
        self.items = items;
        self.replace_from = replace_from;
        self.selected_index = 0;
        self.visible = !self.items.is_empty();
    }

    /// Hide the completion popup and clear items
    pub fn hide(&mut self) {
        self.visible = false;
        self.items.clear();
        self.replace_from = None;
    }

    /// Check if the popup is visible
    #[must_use]
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Get the replacement start index
    #[must_use]
    pub fn replace_from(&self) -> Option<usize> {
        self.replace_from
    }

    /// Get the currently selected completion item
    #[must_use]
    pub fn selected_item(&self) -> Option<&CompletionItem> {
        if self.visible && !self.items.is_empty() {
            self.items.get(self.selected_index)
        } else {
            None
        }
    }

    /// Select the next item in the completions list
    pub fn select_next(&mut self) {
        if !self.items.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.items.len();
        }
    }

    /// Select the previous item in the completions list
    pub fn select_prev(&mut self) {
        if !self.items.is_empty() {
            if self.selected_index == 0 {
                self.selected_index = self.items.len() - 1;
            } else {
                self.selected_index -= 1;
            }
        }
    }

    /// Render the completion popup above the composer area
    pub fn render(&self, frame: &mut Frame, composer_area: Rect) {
        if !self.visible || self.items.is_empty() {
            return;
        }

        // Use the passed area directly — caller (app.rs) already positions it above the composer.
        // No additional Y-offset to avoid double-subtraction bug.
        let max_height = 8;
        let height = (self.items.len() as u16 + 2)
            .min(max_height)
            .min(composer_area.height);
        let popup_area = Rect::new(
            composer_area.x,
            composer_area.y,
            composer_area.width.min(40).max(10),
            height,
        );

        // Clear the area to overlay the popup cleanly
        frame.render_widget(Clear, popup_area);

        // Build list items
        let list_items: Vec<ListItem> = self
            .items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let text = if let Some(desc) = &item.meta {
                    format!(" {} - {}", item.display, desc)
                } else {
                    format!(" {}", item.display)
                };
                let style = if i == self.selected_index {
                    Style::default()
                        .fg(Color::Yellow)
                        .bg(self.colors.user_bg)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(self.colors.user_text)
                };
                ListItem::new(text).style(style)
            })
            .collect();

        let block = Block::default()
            .title(" Completions ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.colors.border));

        let list = List::new(list_items).block(block);

        frame.render_widget(list, popup_area);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_colors() -> ChatColorsRgb {
        ChatColorsRgb {
            user_bg: Color::Reset,
            user_text: Color::Reset,
            assistant_bg: Color::Reset,
            assistant_text: Color::Reset,
            system_bg: Color::Reset,
            system_text: Color::Reset,
            tool_bg: Color::Reset,
            tool_text: Color::Reset,
            code_bg: Color::Reset,
            code_text: Color::Reset,
            border: Color::Reset,
            timestamp: Color::Reset,
        }
    }

    #[test]
    fn test_completion_popup_visibility() {
        let mut popup = CompletionPopup::new(create_test_colors());
        assert!(!popup.is_visible());

        let items = vec![CompletionItem {
            display: "help".to_string(),
            text: "/help".to_string(),
            meta: Some("Show help".to_string()),
        }];

        popup.show(items, None);
        assert!(popup.is_visible());

        popup.hide();
        assert!(!popup.is_visible());
    }

    #[test]
    fn test_completion_popup_navigation() {
        let mut popup = CompletionPopup::new(create_test_colors());
        let items = vec![
            CompletionItem {
                display: "help".to_string(),
                text: "/help".to_string(),
                meta: None,
            },
            CompletionItem {
                display: "quit".to_string(),
                text: "/quit".to_string(),
                meta: None,
            },
        ];

        popup.show(items, None);
        assert_eq!(popup.selected_index, 0);
        assert_eq!(popup.selected_item().unwrap().display, "help");

        popup.select_next();
        assert_eq!(popup.selected_index, 1);
        assert_eq!(popup.selected_item().unwrap().display, "quit");

        popup.select_next();
        assert_eq!(popup.selected_index, 0);

        popup.select_prev();
        assert_eq!(popup.selected_index, 1);
    }
}
