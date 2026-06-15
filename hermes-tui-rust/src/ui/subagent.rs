//! Subagent module - Subagent visualization widget
//!
//! This module provides widgets for displaying subagent activity,
//! status, and results in the TUI.

use chrono::{DateTime, Utc};
use ratatui::{
    layout::Rect,
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

/// Status of a subagent
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubagentStatus {
    /// Subagent is currently running
    Running,
    /// Subagent completed successfully
    Completed,
    /// Subagent failed
    Failed,
    /// Subagent is pending execution
    Pending,
    /// Subagent is idle
    Idle,
}

/// Information about a subagent
#[derive(Debug, Clone)]
pub struct SubagentInfo {
    /// Unique identifier for this subagent
    pub id: String,
    /// Current status
    pub status: SubagentStatus,
    /// The goal/assignment given to the subagent
    pub goal: String,
    /// Summary of results (if completed)
    pub summary: Option<String>,
    /// ID of the parent agent (if any)
    pub parent_id: Option<String>,
    /// When the subagent started
    pub started_at: Option<DateTime<Utc>>,
    /// When the subagent completed
    pub completed_at: Option<DateTime<Utc>>,
}

impl SubagentInfo {
    /// Create new subagent info with running status
    pub fn new(id: impl Into<String>, goal: impl Into<String>, parent_id: Option<String>) -> Self {
        Self {
            id: id.into(),
            status: SubagentStatus::Running,
            goal: goal.into(),
            summary: None,
            parent_id,
            started_at: Some(Utc::now()),
            completed_at: None,
        }
    }

    /// Mark as completed with a summary
    pub fn mark_completed(&mut self, summary: impl Into<String>) {
        self.status = SubagentStatus::Completed;
        self.summary = Some(summary.into());
        self.completed_at = Some(Utc::now());
    }

    /// Mark as failed
    pub fn mark_failed(&mut self) {
        self.status = SubagentStatus::Failed;
        self.completed_at = Some(Utc::now());
    }

    /// Get the status icon and color
    pub fn status_style(&self) -> (&'static str, Style) {
        match self.status {
            SubagentStatus::Running => (" ▶ ", Style::default().fg(Color::Yellow).bold()),
            SubagentStatus::Completed => (" ✓ ", Style::default().fg(Color::Green)),
            SubagentStatus::Failed => (" ✗ ", Style::default().fg(Color::Red).bold()),
            SubagentStatus::Pending => (" ○ ", Style::default().fg(Color::DarkGray)),
            SubagentStatus::Idle => (" − ", Style::default().fg(Color::DarkGray)),
        }
    }
}

/// Widget for displaying a list of subagents
#[derive(Debug, Clone)]
pub struct SubagentList {
    /// List of subagents
    agents: Vec<SubagentInfo>,
    /// Current scroll position
    scroll: u16,
    /// Visible height
    visible_height: u16,
    /// Auto-scroll to latest active subagent
    auto_scroll: bool,
}

impl Default for SubagentList {
    fn default() -> Self {
        Self::new()
    }
}

impl SubagentList {
    /// Create a new subagent list
    pub fn new() -> Self {
        Self {
            agents: Vec::new(),
            scroll: 0,
            visible_height: 0,
            auto_scroll: true,
        }
    }

    /// Add or update a subagent
    pub fn upsert(&mut self, info: SubagentInfo) {
        if let Some(existing) = self.agents.iter_mut().find(|a| a.id == info.id) {
            *existing = info;
        } else {
            self.agents.push(info);
        }
        if self.auto_scroll {
            self.scroll_to_latest();
        }
    }

    /// Remove a subagent
    pub fn remove(&mut self, id: &str) {
        self.agents.retain(|a| a.id != id);
    }

    /// Clear all subagents
    pub fn clear(&mut self) {
        self.agents.clear();
        self.scroll = 0;
    }

    /// Get active (running + pending) subagent count
    pub fn active_count(&self) -> usize {
        self.agents.iter()
            .filter(|a| a.status == SubagentStatus::Running || a.status == SubagentStatus::Pending)
            .count()
    }

    /// Get total subagent count
    pub fn total_count(&self) -> usize {
        self.agents.len()
    }

    /// Check if there are any subagents
    pub fn is_empty(&self) -> bool {
        self.agents.is_empty()
    }

    /// Scroll to the latest active subagent
    pub fn scroll_to_latest(&mut self) {
        if let Some(idx) = self.agents.iter().rposition(|a| a.status == SubagentStatus::Running) {
            self.scroll = idx.saturating_sub(self.visible_height.saturating_sub(1) as usize) as u16;
        } else if !self.agents.is_empty() {
            self.scroll = self.agents.len().saturating_sub(self.visible_height as usize) as u16;
        }
    }

    /// Scroll up
    pub fn scroll_up(&mut self) {
        self.scroll = self.scroll.saturating_sub(1);
        self.auto_scroll = false;
    }

    /// Scroll down
    pub fn scroll_down(&mut self) {
        let max_scroll = self.agents.len().saturating_sub(self.visible_height as usize);
        self.scroll = (self.scroll + 1).min(max_scroll as u16);
        self.auto_scroll = false;
    }

    /// Render the subagent list
    pub fn render(&self, area: Rect, frame: &mut Frame) {
        let title = format!(" Subagents ({}) ", self.active_count());
        let block = Block::default()
            .title(title)
            .title_style(Style::default().fg(Color::Cyan).bold())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        if self.agents.is_empty() {
            let inner = block.inner(area);
            frame.render_widget(block, area);
            let text = Text::from(Line::from(Span::styled(
                " No active subagents",
                Style::default().fg(Color::DarkGray),
            )));
            frame.render_widget(Paragraph::new(text), inner);
            return;
        }

        let inner = block.inner(area);
        frame.render_widget(block, area);

        // Calculate visible range
        let start = self.scroll as usize;
        let end = (start + inner.height as usize).min(self.agents.len());

        let mut y_offset = 0u16;
        for agent in self.agents.iter().skip(start).take(end.saturating_sub(start)) {
            if y_offset >= inner.height {
                break;
            }
            let line_area = Rect::new(inner.x, inner.y + y_offset, inner.width, 1);
            let line = self.build_agent_line(agent);
            frame.render_widget(Paragraph::new(line), line_area);
            y_offset += 1;
        }
    }

    /// Build a single line for a subagent entry
    fn build_agent_line(&self, agent: &SubagentInfo) -> Line<'static> {
        let (icon, icon_style) = agent.status_style();
        let mut spans = Vec::new();

        // Status icon
        spans.push(Span::styled(icon.to_string(), icon_style));

        // Parent relationship
        if agent.parent_id.is_some() {
            spans.push(Span::styled("└ ", Style::default().fg(Color::DarkGray)));
        }

        // Agent ID
        spans.push(Span::styled(
            agent.id.clone(),
            Style::default().bold(),
        ));

        // Goal (truncated)
        let max_goal_len = 40usize;
        let goal = if agent.goal.len() > max_goal_len {
            format!(": {}...", &agent.goal[..max_goal_len.saturating_sub(3)])
        } else {
            format!(": {}", agent.goal)
        };
        spans.push(Span::styled(goal, Style::default().fg(Color::White)));

        // Summary (if completed/failed)
        if let Some(ref summary) = agent.summary {
            let max_summary_len = 30usize;
            let s = if summary.len() > max_summary_len {
                format!(" → {}...", &summary[..max_summary_len.saturating_sub(3)])
            } else {
                format!(" → {}", summary)
            };
            spans.push(Span::styled(s, Style::default().fg(Color::DarkGray)));
        }

        Line::from(spans)
    }

    /// Set visible height
    pub fn set_visible_height(&mut self, height: u16) {
        self.visible_height = height;
    }

    /// Get immutable access to the agent list
    pub fn agents(&self) -> &[SubagentInfo] {
        &self.agents
    }

    /// Get mutable access to the agent list
    pub fn agents_mut(&mut self) -> &mut Vec<SubagentInfo> {
        &mut self.agents
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subagent_info_new() {
        let info = SubagentInfo::new("test-1", "do something", None);
        assert_eq!(info.id, "test-1");
        assert_eq!(info.status, SubagentStatus::Running);
        assert!(info.started_at.is_some());
    }

    #[test]
    fn test_subagent_info_mark_completed() {
        let mut info = SubagentInfo::new("test-1", "do something", None);
        info.mark_completed("done");
        assert_eq!(info.status, SubagentStatus::Completed);
        assert_eq!(info.summary, Some("done".to_string()));
        assert!(info.completed_at.is_some());
    }

    #[test]
    fn test_subagent_info_status_style() {
        let info = SubagentInfo::new("test-1", "do something", None);
        let (icon, _style) = info.status_style();
        assert_eq!(icon, " ▶ ");
    }

    #[test]
    fn test_subagent_list_upsert() {
        let mut list = SubagentList::new();
        assert!(list.is_empty());

        list.upsert(SubagentInfo::new("test-1", "task 1", None));
        assert_eq!(list.total_count(), 1);
        assert_eq!(list.active_count(), 1);

        list.upsert(SubagentInfo::new("test-2", "task 2", None));
        assert_eq!(list.total_count(), 2);

        // Update existing
        let mut info = SubagentInfo::new("test-1", "task 1", None);
        info.mark_completed("done");
        list.upsert(info);
        assert_eq!(list.total_count(), 2);
        assert_eq!(list.active_count(), 1);
    }

    #[test]
    fn test_subagent_list_remove() {
        let mut list = SubagentList::new();
        list.upsert(SubagentInfo::new("test-1", "task 1", None));
        list.remove("test-1");
        assert!(list.is_empty());
    }

    #[test]
    fn test_build_agent_line() {
        let list = SubagentList::new();
        let agent = SubagentInfo::new("worker-1", "analyze data", None);
        let line = list.build_agent_line(&agent);
        assert!(!line.spans.is_empty());
    }
}
