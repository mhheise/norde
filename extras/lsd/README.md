# Norde for lsd

[lsd](https://github.com/lsd-rs/lsd) color themes, one `colors.yaml` per Norde
variant. See the [main README](../../README.md) for the palette and how the
themes are generated.

lsd's themes cover metadata (users, permissions, dates, sizes, and Git status).
File names are colored by the `LS_COLORS` environment variable instead; see
[Norde for `LS_COLORS`](../ls-colors/README.md). The
colors match the [eza](../eza/README.md) themes.

## Install

From the repo root, copy the variant you want:

```sh
mkdir -p ~/.config/lsd
cp extras/lsd/norde/colors.yaml ~/.config/lsd/      # or norde_light
```

Then tell lsd to use it in `~/.config/lsd/config.yaml`:

```yaml
color:
  theme: custom
```

## Customize

Edit [`norde.toml`](../../norde.toml), run `cargo gen -f lsd` from the repo
root, and copy the file again. Don't edit the themes here; they're generated.
The color choices are in
[`src/formats/listing.rs`](../../src/formats/listing.rs), shared with eza.
