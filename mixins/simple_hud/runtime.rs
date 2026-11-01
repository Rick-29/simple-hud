// Simple HUD: a small box in the top-left corner.
//
//   X 124.5                  Deep Dark
//   Y -54.0  Redstone        OVERWORLD
//   Z -342.8
//   ───────────────────────────────────
//   NW 315°               14:32  Day 8
//
// Left: the coordinates (Y coloured by the ores at that height; at diamond
// depth all three turn cyan) and the facing. Right: the biome (coloured by
// kind), the dimension and the time. `/hud` or the toggle key (H) hides or
// shows it, `/where` puts the position in chat; Options > Mods > Simple HUD
// has both switches. This file is both the mod's Cargo library (its tests)
// and the module the bundle adds to the game (`crate::game::simple_hud`).

pub const ID: &str = "simple_hud";
pub const NAME: &str = "Simple HUD";
/// The toggle key: its name, and the key it starts on (rebindable in Options > Mods).
pub const KEY: &str = "toggle";
pub const DEFAULT_KEY: &str = "key.keyboard.h";
pub const VERSION: &str = "0.2.0";
pub const DESCRIPTION: &str = "Coordinates, facing, biome, dimension and time in a corner of the screen. H or /hud hides or shows it, /where puts your position in chat.";

/// Translucent black behind the text, and the divider (0xAARRGGBB).
pub const BACKGROUND: u32 = 0x9000_0000;
pub const DIVIDER: u32 = 0x40FF_FFFF;
// Text colours (0xRRGGBB).
pub const WHITE: u32 = 0xFF_FFFF;
pub const LABEL: u32 = 0x9A_9AA8;
pub const DIM_INK: u32 = 0xA8_B4CC;
pub const TIME: u32 = 0xF2_C55C;

/// A line drawn as pieces of text side by side, each in its own colour.
pub type Line = Vec<(String, u32)>;

/// The box's content: rows of the left column, rows of the right column
/// (drawn right-aligned), and the bottom row under the divider.
#[derive(Clone, Debug, PartialEq)]
pub struct Panel {
    pub left: Vec<Line>,
    pub right: Vec<Line>,
    pub bottom_left: Line,
    pub bottom_right: Line,
}

// ------------------------------------------------------------------ ores

/// An ore band: from this Y up (inclusive), its name and colour.
/// Overworld only, roughly where each ore is most common in 26.3.
pub const ORE_BANDS: [(f64, &str, u32); 5] = [
    (96.0, "Coal", 0x80_8080),
    (48.0, "Iron", 0xD8_AF93),
    (16.0, "Gold", 0xFF_D83D),
    (-16.0, "Lapis", 0x5B_7CFF),
    (-48.0, "Redstone", 0xFF_4D4D),
];
pub const DIAMOND: (&str, u32) = ("Diamond", 0x4D_F0E6);

/// The ore at height `y`: below the last band it is diamond.
pub fn ore_at(y: f64) -> (&'static str, u32) {
    ORE_BANDS
        .iter()
        .find(|(from, _, _)| y >= *from)
        .map(|&(_, name, color)| (name, color))
        .unwrap_or(DIAMOND)
}

// --------------------------------------------------------------- biomes

/// `minecraft:dark_forest` -> `Dark Forest`.
pub fn biome_name(id: &str) -> String {
    let path = id.rsplit(':').next().unwrap_or(id);
    path.split('_')
        .filter(|w| !w.is_empty())
        .map(|w| {
            let mut c = w.chars();
            c.next().map(|f| f.to_uppercase().chain(c).collect::<String>()).unwrap_or_default()
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// A colour for a biome, by its kind (the first match wins).
pub fn biome_color(id: &str) -> u32 {
    let b = id.rsplit(':').next().unwrap_or(id);
    let has = |s: &str| b.contains(s);
    match () {
        _ if has("deep_dark") || has("dripstone") => 0x8C_8C8C, // caves: grey
        _ if has("lush_caves") => 0x7E_D957,
        _ if has("frozen") || has("snowy") || has("ice") || has("grove") => 0xDC_F0FF,
        _ if has("ocean") => 0x4A_8CFF,
        _ if has("river") => 0x6E_B4FF,
        _ if has("beach") => 0xF0_E2A8,
        _ if has("desert") => 0xE8_D4A0, // beige
        _ if has("badlands") => 0xE0_8A4A,
        _ if has("savanna") => 0xD4_C060,
        _ if has("swamp") => 0x8E_A060,
        _ if has("jungle") || has("bamboo") => 0x4C_D05A,
        _ if has("cherry") => 0xFF_B0D8,
        _ if has("mushroom") => 0xD0_8CE0,
        _ if has("taiga") => 0x5E_A878,
        _ if has("dark_forest") || has("pale_garden") => 0x6A_9A5A,
        _ if has("forest") || has("plains") || has("meadow") || has("sunflower") => 0x9C_E07A,
        _ if has("peaks") || has("hills") || has("windswept") || has("stony") => 0xC8_C8C8,
        _ if has("nether") || has("crimson") => 0xFF_6A5A,
        _ if has("warped") => 0x40_D8C0,
        _ if has("soul_sand") || has("basalt") => 0xA0_9890,
        _ if has("end") || has("void") => 0xD0_A0FF,
        _ => WHITE,
    }
}

/// `minecraft:the_nether` -> `NETHER`, as the reference's small caps.
pub fn dimension_name(id: &str) -> String {
    match id {
        "minecraft:overworld" => "OVERWORLD".into(),
        "minecraft:the_nether" => "NETHER".into(),
        "minecraft:the_end" => "THE END".into(),
        other => other.rsplit(':').next().unwrap_or(other).replace('_', " ").to_uppercase(),
    }
}

// ------------------------------------------------------------ facing, time

/// The compass bearing (0 north, 90 east) from Minecraft's yaw (0 faces
/// south, 90 west), and its 8-way name.
pub fn bearing(yaw: f32) -> (u32, &'static str) {
    let deg = (yaw as f64 + 180.0).rem_euclid(360.0);
    let i = ((deg / 45.0).round() as usize) % 8;
    let name = ["N", "NE", "E", "SE", "S", "SW", "W", "NW"][i];
    (deg.round() as u32 % 360, name)
}

/// (`14:32`, day number) from the time of day in ticks (0 is 06:00 on day 1).
pub fn clock(day_time: i64) -> (String, i64) {
    let day = day_time.div_euclid(24000) + 1;
    let minutes = (day_time.rem_euclid(24000) + 6000) % 24000 * 60 / 1000;
    (format!("{:02}:{:02}", minutes / 60, minutes % 60), day)
}

// ------------------------------------------------------------------ panel

/// Everything the box shows. `biome` and `day_time` are none when unknown;
/// `ores` is the "Ore colors" setting.
pub fn panel(
    [x, y, z]: [f64; 3],
    yaw: f32,
    dim: &str,
    biome: Option<&str>,
    day_time: Option<i64>,
    ores: bool,
) -> Panel {
    let overworld = dim == "minecraft:overworld";
    let ore = (ores && overworld).then(|| ore_at(y));
    let diamond = ore.is_some_and(|o| o == DIAMOND);
    // At diamond depth the whole block of coordinates turns cyan.
    let value = if diamond { DIAMOND.1 } else { WHITE };
    let coord = |label: &str, v: f64, c: u32| vec![(format!("{label} "), LABEL), (format!("{v:.1}"), c)];

    let mut y_line = coord("Y", y, ore.map_or(WHITE, |o| o.1));
    if let Some((name, color)) = ore {
        y_line.push((format!("  {name}"), color));
    }
    let left = vec![coord("X", x, value), y_line, coord("Z", z, value)];

    let mut right = Vec::new();
    if let Some(b) = biome {
        right.push(vec![(biome_name(b), biome_color(b))]);
    }
    right.push(vec![(dimension_name(dim), DIM_INK)]);

    let (deg, dir) = bearing(yaw);
    let bottom_left = vec![(format!("{dir} "), WHITE), (format!("{deg}°"), LABEL)];
    let bottom_right = match day_time.map(clock) {
        Some((hhmm, day)) => vec![(hhmm, TIME), (format!("  Day {day}"), LABEL)],
        None => Vec::new(),
    };
    Panel { left, right, bottom_left, bottom_right }
}

/// The text of a line, for widths and tests.
pub fn text(line: &Line) -> String {
    line.iter().map(|(t, _)| t.as_str()).collect()
}

/// `/where`'s line: `12, 70, -4 in minecraft:the_nether`.
pub fn where_line(x: f64, y: f64, z: f64, dimension: &str) -> String {
    format!("{:.0}, {:.0}, {:.0} in {dimension}", x.floor(), y.floor(), z.floor())
}

// --------------------------------------------------------------- settings

/// A stored on/off setting: on unless it says `false`.
pub fn read_flag(stored: Option<String>) -> bool {
    stored.is_none_or(|v| v.trim() != "false")
}

pub fn flag_label(name: &str, on: bool) -> String {
    format!("{name}: {}", if on { "ON" } else { "OFF" })
}

/// The settings page's switches: (stored key, button label).
pub const SHOWN: (&str, &str) = ("shown", "Shown");
pub const ORES: (&str, &str) = ("ores", "Ore colors");
pub const SWITCHES: [(&str, &str); 2] = [SHOWN, ORES];

thread_local! {
    /// The switches' values, read once from their files, then kept.
    static FLAGS: std::cell::RefCell<std::collections::HashMap<&'static str, bool>> =
        std::cell::RefCell::new(std::collections::HashMap::new());
}

/// A switch's value, read once through `get` (its file).
pub fn flag(key: &'static str, get: impl FnOnce() -> Option<String>) -> bool {
    FLAGS.with(|f| *f.borrow_mut().entry(key).or_insert_with(|| read_flag(get())))
}

/// Replaces it (the key, `/hud` or the settings page changed it).
pub fn set_flag(key: &'static str, on: bool) {
    FLAGS.with(|f| f.borrow_mut().insert(key, on));
}

/// Whether the box shows.
pub fn shown(get: impl FnOnce() -> Option<String>) -> bool {
    flag(SHOWN.0, get)
}

pub fn set_shown(on: bool) {
    set_flag(SHOWN.0, on);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ores_by_height() {
        assert_eq!(ore_at(150.0).0, "Coal");
        assert_eq!(ore_at(70.0).0, "Iron");
        assert_eq!(ore_at(20.0).0, "Gold");
        assert_eq!(ore_at(0.0).0, "Lapis");
        assert_eq!(ore_at(-30.0).0, "Redstone");
        assert_eq!(ore_at(-58.0), DIAMOND);
    }

    #[test]
    fn biomes() {
        assert_eq!(biome_name("minecraft:deep_dark"), "Deep Dark");
        assert_eq!(biome_color("minecraft:desert"), 0xE8_D4A0);
        assert_eq!(biome_color("minecraft:deep_dark"), 0x8C_8C8C);
        assert_eq!(biome_color("minecraft:deep_frozen_ocean"), 0xDC_F0FF);
        assert_eq!(biome_color("minecraft:warm_ocean"), 0x4A_8CFF);
        assert_eq!(biome_color("minecraft:the_end"), 0xD0_A0FF);
    }

    #[test]
    fn bearing_and_clock() {
        assert_eq!(bearing(180.0), (0, "N"));
        assert_eq!(bearing(-90.0), (90, "E"));
        assert_eq!(bearing(0.0), (180, "S"));
        assert_eq!(bearing(135.0), (315, "NW"));
        assert_eq!(clock(0), ("06:00".into(), 1));
        assert_eq!(clock(24000 * 7 + 8533), ("14:31".into(), 8));
        assert_eq!(clock(18000), ("00:00".into(), 1));
    }

    #[test]
    fn panel_matches_the_reference() {
        let p = panel([124.5, -54.0, -342.8], 135.0, "minecraft:overworld", Some("minecraft:deep_dark"), Some(24000 * 7 + 8533), true);
        assert_eq!(text(&p.left[0]), "X 124.5");
        assert_eq!(text(&p.left[1]), "Y -54.0  Diamond");
        assert_eq!(p.left[0][1].1, DIAMOND.1); // whole block cyan
        assert_eq!(text(&p.right[0]), "Deep Dark");
        assert_eq!(text(&p.right[1]), "OVERWORLD");
        assert_eq!(text(&p.bottom_left), "NW 315°");
        assert_eq!(text(&p.bottom_right), "14:31  Day 8");
        // Ore colours off, or in the Nether: plain white, no ore name.
        let off = panel([0.0, -54.0, 0.0], 0.0, "minecraft:overworld", None, None, false);
        assert_eq!(text(&off.left[1]), "Y -54.0");
        assert_eq!(off.left[1][1].1, WHITE);
        assert_eq!(off.right.len(), 1);
        let nether = panel([0.0, 20.0, 0.0], 0.0, "minecraft:the_nether", None, None, true);
        assert_eq!(text(&nether.left[1]), "Y 20.0");
    }

    #[test]
    fn flags_default_on() {
        assert!(read_flag(None));
        assert!(!read_flag(Some("false\n".into())));
        assert_eq!(flag_label("Ore colors", false), "Ore colors: OFF");
    }
}
