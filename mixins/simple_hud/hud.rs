// The box in the top-left corner (the debug screen takes that corner).
fn simple_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    use crate::game::simple_hud as m;
    use horizon_client::extras::Corner;
    if !game.native_on(m::ID) || !game.native_corner_shown(Corner::TopLeft) {
        return;
    }
    if !m::shown(|| game.native_setting(m::ID, m::SHOWN.0)) {
        return;
    }
    let Some(p) = game.native_player() else {
        return;
    };
    let ores = m::flag(m::ORES.0, || game.native_setting(m::ID, m::ORES.0));
    let day_time = game.native_clock().map(|c| c.day_time);

    // The biome at the feet, read as the F3 screen reads it (debug_lines.rs):
    // there is no hook point for it, so this follows the game's own fields
    // and may need a change when Horizon renames them.
    let [bx, by, bz] = [p.x, p.y, p.z].map(|v| v.floor() as i32);
    let biome = (by >= game.world.bottom_y() && by < game.world.top_y())
        .then(|| game.world.chunks.get(&(bx >> 4, bz >> 4)))
        .flatten()
        .and_then(|chunk| game.res.biomes().list.get(chunk.biome(bx, by, bz) as usize))
        .map(|b| b.name.clone());

    let panel = m::panel([p.x, p.y, p.z], p.yaw, p.dimension, biome.as_deref(), day_time, ores);
    let game: &crate::game::Game<'_> = game;

    // Layout, in GUI pixels: rows of 10, a gap between the columns, and the
    // divider row above the facing and the time.
    const ROW: f32 = 10.0;
    const PAD: f32 = 4.0;
    const GAP: f32 = 16.0;
    let width = |line: &m::Line| game.native_text_width(&m::text(line));
    let widest = |lines: &[m::Line]| lines.iter().map(width).fold(0.0, f32::max);
    let left_w = widest(&panel.left).max(width(&panel.bottom_left));
    let right_w = widest(&panel.right).max(width(&panel.bottom_right));
    let rows = panel.left.len().max(panel.right.len()) as f32;
    let w = PAD * 2.0 + left_w + GAP + right_w;
    let h = PAD * 2.0 + ROW * rows + 5.0 + ROW;
    let (x, y) = corners.place(Corner::TopLeft, w, h);
    game.native_fill(batch, [x, y, w, h], m::BACKGROUND);

    // Draws a line's pieces one after another from `at`.
    let draw = |batch: &mut horizon_client::hud::HudBatch, line: &m::Line, at: [f32; 2]| {
        let mut cx = at[0];
        for (t, color) in line {
            game.native_text(batch, t, [cx, at[1]], *color, true);
            cx += game.native_text_width(t);
        }
    };
    let right_edge = x + w - PAD;
    for (i, line) in panel.left.iter().enumerate() {
        draw(batch, line, [x + PAD, y + PAD + ROW * i as f32]);
    }
    for (i, line) in panel.right.iter().enumerate() {
        draw(batch, line, [right_edge - width(line), y + PAD + ROW * i as f32]);
    }
    let div_y = y + PAD + ROW * rows + 1.0;
    game.native_fill(batch, [x + PAD, div_y, w - PAD * 2.0, 1.0], m::DIVIDER);
    let bottom_y = div_y + 4.0;
    draw(batch, &panel.bottom_left, [x + PAD, bottom_y]);
    draw(batch, &panel.bottom_right, [right_edge - width(&panel.bottom_right), bottom_y]);
}
