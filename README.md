# Norde

[Nord](https://www.nordtheme.com/)-based color themes, generated from a single
spec. See the [variants](#variants) and
[supported applications](#supported-applications).

Existing Nord themes tend to use only a handful of the palette's colors. Norde
instead puts **all 16 Nord colors to work**, with each color assigned a
consistent semantic role so code is easy to scan at a glance.

## Variants

- **Norde** (dark): its mapping and Nord Deep–style backgrounds follow the
  excellent [Yukinord](https://github.com/yukina3230/yukinord_zed) theme.
- **Norde Light** (light): a principled counterpart with the same role→hue
  logic, with the Aurora/Frost accents deepened for contrast on a bright
  background.

[`preview.html`](preview.html) shows a code sample in every variant.

## Supported applications

Each application's README covers setup, customization, and anything specific to
it.

- [bat and delta](extras/bat/README.md)
- [Ghostty](extras/ghostty/README.md)
- [Helix](extras/helix/README.md)
- [Starship](extras/starship/README.md)
- [Sublime Text](extras/sublime/README.md)
- [Zed](extras/zed/README.md)

To support another one, see [Adding an application](#adding-an-application).

## Syntax roles

Every semantic role keeps the same *hue* across variants; only the lightness
changes for the background. All variants share this mapping:

| Role                              | Hue    | Dark (Norde)     | Light (Norde Light) |
| --------------------------------- | ------ | ---------------- | ------------------- |
| Keywords, tags, constructors      | Frost  | `#88C0D0` nord8  | `#2C7083`           |
| Functions, macros, escapes, regex | Yellow | `#EBCB8B` nord13 | `#8B6518`           |
| Types, enums, namespaces          | Green  | `#A3BE8C` nord14 | `#3F641F`           |
| Strings, URIs                     | Orange | `#D08770` nord12 | `#B95231`           |
| Numbers, booleans, constants      | Purple | `#B48EAD` nord15 | `#9B3F8A`           |
| Comments (italic)                 | Gray   | `#8E929C`        | `#646E85`           |
| Variables, operators, punct.      | Text   | `#D8DEE9` nord4  | `#3B4252` nord1     |

Each **light** accent is *derived from its dark counterpart's Nord hue* (same
hue, re-lightened and re-saturated for a bright background), so every semantic
role keeps one identity across variants. Lightness follows a deliberate curve:
all accents clear **WCAG-AA (≥4.5:1)** against the editor background. Green is
pulled darkest while orange is pulled lightest so the two never collapse
together for red-green color-vision deficiency (~6-point luminance gap). This
favors a single internally consistent system over strict adherence to canonical
Nord values on the light side.

### Deviation from Yukinord

One deliberate change: **numeric/constant literals** (`number`, `boolean`,
`constant`) use **purple (nord15)** rather than yellow. In a literal Yukinord
copy, numbers share yellow with function calls, and purple is used only for
Markdown links, so it sits idle in real code. Moving literals to purple restores
[Nord's documented intent](https://www.nordtheme.com/docs/colors-and-palettes)
(numbers = nord15), separates literals from calls, and gives every palette color
a job.

To revert to Yukinord's exact mapping, set `constant = { color = "yellow" }` in
the `[syntax]` table of [`norde.toml`](norde.toml) and
[regenerate](#regenerating).

## Font styles

- **Comments**: italic (also Markdown `*emphasis*`).
- **Control-flow keywords**: bold (`if`, `else`, `for`, `while`, `match`,
  `return`, `break`, `try`/`catch`, `async`/`await`). Bolding *only* these, not
  every keyword, makes the branching/looping skeleton of a file scannable while
  leaving boilerplate like `let`, `const`, `pub`, `fn`, and imports at regular
  weight. Markdown `**strong**` and `# headings` are bold too.

How much of this is visible depends on the application: some don't style text at
all, and some don't distinguish control flow from other keywords. Bold also
requires a font with a bold weight (SF Mono, JetBrains Mono, Fira Code, … all
qualify). To disable it for control flow, drop `bold = true` from
`keyword_control` in [`norde.toml`](norde.toml) and regenerate.

## Regenerating

Every output is generated from one spec, [`norde.toml`](norde.toml), by a small
Rust tool in [`src/`](src). Edit the spec, never the outputs:

```sh
cargo gen            # regenerate every format
cargo gen --check    # exit non-zero if any output is stale (for CI)
cargo gen -f zed     # render just one format, by name
```

Run these from the repo root. `cargo gen` is an alias (in `.cargo/config.toml`)
for `cargo run --quiet --`.

The spec has three layers:

1. **`[palette]`**: the shared Nord colors.
2. **`[syntax]`**: application-neutral roles (`keyword`, `keyword_control`,
   `function`, `type`, `string`, `constant`, `comment`, …), each with
   `bold`/`italic` and a *color name* that every variant defines. All variants
   share this table, so each role keeps one hue identity.
3. **`[[variants]]`**: per-theme `colors` (the syntax hues plus local tints) and
   semantic `ui` / `terminal` slots such as `ui.surface.panel`,
   `ui.status.error`, `ui.vcs.added`, and `terminal.bright.red`.

Anywhere a color goes, write a hex value (`"#1D2129"`), a name (`"nord8"`), or
either one with an alpha byte (`"nord1/00"`). Unknown names, missing slots, and
typos fail with the exact path, e.g.
``variant "Norde Light": ui.surface: unknown field `hovr` ``. The generator also
warns when any syntax role drops below WCAG-AA (4.5:1) against the editor
background. `cargo test` enforces that for Norde, along with readable terminal
colors.

### Adding an application

Each output format is a module in [`src/formats/`](src/formats) that implements
`Renderer`. It maps the neutral model (syntax roles, UI slots, ANSI colors) onto
the application's own keys. To add one:

1. Write `src/formats/<app>.rs`, using an existing module as a starting point.
2. Register it in `ALL` in `src/formats/mod.rs`.
3. Enable it in `norde.toml`. For example, Zed's entry is:

   ```toml
   [formats.zed]
   path = "extras/zed/themes/norde.json"
   ```

   This renders every variant into one file. If the application wants one file
   per variant, put `{name}` in the path, or `{slug}` for a lowercase file name
   such as `norde_light`.
4. Add `extras/<app>/README.md` and list it under
   [Supported applications](#supported-applications).

## Credits

- [Nord](https://www.nordtheme.com/) color palette by Arctic Ice Studio.
- Dark mapping and Nord Deep backgrounds adapted from
  [Yukinord](https://github.com/yukina3230/yukinord_zed) (itself based on
  [Nord Deep](https://github.com/marlosirapuan/vscode-theme-nord-deep)).
