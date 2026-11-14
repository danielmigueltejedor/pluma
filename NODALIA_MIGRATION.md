# Nodalia migration: Pluma

Category: secondary app.
Status: renamed to Pluma, NDL interface, as of 2026-11-13. Evidence: the pull request that adds this file.

## Current state

Pure-Rust vector illustration app (egui), MIT OR Apache-2.0. Cargo git dependencies come from getnodalia/foundation and nodalia-design. The gate, FreeBSD,
Nix, packaging lint, AGENTS.md and git-sources checks call getnodalia/foundation's reusable
workflows pinned by commit. SonarCloud was removed; CodeQL and Scorecard remain.

## Desired state

No workflow, dependency or document depends on storytold; Pluma has its own release workflow; the
interface follows Foundation's [NDL standard](https://github.com/getnodalia/foundation/blob/main/standards/ndl.md).

## Inherited dependencies still pending

| Dependency | Where | Replacement | Blocked on |
|---|---|---|---|
| `storytold/craft-common-actions` | `.github/workflows/windows-arm64.yml` | Pluma's own release workflows on Foundation's release-tag and release-publish | the CI pull request that follows this one |
| upstream's repository (`docs/upstream.md`) | `.github/workflows/upstream-sync.yml` | none: the weekly sync reads upstream and renames it (`scripts/rebrand.py`, `.github/rebrand.toml`) | — |

Resolved: craft-fonts is read from getnodalia/foundation's `vendor/craft-fonts/main` branch; the
`[patch]` for `storytold/winprint.rs` in `Cargo.toml` only names the old home of crates that are
fetched from getnodalia/foundation.

## Technical debt

- The native file's signature (`"format": "vectorcraft"`), the format's id in the command API
  and the name of the editing data a PDF carries keep the old name: they are what files saved
  before the rename are read by. `.github/rebrand.toml` keeps them.

## Reusable Nodalia components

getnodalia/foundation (vendored crates, fonts, translations, reusable workflows);
getnodalia/nodalia-design (theme, icons).

## NDL

Done in the pull request that renamed the app: the Nodalia theme with Pluma's accent is the
default (Classic and Modern stay in Preferences), the panels are islands, the header has the
hamburger menu (About, Preferences, Keyboard Shortcuts, Quit), the light and dark choice lives in
Preferences only, NDL's round window controls are drawn on Linux only (switchable to the
desktop's), macOS and Windows keep native controls, and on Linux the menus are exported over
D-Bus to the shell's global menu where the shell has one.

## Risks and priorities

1. Releases still run the inherited release workflow.
2. Windows ARM64 packaging still runs the inherited workflow.

## Recommended order

1. A Foundation release workflow, tested with a dry run, then Pluma's release on it.
2. A Foundation Windows ARM64 workflow.
3. Screenshots of the NDL interface in the README and on the website.

## What the first agent should do

1. Read AGENTS.md and this file.
2. Pick the first item of "Recommended order".

## Releases

Pluma releases through `release-auto.yml` like every Nodalia app (Foundation's `release-tag` and
`release-publish`, with `release-linux.yml`, `release-macos.yml` and `release-windows.yml`).
