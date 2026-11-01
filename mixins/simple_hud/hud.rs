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
    if !m::shown(|| game.native_setting(m::ID, "shown")) {
        return;
    }
    let Some(p) = game.native_player() else {
        return;
    };
    let day_time = game.native_clock().map(|c| c.day_time);
    let lines = m::lines(p.x, p.y, p.z, p.yaw, p.dimension, day_time);
    let w = lines.iter().map(|(t, _)| game.native_text_width(t)).fold(0.0, f32::max) + 4.0;
    let h = 10.0 * lines.len() as f32 + 2.0;
    let (x, y) = corners.place(Corner::TopLeft, w, h);
    game.native_fill(batch, [x, y, w, h], m::BACKGROUND);
    for (i, (text, color)) in lines.iter().enumerate() {
        game.native_text(batch, text, [x + 2.0, y + 2.0 + 10.0 * i as f32], *color, true);
    }
}
