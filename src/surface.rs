//! The popover surface: a layer-shell overlay on the bar's edge that takes the pointer and
//! asks for the keyboard when it shows.

use rune::{
    Align, ConfiguredOverlay, Cx, El, KeyPolicy, OverlayInput, OverlayOptions, OverlaySpec,
    ScreenEdge,
};

/// A popover overlay named `name` (the layer-shell namespace), `size` points, on the bottom edge
/// `margin` points from it, `inset` points from the output's end edge. It takes every pointer
/// event and permits `OverlayHandle::request_keys`.
pub fn popover_overlay<T>(
    name: &'static str,
    size: (f32, f32),
    inset: f32,
    margin: f32,
    view: fn(&T, &mut Cx<T>) -> El,
) -> ConfiguredOverlay<T> {
    let spec = OverlaySpec {
        name,
        size,
        edge: ScreenEdge::Bottom,
        margin,
        align: Align::End,
        inset,
        interactive: true,
        view,
    };
    spec.options(OverlayOptions {
        input: OverlayInput::All,
        keys: KeyPolicy::OnDemand,
        ..OverlayOptions::default()
    })
}
