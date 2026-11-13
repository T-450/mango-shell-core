//! Design tokens, converted from `tokens/*.tokens.json` at build time, and the type styles of
//! `docs/DESIGN-SYSTEM.md`, "Type".

use rune::{Color, Spring, TextStyle};

include!(concat!(env!("OUT_DIR"), "/tokens.rs"));

/// The font families behind rune's sans and mono slots (`AppSpec::families`). Both are system
/// packages: core builds rune without its embedded fonts.
pub const FAMILIES: (&str, &str) = ("Inter", "IBM Plex Mono");

/// 13 / 20, regular: everything else.
pub const BODY: TextStyle = TextStyle::new(FONT_SIZE_BODY, FONT_WEIGHT_BODY).lh(20.0);
/// 15 / 20, medium: names, titles.
pub const TITLE: TextStyle = TextStyle::new(FONT_SIZE_TITLE, FONT_WEIGHT_TITLE).lh(20.0);
/// 24 / 28, light: values such as a pairing code.
pub const VALUE: TextStyle = TextStyle::new(FONT_SIZE_VALUE, FONT_WEIGHT_VALUE).lh(28.0);
/// 11 / 16, semibold; draw it uppercase.
pub const CAPTION: TextStyle = TextStyle::new(FONT_SIZE_CAPTION, FONT_WEIGHT_CAPTION)
    .lh(16.0)
    .tracking(FONT_CAPTION_TRACKING * FONT_SIZE_CAPTION);
/// 11 / 16 regular: small metadata.
pub const SMALL: TextStyle = TextStyle::new(FONT_SIZE_CAPTION, FONT_WEIGHT_BODY).lh(16.0);
/// 11 / 16 in the mono family: status lines and commands.
pub const COMMAND: TextStyle = TextStyle::new(FONT_SIZE_CAPTION, FONT_WEIGHT_BODY)
    .lh(16.0)
    .mono();
