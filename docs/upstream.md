# Pluma upstream

| Field | Value |
|---|---|
| Upstream | `storytold/vectorcraft` |
| Last observed | `eddf676611676ac9c54d61461e4521699806a635` |
| Last evaluated | `eddf676611676ac9c54d61461e4521699806a635` |
| Last integrated | `eddf676611676ac9c54d61461e4521699806a635` merged on 2026-11-10 |
| Skipped | Graphite frontend and node graph, because Pluma now tracks VectorCraft |
| Adapted | desktop binary name `pluma` |
| Conflicting | `README.md` (kept ours), `AGENTS.md` (upstream plus our quality contract) |

Previous lineage was Graphite `68c3c759`. That tree is no longer the engine.

## How the sync works

Upstream's code arrives renamed: `scripts/rebrand.py` applies `.github/rebrand.toml` (VectorCraft
becomes Pluma, internal names included) to upstream's tree, and the weekly sync
(`.github/workflows/upstream-sync.yml`) records the result on the `upstream-pluma` branch. Each
commit there has the previous mirror commit and the upstream commit it renames as parents, and
names that commit in an `Upstream-Commit:` trailer. The sync merges the newest mirror commit
into a branch and opens a pull request; files listed in `.github/upstream-keep` keep this
repository's version.

The mirror starts at the renamed `eddf6766`, merged into `master` (with this repository's tree)
by the pull request that renamed the app. `python3 scripts/rebrand.py check` fails when an old
name comes back; a line that must keep one says `rebrand: keep`.
