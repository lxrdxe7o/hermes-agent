use crate::state::config::ThemeColors;
use ratatui::{layout::Rect, Frame};

pub struct KanbanView;

impl KanbanView {
    pub fn render(frame: &mut Frame, area: Rect, _colors: &ThemeColors) {
        use ratatui::widgets::{Block, Borders, Padding, Paragraph};
        // TODO: This is a prototype Kanban layout matching the React example.
        // The task cards are currently hardcoded mocks.
        // Future work: Connect this to the actual gateway Kanban API and durable SQLite store.

        use ratatui::layout::{Constraint, Direction, Layout};
        use ratatui::style::{Color, Modifier, Style};
        use ratatui::text::{Line, Span};

        let time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as usize;
        let offset = (time / 100) % 6;
        let spinner = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"][offset % 10];

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Ratio(1, 3), // Backlog
                Constraint::Ratio(1, 3), // Executing
                Constraint::Ratio(1, 3), // Verified
            ])
            .split(area);

        // 1. Backlog
        let backlog_block = Block::default()
            .title(" 📋 BACKLOG ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
        let backlog_inner = backlog_block.inner(chunks[0]);
        frame.render_widget(backlog_block, chunks[0]);

        let mut backlog_lines = Vec::new();
        backlog_lines.push(Line::from(vec![Span::styled(
            "[ ] Refactor API",
            Style::default()
                .fg(Color::Gray)
                .add_modifier(Modifier::BOLD),
        )]));
        backlog_lines.push(Line::from(vec![Span::styled(
            "src/api.rs",
            Style::default().fg(Color::DarkGray),
        )]));
        backlog_lines.push(Line::from(vec![Span::styled(
            "[#enhancement]",
            Style::default().fg(Color::Rgb(211, 134, 155)),
        )]));
        backlog_lines.push(Line::from(""));

        backlog_lines.push(Line::from(vec![Span::styled(
            "[ ] Setup DB pool",
            Style::default()
                .fg(Color::Gray)
                .add_modifier(Modifier::BOLD),
        )]));
        backlog_lines.push(Line::from(vec![Span::styled(
            "src/db.rs",
            Style::default().fg(Color::DarkGray),
        )]));
        backlog_lines.push(Line::from(vec![Span::styled(
            "[#infra]",
            Style::default().fg(Color::Rgb(211, 134, 155)),
        )]));

        frame.render_widget(
            Paragraph::new(backlog_lines).block(Block::default().padding(Padding::new(1, 1, 1, 1))),
            backlog_inner,
        );

        // 2. Executing
        let executing_block = Block::default()
            .title(Span::styled(
                " ⚡ EXECUTING ",
                Style::default().fg(Color::Rgb(250, 189, 47)),
            ))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(250, 189, 47)));
        let executing_inner = executing_block.inner(chunks[1]);
        frame.render_widget(executing_block, chunks[1]);

        let mut exec_lines = Vec::new();
        exec_lines.push(Line::from(vec![
            Span::styled(
                "[~] Fix JWT Auth ",
                Style::default()
                    .fg(Color::Rgb(250, 189, 47))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                spinner.to_string(),
                Style::default().fg(Color::Rgb(211, 134, 155)),
            ),
        ]));
        exec_lines.push(Line::from(vec![Span::styled(
            "src/auth.rs",
            Style::default().fg(Color::DarkGray),
        )]));
        exec_lines.push(Line::from(vec![Span::styled(
            "[#bug]",
            Style::default().fg(Color::Gray),
        )]));
        exec_lines.push(Line::from(""));

        let progress = (time / 50) % 20;
        let progress_bar = "█".repeat(progress) + &"▒".repeat(20 - progress);
        exec_lines.push(Line::from(vec![
            Span::styled(
                "Processing: ",
                Style::default().fg(Color::Rgb(250, 189, 47)),
            ),
            Span::styled(progress_bar, Style::default().fg(Color::Rgb(131, 165, 152))),
        ]));

        frame.render_widget(
            Paragraph::new(exec_lines).block(Block::default().padding(Padding::new(1, 1, 1, 1))),
            executing_inner,
        );

        // 3. Verified
        let verified_block = Block::default()
            .title(" ✅ VERIFIED ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
        let verified_inner = verified_block.inner(chunks[2]);
        frame.render_widget(verified_block, chunks[2]);

        let mut verified_lines = Vec::new();
        verified_lines.push(Line::from(vec![Span::styled(
            "[x] Update README",
            Style::default()
                .fg(Color::Rgb(184, 187, 38))
                .add_modifier(Modifier::BOLD | Modifier::CROSSED_OUT),
        )]));
        verified_lines.push(Line::from(vec![Span::styled(
            "Commit a1b2",
            Style::default().fg(Color::DarkGray),
        )]));
        verified_lines.push(Line::from(vec![Span::styled(
            "[#docs]",
            Style::default().fg(Color::Rgb(184, 187, 38)),
        )]));

        frame.render_widget(
            Paragraph::new(verified_lines)
                .block(Block::default().padding(Padding::new(1, 1, 1, 1))),
            verified_inner,
        );
    }
}
