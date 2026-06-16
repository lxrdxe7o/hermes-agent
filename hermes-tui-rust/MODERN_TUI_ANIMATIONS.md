# Modern TUI Animations: The Hermes Surface Paradigm (2025/2026)

## 1. Vision: The "Aetheric Command Center"
Hermes is a multi-tier, modular AI agent. The TUI is its **Primary Command Surface**. The aesthetic must reflect the agent's core identity: **Professional, Secure, and Intelligent.** We aim for a "Bioluminescent Deep-Sea" look—dark backgrounds with soft, organic glows that signal cognitive activity without distracting from the data.

---

## 2. Agent-Integrated Animation Stack
Animations in Hermes are not just "flair"; they are **visual telemetry** linked directly to the JSON-RPC protocol events from the Gateway.

| Event (Protocol) | Visual Effect | Functional Purpose |
| :--- | :--- | :--- |
| `message.delta` | **Neural Evolve** | Signals "decryption" of thought into language. |
| `tool.start` | **Bioluminescent Pulse** | Indicates active background execution/sandboxing. |
| `message.complete`| **Coalesce** | Finalizes the response, signaling the end of the turn. |
| `status.change` | **Color Sweep** | Smoothly updates connectivity (e.g., Online -> Offline). |
| `error` | **Aetheric Shake** | High-visibility signal for Tirith security/logic errors. |

---

## 3. protocol-Linked Visual Patterns

### A. Non-Blocking "Neural Evolve"
When `MessageDelta` arrives, we apply a "cipher" effect to the latest word chunk. To ensure we don't break the streaming experience, the animation must be **stateless and additive**.

```rust
// Logic: Animate only the most recent chunk of a streaming message
fn render_streaming_delta(manager: &mut EffectManager, text: &str, area: Rect) {
    if is_new_delta(text) {
        manager.add_effect(
            fx::evolve(300), // Quick 300ms evolve
            area,
            CellFilter::AllNonEmpty
        );
    }
}
```

### B. "Tirith" Security Approval Pulse
Hermes uses the **Tirith** security layer. When an approval is required, the UI shouldn't just wait; it should "demand" attention using a high-fidelity, high-contrast glow.

**TachyonFX DSL for Approval:**
```rust
let approval_glow = fx::sequence(&[
    fx::hsl_shift_fg(20.0, 1.0, 0.5, 400), // Brighten
    fx::hsl_shift_fg(-20.0, -1.0, -0.5, 400), // Dim
]).repeat(); // Pulse until approved
```

### C. "Episodic" Memory Reveal
When Hermes recalls a memory (`semantic_recall`), use a **Spatial Reveal** (Spiral or Diamond) from the center of the memory card to symbolize the "retrieval" from the SQLite FTS5 backend.

---

## 4. Architectural Safeguards (Don't Break the Agent)

### 1. The "EffectManager" Isolation
The `EffectManager` must process frames on a **separate budget** from the JSON-RPC parser.
- **Rule:** If the protocol thread is saturated, the `EffectManager` should drop frames (graceful degradation) to ensure no message latency.
- **Implementation:** Use a `crossbeam-channel` to send "Effect Requests" from the protocol handler to the UI thread.

### 2. Stateless Rendering
Widgets should remain **pure functions of the state**. Animations are "post-processing" overlays.
- **Why:** This ensures that if the animation engine fails, the TUI remains 100% functional with standard Ratatui rendering.

### 3. Sixel/Kitty Buffer Safety
Since Hermes supports **SIXEL GIFs**, the animation loop must be aware of the "SIXEL lock."
- **Rule:** Do not apply `tachyonfx` filters to SIXEL-occupied cells. Use a `CellFilter` to bypass image regions, preventing artifacts and crashes.

---

## 5. Developer Implementation Checklist

1.  [ ] **Protocol Mapping**: Bind `GatewayEvent` types to specific `EffectRequest` triggers.
2.  [ ] **Performance Profiling**: Ensure `terminal.draw` + `effect_manager.process` stays under **12ms** (leaving 4ms buffer for OS/IO).
3.  [ ] **Color Space Integrity**: Use **OKLab** for all HSL shifts to maintain the professional "dark" aesthetic without perceptual lightness spikes.
4.  [ ] **Fallback Mode**: Implement a `--no-animation` flag that completely bypasses the `EffectManager` for legacy terminals (Tirith compliance).

---

*Refined for Hermes Agent Tier-1 Surface Compliance.*
*Synthesized from Online Documentation & Protocol Analysis.*
