//! Mouse module - Mouse event handlers
//!
//! This module provides handlers for mouse events including
//! clicks, drags, scrolls, and hover detection.

use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};

use crate::handlers::input::ParsedMouseEvent;

/// Mouse interaction context for tracking state
#[derive(Debug, Clone, Default)]
pub struct MouseContext {
    pub position: (u16, u16),
    pub pressed_buttons: Vec<MouseButton>,
    pub is_dragging: bool,
    pub drag_start: Option<(u16, u16)>,
    pub last_click: Option<(u16, u16)>,
    pub last_click_time: Option<std::time::Instant>,
}

impl MouseContext {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, event: &MouseEvent) {
        let parsed = ParsedMouseEvent::from(event);
        self.position = (parsed.column, parsed.row);

        match event.kind {
            MouseEventKind::Down(button) => {
                self.pressed_buttons.push(button);
                self.last_click = Some((parsed.column, parsed.row));
                self.last_click_time = Some(std::time::Instant::now());
                self.drag_start = Some((parsed.column, parsed.row));
            }
            MouseEventKind::Up(button) => {
                self.pressed_buttons.retain(|&b| b != button);
                self.is_dragging = false;
                self.drag_start = None;
            }
            MouseEventKind::Drag(_) => {
                self.is_dragging = true;
            }
            MouseEventKind::Moved => {
                if !self.pressed_buttons.is_empty() {
                    self.is_dragging = true;
                }
            }
            MouseEventKind::ScrollDown | MouseEventKind::ScrollUp => {}
            _ => {}
        }
    }

    #[must_use]
    pub fn is_button_pressed(&self, button: MouseButton) -> bool {
        self.pressed_buttons.contains(&button)
    }

    #[must_use]
    pub fn any_button_pressed(&self) -> bool {
        !self.pressed_buttons.is_empty()
    }

    #[must_use]
    pub fn is_dragging(&self) -> bool {
        self.is_dragging
    }
    #[must_use]
    pub fn drag_start(&self) -> Option<(u16, u16)> {
        self.drag_start
    }
    #[must_use]
    pub fn position(&self) -> (u16, u16) {
        self.position
    }

    #[must_use]
    pub fn is_in_rect(&self, x: u16, y: u16, width: u16, height: u16) -> bool {
        let (cx, cy) = self.position;
        cx >= x && cx < x + width && cy >= y && cy < y + height
    }

    pub fn check_double_click(&mut self, event: &MouseEvent) -> bool {
        let parsed = ParsedMouseEvent::from(event);
        let now = std::time::Instant::now();

        if let (Some((last_x, last_y)), Some(last_time)) = (self.last_click, self.last_click_time) {
            let dx = (parsed.column as i16 - last_x as i16).abs();
            let dy = (parsed.row as i16 - last_y as i16).abs();
            let elapsed = now.duration_since(last_time);

            if dx <= 2 && dy <= 2 && elapsed.as_millis() < 500 {
                return true;
            }
        }
        false
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

/// Mouse handler for chat area
#[derive(Debug, Default)]
pub struct ChatMouseHandler {
    click_padding: u16,
}

impl ChatMouseHandler {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn with_padding(padding: u16) -> Self {
        Self {
            click_padding: padding,
        }
    }

    #[must_use]
    pub fn handle_chat_mouse(
        &self,
        event: &MouseEvent,
        chat_area: (u16, u16, u16, u16),
        _message_count: usize,
        visible_start: usize,
        visible_count: usize,
    ) -> ChatMouseAction {
        let parsed = ParsedMouseEvent::from(event);
        let (x, y, width, height) = chat_area;

        if !self.is_in_area(parsed.column, parsed.row, x, y, width, height) {
            return ChatMouseAction::None;
        }

        match event.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                let relative_y = parsed.row.saturating_sub(y);
                if relative_y < height {
                    let message_index = visible_start + relative_y as usize;
                    if message_index < visible_start + visible_count {
                        return ChatMouseAction::MessageClick(message_index);
                    }
                }
                ChatMouseAction::ChatClick(parsed.column, parsed.row)
            }
            MouseEventKind::Down(MouseButton::Right) => {
                ChatMouseAction::ContextMenu(parsed.column, parsed.row)
            }
            MouseEventKind::ScrollUp => ChatMouseAction::ScrollUp,
            MouseEventKind::ScrollDown => ChatMouseAction::ScrollDown,
            _ => ChatMouseAction::None,
        }
    }

    fn is_in_area(&self, col: u16, row: u16, x: u16, y: u16, width: u16, height: u16) -> bool {
        col >= x && col < x + width && row >= y && row < y + height
    }

    #[must_use]
    pub fn click_padding(&self) -> u16 {
        self.click_padding
    }
}

/// Mouse handler for input composer
#[derive(Debug, Default)]
pub struct InputMouseHandler {
    cursor_width: u16,
}

impl InputMouseHandler {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn with_cursor_width(width: u16) -> Self {
        Self {
            cursor_width: width,
        }
    }

    #[must_use]
    pub fn handle_input_mouse(
        &self,
        event: &MouseEvent,
        input_area: (u16, u16, u16, u16),
        _text_length: usize,
    ) -> (InputMouseAction, Option<usize>) {
        let parsed = ParsedMouseEvent::from(event);
        let (x, y, width, _height) = input_area;

        if !self.is_in_area(parsed.column, parsed.row, x, y, width, 1) {
            return (InputMouseAction::None, None);
        }

        match event.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                let relative_x = parsed.column.saturating_sub(x);
                let cursor_pos = relative_x as usize;
                (InputMouseAction::SetCursor(cursor_pos), Some(cursor_pos))
            }
            MouseEventKind::Down(MouseButton::Right) => {
                let relative_x = parsed.column.saturating_sub(x);
                let cursor_pos = relative_x as usize;
                (
                    InputMouseAction::ContextMenuAt(cursor_pos),
                    Some(cursor_pos),
                )
            }
            MouseEventKind::Drag(MouseButton::Left) => {
                let relative_x = parsed.column.saturating_sub(x);
                let cursor_pos = relative_x as usize;
                (InputMouseAction::DragCursor(cursor_pos), Some(cursor_pos))
            }
            _ => (InputMouseAction::None, None),
        }
    }

    fn is_in_area(&self, col: u16, row: u16, x: u16, y: u16, width: u16, height: u16) -> bool {
        col >= x && col < x + width && row >= y && row < y + height
    }

    #[must_use]
    pub fn cursor_width(&self) -> u16 {
        self.cursor_width
    }
}

/// Toolbar button definition
#[derive(Debug, Clone)]
pub struct ToolbarButton {
    pub id: String,
    pub text: String,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub enabled: bool,
}

impl ToolbarButton {
    pub fn new(id: impl Into<String>, text: impl Into<String>, x: u16, y: u16, width: u16) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            x,
            y,
            width,
            enabled: true,
        }
    }

    pub fn with_enabled(
        id: impl Into<String>,
        text: impl Into<String>,
        x: u16,
        y: u16,
        width: u16,
        enabled: bool,
    ) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            x,
            y,
            width,
            enabled,
        }
    }
}

/// Mouse handler for toolbar
#[derive(Debug, Default)]
pub struct ToolbarMouseHandler {
    buttons: Vec<ToolbarButton>,
}

impl ToolbarMouseHandler {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add_button(&mut self, button: ToolbarButton) {
        self.buttons.push(button);
    }

    #[must_use]
    pub fn handle_toolbar_mouse(&self, event: &MouseEvent) -> ToolbarMouseAction {
        let parsed = ParsedMouseEvent::from(event);

        for button in &self.buttons {
            if parsed.column >= button.x
                && parsed.column < button.x + button.width
                && parsed.row == button.y
            {
                match event.kind {
                    MouseEventKind::Down(MouseButton::Left) => {
                        return ToolbarMouseAction::ButtonPress(button.id.clone());
                    }
                    MouseEventKind::Up(MouseButton::Left) => {
                        return ToolbarMouseAction::ButtonRelease(button.id.clone());
                    }
                    _ => {}
                }
            }
        }
        ToolbarMouseAction::None
    }

    #[must_use]
    pub fn buttons(&self) -> &[ToolbarButton] {
        &self.buttons
    }
}

/// Chat mouse actions
#[derive(Debug, Clone, PartialEq)]
pub enum ChatMouseAction {
    MessageClick(usize),
    ChatClick(u16, u16),
    ScrollUp,
    ScrollDown,
    ContextMenu(u16, u16),
    None,
}

/// Input mouse actions
#[derive(Debug, Clone, PartialEq)]
pub enum InputMouseAction {
    SetCursor(usize),
    DragCursor(usize),
    ContextMenuAt(usize),
    None,
}

/// Toolbar mouse actions
#[derive(Debug, Clone, PartialEq)]
pub enum ToolbarMouseAction {
    ButtonPress(String),
    ButtonRelease(String),
    None,
}

/// Swipe direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwipeDirection {
    Up,
    Down,
    Left,
    Right,
}

/// Mouse gesture recognizer
#[derive(Debug, Default)]
pub struct MouseGestureRecognizer {
    swipe_threshold: u16,
    long_press_duration: std::time::Duration,
    gesture_start: Option<(u16, u16)>,
    gesture_start_time: Option<std::time::Instant>,
}

impl MouseGestureRecognizer {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn with_thresholds(swipe_threshold: u16, long_press_duration: std::time::Duration) -> Self {
        Self {
            swipe_threshold,
            long_press_duration,
            gesture_start: None,
            gesture_start_time: None,
        }
    }

    pub fn start_gesture(&mut self, x: u16, y: u16) {
        self.gesture_start = Some((x, y));
        self.gesture_start_time = Some(std::time::Instant::now());
    }

    pub fn end_gesture(&mut self) {
        self.gesture_start = None;
        self.gesture_start_time = None;
    }

    #[must_use]
    pub fn check_long_press(&self) -> bool {
        self.gesture_start_time
            .is_some_and(|start| start.elapsed() >= self.long_press_duration)
    }

    #[must_use]
    pub fn check_swipe(&self, current_x: u16, current_y: u16) -> Option<SwipeDirection> {
        if let Some((start_x, start_y)) = self.gesture_start {
            let dx = (current_x as i16 - start_x as i16).abs();
            let dy = (current_y as i16 - start_y as i16).abs();

            if dx >= self.swipe_threshold as i16 {
                return Some(if current_x > start_x {
                    SwipeDirection::Right
                } else {
                    SwipeDirection::Left
                });
            }
            if dy >= self.swipe_threshold as i16 {
                return Some(if current_y > start_y {
                    SwipeDirection::Down
                } else {
                    SwipeDirection::Up
                });
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::KeyModifiers;

    #[test]
    fn test_mouse_context_update() {
        let mut context = MouseContext::new();
        let down_event = MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 10,
            row: 5,
            modifiers: KeyModifiers::NONE,
        };
        context.update(&down_event);
        assert_eq!(context.position(), (10, 5));
        assert!(context.is_button_pressed(MouseButton::Left));
        assert_eq!(context.drag_start(), Some((10, 5)));
    }

    #[test]
    fn test_mouse_context_double_click() {
        let mut context = MouseContext::new();
        let first_click = MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 10,
            row: 5,
            modifiers: KeyModifiers::NONE,
        };
        context.update(&first_click);
        std::thread::sleep(std::time::Duration::from_millis(100));
        let second_click = MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 10,
            row: 5,
            modifiers: KeyModifiers::NONE,
        };
        assert!(context.check_double_click(&second_click));
    }

    #[test]
    fn test_chat_mouse_handler() {
        let handler = ChatMouseHandler::new();
        let click_event = MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 5,
            row: 5,
            modifiers: KeyModifiers::NONE,
        };
        let scroll_up = MouseEvent {
            kind: MouseEventKind::ScrollUp,
            column: 5,
            row: 5,
            modifiers: KeyModifiers::NONE,
        };
        let action = handler.handle_chat_mouse(&click_event, (0, 0, 100, 100), 10, 0, 10);
        match action {
            ChatMouseAction::MessageClick(_) => {}
            _ => panic!("Expected MessageClick"),
        }
        assert_eq!(
            handler.handle_chat_mouse(&scroll_up, (0, 0, 100, 100), 10, 0, 10),
            ChatMouseAction::ScrollUp
        );
    }

    #[test]
    fn test_input_mouse_handler() {
        let handler = InputMouseHandler::new();
        let click_event = MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 5,
            row: 0,
            modifiers: KeyModifiers::NONE,
        };
        let (action, pos) = handler.handle_input_mouse(&click_event, (0, 0, 100, 1), 50);
        match action {
            InputMouseAction::SetCursor(p) => {
                assert_eq!(p, 5);
                assert_eq!(pos, Some(5));
            }
            _ => panic!("Expected SetCursor"),
        }
    }

    #[test]
    fn test_gesture_recognizer() {
        let mut recognizer =
            MouseGestureRecognizer::with_thresholds(10, std::time::Duration::from_millis(500));
        recognizer.start_gesture(10, 10);
        assert!(!recognizer.check_long_press());
        assert_eq!(recognizer.check_swipe(50, 10), Some(SwipeDirection::Right));
        recognizer.end_gesture();
    }
}
