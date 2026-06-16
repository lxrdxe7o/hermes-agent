//! Model picker component for TUI
//!
//! This module provides an overlay dialog for choosing a model,
//! organized by provider.

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem},
    Frame,
};

use crate::protocol::types::ModelOptionProvider;
use crate::state::config::ChatColorsRgb;

/// Current stage of the model picker
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ModelPickerStage {
    /// Selecting a provider
    Provider,
    /// Selecting a model within the current provider
    Model,
}

/// Model picker popup component
#[derive(Debug, Clone)]
pub struct ModelPicker {
    /// Providers to select from
    providers: Vec<ModelOptionProvider>,
    /// Visibility state
    visible: bool,
    /// Current navigation stage
    stage: ModelPickerStage,
    /// Selected provider index
    provider_index: usize,
    /// Selected model index within the current provider
    model_index: usize,
    /// UI colors from configuration
    colors: ChatColorsRgb,
}

impl ModelPicker {
    /// Create a new model picker instance
    #[must_use]
    pub fn new(colors: ChatColorsRgb) -> Self {
        Self {
            providers: Vec::new(),
            visible: false,
            stage: ModelPickerStage::Provider,
            provider_index: 0,
            model_index: 0,
            colors,
        }
    }

    /// Show the model picker with providers
    pub fn show(&mut self, providers: Vec<ModelOptionProvider>) {
        self.providers = providers;
        self.stage = ModelPickerStage::Provider;
        self.provider_index = 0;
        self.model_index = 0;
        self.visible = !self.providers.is_empty();
    }

    /// Hide the model picker
    pub fn hide(&mut self) {
        self.visible = false;
        self.providers.clear();
        self.stage = ModelPickerStage::Provider;
        self.provider_index = 0;
        self.model_index = 0;
    }

    /// Check if the model picker is visible
    #[must_use]
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Get the currently selected provider
    #[must_use]
    pub fn selected_provider(&self) -> Option<&ModelOptionProvider> {
        if self.visible {
            self.providers.get(self.provider_index)
        } else {
            None
        }
    }

    /// Get the currently selected model name
    #[must_use]
    pub fn selected_model(&self) -> Option<String> {
        let provider = self.selected_provider()?;
        let models = provider.models.as_ref()?;
        models.get(self.model_index).cloned()
    }

    /// Select the next item in the current stage
    pub fn select_next(&mut self) {
        match self.stage {
            ModelPickerStage::Provider => {
                if !self.providers.is_empty() {
                    self.provider_index = (self.provider_index + 1) % self.providers.len();
                    self.model_index = 0;
                }
            }
            ModelPickerStage::Model => {
                if let Some(provider) = self.providers.get(self.provider_index) {
                    if let Some(models) = &provider.models {
                        if !models.is_empty() {
                            self.model_index = (self.model_index + 1) % models.len();
                        }
                    }
                }
            }
        }
    }

    /// Select the previous item in the current stage
    pub fn select_prev(&mut self) {
        match self.stage {
            ModelPickerStage::Provider => {
                if !self.providers.is_empty() {
                    self.provider_index = if self.provider_index == 0 {
                        self.providers.len() - 1
                    } else {
                        self.provider_index - 1
                    };
                    self.model_index = 0;
                }
            }
            ModelPickerStage::Model => {
                if let Some(provider) = self.providers.get(self.provider_index) {
                    if let Some(models) = &provider.models {
                        if !models.is_empty() {
                            self.model_index = if self.model_index == 0 {
                                models.len() - 1
                            } else {
                                self.model_index - 1
                            };
                        }
                    }
                }
            }
        }
    }

    /// Enter the model list for the selected provider
    pub fn enter_provider(&mut self) -> bool {
        if self.stage == ModelPickerStage::Provider {
            if let Some(provider) = self.providers.get(self.provider_index) {
                if provider.models.as_ref().is_some_and(|m| !m.is_empty()) {
                    self.stage = ModelPickerStage::Model;
                    self.model_index = 0;
                    return true;
                }
            }
        }
        false
    }

    /// Go back to provider selection
    pub fn back_to_providers(&mut self) {
        self.stage = ModelPickerStage::Provider;
        self.model_index = 0;
    }

    /// Render the model picker popup
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        if !self.visible || self.providers.is_empty() {
            return;
        }

        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - 70) / 2),
                Constraint::Percentage(70),
                Constraint::Percentage((100 - 70) / 2),
            ])
            .split(area);

        let popup_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - 80) / 2),
                Constraint::Percentage(80),
                Constraint::Percentage((100 - 80) / 2),
            ])
            .split(popup_layout[1])[1];

        frame.render_widget(Clear, popup_area);

        match self.stage {
            ModelPickerStage::Provider => self.render_providers(frame, popup_area),
            ModelPickerStage::Model => self.render_models(frame, popup_area),
        }
    }

    fn render_providers(&self, frame: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self
            .providers
            .iter()
            .enumerate()
            .map(|(i, p)| {
                let current = p.is_current.unwrap_or(false);
                let auth = p.authenticated.unwrap_or(true);
                let model_count = p
                    .total_models
                    .or(p.models.as_ref().map(std::vec::Vec::len))
                    .unwrap_or(0);
                let suffix = if current { " ✓" } else { "" };
                let status = if auth { "" } else { " (needs key)" };
                let text = format!("{}{}{} — {} models", p.name, suffix, status, model_count);
                let style = if i == self.provider_index {
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
            .title(" Select Provider ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.colors.border));

        let list = List::new(items).block(block);
        frame.render_widget(list, area);

        let hint = Line::from(vec![Span::raw(
            "↑↓ navigate · Enter choose provider · Esc close",
        )]);
        frame.render_widget(
            ratatui::widgets::Paragraph::new(hint),
            Rect::new(area.x, area.y + area.height - 1, area.width, 1),
        );
    }

    fn render_models(&self, frame: &mut Frame, area: Rect) {
        let provider = match self.providers.get(self.provider_index) {
            Some(p) => p,
            None => return,
        };
        let models = match &provider.models {
            Some(m) => m,
            None => return,
        };

        let title = format!(" {} Models ", provider.name);
        let items: Vec<ListItem> = models
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let style = if i == self.model_index {
                    Style::default()
                        .fg(Color::Yellow)
                        .bg(self.colors.user_bg)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(self.colors.user_text)
                };
                ListItem::new(format!(" {m}")).style(style)
            })
            .collect();

        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.colors.border));

        let list = List::new(items).block(block);
        frame.render_widget(list, area);

        let hint = Line::from(vec![Span::raw(
            "↑↓ navigate · Enter select model · Esc back",
        )]);
        frame.render_widget(
            ratatui::widgets::Paragraph::new(hint),
            Rect::new(area.x, area.y + area.height - 1, area.width, 1),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn colors() -> ChatColorsRgb {
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
    fn test_model_picker_navigation() {
        let mut picker = ModelPicker::new(colors());
        picker.show(vec![
            ModelOptionProvider {
                name: "Anthropic".to_string(),
                slug: "anthropic".to_string(),
                models: Some(vec!["claude-3".to_string()]),
                total_models: None,
                authenticated: Some(true),
                is_current: Some(true),
            },
            ModelOptionProvider {
                name: "OpenAI".to_string(),
                slug: "openai".to_string(),
                models: Some(vec!["gpt-4".to_string()]),
                total_models: None,
                authenticated: Some(true),
                is_current: None,
            },
        ]);

        assert!(picker.is_visible());
        picker.select_next();
        assert_eq!(picker.provider_index, 1);
        assert!(picker.enter_provider());
        assert_eq!(picker.selected_model().as_deref(), Some("gpt-4"));
        picker.back_to_providers();
        assert_eq!(picker.stage, ModelPickerStage::Provider);
    }
}
