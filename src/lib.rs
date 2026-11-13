//! What `docs/DESIGN-SYSTEM.md` specifies for two or more mango-shell components: design tokens
//! and their palette mapping, theme loading, the bevelled panel, corner caps and focus ring, the
//! row rim, the command line under each popover, and the popover surface settings.
//!
//! Components reach rune only through [`rune`], so a component cannot build a second copy.
#![forbid(unsafe_code)]

pub use rune;

pub mod bevel;
pub mod command_line;
pub mod contrast;
pub mod mapping;
pub mod surface;
pub mod theme;
pub mod tokens;
