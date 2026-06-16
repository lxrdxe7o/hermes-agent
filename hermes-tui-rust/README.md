# Hermes TUI Rust

A Rust-based Terminal User Interface for Hermes Agent, inspired by oh-my-pi's TUI.

## Status

🚧 **Under Development** - This is a work in progress to port oh-my-pi's TUI features to Hermes Agent.

## Overview

This project provides a fast, native terminal experience for Hermes Agent users, with full compatibility to the existing JSON-RPC protocol used by the TypeScript TUI.

## Features (Planned)

- [x] Core chat interface with streaming and Markdown support
- [x] View system with multi-tab switching (Dashboard, IDE, Kanban, Chat)
- [x] Animated Sixel GIF logo support (`bebop.gif`)
- [x] Gruvbox theme (default) with high-contrast palette
- [x] Integrated completion system (slash commands and file paths)
- [x] Categorized model picker with provider-level grouping
- [x] Enhanced prompt flows for approvals, clarify, and secrets
- [x] Native mouse support (scrolling) and bracketed paste
- [x] Improved tool card rendering with automatic text wrapping

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    HERMES CORE (Python)                       │
│  ┌─────────────────────────────────────────────────────────┐│
│  │                 tui_gateway/server.py                        ││
│  │  JSON-RPC Server - Sessions, Agents, Tool Execution         ││
│  └─────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────┘
                              │ JSON-RPC over stdio
                              ▼
┌─────────────────────────────────────────────────────────────┐
│  hermes-tui-rust (Rust/ratatui)                              │
│  - Protocol layer (serde_json, tokio)                         │
│  - UI layer (ratatui, crossterm)                              │
│  - State management                                           │
│  - Event handling                                            │
└─────────────────────────────────────────────────────────────┘
```

## Protocol

Uses the exact same JSON-RPC protocol as the existing TypeScript TUI. See:
- `tui_gateway/server.py` - Server implementation
- `ui-tui/src/gatewayTypes.ts` - TypeScript type definitions

The Rust implementation mirrors these types in `src/protocol/types.rs`.

## Usage

### As standalone binary

```bash
# Build
cargo build --release

# Run (connects to hermes gateway via stdio)
./target/release/hermes-tui-rust
```

### Via Hermes installer

```bash
# The Rust TUI will be automatically installed and updated
# via the Hermes installer (hermes update)
hermes update
hermes --tui-rust
```

### Configuration

```yaml
# In ~/.hermes/config.yaml
display:
  interface: rust  # Use Rust TUI
```

Or via environment variable:
```bash
export HERMES_TUI_RUST=1
hermes
```

## Development

### Prerequisites

- Rust 1.75+
- Cargo
- Python 3.11+ (for Hermes gateway)

### Setup

```bash
cd hermes-tui-rust
cargo build
```

### Running

The Rust TUI communicates with the Hermes gateway via JSON-RPC over stdio. To test:

```bash
# In one terminal: Start the gateway
python -m tui_gateway.entry

# In another terminal: Run the TUI (connected via stdio pipe)
# Note: Actual integration will be handled by hermes_cli/main.py
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_function_name

# Run with logging
RUST_LOG=debug cargo test
```

## Project Structure

```
hermes-tui-rust/
├── Cargo.toml                 # Dependencies and build configuration
├── README.md                  # This file
├── build.rs                   # Build script (version info)
├── src/
│   ├── main.rs               # Entry point
│   ├── app.rs                # Main App struct and event loop
│   ├── ui/                   # UI components
│   │   ├── mod.rs            # UI module exports
│   │   ├── chat.rs           # Chat transcript rendering
│   │   ├── dashboard.rs      # Main dashboard with animated telemetry
│   │   ├── ide.rs            # 3-column IDE layout prototype
│   │   ├── kanban.rs         # Kanban board layout prototype
│   │   ├── gif.rs            # Sixel-based animated GIF playback
│   │   ├── composer.rs       # Input composer with history
│   │   ├── toolbar.rs        # Status bar with animations
│   │   ├── cards.rs          # Tool/message result cards
│   │   └── prompts.rs        # Selectable choice overlays
│   ├── protocol/             # JSON-RPC protocol
│   │   ├── mod.rs            # Protocol module exports
│   │   ├── types.rs          # Message types
│   │   ├── client.rs         # JSON-RPC client
│   │   └── transport.rs      # stdio transport
│   ├── state/                # Application state
│   │   ├── mod.rs            # State module exports
│   │   ├── session.rs        # Session management
│   │   ├── config.rs         # Config loading
│   │   └── messages.rs       # Message history
│   ├── handlers/             # Event handlers
│   │   ├── mod.rs            # Handlers module exports
│   │   ├── input.rs          # Input handling
│   │   ├── keys.rs           # Keybindings
│   │   └── mouse.rs          # Mouse events
│   └── utils/               # Utilities
│       ├── mod.rs            # Utils module exports
│       ├── text.rs           # Text wrapping/utilities
│       └── ansi.rs           # ANSI code handling
├── tests/
│   ├── protocol/            # Protocol tests
│   │   ├── test_types.rs     # Type serialization tests
│   │   ├── test_client.rs    # Client tests
│   │   └── test_transport.rs # Transport tests
│   ├── ui/                  # UI tests
│   │   ├── test_chat.rs      # Chat tests
│   │   └── test_composer.rs # Composer tests
│   ├── integration/         # Integration tests
│   │   ├── test_session.rs   # Session tests
│   │   └── test_tools.rs     # Tool tests
│   └── mock/                # Mock gateway for testing
│       ├── server.rs        # Mock server
│       └── messages.rs      # Mock messages
└── .gitignore
```

## Quality Gates

All code must pass through these gates before being merged:

1. **Compilation**: Code must compile without warnings
2. **Tests**: All tests must pass
3. **Clippy**: `cargo clippy` must pass
4. **Format**: `cargo fmt` must be clean
5. **Review**: Code review by at least one other developer
6. **Protocol Compatibility**: Must not break existing JSON-RPC protocol
7. **Fallback**: TypeScript TUI must still work as fallback

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes following [Conventional Commits](https://www.conventionalcommits.org/)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

MIT License - see LICENSE file for details.

## Acknowledgments

- Inspired by [oh-my-pi](https://github.com/can1357/oh-my-pi) TUI
- Built with [ratatui](https://github.com/ratatui-org/ratatui) and [crossterm](https://github.com/crossterm-rs/crossterm)
- Part of [Hermes Agent](https://github.com/NousResearch/hermes-agent)
