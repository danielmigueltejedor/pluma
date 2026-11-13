# Nodalia migration: Pluma

Category: secondary app.
Status: Base Nodalia preparada, as of 2026-11-13. Evidence: the pull request that adds this file.

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
| `storytold/craft-common-actions` | `.github/workflows/release.yml`, `.github/workflows/windows-arm64.yml` | Foundation reusable workflows (release, refresh-contributors, windows-arm64) | writing and testing them |
| `storytold/craft-fonts` | `crates/text/build.rs` | getnodalia/foundation `vendor/craft-fonts/main` | repointing |
| `storytold/pdfcraft` | `packaging/macos/package.sh` | to decide | — |
| `storytold/vectorcraft` | `.github/pull_request_template.md`, `.github/workflows/upstream-sync.yml`, `Cargo.toml`, `packaging/linux/ai.storyteller.vectorcraft.metainfo.xml.in`, `packaging/linux/flatpak-oci.sh`, `packaging/linux/package.sh`, `packaging/windows/sign.ps1` | none: intended upstream sync | — |
| `storytold/winprint.rs` | `Cargo.toml` | to decide | — |

## Technical debt

- Internal names inherited from upstream (crate, binary and package names) remain where saved
  settings, files and the upstream sync depend on them.

## Reusable Nodalia components

getnodalia/foundation (vendored crates, fonts, translations, reusable workflows);
getnodalia/nodalia-design (theme, icons).

## NDL work pending

Check the interface against Foundation's NDL standard: hamburger menu with About, Preferences and
Keyboard shortcuts; theme selector only in Preferences; NDL window controls on Linux only; global
menu exported over D-Bus on Linux.

## Risks and priorities

1. Releases still run the inherited release workflow.
2. Windows ARM64 packaging still runs the inherited workflow.

## Recommended order

1. A Foundation release workflow, tested with a dry run, then Pluma's release on it.
2. A Foundation Windows ARM64 workflow.
3. NDL review of the interface.

## What the first agent should do

1. Read AGENTS.md and this file.
2. Pick the first item of "Recommended order".
