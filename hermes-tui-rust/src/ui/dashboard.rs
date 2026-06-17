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
        animation_frame: u64,
        is_running: bool,
    ) {
        // TODO: This is a prototype layout matching the React example.
        // The telemetry values are currently hardcoded or derived from time.
        // Future work: Connect this to real system metrics (CPU/MEM/NET).

        use ratatui::layout::{Alignment, Constraint, Direction, Layout};
        use ratatui::style::{Color, Modifier, Style};
        use ratatui::text::{Line, Span};
        use ratatui::widgets::{Block, BorderType, Borders, Gauge, Padding, Paragraph};

        let time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as usize;
        let offset = (time / 100) % 6;

        // Main layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(7), // Title/Banner
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
            .block(Block::default().padding(Padding::new(0, 0, 1, 0)));
        frame.render_widget(title_para, chunks[0]);

        // Content border
        let content_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .border_style(Style::default().fg(colors.primary.clone().into()))
            .padding(Padding::new(2, 2, 1, 1));
        let inner_area = content_block.inner(chunks[1]);
        frame.render_widget(content_block, chunks[1]);
        crate::ui::borders::render_gradient_border(frame.buffer_mut(), chunks[1], animation_frame, true, is_running);

        let content_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30), // Left column (GIF/Telemetry)
                Constraint::Percentage(70), // Right column (Lists)
            ])
            .spacing(2)
            .split(inner_area);

        // Left Column
        let left_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(10), // GIF
                Constraint::Min(10),    // Telemetry
            ])
            .spacing(1)
            .split(content_chunks[0]);

        if let Some(gif_data) = gif {
            let frame_str = gif_data.get_frame(time_ms(), 80);
            let gif_para = Paragraph::new(frame_str).alignment(Alignment::Center);
            frame.render_widget(gif_para, left_chunks[0]);
        }

        // Telemetry
        let cpu = (10 + (offset * 5)) as u16;
        let mem = (40 + offset) as u16;
        let net = 200 + (offset * 100);

        let tel_block = Block::default()
            .title(Span::styled(
                " TELEMETRY ",
                Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD),
            ))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray))
            .padding(Padding::new(1, 1, 1, 1));
        
        let tel_inner = tel_block.inner(left_chunks[1]);
        frame.render_widget(tel_block, left_chunks[1]);

        let tel_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2), // CPU
                Constraint::Length(2), // MEM
                Constraint::Length(2), // NET
                Constraint::Min(0),
            ])
            .spacing(1)
            .split(tel_inner);

        // CPU Gauge
        let cpu_gauge = Gauge::default()
            .block(Block::default().title("CPU Usage"))
            .gauge_style(Style::default().fg(Color::Rgb(131, 165, 152)).bg(Color::Rgb(40, 40, 40)))
            .percent(cpu);
        frame.render_widget(cpu_gauge, tel_layout[0]);

        // MEM Gauge
        let mem_gauge = Gauge::default()
            .block(Block::default().title("Memory"))
            .gauge_style(Style::default().fg(Color::Rgb(211, 134, 155)).bg(Color::Rgb(40, 40, 40)))
            .percent(mem);
        frame.render_widget(mem_gauge, tel_layout[1]);

        // NET Info
        let is_streaming = offset % 2 == 0;
        let net_text = Line::from(vec![
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
        ]);
        frame.render_widget(Paragraph::new(net_text), tel_layout[2]);

        // Right Column
        let right_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(8), // Available Tools
                Constraint::Length(4), // MCP Servers
                Constraint::Min(10),   // Available Skills
            ])
            .spacing(1)
            .split(content_chunks[1]);

        // Available Tools
        let mut tools_lines = Vec::new();
        tools_lines.push(Line::from(vec![
            Span::styled("browser:       ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                "browser_back, browser_click, browser_close, browser_open",
                Style::default().fg(Color::Gray),
            ),
        ]));
        tools_lines.push(Line::from(vec![
            Span::styled("browser-cdp:   ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                "browser_cdp_call, browser_dialog_accept, browser_dialog_dismiss",
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
        tools_lines.push(Line::from(Span::styled(
            "(and 22 more toolsets...)",
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::ITALIC),
        )));
        frame.render_widget(
            Paragraph::new(tools_lines).block(Block::default().title(Span::styled(
                " Available Tools ",
                Style::default().fg(Color::Rgb(250, 189, 47)).add_modifier(Modifier::BOLD),
            )).borders(Borders::BOTTOM).border_style(Style::default().fg(Color::DarkGray))),
            right_chunks[0],
        );

        // MCP Servers
        let spinner = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"][offset % 10];
        let mcp_lines = vec![
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
        frame.render_widget(
            Paragraph::new(mcp_lines).block(Block::default().title(Span::styled(
                " MCP Servers ",
                Style::default().fg(Color::Rgb(250, 189, 47)).add_modifier(Modifier::BOLD),
            )).borders(Borders::BOTTOM).border_style(Style::default().fg(Color::DarkGray))),
            right_chunks[1],
        );

        // Available Skills
        let mut skills_lines = Vec::new();
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
        frame.render_widget(
            Paragraph::new(skills_lines).block(Block::default().title(Span::styled(
                " Available Skills ",
                Style::default().fg(Color::Rgb(250, 189, 47)).add_modifier(Modifier::BOLD),
            ))),
            right_chunks[2],
        );
    }
}
fn time_ms() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
