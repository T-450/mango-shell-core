//! Loading a user theme file over the built-in tokens.
//!
//! A theme file is rune's theme shape (`rune::theme::Theme::from_json`) plus a top-level
//! `"variant": "dark" | "light"` that picks the section to read. It holds colours only; numbers
//! always come from the built-in tokens. A file that is missing a key, or fails to parse, is
//! ignored as a whole, so a broken theme never stops a component.

use rune::theme::Theme;
use serde_json::Value;

use crate::tokens::{DARK, LIGHT, Tokens};

impl Tokens {
    /// The built-in tokens for one variant.
    pub const fn defaults(dark: bool) -> Tokens {
        if dark { DARK } else { LIGHT }
    }

    /// Tokens from the contents of a user theme file, read once by the caller. Returns the
    /// tokens and, when the file was rejected, one line saying why. No file means the dark
    /// defaults; a rejected file means the defaults of the variant it names (dark when it
    /// names none).
    pub fn load(user: Option<&str>) -> (Tokens, Option<String>) {
        let Some(src) = user else { return (DARK, None) };
        let dark = variant(src) != Some(Variant::Light);
        match Theme::from_json(src, dark).and_then(|theme| Tokens::from_theme(&theme)) {
            Ok(tokens) => (tokens, None),
            Err(error) => (
                Tokens::defaults(dark),
                Some(format!("theme file ignored: {error}")),
            ),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Variant {
    Dark,
    Light,
}

fn variant(src: &str) -> Option<Variant> {
    let value: Value = serde_json::from_str(src).ok()?;
    match value.get("variant")?.as_str()? {
        "dark" => Some(Variant::Dark),
        "light" => Some(Variant::Light),
        _ => None,
    }
}

#[cfg(test)]
#[path = "theme_tests.rs"]
mod tests;
