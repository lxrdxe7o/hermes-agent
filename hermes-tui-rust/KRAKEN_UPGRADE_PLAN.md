# Hermes Rust TUI - Kraken Theme & Complete UI Overhaul

## Background & Motivation
The current Rust TUI for Hermes (`hermes-tui-rust`) is functional but visually basic, lacking the polished, immersive experience of the original Python TUI. The goal is to completely overhaul the Rust TUI layout to match the Python implementation pixel-perfectly, using the "Kraken" deep-sea Monokai theme (purple, green, and neon accents) and its associated ASCII branding. 

*Note: As requested, this plan will be moved to the `hermes-tui-rust` project root upon implementation.*

## Scope & Impact
- **Theming Engine:** Expanding `hermes-tui-rust/src/state/config.rs` to include the Kraken theme.
- **Layout Architecture:** Updating `app.rs` and `ui/mod.rs` to support a sticky banner, dynamic message bubbles, and rich status bars.
- **Visuals:** Adding ASCII art logos, thick block borders, specific tool/agent emojis (`🦑`, `≡`), and deep-sea Monokai color mappings.
- **Animations:** Implementing the Kraken "thinking verbs" and spinner states in the toolbar.

## Implementation Steps

### 1. Theme Configuration Updates (`src/state/config.rs`)
- Add `Kraken` to the `BuiltinTheme` enum.
- Define `kraken_theme()` with the deep-sea Monokai color palette:
  - Background: `#272822` (Input area) / `#1B1D1E` (Status bar)
  - Text: `#F8F8F2`
  - Primary/Success: `#A6E22E` (Neon Green)
  - Secondary/Border: `#AE81FF` (Purple)
  - Accent/Label: `#E6DB74` (Yellow)
  - Error: `#F92672` (Pink)
  - Warning: `#FD971F` (Orange)
- Map `ChatColors` to distinguish User, Assistant, Tool, and System bubbles with distinct background tints and border colors.

### 2. Banner & Sticky Header Component (`src/ui/banner.rs`)
- Create a new `banner.rs` component module (and expose it in `ui/mod.rs`).
- Embed the Kraken ASCII logo (the `██████` variant or the squid art) as a static ratatui `Paragraph` with styled spans.
- Update `app.rs` layout constraints to render this banner above the chat area, either as a sticky header or at the top of the initial chat scroll bounds.

### 3. Chat Area Refactor (`src/ui/chat.rs`)
- Refactor the generic list rendering into distinct **Message Bubbles**.
- **User Messages:** Align right or use a distinct muted background tint.
- **Assistant Messages:** Use the Kraken Purple (`#AE81FF`) border and prefix with ` ≡ Kraken `.
- Use `ratatui::widgets::Block` with `BorderType::Rounded` or `Thick` to visually separate each message.
- Add support for displaying Tool Emojis (`📜`, `🔍`, `🦑`) alongside tool execution blocks.

### 4. Input Composer Enhancement (`src/ui/composer.rs`)
- Update the input block to use a styled `Rule` above it (like the `input_rule` color `#75715E`).
- Use the prompt symbol `≡` instead of `>>`.
- Improve dynamic resizing to accommodate multi-line input gracefully, ensuring borders remain intact.

### 5. Toolbar and Spinner Integration (`src/ui/toolbar.rs`)
- Update the status bar background to `#1B1D1E` with `#E6DB74` text accents.
- Implement a spinner animation sequence using the Kraken waiting faces: `(≡)`, `(≌)`, `(‿)`, `(◈)`, `(Ψ)`, `(🦑)`.
- Randomize or cycle through the Kraken "thinking verbs" (e.g., *stirring the abyss*, *unfurling tentacles*, *sounding the trench*) when waiting for gateway responses.

## Verification
- Run `cargo run` within `hermes-tui-rust` to verify the UI compiles.
- Ensure the Kraken ASCII logo renders correctly without wrapping issues on standard terminal sizes (e.g., 80x24 minimum).
- Verify message bubbles correctly color-code based on role (User vs Kraken Agent).
- Trigger a command to observe the status bar spinner and thinking verbs.

## Migration & Rollback
- The `BuiltinTheme::Kraken` can be switched back to `BuiltinTheme::Dark` or `Default` via configuration if visual regressions occur.
- Git commits will be logically separated by component (Config -> Banner -> Chat -> Toolbar -> App Layout) to allow partial rollbacks if layout constraints conflict.
