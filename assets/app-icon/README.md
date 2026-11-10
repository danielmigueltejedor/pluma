# Pluma app icon

**Figure:** René Descartes. He gave geometry its coordinates, the basis of vector drawing.

**Style:** a black-and-white portrait cut out over a full-bleed colour field, head and shoulders, in
the Nodalia icon family (one historical figure per app, one colour per app).

**Tile:** 512 × 512 viewBox, rounded square `rx=112`, clipped; no border or roundel.

**Provenance:** the portrait is "Portret van René Descartes René Descartes (titel op object), RP-P-1910-4588.jpg" from
[Wikimedia Commons](https://commons.wikimedia.org/wiki/File:Portret_van_René_Descartes_René_Descartes_(titel_op_object),_RP-P-1910-4588.jpg) (CC0). The figure was separated from its
background, converted to black and white and placed over the app colour.
Licence: see [LICENSE.txt](LICENSE.txt).

## Files

| File | What | Used by |
|---|---|---|
| `vectorcraft.svg` | master: the portrait as a 1024 px image embedded in an SVG with the tile clip (canonical) |
| `vectorcraft-small.svg` | lighter 512 px version; also the scalable Linux icon and the web favicon where there is one |
| `vectorcraft-1024.png` | full tile, 1024 px | docs, store listings |
| `vectorcraft-macos-512.png` | tile with Apple margin, 512 px | runtime Dock icon (`apps/vectorcraft/src/main.rs`) |
| `vectorcraft.icns` | macOS icon set, 16 to 1024 px | `cargo xtask bundle` (`CFBundleIconFile`) |
| `vectorcraft.ico` | Windows icon, 16 to 256 px | `apps/vectorcraft/build.rs` (embedded in the `.exe`) |
| `hicolor/<size>/apps/ai.storyteller.vectorcraft.png` | Linux theme icons, 16 to 512 px; the 256 one is also the runtime icon on Windows and Linux, the 128 one the brand mark in the app bar and About box (`crates/ui-egui/src/brand.rs`) | `packaging/linux/ai.storyteller.vectorcraft.desktop` |
| `hicolor/scalable/apps/ai.storyteller.vectorcraft.svg` | Linux scalable icon | as above |

## How it reaches each OS

- **macOS:** the app sets the Dock / app-switcher icon at runtime (`ViewportBuilder::with_icon`); the
  `.app` from `cargo xtask bundle` carries `vectorcraft.icns`.
- **Windows:** `build.rs` embeds `vectorcraft.ico` and VERSIONINFO with `winresource` (taskbar, Start
  menu, Explorer, Alt-Tab); the runtime icon covers the title bar.
- **Linux:** install `packaging/linux/ai.storyteller.vectorcraft.desktop` to `share/applications/` and
  `hicolor/` to `share/icons/`. The app sets its Wayland app ID / X11 class to
  `ai.storyteller.vectorcraft`, so the dock matches the window to the launcher.

## Regenerate

The icon is a raster portrait, so the SVG files embed an image instead of traced paths. The
derived files in this directory were rendered directly from the portrait. `packaging/icons.sh`
still works on `vectorcraft.svg` (resvg renders embedded images), but its output will differ slightly
from the committed files in resampling.
