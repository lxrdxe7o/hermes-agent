use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
};

const TOP_LEFT: &str = "┏";
const TOP_RIGHT: &str = "┓";
const BOTTOM_LEFT: &str = "┗";
const BOTTOM_RIGHT: &str = "┛";
const VERTICAL: &str = "┃";
const HORIZONTAL: &str = "━";

const DIM_TOP_LEFT: &str = "┌";
const DIM_TOP_RIGHT: &str = "┐";
const DIM_BOTTOM_LEFT: &str = "└";
const DIM_BOTTOM_RIGHT: &str = "┘";
const DIM_VERTICAL: &str = "│";
const DIM_HORIZONTAL: &str = "─";

/// Renders an animated gradient border if focused, otherwise a dim standard border.
pub fn render_gradient_border(buf: &mut Buffer, area: Rect, animation_frame: u64, focused: bool) {
    if area.width < 2 || area.height < 2 {
        return;
    }

    if !focused {
        render_dim_border(buf, area);
        return;
    }

    // Base time factor for animations
    let t = animation_frame as f64 * 0.12;

    // Corners
    draw_border_cell(buf, area.x, area.y, area, t, TOP_LEFT);
    draw_border_cell(buf, area.x + area.width - 1, area.y, area, t, TOP_RIGHT);
    draw_border_cell(buf, area.x, area.y + area.height - 1, area, t, BOTTOM_LEFT);
    draw_border_cell(
        buf,
        area.x + area.width - 1,
        area.y + area.height - 1,
        area,
        t,
        BOTTOM_RIGHT,
    );

    // Top and Bottom
    for x in (area.x + 1)..(area.x + area.width - 1) {
        draw_border_cell(buf, x, area.y, area, t, HORIZONTAL);
        draw_border_cell(buf, x, area.y + area.height - 1, area, t, HORIZONTAL);
    }

    // Left and Right
    for y in (area.y + 1)..(area.y + area.height - 1) {
        draw_border_cell(buf, area.x, y, area, t, VERTICAL);
        draw_border_cell(buf, area.x + area.width - 1, y, area, t, VERTICAL);
    }
}

fn draw_border_cell(buf: &mut Buffer, x: u16, y: u16, area: Rect, t: f64, symbol: &str) {
    if x < buf.area.width && y < buf.area.height {
        let cell = &mut buf[(x, y)];
        
        // Only overwrite the symbol if it's a standard border character or empty/space
        // This preserves titles rendered by Block::title()
        let current_sym = cell.symbol();
        if current_sym == " " || current_sym.is_empty() || is_border_symbol(current_sym) {
            cell.set_symbol(symbol);
        }

        let color = get_color(x - area.x, y - area.y, area.width, area.height, t);
        
        // Apply color and reset background/modifiers to prevent artifacts
        cell.fg = color;
        cell.bg = Color::Reset;
        cell.modifier = Modifier::empty();
    }
}

fn is_border_symbol(sym: &str) -> bool {
    matches!(sym, 
        "┌" | "┐" | "└" | "┘" | "│" | "─" | // Light
        "┏" | "┓" | "┗" | "┛" | "┃" | "━" | // Thick
        "╔" | "╗" | "╚" | "╝" | "║" | "═" | // Double
        "╭" | "╮" | "╯" | "╰"               // Rounded
    )
}

fn get_perimeter_pos(x: u16, y: u16, width: u16, height: u16) -> f64 {
    let w = width.saturating_sub(1) as f64;
    let h = height.saturating_sub(1) as f64;
    let xf = x as f64;
    let yf = y as f64;

    if yf == 0.0 {
        xf
    } else if xf == w {
        w + yf
    } else if yf == h {
        w + h + (w - xf)
    } else {
        w + h + w + (h - yf)
    }
}

fn calculate_pulse(pos: f64, perimeter: f64, t_scaled: f64, width: f64) -> f64 {
    if perimeter <= 0.0 { return 0.0; }
    let center = t_scaled.rem_euclid(perimeter);
    let mut d = (pos - center).abs();
    if d > perimeter / 2.0 {
        d = perimeter - d;
    }
    if d < width {
        let val = 1.0 - d / width;
        val * val // Quadratic falloff for sharper, neon-like pulses
    } else {
        0.0
    }
}

fn get_color(x: u16, y: u16, width: u16, height: u16, t: f64) -> Color {
    let p_pos = get_perimeter_pos(x, y, width, height);
    let w = width.saturating_sub(1) as f64;
    let h = height.saturating_sub(1) as f64;
    let perimeter = 2.0 * (w + h);
    
    if perimeter <= 0.0 {
        return Color::Rgb(100, 100, 100);
    }

    let norm_p = p_pos / perimeter;

    // 1. Base Gradient (Multi-frequency with Chromatic Shift)
    // We use different speeds for R, G, and B to create a prismatic effect.
    let r_base = (norm_p * 6.28 + t * 0.5).sin() * 0.5 + 0.5;
    let g_base = (norm_p * 6.28 + t * 0.65).sin() * 0.5 + 0.5;
    let b_base = (norm_p * 6.28 + t * 0.8).sin() * 0.5 + 0.5;

    // Add higher-frequency jitter for organic "aetheric" richness
    let r_rich = (r_base + (norm_p * 12.5 + t * 1.2).sin() * 0.15) / 1.15;
    let g_rich = (g_base + (norm_p * 15.0 + t * 1.4).sin() * 0.15) / 1.15;
    let b_rich = (b_base + (norm_p * 10.0 + t * 1.0).sin() * 0.15) / 1.15;

    // 2. Traveling Pulses (Bright traveling lights)
    // Pulse 1: Fast, forward, cyan/white leaning
    let p1 = calculate_pulse(p_pos, perimeter, t * 12.0, 8.0);
    // Pulse 2: Slower, backward, magenta leaning
    let p2 = calculate_pulse(p_pos, perimeter, -t * 6.0, 12.0);
    // Pulse 3: Very fast, forward, prismatic/white
    let p3 = calculate_pulse(p_pos, perimeter, t * 25.0, 4.0);

    // 3. Corner Flares (Bloom on corners)
    let is_corner = (x == 0 || x == width - 1) && (y == 0 || y == height - 1);
    let flare = if is_corner {
        ((t * 4.0).sin() * 0.5 + 0.5).powi(2) * 0.8
    } else {
        0.0
    };

    // 4. Final Color Composition
    // Mix the components. Pulses and flares add intensity.
    let mut r = r_rich * 0.3 + p1 * 0.2 + p2 * 0.8 + p3 * 1.0 + flare;
    let mut g = g_rich * 0.3 + p1 * 0.8 + p2 * 0.2 + p3 * 1.0 + flare;
    let mut b = b_rich * 0.3 + p1 * 1.0 + p2 * 0.5 + p3 * 1.0 + flare;

    // Subtly pulsate overall brightness (Global Shimmer)
    let shimmer = (t * 1.5).sin() * 0.05 + 0.95;
    r *= shimmer;
    g *= shimmer;
    b *= shimmer;

    // Convert to RGB888 with clamping
    let r_u8 = (r * 255.0).clamp(0.0, 255.0) as u8;
    let g_u8 = (g * 255.0).clamp(0.0, 255.0) as u8;
    let b_u8 = (b * 255.0).clamp(0.0, 255.0) as u8;

    Color::Rgb(r_u8, g_u8, b_u8)
}

fn render_dim_border(buf: &mut Buffer, area: Rect) {
    let style = Style::default().fg(Color::Indexed(240));

    // Corners
    draw_dim_cell(buf, area.x, area.y, DIM_TOP_LEFT, style);
    draw_dim_cell(buf, area.x + area.width - 1, area.y, DIM_TOP_RIGHT, style);
    draw_dim_cell(buf, area.x, area.y + area.height - 1, DIM_BOTTOM_LEFT, style);
    draw_dim_cell(buf, area.x + area.width - 1, area.y + area.height - 1, DIM_BOTTOM_RIGHT, style);

    // Top and Bottom
    for x in (area.x + 1)..(area.x + area.width - 1) {
        draw_dim_cell(buf, x, area.y, DIM_HORIZONTAL, style);
        draw_dim_cell(buf, x, area.y + area.height - 1, DIM_HORIZONTAL, style);
    }

    // Left and Right
    for y in (area.y + 1)..(area.y + area.height - 1) {
        draw_dim_cell(buf, area.x, y, DIM_VERTICAL, style);
        draw_dim_cell(buf, area.x + area.width - 1, y, DIM_VERTICAL, style);
    }
}

fn draw_dim_cell(buf: &mut Buffer, x: u16, y: u16, symbol: &str, style: Style) {
    if x < buf.area.width && y < buf.area.height {
        let cell = &mut buf[(x, y)];
        
        let current_sym = cell.symbol();
        if current_sym == " " || current_sym.is_empty() || is_border_symbol(current_sym) {
            cell.set_symbol(symbol);
        }
        
        // Explicitly reset to ensure no artifacts
        cell.bg = Color::Reset;
        cell.modifier = Modifier::empty();
        
        if let Some(fg) = style.fg {
            cell.fg = fg;
        }
    }
}
