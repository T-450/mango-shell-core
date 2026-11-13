# AGENTS.md

mango-shell-core holds what `docs/DESIGN-SYSTEM.md` (in the mango-shell umbrella workspace)
specifies for two or more shell components: design tokens and their palette mapping, theme
loading, the bevelled panel, corner caps, focus ring and row rim, the command line under each
popover, and the popover surface settings. It also owns the single rune pin and re-exports rune.

## Admission rule

Code enters core only when the design system specifies it for two or more components. Anything
one component needs (BlueZ, wire protocols, device controls and icons) stays in that component.

## Rules

- Toolchain: Rust 1.99.1 (`rust-toolchain.toml`); gate tools from `mise.toml` (`mise install`).
- Lints are in `Cargo.toml` and `clippy.toml`. Never silence one with `#[allow]` or `#[expect]`;
  change the code. Build scripts read the environment with `env::var_os` and fail by returning
  `Err`.
- No `unwrap`, `expect`, `panic` outside tests. Public items carry doc comments.
- `tokens/light-2026-fui.tokens.json` is generated (`just light-tokens`); never edit it by hand.

## Bumping rune

1. In core: change the rune `rev` and the four `[patch.crates-io]` revs in `Cargo.toml` in one
   commit; run `cargo deny check bans` and replace `deny.toml`'s `skip` list with what it
   reports.
2. In each component: move its core rev and its `[patch.crates-io]` revs to match, in one commit.

## Gates

- After every edit: `just fast`.
- Before a change lands: `just gate` and `just hit-check`.
