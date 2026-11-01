# Horizon Modding in Rust – Overview 

Horizon reimplements Minecraft Java in Rust and uses WebAssembly “.hmod” plugins for mods. A Horizon mod is a Rust project compiled to Wasm, declared with a `horizon-mod.toml` manifest (ID, version, “side” = client/server, and *capabilities*). For example, **game-read** lets the mod read the player’s state (position, facing, dimension, game clock, etc.), and **hud** lets it draw text in a corner of the screen. A mod registers *hooks* (like `init`, `tick`, `frame`, `command`, `chat`) via a `Registration` in Rust. Unused hooks cost nothing (a pure HUD mod needn’t use `frame` unless continuous updates are needed). In practice, a simple HUD mod will: import `horizon_mod_sdk::{Mod, Registration, game, hud, chat}`; declare in `horizon-mod.toml` that it uses `game-read` and `hud`; and in the `init()` return a `Registration::new()` with e.g. `.on_tick().command("coords")` to update each tick and handle commands. 

# HUD Layout and Design 

The requested HUD is a **minimal text panel** (up to 16 lines) in a corner of the screen (e.g. top-left). We recommend listing information in separate lines, for example: 

- **Coordinates:** `X Y Z` (each to one decimal or integer), colored by dimension or altitude (e.g. diamond layer *Y* in blue or silver)  
- **Facing Direction:** e.g. “Facing North (-Z)”  
- **Time of Day:** converted from ticks to “Day N, HH:MM” (like Horizon’s example)  
- **Biome/Climate & Dimension:** e.g. “Plains, Overworld” or “Snowy Tundra, Overworld”, possibly color-coded (cold=blue, hot=red)  

A sketch diagram of the top-left panel might look like:

```
╔════════════════════════════╗
║ XYZ 123.4 /  64.0 / 256.7   (white)    ║
║ Facing South (+Z)          (white)    ║
║ Day  5, 14:07             (yellow)    ║
║ Biome: Forest (temperate) (green)    ║
║ Dimension: Overworld (blue)          ║
╚════════════════════════════╝
```

Each line is a `hud::Line` with text and a hex RGB color. For example, using 0xRRGGBB values: white (`0xFFFFFF`) for coordinates, yellow (`0xFFFF00`) for time, and a color per biome or dimension (e.g. blue for cold snowy biomes, red for desert) to give a subtle visual cue.

# Implementation Outline 

1. **Project setup:** Install the Horizon mod tool (`cargo install --git https://github.com/horizon-rust/horizon cargo-horizon-mod`) and create a new mod:  
   ```bash
   rustup target add wasm32-unknown-unknown 
   cargo horizon-mod new simple-hud
   cd simple-hud
   ```  
   This makes a Rust `cdylib` with `horizon_mod_sdk` dependency and a `horizon-mod.toml`.

2. **Manifest (`horizon-mod.toml`):** Set `side = "client"` and declare needed capabilities, e.g.:
   ```toml
   id = "simple_hud"
   version = "0.1.0"
   side = "client"
   api = "0.1"
   capabilities = ["game-read", "hud"]
   ```  
   This matches what the code imports; mismatches will cause a load error.

3. **Rust code (`src/lib.rs`):** Implement the `Mod` trait. For example:

   ```rust
   use horizon_mod_sdk::{Mod, Registration, game, hud, chat};
   use horizon_mod_sdk::hud::{Corner, Line};

   struct SimpleHud {
       shown: bool,
       last_lines: Vec<String>,
   }

   impl Mod for SimpleHud {
       fn init() -> Result<(Self, Registration), String> {
           let state = SimpleHud { shown: true, last_lines: Vec::new() };
           // Register tick and a chat command to toggle HUD
           Ok((state, Registration::new().on_tick().command("hud")))
       }
       fn tick(&mut self, _tick: u64) {
           // If HUD is off, clear if needed
           if !self.shown {
               if !self.last_lines.is_empty() {
                   hud::clear();
                   self.last_lines.clear();
               }
               return;
           }
           // Fetch player and clock info
           if let Some(player) = game::get_player() {
               let clock = game::get_clock().unwrap_or_default();
               // Format lines
               let coords_line = format!("XYZ {:.1} / {:.1} / {:.1}", player.x, player.y, player.z);
               let facing = format!("Facing {}", normalize_facing(player.yaw));
               let time = format_time(clock);
               let dimension = format!("{} ({})", player.biome, player.dimension);
               let lines = vec![coords_line, facing, time, dimension];
               // Only update if changed
               if lines != self.last_lines {
                   // Build hud panel with colors
                   let panel: Vec<Line> = lines.into_iter().map(|text| {
                       Line { text, color: choose_color_for_line(/*..*/)} 
                   }).collect();
                   hud::set_panel(Corner::TopLeft, &panel);
                   self.last_lines = panel.iter().map(|l| l.text.clone()).collect();
               }
           }
       }
       fn command(&mut self, name: &str, _args: &str) -> Result<Option<String>, String> {
           if name == "hud" {
               self.shown = !self.shown;
               let msg = if self.shown { "HUD shown" } else { "HUD hidden" };
               chat::show(msg); 
           }
           Ok(None)
       }
   }
   horizon_mod_sdk::export_mod!(SimpleHud);
   ```

   Key points:
   - **Hooks:** We use `.on_tick()` to update 20 times/sec, and `.command("hud")` to toggle.  
   - **Reading data:** `game::get_player()` returns a `Player` with `x,y,z,yaw,dimension` (and possibly a `biome` field). `game::get_clock()` gives time in ticks to convert to day/hour.  
   - **Formatting:** Example functions from Horizon show converting `clock.day_time` to “Day X, HH:MM”, and a helper for facing from `yaw`.  
   - **HUD panel:** Build a `Vec<hud::Line>` and call `hud::set_panel(Corner::TopLeft, &panel)` to draw. If the text hasn’t changed, skip calling again (as in the coords example) to save calls. Use `hud::clear()` when hiding.  

4. **Colors and Scaling:** Each `Line` has an RGB color. For example, set the coordinate text color white (`0xFFFFFF`), change it to blue (e.g. `0x00AFFF`) if `player.y` is in diamond level range, or red (`0xFF0000`) if in Nether (you can inspect `player.dimension`). For biome/climate lines, you might inspect `player.biome` or temperature to pick a color: blue for snow/frost, green for forests, orange for desert, etc. The `hud` text should respect the user’s UI scaling automatically (as in vanilla), but test it at various GUI scales. 

5. **Building and testing:** Use `cargo horizon-mod run` to hot-reload the mod into Horizon (game auto-reloads on code changes). You can also write offline tests (in `tests/*.toml`) to simulate ticks/commands and assert the HUD content (Horizon supports this). Before release, run `cargo horizon-mod check` to verify the manifest and do a dry-run. 

# Example and References 

The Horizon SDK repo includes a **coords-hud example** that does almost exactly this: it formats “XYZ …”, facing and time, and draws a panel. Its `tick` implementation shows how to fetch the player and clock, build text lines, and call `hud::set_panel`. In that example each line is white; you can adapt it to use different `color` values. 

Performance is excellent: a simple HUD mod adds *under 0.01 ms per frame* overhead, so it won’t noticeably impact gameplay. 

**In summary:** use Horizon’s Rust/Wasm mod system with the `hud` and `game-read` capabilities. Update your panel each tick with formatted strings (coords, direction, day/time, biome/dimension). Use small, color-coded text and provide a simple toggle command. This yields a minimal, non-intrusive “F3-lite” display that helps the user without overwhelming them. 

**Sources:** Horizon modding docs and examples.