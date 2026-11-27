# Norde for Helix

[Helix](https://helix-editor.com) themes, one file per Norde variant. See the
[main README](../../README.md) for the palette and how the themes are generated.
They target the current Helix release.

## Install

From the repo root:

```sh
mkdir -p ~/.config/helix/themes
cp extras/helix/norde*.toml ~/.config/helix/themes/
```

Then set the theme in Helix's `config.toml` (`:config-open`), or try one with
`:theme norde`:

```toml
theme = "norde"
```

To follow the terminal's light or dark mode, use a `[theme]` table instead:

```toml
[theme]
dark = "norde"
light = "norde_light"
```

Helix detects the mode only in terminals that report it; otherwise it uses the
dark theme.

## Customize

Every color goes through the theme's `[palette]`, named after its Norde role or
slot (`keyword`, `selection`, `error`, …). To change one for yourself, create a
theme that inherits from Norde:

```toml
# ~/.config/helix/themes/my_norde.toml
inherits = "norde"

[palette]
comment = "#9AA3B2"
```

To change the theme itself, edit [`norde.toml`](../../norde.toml), run
`cargo gen -f helix` from the repo root, and copy the files again. Don't edit
the themes here; they're generated.
[`src/formats/helix.rs`](../../src/formats/helix.rs) maps Helix's keys onto
Norde's roles and slots.
