// /where puts the position in chat; /hud hides or shows the box.
fn simple_hud_command(game: &mut crate::game::Game<'_>, command: &str, ran: &mut bool) {
    use crate::game::simple_hud as m;
    let name = command.split(' ').next().unwrap_or("");
    if !game.native_on(m::ID) || !matches!(name, "where" | "hud") {
        return;
    }
    *ran = true;
    if name == "hud" {
        let shown = !m::shown(|| game.native_setting(m::ID, "shown"));
        m::set_shown(shown);
        game.native_set_setting(m::ID, "shown", if shown { "true" } else { "false" });
        game.native_chat(if shown { "HUD shown" } else { "HUD hidden" });
        return;
    }
    match game.native_player() {
        Some(p) => game.native_chat(&m::where_line(p.x, p.y, p.z, p.dimension)),
        None => game.native_chat("Not in a world"),
    }
}
