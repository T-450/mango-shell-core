//! Chamfered ("bevelled") corners: the panel outline, corner caps for controls, the focus ring
//! and the row rim (`docs/DESIGN-SYSTEM.md`, "Shape" and "Interaction rules").
//!
//! rune draws rectangles, so a control is a rectangle whose cut corners are covered by small
//! inert triangles in the colour underneath ("caps"). Caps take no input: a press on a cap
//! reaches the control. The panel itself sits on a transparent surface, where a cap would hide
//! nothing, so it is drawn as one filled polygon instead.

use rune::shape::LineCap;
use rune::{Color, El, Shape, canvas, el};

use crate::tokens::{
    GLASS_HIGHLIGHT_ALPHA, SHAPE_CARD, SHAPE_CONTROL, SHAPE_NESTED, SHAPE_PANEL, Tokens,
};

/// A chamfer size and the corners it cuts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bevel {
    /// 2 px, all corners: rows, segments, icon boxes.
    Nested,
    /// 4 px, all corners: buttons, inputs, chips.
    Control,
    /// 8 px, top-left and bottom-right: cards, bars.
    Card,
    /// 10 px, top-left and bottom-right: popovers, dialogs.
    Panel,
}

/// One corner of a box.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Corner {
    /// Top left.
    TopLeft,
    /// Top right.
    TopRight,
    /// Bottom right.
    BottomRight,
    /// Bottom left.
    BottomLeft,
}

impl Corner {
    /// Clockwise from the top left.
    pub const ALL: [Corner; 4] = [
        Corner::TopLeft,
        Corner::TopRight,
        Corner::BottomRight,
        Corner::BottomLeft,
    ];
}

impl Bevel {
    /// The chamfer's leg length in points.
    pub const fn size(self) -> f32 {
        match self {
            Bevel::Nested => SHAPE_NESTED,
            Bevel::Control => SHAPE_CONTROL,
            Bevel::Card => SHAPE_CARD,
            Bevel::Panel => SHAPE_PANEL,
        }
    }

    /// Whether this bevel cuts `corner`.
    pub const fn cuts(self, corner: Corner) -> bool {
        match self {
            Bevel::Nested | Bevel::Control => true,
            Bevel::Card | Bevel::Panel => matches!(corner, Corner::TopLeft | Corner::BottomRight),
        }
    }
}

/// The outline of a `width` x `height` box with `bevel`'s corners cut, moved `inset` points
/// inward. Clockwise from the top-left.
pub fn outline(width: f32, height: f32, bevel: Bevel, inset: f32) -> Vec<[f32; 2]> {
    // The diagonal moved in by `inset` is x + y = size + inset * sqrt 2; it meets the edges
    // moved in by `inset` this far from the moved corner.
    let leg = bevel.size() + inset * (std::f32::consts::SQRT_2 - 2.0);
    let (left, top) = (inset, inset);
    let (right, bottom) = (width - inset, height - inset);
    let mut points = Vec::with_capacity(8);
    for corner in Corner::ALL {
        let cut = bevel.cuts(corner);
        let pair = match corner {
            Corner::TopLeft => [[left, top + leg], [left + leg, top]],
            Corner::TopRight => [[right - leg, top], [right, top + leg]],
            Corner::BottomRight => [[right, bottom - leg], [right - leg, bottom]],
            Corner::BottomLeft => [[left + leg, bottom], [left, bottom - leg]],
        };
        let square = match corner {
            Corner::TopLeft => [left, top],
            Corner::TopRight => [right, top],
            Corner::BottomRight => [right, bottom],
            Corner::BottomLeft => [left, bottom],
        };
        if cut {
            points.extend(pair);
        } else {
            points.push(square);
        }
    }
    points
}

/// The cap for `corner` in its own `box_size` square: the triangle outside a chamfer of leg
/// `leg`, and, with a rim `width`, the band that carries the rim along the chamfer.
pub fn cap_geometry(
    corner: Corner,
    leg: f32,
    width: f32,
    box_size: f32,
) -> ([[f32; 2]; 3], Option<[[f32; 2]; 4]>) {
    // Drawn for the top-left corner, then mirrored into place.
    let k = width * (std::f32::consts::SQRT_2 - 1.0);
    let triangle = [[0.0, 0.0], [leg, 0.0], [0.0, leg]];
    let band = [[leg, 0.0], [leg + k, width], [width, leg + k], [0.0, leg]];
    let place = |[x, y]: [f32; 2]| match corner {
        Corner::TopLeft => [x, y],
        Corner::TopRight => [box_size - x, y],
        Corner::BottomRight => [box_size - x, box_size - y],
        Corner::BottomLeft => [x, box_size - y],
    };
    (triangle.map(place), (width > 0.0).then(|| band.map(place)))
}

/// Pins one inert cap per corner `bevel` cuts onto `content`'s corners.
fn caps(mut content: El, bevel: Bevel, leg: f32, under: Color, rim: Option<(Color, f32)>) -> El {
    let width = rim.map_or(0.0, |(_, w)| w);
    let box_size = leg + width;
    for corner in Corner::ALL.into_iter().filter(|c| bevel.cuts(*c)) {
        let (triangle, band) = cap_geometry(corner, leg, width, box_size);
        let mut shape = Shape::new().fill(triangle.to_vec(), under);
        if let (Some(band), Some((color, _))) = (band, rim) {
            shape = shape.fill(band.to_vec(), color);
        }
        let cap = canvas(shape).size(box_size, box_size).inert();
        let at = Some(0.0);
        content = content.child(match corner {
            Corner::TopLeft => cap.pin(at, at, None, None),
            Corner::TopRight => cap.pin(None, at, at, None),
            Corner::BottomRight => cap.pin(None, None, at, at),
            Corner::BottomLeft => cap.pin(at, None, None, at),
        });
    }
    content
}

/// Cuts `bevel`'s corners off `content`, a rectangle on a surface of colour `under`. With a
/// `rim` colour, the 1 px rim that `content` draws with `El::border` continues along each cut.
pub fn bevel_frame(content: El, bevel: Bevel, under: Color, rim: Option<Color>) -> El {
    caps(content, bevel, bevel.size(), under, rim.map(|c| (c, 1.0)))
}

/// The design's focus ring around `content`: a 2 px gap in `under`, then 2 px of
/// `accent.focus`, both following the bevel.
pub fn bevel_ring(content: El, bevel: Bevel, under: Color, tokens: &Tokens) -> El {
    const GAP: f32 = 2.0;
    const RING: f32 = 2.0;
    let outset = GAP + RING;
    let ring = el()
        .pin(Some(-outset), Some(-outset), Some(-outset), Some(-outset))
        .border(RING, tokens.accent_focus)
        .inert();
    // The ring's chamfer grows with its spread, as CSS's box-shadow spread does.
    let ring = caps(
        ring,
        bevel,
        bevel.size() + outset,
        under,
        Some((tokens.accent_focus, RING)),
    );
    bevel_frame(content, bevel, under, None).child(ring)
}

/// The focus rim of a full-width row: 1 px of `accent.signal` along a [`Bevel::Nested`] outline.
pub fn row_rim(row: El, under: Color, tokens: &Tokens) -> El {
    bevel_frame(
        row.border(1.0, tokens.accent_signal),
        Bevel::Nested,
        under,
        Some(tokens.accent_signal),
    )
}

/// A popover panel of `size` holding `content`: the [`Bevel::Panel`] outline filled with
/// `bg.bar`, a 1 px `line.strong` rim and a 1 px white top highlight, on a transparent surface.
pub fn panel(content: El, size: (f32, f32), tokens: &Tokens) -> El {
    let (w, h) = size;
    let leg = Bevel::Panel.size();
    let highlight = Color::WHITE.alpha(GLASS_HIGHLIGHT_ALPHA);
    // The rim is the outline in `line.strong` with the outline 1 px in laid over it in `bg.bar`.
    let backdrop = Shape::new()
        .fill(outline(w, h, Bevel::Panel, 0.0), tokens.line_strong)
        .fill(outline(w, h, Bevel::Panel, 1.0), tokens.bg_bar)
        .line([leg, 1.5], [w - 1.0, 1.5], 1.0, LineCap::Butt, highlight);
    el().size(w, h)
        .child(canvas(backdrop).size(w, h).absolute(0.0, 0.0).inert())
        .child(content.size(w, h))
}

#[cfg(test)]
#[path = "bevel_tests.rs"]
mod tests;
