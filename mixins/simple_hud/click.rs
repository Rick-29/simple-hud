// A click on the mod's settings page: flip that switch, and keep it.
fn simple_hud_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::simple_hud as m;
    if mod_id != m::ID {
        return;
    }
    let Some(&(key, _)) = m::SWITCHES.get(button) else {
        return;
    };
    let on = !m::flag(key, || game.native_setting(m::ID, key));
    m::set_flag(key, on);
    game.native_set_setting(m::ID, key, if on { "true" } else { "false" });
}
