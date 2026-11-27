# Norde for bat and delta

TextMate `.tmTheme` themes, one file per Norde variant, for
[bat](https://github.com/sharkdp/bat) and
[delta](https://github.com/dandavison/delta), which reads bat's themes. See the
[main README](../../README.md) for the palette and how the themes are generated.

## Install

From the repo root:

```sh
mkdir -p "$(bat --config-dir)/themes"
cp extras/bat/*.tmTheme "$(bat --config-dir)/themes/"
bat cache --build
```

`bat --list-themes` should now include **Norde** and **Norde Light**.

### bat

Set the theme in bat's config file (`bat --config-file` shows where it is):

```sh
--theme="Norde"
```

Or follow the system appearance:

```sh
--theme=auto:system
--theme-dark="Norde"
--theme-light="Norde Light"
```

bat draws comments in italics only with `--italic-text=always`.

### delta

delta uses the themes from bat's cache, so install them as above, then set the
syntax theme in your Git config:

```gitconfig
[delta]
    syntax-theme = Norde
```

delta's docs recommend a bat version that matches your delta release, since a
mismatched cache can crash delta.

## Customize

Edit [`norde.toml`](../../norde.toml), run `cargo gen -f tmtheme` from the repo
root, then copy the files and rebuild bat's cache again. Don't edit the themes
here; they're generated. The scope rules are shared with the Sublime Text
schemes, in [`src/formats/textmate.rs`](../../src/formats/textmate.rs).
