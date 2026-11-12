# macOS DMG window

What Finder shows when the DMG opens: a 660 × 400 pt background with the app icon and the
`Applications` link side by side. `package.sh` copies these files into the image; nothing here is
generated at build time, so the DMG builds with a plain `hdiutil create -srcfolder` (no Finder
scripting on CI, nothing extra installed in the signing job).

| File | What |
|---|---|
| `background.svg` | Source of the background, drawn by `tools/dmg-background/draw_background.py` in [nodalia-design](https://github.com/getnodalia/nodalia-design): two Nodalia islands on the window colour, the left one in the Pluma colour (`#F2782F`) with a flat picture of what Pluma does, the right one white under the icons. Its text (Sora) is outlined, so rendering it needs no fonts. |
| `background.tiff` | The background at 1x (660 × 400 px, 72 dpi) and 2x (1320 × 800 px, 144 dpi) in one HiDPI TIFF (Deflate, sRGB). Goes to `.background/background.tiff`. |
| `DS_Store` | Finder's view settings for the volume: window size, icon size 128, Pluma.app at (326, 205), `Applications` at (574, 205), and the background. Goes to `.DS_Store`. |
| `generate.py` | Writes `background.tiff` and `DS_Store` from the SVG and the layout above. |

## Rules

- **The volume name has no version** (`Pluma`, not `Pluma <version>`). `.DS_Store` points at the
  background through an alias that includes the volume name, so a versioned name loses the
  background. The DMG file name still carries the version.
- **Finder draws the icon labels in black in light and dark mode** when a window has a background,
  so the area under both icons stays light (the white island).
- **Nothing goes inside the icon boxes:** artwork keeps 10 pt clear of each 128 pt icon box and of
  the label strip under it.

## Regenerate

Regenerate `background.svg` with nodalia-design's `tools/dmg-background/draw_background.py` (or the layout constants in `generate.py`), then run, on any OS:

```sh
pip install 'pillow>=12' ds_store==1.3.3 mac_alias==2.2.3
python3 packaging/macos/dmg/generate.py   # needs resvg on PATH, as packaging/icons.sh
```

It renders the SVG with resvg using only the fonts in `assets/fonts`, so the pixels are the same
on every machine, and writes `DS_Store` from scratch: the background alias holds the volume name
and `/.background/background.tiff`, nothing from the machine that ran it. The window is 660 × 432
with Finder's 32 pt title bar; the content area is 660 × 400.
