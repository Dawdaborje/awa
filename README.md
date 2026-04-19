# awa

Personal desktop mascot. Floats around your screen, follows your mouse, stands on your app bars.  
Built in Rust.

## Features

- Transparent, borderless, always-on-top window
- Spring-physics mouse following with a dead zone (she won't get in your way)
- State machine: idle → walking → running → sitting
- Mode system: cute / sexy / focus (right-click to cycle)
- Sprite sheet animation via Aseprite JSON atlas

## Building

```bash
cargo build --release
./target/release/awa
```

### Linux notes

On X11, global mouse tracking works out of the box.  
On Wayland (KDE Plasma 6), you may need `libei` support — check your compositor settings.  
The "stand on app bars" feature requires X11 for window enumeration.

## Adding your sprites

1. Create your character in Aseprite
2. Export as sprite sheet: **File → Export Sprite Sheet**
   - Format: PNG + JSON (Array)
   - Use frame tags to name animations: `idle`, `walk`, `run`, `sit`, `wave`
3. Put the files in `assets/cute/` and `assets/sexy/`
4. Replace the placeholder draw in `src/mascot.rs` with `SpriteSheet::blit_frame()`

## Project structure

```
src/
  main.rs      — event loop, window setup
  mascot.rs    — state machine, mode system, draw
  physics.rs   — spring-damper following
  sprite.rs    — Aseprite atlas parser + animator
  input.rs     — global mouse hook (rdev)
assets/
  cute/        — cute mode sprite sheets
  sexy/        — sexy mode sprite sheets
  focus/       — focus mode sprite sheets
```

## Roadmap

- [ ] Real sprite rendering (drop in your Aseprite sheets)
- [ ] Window edge detection — stand on taskbars (X11/Win32)
- [ ] System tray icon for mode switching
- [ ] Random idle animations (yawn, stretch, look around)
- [ ] React to events (wave when you open a terminal, sleep after inactivity)
- [ ] Config file (personality settings, dead zone, spring stiffness)
