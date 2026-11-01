// The mod's settings page: one button per switch (shown, ore colours).
fn simple_hud_page(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::simple_hud as m;
    if mod_id != m::ID {
        return;
    }
    for (key, label) in m::SWITCHES {
        let on = m::flag(key, || game.native_setting(m::ID, key));
        buttons.push(m::flag_label(label, on));
    }
}
