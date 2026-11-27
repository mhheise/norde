# Norde for eza

[eza](https://github.com/eza-community/eza) themes, one `theme.yml` per Norde
variant. See the [main README](../../README.md) for the palette and how the
themes are generated.

eza and [lsd](../lsd/README.md) share the same color choices: file names and
permissions use Norde's syntax hues, and secondary metadata (dates, inodes,
punctuation) is muted.

## Install

eza reads `theme.yml` from `~/.config/eza/` (or `$EZA_CONFIG_DIR`). From the
repo root, copy the variant you want:

```sh
mkdir -p ~/.config/eza
cp extras/eza/norde/theme.yml ~/.config/eza/        # or norde_light
```

`EZA_COLORS` and `LS_COLORS` override the theme, so unset them if colors don't
change.

## Customize

Edit [`norde.toml`](../../norde.toml), run `cargo gen -f eza` from the repo
root, and copy the file again. Don't edit the themes here; they're generated.
The color choices are in
[`src/formats/listing.rs`](../../src/formats/listing.rs), shared with lsd.
