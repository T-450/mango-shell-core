//! The 24 px status line under each popover: the command the hovered or focused control runs,
//! then the key hints (`docs/DESIGN-SYSTEM.md`, "Interaction rules").

use rune::{El, col, row, rule, spacer, text};

use crate::tokens::{COMMAND, Tokens};

/// The line's height under its hairline, in points.
pub const HEIGHT: f32 = 24.0;

/// `command` on the left in `text.secondary`, `hint` on the right in `text.tertiary`, under a
/// 1 px `line.hairline`. It spans the panel's full width, so place it outside the panel's
/// padding.
pub fn command_line(command: &str, hint: &str, tokens: &Tokens) -> El {
    let line = row()
        .w_full()
        .h(HEIGHT)
        .items_center()
        .gap(12.0)
        .pl(16.0)
        .pr(12.0)
        .child(
            text(command)
                .font(COMMAND)
                .color(tokens.text_secondary)
                .truncate()
                .shrink(),
        )
        .child(spacer())
        .child(text(hint).font(COMMAND).color(tokens.text_tertiary));
    col()
        .w_full()
        .child(rule(1.0, tokens.line_hairline))
        .child(line)
}
