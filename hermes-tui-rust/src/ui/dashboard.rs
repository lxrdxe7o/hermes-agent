use crate::state::config::ThemeColors;
use crate::ui::gif::AnimatedGif;
use ratatui::{layout::Rect, Frame};

pub struct DashboardView;

impl DashboardView {
    pub fn render(
        frame: &mut Frame,
        area: Rect,
        gif: Option<&mut AnimatedGif>,
        colors: &ThemeColors,
    ) {
        // TODO: This is a prototype layout matching the React example.
        // The telemetry values are currently hardcoded or derived from time.
        // Future work: Connect this to real system metrics (CPU/MEM/NET).

        use ratatui::layout::{Alignment, Constraint, Direction, Layout};
        use ratatui::style::{Color, Modifier, Style};
        use ratatui::text::{Line, Span};
        use ratatui::widgets::{Block, BorderType, Borders, Padding, Paragraph};

        let time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as usize;
        let offset = (time / 100) % 6;

        // Main layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(8), // Title
                Constraint::Min(20),   // Content
            ])
            .split(area);

        // Title
        let title_text = r"██   ██ ██████  █████  ██   ██ ███████ ███    ██     █████  ██████  ███████ ███    ██ ████████
██  ██  ██   ██ ██   ██ ██  ██  ██      ████   ██    ██   ██ ██       ██      ████   ██    ██   
█████   ██████  ███████ █████   █████   ██ ██  ██    ███████ ██   ███ █████   ██ ██  ██    ██   
██  ██  ██   ██ ██   ██ ██  ██  ██      ██  ██ ██    ██   ██ ██    ██ ██      ██  ██ ██    ██   
██   ██ ██   ██ ██   ██ ██   ██ ███████ ██   ████    ██   ██  ██████  ███████ ██   ████    ██";

        let mut title_lines = Vec::new();
        for line in title_text.lines() {
            title_lines.push(Line::from(Span::styled(
                line,
                Style::default()
                    .fg(Color::Rgb(250, 189, 47))
                    .add_modifier(Modifier::BOLD),
            )));
        }
        let title_para = Paragraph::new(title_lines)
            .alignment(Alignment::Center)
            .block(Block::default().padding(Padding::new(0, 0, 1, 1)));
        frame.render_widget(title_para, chunks[0]);

        // Content border
        let content_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .border_style(Style::default().fg(colors.primary.clone().into()))
            .padding(Padding::new(2, 2, 2, 2));
        let inner_area = content_block.inner(chunks[1]);
        frame.render_widget(content_block, chunks[1]);

        // Header inside content
        let header_text = Line::from(vec![
            Span::styled(
                "Hermes Agent v0.16.0 (2026.6.5) ",
                Style::default()
                    .fg(Color::Rgb(250, 189, 47))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("· ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                "upstream bd16e524",
                Style::default()
                    .fg(Color::Rgb(250, 189, 47))
                    .add_modifier(Modifier::BOLD),
            ),
        ]);
        let header_para = Paragraph::new(header_text).alignment(Alignment::Center);
        frame.render_widget(
            header_para,
            Rect {
                x: inner_area.x,
                y: inner_area.y,
                width: inner_area.width,
                height: 1,
            },
        );

        let content_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(30), // Left column (GIF/Telemetry)
                Constraint::Min(40),    // Right column (Lists)
            ])
            .margin(2)
            .split(inner_area);

        // Left Column
        let left_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(12), // GIF
                Constraint::Length(8),  // Telemetry
            ])
            .split(content_chunks[0]);

        if let Some(gif_data) = gif {
            let frame_str = gif_data.get_frame(time_ms(), 80);
            let gif_para = Paragraph::new(frame_str).alignment(Alignment::Center);
            frame.render_widget(gif_para, left_chunks[0]);
        }

        // Telemetry
        let cpu = 10 + (offset * 5);
        let mem = 40 + offset;
        let net = 200 + (offset * 100);

        let mut tel_lines = Vec::new();
        tel_lines.push(Line::from(Span::styled(
            "TELEMETRY",
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )));

        let cpu_bar = "█".repeat(cpu / 10) + &"░".repeat(10 - (cpu / 10));
        tel_lines.push(Line::from(vec![
            Span::styled("CPU ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{cpu_bar} "),
                Style::default().fg(Color::Rgb(131, 165, 152)),
            ),
            Span::styled(format!("{cpu}%"), Style::default().fg(Color::Gray)),
        ]));

        let mem_bar = "█".repeat(mem / 10) + &"░".repeat(10 - (mem / 10));
        tel_lines.push(Line::from(vec![
            Span::styled("MEM ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{mem_bar} "),
                Style::default().fg(Color::Rgb(211, 134, 155)),
            ),
            Span::styled(format!("{mem}%"), Style::default().fg(Color::Gray)),
        ]));

        let is_streaming = offset % 2 == 0;
        tel_lines.push(Line::from(vec![
            Span::styled("NET ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                if is_streaming {
                    ">> STREAMING "
                } else {
                    "<> IDLE      "
                },
                Style::default().fg(Color::Rgb(250, 189, 47)),
            ),
            Span::styled(format!("{net}kb/s"), Style::default().fg(Color::Gray)),
        ]));

        let tel_para = Paragraph::new(tel_lines).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        );
        frame.render_widget(tel_para, left_chunks[1]);

        // Right Column
        let right_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(7), // Available Tools
                Constraint::Length(3), // MCP Servers
                Constraint::Min(10),   // Available Skills
            ])
            .split(content_chunks[1]);

        // Available Tools
        let mut tools_lines = Vec::new();
        tools_lines.push(Line::from(Span::styled(
            "Available Tools",
            Style::default()
                .fg(Color::Rgb(250, 189, 47))
                .add_modifier(Modifier::BOLD),
        )));
        tools_lines.push(Line::from(vec![
            Span::styled("browser:       ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                "browser_back, browser_click, ...",
                Style::default().fg(Color::Gray),
            ),
        ]));
        tools_lines.push(Line::from(vec![
            Span::styled("browser-cdp:   ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                "browser_cdp, browser_dialog",
                Style::default().fg(Color::Gray),
            ),
        ]));
        tools_lines.push(Line::from(vec![
            Span::styled("clarify:       ", Style::default().fg(Color::DarkGray)),
            Span::styled("clarify", Style::default().fg(Color::Rgb(211, 134, 155))),
        ]));
        tools_lines.push(Line::from(vec![
            Span::styled("code_execution:", Style::default().fg(Color::DarkGray)),
            Span::styled(
                "execute_code",
                Style::default().fg(Color::Rgb(250, 189, 47)),
            ),
        ]));
        tools_lines.push(Line::from(vec![
            Span::styled("computer_use:  ", Style::default().fg(Color::DarkGray)),
            Span::styled("computer_use", Style::default().fg(Color::Gray)),
        ]));
        tools_lines.push(Line::from(Span::styled(
            "(and 22 more toolsets...)",
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::ITALIC),
        )));
        frame.render_widget(Paragraph::new(tools_lines), right_chunks[0]);

        // MCP Servers
        let spinner = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"][offset % 10];
        let mcp_lines = vec![
            Line::from(Span::styled(
                "MCP Servers",
                Style::default()
                    .fg(Color::Rgb(250, 189, 47))
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(vec![
                Span::styled(
                    "playwright (stdio) — ",
                    Style::default().fg(Color::DarkGray),
                ),
                Span::styled(
                    format!("connecting {spinner}"),
                    Style::default().fg(Color::Rgb(250, 189, 47)),
                ),
            ]),
        ];
        frame.render_widget(Paragraph::new(mcp_lines), right_chunks[1]);

        // Available Skills
        let mut skills_lines = Vec::new();
        skills_lines.push(Line::from(Span::styled(
            "Available Skills",
            Style::default()
                .fg(Color::Rgb(250, 189, 47))
                .add_modifier(Modifier::BOLD),
        )));
        let skills = [
            ("autonomous-ai-agents", "coding-agents, hermes-agent..."),
            ("creative", "architecture-diagram, ascii-art..."),
            ("data-science", "jupyter-live-kernel"),
            ("devops", "kanban-orchestrator, kanban-worker..."),
            ("email", "himalaya"),
            ("fullstack-webdev", "React Patterns, Tailwind CSS..."),
        ];
        for (cat, tools) in skills {
            skills_lines.push(Line::from(vec![
                Span::styled(format!("{cat:25}"), Style::default().fg(Color::DarkGray)),
                Span::styled(tools, Style::default().fg(Color::Gray)),
            ]));
        }
        frame.render_widget(Paragraph::new(skills_lines), right_chunks[2]);
    }
}
fn time_ms() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
