// The mod's row in Options > Mods.
fn simple_hud_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::simple_hud as m;
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: vec![[m::KEY.into(), "Show or hide the HUD".into(), m::DEFAULT_KEY.into()]],
    });
}
