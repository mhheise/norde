# Norde for Zed

[Zed](https://zed.dev) extension with every Norde variant. See the
[main README](../../README.md) for the palette, syntax roles, and how the themes
are generated.

## Install

Install it as a dev extension:

1. Run `zed: extensions` and choose **Install Dev Extension**.
2. Select this folder (`extras/zed`), not the repo root.
3. Run `theme selector: toggle` and pick a variant.

To follow the system appearance, set both in Zed's `settings.json`:

```jsonc
{
  "theme": {
    "mode": "system",
    "dark": "Norde",
    "light": "Norde Light"
  }
}
```

## Customize

For a quick personal tweak, override individual styles in Zed's `settings.json`:

```jsonc
{
  "theme_overrides": {
    "Norde": {
      "syntax": { "comment": { "font_style": "normal" } }
    }
  }
}
```

To change the theme itself, edit [`norde.toml`](../../norde.toml) and run
`cargo gen -f zed` from the repo root. Don't edit `themes/norde.json`; it's
generated. [`src/formats/zed.rs`](../../src/formats/zed.rs) maps Zed's UI keys
and tree-sitter captures onto Norde's slots and roles.

## Bold control-flow keywords

Bold targets tree-sitter's granular keyword captures (`keyword.control`,
`keyword.conditional`, …), which all map to the `keyword_control` role. Coverage
is **grammar-dependent**: a language whose grammar only emits a flat `keyword`
capture will show no bolded keywords (it degrades gracefully rather than bolding
everything).

## Optional: semantic highlighting

Zed can also style tokens by meaning, using information from the language
server. For example, it can underline mutable bindings, as
[pale-fire](https://github.com/matklad/pale-fire) does, or strike through
deprecated APIs.

These are editor settings that apply under any theme, so they're provided as a
recipe rather than as part of the theme. The essentials, for Zed's
`settings.json`:

```jsonc
{
  "semantic_tokens": "combined", // keep Norde's colors, layer modifiers on top
  "global_lsp_settings": {
    "semantic_token_rules": [
      { "token_type": "variable", "token_modifiers": ["mutable"], "underline": true },
      { "token_type": "parameter", "token_modifiers": ["mutable"], "underline": true },
      { "token_modifiers": ["deprecated"], "strikethrough": true }
    ]
  }
}
```

[`semantic_token_rules.jsonc`](semantic_token_rules.jsonc) has the same rules
plus commented-out extras (unsafe, static, standard library).

`semantic_tokens` controls how semantic tokens combine with tree-sitter:

- `"off"` (Zed's default): tree-sitter only.
- `"combined"` (recommended): layers semantic tokens over tree-sitter, which
  keeps the Norde palette.
- `"full"`: replaces tree-sitter entirely, so the language server drives all
  syntax coloring.

`semantic_token_rules` sets the styles. Earlier rules take precedence. The
recipe favors decorations over fixed colors so it looks right in every variant:
`"underline": true` uses the token's own color, while a hex string underlines in
a fixed one. Modifier names vary by language server; the
`editor: toggle semantic tokens highlight` action shows the ones in use.

## Publish

Set `path = "extras/zed"` in the extension's entry in `extensions.toml` in
[zed-industries/extensions](https://github.com/zed-industries/extensions).
`LICENSE` here is a symlink to the root one, because Zed requires a license
inside the extension's own directory.
