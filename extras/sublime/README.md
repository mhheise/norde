# Norde for Sublime Text

[Sublime Text](https://www.sublimetext.com) color schemes, one file per Norde
variant. See the [main README](../../README.md) for the palette and how the
themes are generated.

## Install

From the repo root:

```sh
cp extras/sublime/*.sublime-color-scheme \
  ~/Library/Application\ Support/Sublime\ Text/Packages/User/
```

On Linux, the `Packages/User` folder is in `~/.config/sublime-text/`. To find it
on any platform, choose **Preferences → Browse Packages…**.

Then pick a scheme with **UI: Select Color Scheme** in the command palette, or
follow the system appearance in Sublime's settings:

```jsonc
{
  "color_scheme": "auto",
  "dark_color_scheme": "Norde.sublime-color-scheme",
  "light_color_scheme": "Norde Light.sublime-color-scheme"
}
```

## Customize

Edit [`norde.toml`](../../norde.toml), run `cargo gen -f sublime` from the repo
root, and copy the files again. Don't edit the schemes here; they're generated.
[`src/formats/sublime.rs`](../../src/formats/sublime.rs) maps Sublime's scopes
onto Norde's roles and slots.
