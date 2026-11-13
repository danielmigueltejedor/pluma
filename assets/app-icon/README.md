# Pluma app icon

**Creature:** a regal engraved dragon, Pluma's mascot.

**Style:** an engraving-style portrait in the Crafting Apps owl template (the FilmCraft owl): a tight
head-and-shoulders portrait on a full-bleed field, no frame or roundel, with the body running off the tile.

**Palette** (exactly three colours):

| Colour | Hex | Used for |
|---|---|---|
| Ink | `#0b0b0c` | line work and contour |
| Paper | `#efe9dc` | the figure |
| Pluma colour | `#e8573f` | the full-bleed field |

**Tile:** `viewBox="0 0 512 512"`, a rounded square with `rx=112`, clipped. The macOS files add Apple's
transparent margin (tile = 824/1024 of the canvas); Windows and Linux use the tile edge to edge.

**Provenance:** the owner's original Nodalia drawing, vectorised. `pluma.svg` is the canonical
artwork. Licence: see `LICENSE.txt`.

## Files

| File | What | Used by |
|---|---|---|
| `pluma.svg` | master vector (traced at 2048 px, about 700 KB) | source for everything below |
| `pluma-small.svg` | lighter vector (traced at 1024 px, about 370 KB) | `hicolor/scalable` |
| `pluma-1024.png` | full tile, 1024 px | docs, store listings |
| `pluma-macos-512.png` | tile with Apple margin, 512 px | runtime Dock icon (`apps/pluma/src/main.rs`) |
| `pluma.icns` | macOS icon set, 16 to 1024 px | `cargo xtask bundle` (`CFBundleIconFile`) |
| `pluma.ico` | Windows icon, 16 to 256 px | `apps/pluma/build.rs` (embedded in the `.exe`) |
| `hicolor/<size>/apps/dev.danielmiguelt.Pluma.png` | Linux theme icons, 16 to 512 px; the 256 one is also the runtime icon on Windows and Linux, the 128 one the brand mark in the app bar and About box (`crates/ui-egui/src/brand.rs`) | `packaging/linux/dev.danielmiguelt.Pluma.desktop` |
| `hicolor/scalable/apps/dev.danielmiguelt.Pluma.svg` | Linux scalable icon | as above |

## How it reaches each OS

- **macOS:** the app sets the Dock / app-switcher icon at runtime (`ViewportBuilder::with_icon`); the
  `.app` from `cargo xtask bundle` carries `pluma.icns`.
- **Windows:** `build.rs` embeds `pluma.ico` and VERSIONINFO with `winresource` (taskbar, Start
  menu, Explorer, Alt-Tab); the runtime icon covers the title bar.
- **Linux:** install `packaging/linux/dev.danielmiguelt.Pluma.desktop` to `share/applications/` and
  `hicolor/` to `share/icons/`. The app sets its Wayland app ID / X11 class to
  `dev.danielmiguelt.Pluma`, so the dock matches the window to the launcher.

## Regenerate

Edit or replace `pluma.svg` (and `pluma-small.svg`), then run `packaging/icons.sh`
(needs `resvg`; `iconutil` on macOS for the `.icns`). It writes every PNG, the `.ico` (via
`cargo xtask ico`) and the `.icns`.
