// Two boxes: coordinates and facing in the top-left corner, biome, dimension
// and time in the top-right corner (the debug screen takes those corners).
fn simple_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    use crate::game::simple_hud as m;
    use horizon_client::extras::Corner;
    if !game.native_on(m::ID) {
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

    // Left box: X, Y, Z, then the facing. Right box: biome, dimension, then
    // the time, each line against the right edge.
    let mut left = panel.left;
    left.push(panel.bottom_left);
    let mut right = panel.right;
    if !panel.bottom_right.is_empty() {
        right.push(panel.bottom_right);
    }

    const ROW: f32 = 10.0;
    const PAD: f32 = 4.0;
    let width = |line: &m::Line| game.native_text_width(&m::text(line));
    let mut draw_box = |batch: &mut horizon_client::hud::HudBatch, corner: Corner, lines: &[m::Line]| {
        if lines.is_empty() || !game.native_corner_shown(corner) {
            return;
        }
        let inner = lines.iter().map(width).fold(0.0, f32::max);
        let w = PAD * 2.0 + inner;
        let h = PAD * 2.0 + ROW * lines.len() as f32 - 2.0;
        let (x, y) = corners.place(corner, w, h);
        game.native_fill(batch, [x, y, w, h], m::BACKGROUND);
        for (i, line) in lines.iter().enumerate() {
            // Left box: from the left edge; right box: against the right edge.
            let mut cx = if corner.left() { x + PAD } else { x + w - PAD - width(line) };
            let cy = y + PAD + ROW * i as f32;
            for (t, color) in line {
                game.native_text(batch, t, [cx, cy], *color, true);
                cx += game.native_text_width(t);
            }
        }
    };
    draw_box(batch, Corner::TopLeft, &left);
    draw_box(batch, Corner::TopRight, &right);
}
