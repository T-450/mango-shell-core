# Gates for mango-shell-core. Tools come from mise.toml (`mise install`).

# Format check and type check: after every edit.
fast:
    cargo fmt --all -- --check
    cargo check --all-targets --message-format=short

# Everything a change must pass before it lands.
gate:
    cargo fmt --all -- --check
    cargo clippy --all-targets --features accessibility -- -D warnings
    just _tool cargo-nextest "cargo nextest run --features accessibility" "cargo test --features accessibility"
    just _tool cargo-machete "cargo machete" ""
    just _tool cargo-deny "cargo deny check" ""

# A press inside a corner cap reaches the control under it.
hit-check:
    cargo run -q --example bevel-hit -- --script examples/bevel-hit/hit.json --tree target/hit-tree.json --at mark:end
    cargo run -q --example bevel-hit -- --check target/hit-tree.json

# Regenerate the light token file from its palette.
light-tokens:
    cargo run -q --example palette-to-tokens -- tokens/palettes/light-2026.toml > tokens/light-2026-fui.tokens.json

# Run `cmd` when `tool` is installed; else run `fallback`, or skip with a notice. Under CI a
# missing tool fails.
_tool tool cmd fallback:
    #!/bin/sh
    if command -v {{tool}} >/dev/null 2>&1; then exec {{cmd}}; fi
    if [ -n "${CI:-}" ]; then echo "{{tool}} is required under CI" >&2; exit 1; fi
    if [ -n "{{fallback}}" ]; then exec {{fallback}}; fi
    echo "skip: {{tool}} not installed (mise install)"
