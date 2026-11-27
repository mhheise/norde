# Norde for Starship

A powerline [Starship](https://starship.rs) prompt in Norde colors, adapted
from Starship's
[Catppuccin Powerline](https://starship.rs/presets/catppuccin-powerline) preset.
See the [main README](../../README.md) for the palette and how the themes are
generated.

The icons need a [Nerd Font](https://www.nerdfonts.com), and some modules need a
recent Starship release.

## Install

[`starship.toml`](starship.toml) is a complete Starship config, so it replaces
an existing one. From the repo root:

```sh
cp ~/.config/starship.toml ~/.config/starship.toml.bak  # if you have one
cp extras/starship/starship.toml ~/.config/starship.toml
```

Or point Starship at it without replacing anything:

```sh
export STARSHIP_CONFIG=~/path/to/norde/extras/starship/starship.toml
```

## Pick a variant

Starship can't detect the terminal's light or dark mode, so the config selects
a palette by name. It defaults to `norde`; for Norde Light, change the line near
the top:

```toml
palette = 'norde_light'
```

## Customize

Each segment has its own palette entry (`os`, `directory`, `git`, `language`,
`environment`, and `time`), as do `segment_text` and the prompt symbols
(`success`, `error`, `replace`, and `visual`). Override them in a copy of the
config, or edit [`norde.toml`](../../norde.toml) and run `cargo gen -f starship`
from the repo root.

The layout itself (segments, icons, and modules) is
[`src/formats/starship_layout.toml`](../../src/formats/starship_layout.toml).
Edit that rather than the generated `starship.toml`, then regenerate.
