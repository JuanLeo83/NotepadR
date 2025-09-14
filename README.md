# 📝 Notepad R

![license](https://img.shields.io/badge/license-MIT-blue.svg) [![build](https://github.com/JuanLeo83/NotepadR/actions/workflows/ci.yml/badge.svg)](https://github.com/JuanLeo83/NotepadR/actions/workflows/ci.yml) ![rust](https://img.shields.io/badge/rust-stable-000000.svg?logo=rust)

A small cross-platform notepad built with Rust, eframe (egui) and wgpu.

✨ Features
- Light / Dark theme
- Font family & size selection (loaded from assets)
- Default folder for open/save dialogs
- UI language selection (English / Spanish / French)
- Confirm before closing if there are unsaved changes

🚀 Quick start

Prerequisites: Rust toolchain (rustup + cargo)

Build and run in debug:

```bash
cargo run
```

Build release (optimized binary):

```bash
cargo build --release
# executable: target/release/Notepad
```

🗂️ Project layout

- assets/fonts/ — bundled font files (TTF) included with include_bytes!
- assets/strings/ — JSON translation files (en.json / es.json / fr.json)
- src/ — application source code

🌐 Localization

Translations are embedded into the binary using `include_str!` and parsed at startup. The app only loads the selected language into memory at runtime. Translation keys live in `assets/strings/*.json` — use the `AppState::text(key)` helper to retrieve localized strings.

If you add or change translation files, recompile the project so the new strings are embedded.

⚙️ Configuration

User settings are saved to the standard OS config directory, for example:
- Windows: `%APPDATA%\\NotepadR\\config.json`
- macOS: `~/Library/Application Support/NotepadR/config.json`
- Linux: `~/.config/NotepadR/config.json`

🔧 Tips to reduce binary size

- Build with `cargo build --release`
- Enable LTO and optimize-for-size in `Cargo.toml` profile
- Avoid embedding large assets (fonts) if you want a smaller binary
- Use `strip` or `RUSTFLAGS="-C link-arg=-s"` to remove symbols

📝 Development notes

- Fonts must be placed in `assets/fonts/` and registered in `setup_custom_fonts`.
- Translation keys: keep English keys as the source of truth and update other language JSON files accordingly.
- Settings UI uses `settings_state.current` and `settings_state.unsaved` to allow cancel/apply behavior.

📄 License

This project is released under the MIT License. See `LICENSE.md` for details.

If you want, I can add CI checks for missing translation keys, or generate a translation helper to list missing keys automatically.
