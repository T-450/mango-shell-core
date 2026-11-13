//! Shows that a press inside a corner cap reaches the control underneath.
//!
//! `just hit-check` renders this app headless with `hit.json`, which taps inside the button's
//! top-left cap, writes rune's element tree, then runs it again with `--check <tree.json>`.
//! The check passes when the button is hovered and its label reads "pressed 1".

use std::fs;

use mango_shell_core::bevel::{Bevel, bevel_frame};
use mango_shell_core::rune::{AppSpec, Cx, El, el, text};
use mango_shell_core::tokens::{BODY, DARK, FAMILIES};

/// Where the button sits in the window; `hit.json` taps at its top-left corner plus one point.
const BUTTON_AT: (f32, f32) = (20.0, 20.0);

#[derive(Default)]
struct State {
    presses: u32,
}

fn view(s: &State, _: &mut Cx<State>) -> El {
    let t = DARK;
    let label = format!("pressed {}", s.presses);
    let button = el()
        .key("button")
        .label(label.clone())
        .size(120.0, 24.0)
        .center()
        .fill(t.bg_raised)
        .border(1.0, t.line_strong)
        .hover(t.text_secondary.alpha(0.06))
        .on_press(|s: &mut State| s.presses += 1)
        .child(text(label).font(BODY).color(t.text_primary));
    el().w_full().h_full().fill(t.bg_bar).child(
        bevel_frame(button, Bevel::Control, t.bg_bar, Some(t.line_strong))
            .absolute(BUTTON_AT.0, BUTTON_AT.1),
    )
}

fn check(path: &str) -> Result<(), String> {
    let src = fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
    let tree: serde_json::Value = serde_json::from_str(&src).map_err(|e| format!("{path}: {e}"))?;
    let button = tree
        .get("elements")
        .and_then(serde_json::Value::as_array)
        .and_then(|els| {
            els.iter()
                .find(|e| e.get("key").and_then(serde_json::Value::as_str) == Some("button"))
        })
        .ok_or("no element keyed \"button\" in the tree")?;
    let hovered = button
        .get("hover")
        .and_then(serde_json::Value::as_f64)
        .is_some_and(|h| h > 0.0);
    let label = button
        .get("label")
        .and_then(serde_json::Value::as_str)
        .unwrap_or_default();
    if hovered && label == "pressed 1" {
        Ok(())
    } else {
        Err(format!(
            "the cap swallowed the press: hovered {hovered}, label {label:?}"
        ))
    }
}

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if let [_, flag, path] = args.as_slice()
        && flag == "--check"
    {
        return check(path);
    }
    mango_shell_core::rune::run(AppSpec {
        families: FAMILIES,
        ..AppSpec::new("bevel-hit", (160.0, 64.0), State::default, view)
    });
    Ok(())
}
