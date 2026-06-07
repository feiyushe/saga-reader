# Dynamics

> Last updated: 2026-06-07

## Active Constraints

- **Cross-compile required** for Windows builds from macOS — uses `cargo-xwin` (see `package.json` `build:desktop-win64` script)
- **Tauri 2.x unstable features** enabled (`tray-icon`, `unstable` in `app/src-tauri/Cargo.toml`) — API may change on Tauri upgrades
- **Bun as primary package manager** — npm/pnpm supported but Bun is canonical; Bun registry mirror configured in `bunfig.toml`

## Known Workarounds

- Release profile uses `strip = "none"` instead of `"symbols"` to preserve debug info for profiling with `samply` (see `Cargo.toml` comment)

## Items Under Consideration

- Additional search providers beyond Bing and Baidu (e.g., Google)
- Additional LLM providers beyond the current 5
- More i18n locales beyond zh/en
