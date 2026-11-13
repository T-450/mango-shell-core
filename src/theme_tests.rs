use super::*;
use crate::mapping::to_hex;
use crate::tokens::KEYS;

/// A complete theme file in `variant`, every key set to `hex`, minus `skip`.
fn theme_file(variant: &str, hex: &str, skip: Option<&str>) -> String {
    let tokens: serde_json::Map<String, Value> = KEYS
        .iter()
        .filter(|key| Some(**key) != skip)
        .map(|key| ((*key).to_owned(), Value::from(hex)))
        .collect();
    serde_json::json!({ "variant": variant, "palette": {}, variant: tokens }).to_string()
}

fn bar_hex(tokens: &Tokens) -> String {
    let c = tokens.bg_bar;
    let hex = c.hex();
    hex.get(..7).unwrap_or(&hex).to_owned()
}

#[test]
fn load_without_file_returns_dark_defaults_and_no_log() {
    assert_eq!(Tokens::load(None), (DARK, None));
}

#[test]
fn load_of_empty_file_falls_back_to_dark_with_a_log_line() {
    let (tokens, log) = Tokens::load(Some(""));
    assert_eq!((tokens, log.is_some()), (DARK, true));
}

#[test]
fn load_of_invalid_json_falls_back_to_dark_with_a_log_line() {
    let (tokens, log) = Tokens::load(Some("{ not json"));
    assert_eq!((tokens, log.is_some()), (DARK, true));
}

#[test]
fn load_of_dark_file_missing_a_key_falls_back_with_the_key_named() {
    let (tokens, log) = Tokens::load(Some(&theme_file("dark", "#010203", Some("signal.empty"))));
    assert_eq!(tokens, DARK);
    assert!(log.is_some_and(|line| line.contains("signal.empty")));
}

#[test]
fn load_keeps_a_file_with_an_extra_unknown_key() {
    let mut file: Value = serde_json::from_str(&theme_file("dark", "#010203", None)).unwrap();
    file["dark"]["mystery.token"] = Value::from("#ffffff");
    let (tokens, log) = Tokens::load(Some(&file.to_string()));
    assert_eq!((bar_hex(&tokens), log), ("#010203".to_owned(), None));
}

#[test]
fn load_of_light_file_reads_its_light_section() {
    let (tokens, log) = Tokens::load(Some(&theme_file("light", "#fafafa", None)));
    assert_eq!(
        (bar_hex(&tokens), tokens.dark, log),
        ("#fafafa".to_owned(), false, None)
    );
}

#[test]
fn load_of_light_file_missing_a_key_falls_back_to_light_defaults() {
    let (tokens, log) = Tokens::load(Some(&theme_file("light", "#fafafa", Some("bg.bar"))));
    assert_eq!((tokens, log.is_some()), (LIGHT, true));
}

#[test]
fn defaults_round_trip_through_the_hex_of_the_token_files() {
    assert_eq!(bar_hex(&Tokens::defaults(true)), to_hex([0x12, 0x13, 0x14]));
}
