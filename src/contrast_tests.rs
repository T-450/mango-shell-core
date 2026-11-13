use std::collections::BTreeMap;

use super::*;
use crate::mapping::{self, Rgb, parse_hex};
use crate::tokens::{DARK, KEYS, LIGHT, Tokens};

#[path = "../examples/palette-to-tokens/palette.rs"]
mod palette;

const DARK_FILE: &str = include_str!("../tokens/dark-2026-fui.tokens.json");
const LIGHT_FILE: &str = include_str!("../tokens/light-2026-fui.tokens.json");
const DARK_PALETTE: &str = include_str!("../tokens/palettes/dark-2026.toml");
const LIGHT_PALETTE: &str = include_str!("../tokens/palettes/light-2026.toml");

/// Goal 3's thresholds (`docs/designs/blueh.md`).
const TEXT_MIN: f32 = 4.5;
const RAISED_MIN: f32 = 1.05;

/// Tokens drawn as text on `bg.bar`.
fn text_tokens(t: &Tokens) -> [(&'static str, Color); 7] {
    [
        ("text.primary", t.text_primary),
        ("text.secondary", t.text_secondary),
        ("text.tertiary", t.text_tertiary),
        ("accent.signal", t.accent_signal),
        ("status.ok", t.status_ok),
        ("status.warn", t.status_warn),
        ("status.critical", t.status_critical),
    ]
}

fn file_colors(src: &str) -> BTreeMap<&'static str, Rgb> {
    let doc: serde_json::Value = serde_json::from_str(src).unwrap();
    KEYS.iter()
        .map(|key| {
            let node = key.split('.').fold(&doc["color"], |node, part| &node[part]);
            (
                *key,
                parse_hex(node["$value"]["hex"].as_str().unwrap()).unwrap(),
            )
        })
        .collect()
}

fn mapped(src: &str) -> BTreeMap<&'static str, Rgb> {
    mapping::apply(&palette::palette_from_toml(src).unwrap()).unwrap()
}

#[test]
fn mapping_covers_every_token_key() {
    let mapped: Vec<_> = mapping::MAPPING.iter().map(|(key, _)| *key).collect();
    assert_eq!(mapped, KEYS);
}

#[test]
fn mapping_of_dark_2026_reproduces_the_dark_token_file_within_the_design_tolerances() {
    // docs/designs/blueh.md, "Theming": the mixes land within these steps of the design file.
    let tolerance = |key: &str| match key {
        "line.hairline" | "text.primary" => 1,
        "text.tertiary" | "signal.empty" => 2,
        _ => 0,
    };
    let (got, want) = (mapped(DARK_PALETTE), file_colors(DARK_FILE));
    let off: Vec<_> = KEYS
        .iter()
        .filter(|key| {
            got[*key]
                .iter()
                .zip(want[*key])
                .any(|(g, w)| g.abs_diff(w) > tolerance(key))
        })
        .collect();
    assert!(off.is_empty(), "outside tolerance: {off:?}");
}

#[test]
fn mapping_of_light_2026_equals_the_light_token_file() {
    assert_eq!(mapped(LIGHT_PALETTE), file_colors(LIGHT_FILE));
}

fn below_text_minimum(t: &Tokens) -> Vec<&'static str> {
    text_tokens(t)
        .into_iter()
        .filter(|(_, c)| ratio(*c, t.bg_bar) < TEXT_MIN)
        .map(|(key, _)| key)
        .collect()
}

#[test]
fn dark_text_tokens_reach_4_5_to_1_on_the_bar() {
    assert_eq!(below_text_minimum(&DARK), Vec::<&str>::new());
}

#[test]
fn light_text_tokens_reach_4_5_to_1_on_the_bar() {
    assert_eq!(below_text_minimum(&LIGHT), Vec::<&str>::new());
}

#[test]
fn signal_and_focus_accents_differ_in_both_variants() {
    assert!(DARK.accent_signal != DARK.accent_focus && LIGHT.accent_signal != LIGHT.accent_focus);
}

#[test]
fn raised_background_stands_apart_from_the_bar_in_both_variants() {
    let raised = |t: &Tokens| ratio(t.bg_raised, t.bg_bar);
    assert!(raised(&DARK) >= RAISED_MIN && raised(&LIGHT) >= RAISED_MIN);
}

#[test]
fn ratio_of_black_on_white_is_21() {
    assert!((ratio(Color::BLACK, Color::WHITE) - 21.0).abs() < 0.01);
}

#[test]
fn mix_rounds_exact_halves_to_even_as_the_theme_tool_does() {
    // Python's round(0.5) == 0 and round(1.5) == 2.
    assert_eq!(mapping::mix([0, 1, 2], [1, 2, 3], 500), [0, 2, 2]);
}
