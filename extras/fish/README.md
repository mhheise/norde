# Norde for fish

A [fish](https://fishshell.com) theme for syntax highlighting on the command
line and in the completion pager. See the [main README](../../README.md) for the
palette and how the themes are generated.

One file holds every variant: fish picks the `[dark]` or `[light]` section to
match the terminal's background, and uses the dark one when the terminal doesn't
report it. Switching sections needs fish 4.3 or later.

## Install

From the repo root:

```sh
mkdir -p ~/.config/fish/themes
cp extras/fish/norde.theme ~/.config/fish/themes/
```

Then load it from `~/.config/fish/config.fish`, so it applies to every session
and keeps following the terminal's background:

```fish
fish_config theme choose norde
```

To color file names in `ls` and other tools too, source the
[`LS_COLORS` snippet](../ls-colors/README.md) from the same file.

## Customize

Edit [`norde.toml`](../../norde.toml), run `cargo gen -f fish` from the repo
root, and copy the file again. Don't edit the theme here; it's generated.
[`src/formats/fish.rs`](../../src/formats/fish.rs) maps fish's color variables
onto Norde's roles and slots.
