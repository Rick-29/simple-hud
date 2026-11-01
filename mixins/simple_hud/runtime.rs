// Simple HUD: a small "F3-lite" box in the top-left corner with the player's
// coordinates, facing, the day and time, and the dimension, each line in its
// own colour. `/hud` hides or shows it, `/where` puts the position in chat.
// This file is both the mod's Cargo library (its tests) and the module the
// bundle adds to the game (`crate::game::simple_hud`).

pub const ID: &str = "simple_hud";
pub const NAME: &str = "Simple HUD";
/// The toggle key: its name, and the key it starts on (rebindable in Options > Mods).
pub const KEY: &str = "toggle";
pub const DEFAULT_KEY: &str = "key.keyboard.h";
pub const VERSION: &str = "0.1.0";
pub const DESCRIPTION: &str =
    "Coordinates, facing, time of day and dimension in a corner of the screen. /hud hides or shows it, /where puts your position in chat.";

/// Translucent black behind the text (0xAARRGGBB).
pub const BACKGROUND: u32 = 0x7800_0000;
// Text colours (0xRRGGBB).
pub const WHITE: u32 = 0xFF_FFFF;
pub const YELLOW: u32 = 0xFF_FF55;
pub const DIAMOND: u32 = 0x55_FFFF;
pub const NETHER: u32 = 0xFF_5555;
pub const END: u32 = 0xD0_8CFF;
pub const OVERWORLD: u32 = 0x55_FF55;

/// One line of the box: its text and colour.
pub type Line = (String, u32);

/// Direction.fromYRot: yaw 0 faces south, 90 west.
pub fn facing(yaw: f32) -> &'static str {
    let i = (yaw / 90.0 + 0.5).floor() as i32 & 3;
    ["South (+Z)", "West (-X)", "North (-Z)", "East (+X)"][i as usize]
}

/// `Day 5, 14:07` from the time of day in ticks (0 is 06:00 on day 1).
pub fn time_line(day_time: i64) -> String {
    let day = day_time.div_euclid(24000) + 1;
    let t = day_time.rem_euclid(24000);
    let minutes = (t + 6000) % 24000 * 60 / 1000; // minutes since midnight
    format!("Day {day}, {:02}:{:02}", minutes / 60, minutes % 60)
}

/// `minecraft:the_nether` -> `Nether`, and its colour.
pub fn dimension(id: &str) -> (&str, u32) {
    match id {
        "minecraft:overworld" => ("Overworld", OVERWORLD),
        "minecraft:the_nether" => ("Nether", NETHER),
        "minecraft:the_end" => ("The End", END),
        other => (other.strip_prefix("minecraft:").unwrap_or(other), WHITE),
    }
}

/// The coordinates' colour: red in the Nether, cyan at diamond depth.
pub fn coords_color(y: f64, dim: &str) -> u32 {
    match dim {
        "minecraft:the_nether" => NETHER,
        "minecraft:overworld" if (-64.0..=16.0).contains(&y) => DIAMOND,
        _ => WHITE,
    }
}

/// The box's lines. `day_time` is none when the clock can't be read.
pub fn lines(x: f64, y: f64, z: f64, yaw: f32, dim: &str, day_time: Option<i64>) -> Vec<Line> {
    let (dim_name, dim_color) = dimension(dim);
    let mut out = vec![
        (format!("XYZ {x:.1} / {y:.1} / {z:.1}"), coords_color(y, dim)),
        (format!("Facing {}", facing(yaw)), WHITE),
    ];
    if let Some(t) = day_time {
        out.push((time_line(t), YELLOW));
    }
    out.push((format!("Dimension: {dim_name}"), dim_color));
    out
}

/// `/where`'s line: `12, 70, -4 in minecraft:the_nether`.
pub fn where_line(x: f64, y: f64, z: f64, dimension: &str) -> String {
    format!("{:.0}, {:.0}, {:.0} in {dimension}", x.floor(), y.floor(), z.floor())
}

/// The stored `shown` value: shown unless it says `false`.
pub fn read_shown(stored: Option<String>) -> bool {
    stored.is_none_or(|v| v.trim() != "false")
}

pub fn shown_label(shown: bool) -> String {
    format!("Shown: {}", if shown { "ON" } else { "OFF" })
}

thread_local! {
    /// Whether the box shows, read once from its file, then kept.
    static SHOWN: std::cell::Cell<Option<bool>> = const { std::cell::Cell::new(None) };
}

/// Whether the box shows, read once through `get` (the `shown` file).
pub fn shown(get: impl FnOnce() -> Option<String>) -> bool {
    SHOWN.with(|s| {
        let found = s.get().unwrap_or_else(|| read_shown(get()));
        s.set(Some(found));
        found
    })
}

/// Replaces it (`/hud` or the settings page changed it).
pub fn set_shown(shown: bool) {
    SHOWN.with(|s| s.set(Some(shown)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn facing_follows_vanilla_yaw() {
        assert_eq!(facing(0.0), "South (+Z)");
        assert_eq!(facing(90.0), "West (-X)");
        assert_eq!(facing(-180.0), "North (-Z)");
        assert_eq!(facing(-90.0), "East (+X)");
    }

    #[test]
    fn time_of_day() {
        assert_eq!(time_line(0), "Day 1, 06:00");
        assert_eq!(time_line(6000), "Day 1, 12:00");
        assert_eq!(time_line(18000), "Day 1, 00:00");
        assert_eq!(time_line(24000 * 4 + 8116), "Day 5, 14:06");
        assert_eq!(time_line(-1000), "Day 0, 05:00");
    }

    #[test]
    fn colours_and_lines() {
        assert_eq!(coords_color(-58.0, "minecraft:overworld"), DIAMOND);
        assert_eq!(coords_color(70.0, "minecraft:overworld"), WHITE);
        assert_eq!(coords_color(70.0, "minecraft:the_nether"), NETHER);
        let l = lines(12.5, 64.0, -3.25, 180.0, "minecraft:the_end", Some(0));
        assert_eq!(l[0].0, "XYZ 12.5 / 64.0 / -3.2");
        assert_eq!(l[1].0, "Facing North (-Z)");
        assert_eq!(l[2], ("Day 1, 06:00".into(), YELLOW));
        assert_eq!(l[3], ("Dimension: The End".into(), END));
        assert_eq!(lines(0.0, 0.0, 0.0, 0.0, "minecraft:overworld", None).len(), 3);
    }

    #[test]
    fn shown_unless_stored_false() {
        assert!(read_shown(None));
        assert!(!read_shown(Some("false\n".into())));
        assert_eq!(shown_label(false), "Shown: OFF");
    }
}
