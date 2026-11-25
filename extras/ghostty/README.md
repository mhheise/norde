# Norde for Ghostty

[Ghostty](https://ghostty.org) themes, one file per Norde variant. See the
[main README](../../README.md) for the palette and how the themes are generated.

## Install

From the repo root:

```sh
mkdir -p ~/.config/ghostty/themes
cp extras/ghostty/Norde* ~/.config/ghostty/themes/
```

Then set the theme in the Ghostty config and reload it:

```ini
# one variant
theme = Norde

# or follow the system appearance
theme = light:Norde Light,dark:Norde
```

## Customize

Edit [`norde.toml`](../../norde.toml), run `cargo gen -f ghostty` from the repo
root, and copy the files again. Don't edit the theme files here; they're
generated. The relevant slots are `terminal.*`, `ui.accent` (cursor), and
`ui.surface.selection`, mapped in
[`src/formats/ghostty.rs`](../../src/formats/ghostty.rs).
