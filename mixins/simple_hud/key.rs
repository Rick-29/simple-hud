// The toggle key (H unless the player picks another in Options > Mods):
// shows or hides the box, and keeps the choice. Only while playing, so typing
// in chat or a sign doesn't toggle it.
fn simple_hud_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::simple_hud as m;
    if !pressed || !game.native_on(m::ID) || !game.native_playing() {
        return;
    }
    if key != game.native_mod_key(m::ID, m::KEY, m::DEFAULT_KEY) {
        return;
    }
    let shown = !m::shown(|| game.native_setting(m::ID, "shown"));
    m::set_shown(shown);
    game.native_set_setting(m::ID, "shown", if shown { "true" } else { "false" });
}
