# Pluma app icon

**Figure:** René Descartes. He gave geometry its coordinates, the basis of vector drawing.

**Style:** a black-and-white portrait cut out over a full-bleed colour field, head and shoulders, in
the Nodalia icon family (one historical figure per app, one colour per app).

**Tile:** 512 × 512 viewBox, rounded square `rx=112`, clipped; no border or roundel.

**Provenance:** the portrait is "Portret van René Descartes René Descartes (titel op object), RP-P-1910-4588.jpg" from
[Wikimedia Commons](https://commons.wikimedia.org/wiki/File:Portret_van_René_Descartes_René_Descartes_(titel_op_object),_RP-P-1910-4588.jpg) (CC0). The figure was separated from its
background, converted to black and white and placed over the app colour.
Licence: see [LICENSE.txt](LICENSE.txt).

**Linux:** the hicolor icons (`hicolor/scalable` and every PNG size) put the tile on the Nodalix
app grid, like the Colloid icons around it: a 224 px tile inside a 256 px canvas (16 px margin),
a 51.5 px corner radius and a soft shadow. The scalable SVG is made by `tools/nodalia-app-icon.py`
in [nodalix-icon-theme](https://github.com/danielmigueltejedor/nodalix-icon-theme) and
`packaging/icons.sh` renders the PNG sizes from it. macOS, the Windows `.ico` and the
1024 px render keep the full-bleed tile. The window and taskbar icon on Windows is the 256 px
hicolor PNG, so it has the Linux margin and shadow.

## Files

| File | What | Used by |
|---|---|---|
| `pluma.svg` | master: the portrait as a 1024 px image embedded in an SVG with the tile clip (canonical) |
| `pluma-small.svg` | lighter 512 px version; also the web favicon where there is one |
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
  `hicolor/` to `share/icons/`, with a copy of each `dev.danielmiguelt.Pluma.*` icon named
  `dev.danielmiguelt.Pluma.*`: the `.desktop` file's `Icon=` is that name (`packaging/linux/package.sh` does both). The app sets its Wayland app ID / X11 class to
  `dev.danielmiguelt.Pluma`, so the dock matches the window to the launcher.

## Regenerate

The icon is a raster portrait, so the SVG files embed an image instead of traced paths. The
derived files in this directory were rendered directly from the portrait. `packaging/icons.sh`
still works on `pluma.svg` (resvg renders embedded images), but its output will differ slightly
from the committed files in resampling.
