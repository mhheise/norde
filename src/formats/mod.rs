//! Output formats, each with its own [`Renderer`].
//!
//! Adding an application takes one module and one line in [`ALL`].

use anyhow::Result;

use crate::theme::{Family, Variant};

mod eza;
mod ghostty;
mod helix;
mod listing;
mod ls_colors;
mod lsd;
mod preview;
mod starship;
mod sublime;
mod textmate;
mod tmtheme;
mod zed;

/// Renders themes in one output format.
pub trait Renderer: Sync {
    /// The format's key under `[formats.<name>]` in the spec.
    fn name(&self) -> &'static str;

    /// Render `variants` into one file's contents.
    ///
    /// Called once with every variant or, when the configured path contains
    /// `{name}` or `{slug}`, once per variant with a single-element slice. Formats that can
    /// only hold one theme should reject `variants.len() != 1`.
    fn render(&self, family: &Family, variants: &[Variant]) -> Result<String>;
}

pub static ALL: &[&dyn Renderer] = &[
    &zed::Zed,
    &preview::Preview,
    &ghostty::Ghostty,
    &sublime::Sublime,
    &helix::Helix,
    &starship::Starship,
    &tmtheme::TmTheme,
    &eza::Eza,
    &lsd::Lsd,
    &ls_colors::LsColors,
];

pub fn lookup(name: &str) -> Option<&'static dyn Renderer> {
    ALL.iter().copied().find(|t| t.name() == name)
}

pub fn names() -> Vec<&'static str> {
    ALL.iter().map(|t| t.name()).collect()
}
