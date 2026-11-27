# Norde for `LS_COLORS`

Shell snippets that set the `LS_COLORS` environment variable, one per Norde
variant. GNU `ls`, [lsd](../lsd/README.md), fd, tree, and many other tools read
it to color file names. See the [main README](../../README.md) for the palette
and how the themes are generated.

The colors match the [eza](../eza/README.md) and lsd themes: directories,
executables, links, and special files by kind, and media, archives, keys,
temporary, compiled, and build files by name.

macOS's built-in `ls` reads `LSCOLORS` instead, so it isn't affected; GNU `ls`
from Homebrew's coreutils (`gls`) is.

## Install

Source the variant you want from your shell's startup file, such as
`~/.zshrc` or `~/.bashrc`:

```sh
source ~/path/to/norde/extras/ls-colors/norde.sh   # or norde_light.sh
```

The snippets are tested with bash, zsh, and fish.

## Customize

Edit [`norde.toml`](../../norde.toml), run `cargo gen -f ls_colors` from the
repo root, and open a new shell. Don't edit the snippets here; they're
generated. The color choices are in
[`src/formats/listing.rs`](../../src/formats/listing.rs) and the file-type
lists in [`src/formats/ls_colors.rs`](../../src/formats/ls_colors.rs).
