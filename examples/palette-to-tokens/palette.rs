//! Reads a theme-tool palette TOML into a [`Palette`]; shared by the generator and core's tests.

use super::mapping::{Palette, parse_hex};

/// `[meta] variant`, `[colors]`, and `[terminal]` (its keys as `term_<key>`, `ansi` as
/// `ansi0` to `ansi15`), as the theme tool names them.
pub fn palette_from_toml(src: &str) -> Result<Palette, String> {
    let doc: toml::Table = src.parse().map_err(|e| format!("palette: {e}"))?;
    let section = |name: &str| {
        doc.get(name)
            .and_then(toml::Value::as_table)
            .ok_or(format!("palette: no [{name}]"))
    };
    let hex = |name: &str, v: &toml::Value| {
        v.as_str()
            .and_then(parse_hex)
            .ok_or_else(|| format!("palette: {name} is not #rrggbb"))
    };
    let dark = section("meta")?
        .get("variant")
        .and_then(toml::Value::as_str)
        == Some("dark");
    let mut palette = Palette {
        dark,
        ..Palette::default()
    };
    for (name, value) in section("colors")? {
        palette.roles.insert(name.clone(), hex(name, value)?);
    }
    for (name, value) in section("terminal")? {
        if name == "ansi" {
            let ansi = value
                .as_array()
                .ok_or("palette: terminal.ansi is not a list")?;
            for (i, c) in ansi.iter().enumerate() {
                palette.roles.insert(format!("ansi{i}"), hex(name, c)?);
            }
        } else {
            palette
                .roles
                .insert(format!("term_{name}"), hex(name, value)?);
        }
    }
    Ok(palette)
}
