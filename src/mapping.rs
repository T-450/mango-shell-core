//! The one table that turns a theme-tool palette (`~/dotfiles/theme/palettes/*.toml`) into the
//! design tokens. Core's light defaults and the user's theme template both follow it.

use std::collections::BTreeMap;

/// An sRGB colour as bytes.
pub type Rgb = [u8; 3];

/// How one token is derived from palette roles.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Rule {
    /// A palette role as is (`bg_desktop`, `ansi4`, `term_cursor`).
    Slot(&'static str),
    /// `Mix(a, b, t)` moves role `a` toward role `b` by `t` thousandths, per sRGB byte, as the
    /// theme tool's `mix:b:0.ttt` filter does.
    Mix(&'static str, &'static str, u16),
}

/// Every token's rule, for every palette.
pub const MAPPING: [(&str, Rule); 15] = [
    ("bg.bar", Rule::Slot("bg_desktop")),
    ("bg.raised", Rule::Mix("bg_desktop", "fg_editor", 83)),
    ("line.hairline", Rule::Mix("bg_desktop", "fg_editor", 160)),
    ("line.strong", Rule::Slot("fg_disabled")),
    ("text.primary", Rule::Mix("fg_editor", "fg_bright", 780)),
    ("text.secondary", Rule::Slot("fg_editor")),
    ("text.tertiary", Rule::Mix("fg_editor", "bg_desktop", 190)),
    ("selection", Rule::Slot("selection_bg")),
    ("text.on-selection", Rule::Slot("selection_fg")),
    ("accent.signal", Rule::Slot("ansi4")),
    ("accent.focus", Rule::Slot("ansi12")),
    ("status.ok", Rule::Slot("ansi2")),
    ("status.warn", Rule::Slot("ansi3")),
    ("status.critical", Rule::Slot("ansi1")),
    ("signal.empty", Rule::Mix("bg_desktop", "fg_editor", 245)),
];

/// Rules that replace a [`MAPPING`] rule for light palettes: `light-2026`'s ANSI 4 and 12 are
/// both `#0550ae`, so the focus ring takes the cursor colour to stay apart from the signal.
pub const LIGHT_ONLY: [(&str, Rule); 1] = [("accent.focus", Rule::Slot("term_cursor"))];

/// A palette's roles by the theme tool's names: `[colors]` as is, `[terminal]` keys as
/// `term_<key>`, and `ansi0` to `ansi15`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Palette {
    /// `[meta] variant == "dark"`.
    pub dark: bool,
    /// Role name to colour.
    pub roles: BTreeMap<String, Rgb>,
}

/// The tokens for `palette`, by key.
///
/// # Errors
///
/// Names the first role a rule needs that the palette lacks.
pub fn apply(palette: &Palette) -> Result<BTreeMap<&'static str, Rgb>, String> {
    let slot = |name: &str| {
        palette
            .roles
            .get(name)
            .copied()
            .ok_or_else(|| format!("palette has no role {name:?}"))
    };
    MAPPING
        .iter()
        .map(|&(key, rule)| {
            let rule = LIGHT_ONLY
                .iter()
                .find(|(light_key, _)| !palette.dark && *light_key == key)
                .map_or(rule, |&(_, light_rule)| light_rule);
            let rgb = match rule {
                Rule::Slot(name) => slot(name)?,
                Rule::Mix(a, b, t) => mix(slot(a)?, slot(b)?, t),
            };
            Ok((key, rgb))
        })
        .collect()
}

/// `a` moved toward `b` by `permille` thousandths, each byte rounded half to even as the theme
/// tool's Python `round` does.
pub fn mix(a: Rgb, b: Rgb, permille: u16) -> Rgb {
    let channel = |x: u8, y: u8| {
        let scaled = i32::from(x) * 1000 + (i32::from(y) - i32::from(x)) * i32::from(permille);
        let (whole, rest) = (scaled.div_euclid(1000), scaled.rem_euclid(1000));
        let up = rest > 500 || (rest == 500 && whole % 2 == 1);
        u8::try_from(whole + i32::from(up)).unwrap_or(if scaled < 0 { 0 } else { u8::MAX })
    };
    [
        channel(a[0], b[0]),
        channel(a[1], b[1]),
        channel(a[2], b[2]),
    ]
}

/// `#rrggbb` to bytes.
pub fn parse_hex(hex: &str) -> Option<Rgb> {
    let h = hex.strip_prefix('#').filter(|h| h.len() == 6)?;
    let byte = |i: usize| u8::from_str_radix(h.get(i..i + 2)?, 16).ok();
    Some([byte(0)?, byte(2)?, byte(4)?])
}

/// Bytes to `#rrggbb`.
pub fn to_hex(rgb: Rgb) -> String {
    format!("#{:02x}{:02x}{:02x}", rgb[0], rgb[1], rgb[2])
}
