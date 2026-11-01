// The mod's settings page: one button, the box shown or hidden.
fn simple_hud_page(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::simple_hud as m;
    if mod_id != m::ID {
        return;
    }
    buttons.push(m::shown_label(m::shown(|| game.native_setting(m::ID, "shown"))));
}
