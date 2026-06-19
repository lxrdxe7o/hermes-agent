use crate::state::config::ThemeColors;
use ratatui::{layout::Rect, Frame};

pub struct KanbanView;

impl KanbanView {
    pub fn render(
        frame: &mut Frame,
        area: Rect,
        _colors: &ThemeColors,
        animation_frame: u64,
        is_running: bool,
    ) {
        use ratatui::layout::{Constraint, Direction, Layout};
        use ratatui::style::{Color, Modifier, Style};
        use ratatui::text::{Line, Span};
        use ratatui::widgets::{Block, BorderType, Borders, LineGauge, Padding, Paragraph};

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
            .spacing(1)
            .split(area);

        // 1. Backlog
        let backlog_block = Block::default()
            .title(Span::styled(
                " 📋 BACKLOG ",
                Style::default()
                    .fg(Color::Gray)
                    .add_modifier(Modifier::BOLD),
            ))
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .border_style(Style::default().fg(Color::DarkGray));
        let backlog_inner = backlog_block.inner(chunks[0]);
        frame.render_widget(backlog_block, chunks[0]);
        crate::ui::borders::render_gradient_border(
            frame.buffer_mut(),
            chunks[0],
            animation_frame,
            true,
            is_running,
        );

        let mut backlog_lines = Vec::new();
        backlog_lines.push(Line::from(vec![Span::styled(
            "● Refactor API",
            Style::default()
                .fg(Color::Gray)
                .add_modifier(Modifier::BOLD),
        )]));
        backlog_lines.push(Line::from(vec![Span::styled(
            "  src/api.rs",
            Style::default().fg(Color::DarkGray),
        )]));
        backlog_lines.push(Line::from(vec![Span::styled(
            "  [#enhancement]",
            Style::default().fg(Color::Rgb(211, 134, 155)),
        )]));
        backlog_lines.push(Line::from(""));

        backlog_lines.push(Line::from(vec![Span::styled(
            "● Setup DB pool",
            Style::default()
                .fg(Color::Gray)
                .add_modifier(Modifier::BOLD),
        )]));
        backlog_lines.push(Line::from(vec![Span::styled(
            "  src/db.rs",
            Style::default().fg(Color::DarkGray),
        )]));
        backlog_lines.push(Line::from(vec![Span::styled(
            "  [#infra]",
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
                Style::default()
                    .fg(Color::Rgb(250, 189, 47))
                    .add_modifier(Modifier::BOLD),
            ))
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .border_style(Style::default().fg(Color::Rgb(250, 189, 47)));
        let executing_inner = executing_block.inner(chunks[1]);
        frame.render_widget(executing_block, chunks[1]);
        crate::ui::borders::render_gradient_border(
            frame.buffer_mut(),
            chunks[1],
            animation_frame,
            true,
            is_running,
        );

        let exec_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(4), // Card 1
                Constraint::Length(2), // Progress 1
                Constraint::Min(0),
            ])
            .margin(1)
            .split(executing_inner);

        let mut exec_lines = Vec::new();
        exec_lines.push(Line::from(vec![
            Span::styled(
                "○ Fix JWT Auth ",
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
            "  src/auth.rs",
            Style::default().fg(Color::DarkGray),
        )]));
        exec_lines.push(Line::from(vec![Span::styled(
            "  [#bug]",
            Style::default().fg(Color::Gray),
        )]));

        frame.render_widget(Paragraph::new(exec_lines), exec_layout[0]);

        let progress = ((time / 50) % 101) as f64;
        let progress_gauge = LineGauge::default()
            .block(Block::default().title("Processing"))
            .filled_style(Style::default().fg(Color::Rgb(131, 165, 152)))
            .unfilled_style(Style::default().fg(Color::Rgb(40, 40, 40)))
            .ratio(progress / 100.0);
        frame.render_widget(progress_gauge, exec_layout[1]);

        // 3. Verified
        let verified_block = Block::default()
            .title(Span::styled(
                " ✅ VERIFIED ",
                Style::default()
                    .fg(Color::Rgb(184, 187, 38))
                    .add_modifier(Modifier::BOLD),
            ))
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .border_style(Style::default().fg(Color::DarkGray));
        let verified_inner = verified_block.inner(chunks[2]);
        frame.render_widget(verified_block, chunks[2]);
        crate::ui::borders::render_gradient_border(
            frame.buffer_mut(),
            chunks[2],
            animation_frame,
            true,
            is_running,
        );

        let mut verified_lines = Vec::new();
        verified_lines.push(Line::from(vec![Span::styled(
            "✓ Update README",
            Style::default()
                .fg(Color::Rgb(184, 187, 38))
                .add_modifier(Modifier::BOLD | Modifier::CROSSED_OUT),
        )]));
        verified_lines.push(Line::from(vec![Span::styled(
            "  Commit a1b2",
            Style::default().fg(Color::DarkGray),
        )]));
        verified_lines.push(Line::from(vec![Span::styled(
            "  [#docs]",
            Style::default().fg(Color::Rgb(184, 187, 38)),
        )]));

        frame.render_widget(
            Paragraph::new(verified_lines)
                .block(Block::default().padding(Padding::new(1, 1, 1, 1))),
            verified_inner,
        );
    }
}
