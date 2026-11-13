//! WCAG 2.x contrast, for the token checks of `docs/DESIGN-SYSTEM.md`, "Light mode".

use rune::Color;

/// The contrast ratio between two opaque colours, from 1 to 21.
pub fn ratio(a: Color, b: Color) -> f32 {
    // rune's colours are linear sRGB, so this is WCAG's relative luminance.
    let (la, lb) = (a.luminance(), b.luminance());
    (la.max(lb) + 0.05) / (la.min(lb) + 0.05)
}

#[cfg(test)]
#[path = "contrast_tests.rs"]
mod tests;
