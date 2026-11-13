//! Writes the colour tokens of a theme-tool palette as a DTCG token file on stdout:
//! `cargo run --example palette-to-tokens -- tokens/palettes/light-2026.toml >
//! tokens/light-2026-fui.tokens.json`. The light token file is generated this way and committed.

use std::fs;
use std::io::Write as _;

use mango_shell_core::mapping::{self, apply, to_hex};
use serde_json::{Map, Value, json};

mod palette;

fn main() -> Result<(), String> {
    let path = std::env::args()
        .nth(1)
        .ok_or("usage: palette-to-tokens <palette.toml>")?;
    let src = fs::read_to_string(&path).map_err(|e| format!("{path}: {e}"))?;
    let palette = palette::palette_from_toml(&src)?;
    let name = path
        .rsplit('/')
        .next()
        .unwrap_or(&path)
        .trim_end_matches(".toml");
    let mut color = Map::new();
    for (key, rgb) in apply(&palette)? {
        let mut parts: Vec<&str> = key.split('.').collect();
        let leaf = parts.pop().ok_or("empty token key")?;
        let mut node = &mut color;
        for part in parts {
            node = node
                .entry(part)
                .or_insert_with(|| Value::Object(Map::new()))
                .as_object_mut()
                .ok_or_else(|| format!("{key}: a token and a group share a name"))?;
        }
        node.insert(
            leaf.to_owned(),
            json!({ "$type": "color", "$value": { "colorSpace": "srgb", "hex": to_hex(rgb) } }),
        );
    }
    let description = format!(
        "Colour tokens for mango-shell, generated from tokens/palettes/{name}.toml by `cargo run --example palette-to-tokens`. Do not edit."
    );
    let doc = json!({ "$description": description, "color": color });
    let text = serde_json::to_string_pretty(&doc).map_err(|e| e.to_string())?;
    writeln!(std::io::stdout(), "{text}").map_err(|e| e.to_string())
}
