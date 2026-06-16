use crate::state::config::ThemeColors;
use crate::ui::chat::ChatComponent;
use ratatui::{layout::Rect, Frame};

pub struct IdeView;

impl IdeView {
    pub fn render(
        frame: &mut Frame,
        area: Rect,
        _colors: &ThemeColors,
        chat_component: &mut ChatComponent,
        connected: bool,
        card_manager: &crate::ui::cards::CardManager,
        subagent_list: &crate::ui::subagent::SubagentList,
    ) {
        // TODO: This is a prototype IDE layout matching the React example.
        // The File Tree and Editor/Diff are currently hardcoded mocks.
        // Future work: Connect the File Tree to the actual project structure and the
        // Editor to the gateway's edit events.

        use ratatui::layout::{Constraint, Direction, Layout};
        use ratatui::style::{Color, Modifier, Style};
        use ratatui::text::{Line, Span};
        use ratatui::widgets::{Block, Borders, Padding, Paragraph};

        let time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as usize;
        let offset = (time / 100) % 6;
        let spinner = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"][offset % 10];

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(25),     // Tree
                Constraint::Percentage(50), // Editor
                Constraint::Percentage(50), // Chat
            ])
            .split(area);

        // 1. File Tree
        let mut tree_lines = Vec::new();
        tree_lines.push(Line::from(Span::styled(
            "oh-my-pi-core/",
            Style::default()
                .fg(Color::Gray)
                .add_modifier(Modifier::BOLD),
        )));
        tree_lines.push(Line::from(Span::styled(
            "├── .github/",
            Style::default().fg(Color::DarkGray),
        )));
        tree_lines.push(Line::from(Span::styled(
            "├── src/",
            Style::default().fg(Color::DarkGray),
        )));
        tree_lines.push(Line::from(vec![
            Span::styled("│   ├── ", Style::default().fg(Color::DarkGray)),
            Span::styled("tools.rs", Style::default().fg(Color::Rgb(250, 189, 47))),
        ]));
        tree_lines.push(Line::from(Span::styled(
            "│   ├── llm.rs",
            Style::default().fg(Color::DarkGray),
        )));
        tree_lines.push(Line::from(Span::styled(
            "│   └── main.rs",
            Style::default().fg(Color::DarkGray),
        )));
        tree_lines.push(Line::from(Span::styled(
            "├── Cargo.toml",
            Style::default().fg(Color::DarkGray),
        )));
        tree_lines.push(Line::from(Span::styled(
            "└── README.md",
            Style::default().fg(Color::DarkGray),
        )));

        let tree_block = Block::default()
            .title(" TREE ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray))
            .padding(Padding::new(1, 1, 1, 1));
        frame.render_widget(Paragraph::new(tree_lines).block(tree_block), chunks[0]);

        // 2. Editor
        let mut diff_lines = Vec::new();
        diff_lines.push(Line::from(Span::styled(
            " fn execute_tool(req: ToolCall) {",
            Style::default().fg(Color::DarkGray),
        )));
        diff_lines.push(Line::from(Span::styled(
            "-    let mut res = String::new();",
            Style::default()
                .fg(Color::Rgb(204, 36, 29))
                .bg(Color::Rgb(204, 36, 29)),
        ))); // red background with some opacity? Ratatui doesn't do opacity easily, so just red fg
        diff_lines.push(Line::from(Span::styled(
            "-    let mut res = String::new();",
            Style::default().fg(Color::Rgb(204, 36, 29)),
        )));
        diff_lines.push(Line::from(Span::styled(
            "+    let mut res = ToolOutput::new();",
            Style::default().fg(Color::Rgb(184, 187, 38)),
        )));
        diff_lines.push(Line::from(Span::styled(
            "     match req.name.as_str() {",
            Style::default().fg(Color::DarkGray),
        )));
        diff_lines.push(Line::from(Span::styled(
            "+        \"read_file\" => {",
            Style::default().fg(Color::Rgb(184, 187, 38)),
        )));
        diff_lines.push(Line::from(Span::styled(
            "+            res.set_data(fs::read_to_string(&req.args)?);",
            Style::default().fg(Color::Rgb(184, 187, 38)),
        )));
        diff_lines.push(Line::from(Span::styled(
            "+        }",
            Style::default().fg(Color::Rgb(184, 187, 38)),
        )));
        diff_lines.push(Line::from(Span::styled(
            "         _ => return Err(Error::UnknownTool),",
            Style::default().fg(Color::DarkGray),
        )));
        diff_lines.push(Line::from(Span::styled(
            "     }",
            Style::default().fg(Color::DarkGray),
        )));
        diff_lines.push(Line::from(Span::styled(
            " }",
            Style::default().fg(Color::DarkGray),
        )));

        let active_border = if offset % 2 == 0 {
            Color::Rgb(250, 189, 47)
        } else {
            Color::DarkGray
        };
        let editor_title = format!(" 📝 src/tools.rs [LSP: Active {spinner}] ");

        let editor_block = Block::default()
            .title(Span::styled(
                editor_title,
                Style::default().fg(Color::Rgb(250, 189, 47)),
            ))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(active_border))
            .padding(Padding::new(1, 1, 1, 1));

        frame.render_widget(Paragraph::new(diff_lines).block(editor_block), chunks[1]);

        // 3. Chat
        let chat_block = Block::default()
            .title(Span::styled(
                " CHAT ",
                Style::default().fg(Color::Rgb(211, 134, 155)),
            ))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));

        let chat_inner = chat_block.inner(chunks[2]);
        frame.render_widget(chat_block, chunks[2]);

        chat_component.set_visible_height(chat_inner.height.saturating_sub(2));
        chat_component.render(frame, chat_inner, card_manager, subagent_list, connected);
    }
}
