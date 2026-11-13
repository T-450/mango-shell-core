# mango-shell-core

Shared code for the components of a MangoWM desktop shell: design tokens converted from the
design system's DTCG files at build time, theme loading with fallback, chamfered panels, corner
caps and focus rings, the popover command line, and the layer-shell popover surface. It
re-exports [rune](https://github.com/anomalyco/rune), so a component that reaches rune only
through core builds one copy of it.

## Using it from a component

```toml
[dependencies]
mango-shell-core = { git = "https://github.com/T-450/mango-shell-core", rev = "<core rev>", features = ["accessibility"] }

# A dependency's [patch] does not carry over: repeat core's block, at core's rune rev.
[patch.crates-io]
winit = { git = "https://github.com/anomalyco/rune", rev = "<rune rev>" }
wgpu-hal = { git = "https://github.com/anomalyco/rune", rev = "<rune rev>" }
parley_engine = { git = "https://github.com/anomalyco/rune", rev = "<rune rev>" }
accesskit_windows = { git = "https://github.com/anomalyco/rune", rev = "<rune rev>" }
```

Set the app's font slots to `mango_shell_core::tokens::FAMILIES` (Inter and IBM Plex Mono, both
system packages; rune's embedded fonts are off).

## Theme files

`Tokens::load` reads rune's theme shape plus a `"variant"` key:

```json
{ "variant": "light", "palette": {}, "light": { "bg.bar": "#ffffff", "...": "..." } }
```

Every key in `tokens::KEYS` must be present, or the file is ignored and the variant's defaults
are used. `mapping::MAPPING` is the rule that turns a theme-tool palette into these keys.

## Development

`mise install`, then `just fast`, `just gate`, `just hit-check`. MIT licensed.
