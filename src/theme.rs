//! The theme model and loading it from a TOML spec.
//!
//! A spec is written with color *references* (`"nord8"`, `"#1D2129"`,
//! `"nord1/00"`). Loading resolves every reference per variant, so formats only
//! ever see concrete [`Color`]s.

use std::collections::BTreeMap;
use std::path::Path;

use anyhow::{Context, Result, anyhow, bail};
use serde::Deserialize;
use serde::de::DeserializeOwned;
use toml::{Table, Value};

use crate::color::{Color, parse_alpha};

// ---------------------------------------------------------------------------
// Resolved model (what formats consume)
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct Family {
    pub name: String,
    pub author: String,
    pub formats: BTreeMap<String, FormatConfig>,
    pub variants: Vec<Variant>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FormatConfig {
    /// Output path, relative to the spec file.
    ///
    /// `{name}` expands to each variant's name and `{slug}` to a file-friendly
    /// form of it (`Norde Light` becomes `norde_light`). Either makes the
    /// format render one file per variant.
    pub path: String,
}

impl FormatConfig {
    pub fn per_variant(&self) -> bool {
        self.path.contains("{name}") || self.path.contains("{slug}")
    }

    pub fn path_for(&self, v: &Variant) -> String {
        self.path
            .replace("{name}", &v.name)
            .replace("{slug}", &slug(&v.name))
    }
}

/// Lowercase, with each run of other characters replaced by one `_`.
fn slug(name: &str) -> String {
    let mut out = String::new();
    for c in name.chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c.to_ascii_lowercase());
        } else if !out.is_empty() && !out.ends_with('_') {
            out.push('_');
        }
    }
    out.trim_end_matches('_').to_owned()
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Variant {
    pub name: String,
    pub appearance: Appearance,
    pub syntax: Syntax,
    pub ui: Ui,
    pub terminal: Terminal,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Appearance {
    Dark,
    Light,
}

impl Appearance {
    pub fn as_str(self) -> &'static str {
        match self {
            Appearance::Dark => "dark",
            Appearance::Light => "light",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Style {
    pub color: Color,
    #[serde(default)]
    pub bold: bool,
    #[serde(default)]
    pub italic: bool,
}

/// Application-neutral syntax roles.
///
/// Each format maps its own scope or capture names onto these.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Syntax {
    pub comment: Style,
    pub keyword: Style,
    /// Control flow: `if`, `for`, `return`, `match`, `await`, etc.
    pub keyword_control: Style,
    pub tag: Style,
    pub constructor: Style,
    /// `self`, `this`, and builtins that look like variables.
    pub variable_special: Style,
    pub function: Style,
    /// Escapes, regexes, symbols.
    pub string_special: Style,
    #[serde(rename = "type")]
    pub ty: Style,
    pub namespace: Style,
    pub string: Style,
    /// Numbers, booleans, named constants.
    pub constant: Style,
    pub variable: Style,
    pub property: Style,
    pub attribute: Style,
    pub operator: Style,
    pub punctuation: Style,
    pub markup_heading: Style,
    pub markup_emphasis: Style,
    pub markup_strong: Style,
    pub markup_raw: Style,
    pub markup_link_text: Style,
    pub markup_link_url: Style,
}

impl Syntax {
    /// Every role with its spec name, for formats and lints that iterate.
    pub fn roles(&self) -> [(&'static str, &Style); 23] {
        [
            ("comment", &self.comment),
            ("keyword", &self.keyword),
            ("keyword_control", &self.keyword_control),
            ("tag", &self.tag),
            ("constructor", &self.constructor),
            ("variable_special", &self.variable_special),
            ("function", &self.function),
            ("string_special", &self.string_special),
            ("type", &self.ty),
            ("namespace", &self.namespace),
            ("string", &self.string),
            ("constant", &self.constant),
            ("variable", &self.variable),
            ("property", &self.property),
            ("attribute", &self.attribute),
            ("operator", &self.operator),
            ("punctuation", &self.punctuation),
            ("markup_heading", &self.markup_heading),
            ("markup_emphasis", &self.markup_emphasis),
            ("markup_strong", &self.markup_strong),
            ("markup_raw", &self.markup_raw),
            ("markup_link_text", &self.markup_link_text),
            ("markup_link_url", &self.markup_link_url),
        ]
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Ui {
    /// Focus rings, cursor, links, accented text and icons.
    pub accent: Color,
    pub surface: Surface,
    pub border: Border,
    pub text: Text,
    pub editor: Editor,
    pub scrollbar: Scrollbar,
    pub status: Status,
    pub vcs: Vcs,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Surface {
    /// Editor and primary surfaces.
    pub background: Color,
    /// Status bar, title bar.
    pub chrome: Color,
    /// Side panels, tab bar, inactive tabs.
    pub panel: Color,
    pub hover: Color,
    pub selected: Color,
    pub active_line: Color,
    /// Text selection (opaque; formats may add translucency).
    pub selection: Color,
    pub drop_target: Color,
    pub search_match: Color,
    pub debugger_line: Color,
    pub document_highlight: Color,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Border {
    pub default: Color,
    pub variant: Color,
    pub transparent: Color,
    /// Hairlines: scrollbar track border, active wrap guide.
    pub faint: Color,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Text {
    pub default: Color,
    pub muted: Color,
    pub placeholder: Color,
    pub disabled: Color,
    pub icon_muted: Color,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Editor {
    pub line_number: Color,
    pub line_number_active: Color,
    /// Whitespace markers, wrap guide.
    pub invisible: Color,
    pub indent_guide: Color,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Scrollbar {
    pub thumb: Color,
    pub thumb_hover: Color,
    pub thumb_active: Color,
    pub thumb_border: Color,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Status {
    pub error: Color,
    pub warning: Color,
    pub success: Color,
    pub info: Color,
    pub hint: Color,
    pub predictive: Color,
    /// Hidden or unreachable code.
    pub hidden: Color,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Vcs {
    pub added: Color,
    pub modified: Color,
    pub deleted: Color,
    pub renamed: Color,
    pub conflict: Color,
    pub ignored: Color,
    pub conflict_ours: Color,
    pub conflict_theirs: Color,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Terminal {
    pub background: Color,
    pub foreground: Color,
    pub bright_foreground: Color,
    pub dim_foreground: Color,
    pub normal: Ansi,
    pub bright: Ansi,
    /// Optional; falls back to `normal`.
    pub dim: Option<Ansi>,
}

impl Terminal {
    pub fn dim(&self) -> &Ansi {
        self.dim.as_ref().unwrap_or(&self.normal)
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Ansi {
    pub black: Color,
    pub red: Color,
    pub green: Color,
    pub yellow: Color,
    pub blue: Color,
    pub magenta: Color,
    pub cyan: Color,
    pub white: Color,
}

impl Ansi {
    pub const NAMES: [&'static str; 8] = [
        "black", "red", "green", "yellow", "blue", "magenta", "cyan", "white",
    ];

    /// In ANSI index order, from black (0) to white (7).
    pub fn colors(&self) -> [Color; 8] {
        [
            self.black,
            self.red,
            self.green,
            self.yellow,
            self.blue,
            self.magenta,
            self.cyan,
            self.white,
        ]
    }
}

// ---------------------------------------------------------------------------
// Loading
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Header {
    name: String,
    author: String,
    #[serde(default)]
    formats: BTreeMap<String, FormatConfig>,
}

impl Family {
    pub fn load(path: &Path) -> Result<Family> {
        let src =
            std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
        Family::parse(&src).with_context(|| format!("in {}", path.display()))
    }

    pub fn parse(src: &str) -> Result<Family> {
        let mut root: Table = src.parse()?;

        let palette = take_table(&mut root, "palette")?.unwrap_or_default();
        let syntax = take_table(&mut root, "syntax")?.context("missing [syntax] table")?;
        let variants = match root.remove("variants") {
            Some(Value::Array(vs)) if !vs.is_empty() => vs,
            Some(Value::Array(_)) | None => bail!("define at least one [[variants]]"),
            Some(_) => bail!("`variants` must be an array of tables ([[variants]])"),
        };
        let header: Header = deserialize(root)?;

        // The palette holds literals only, so it's the same in every variant.
        let mut palette_colors = BTreeMap::new();
        for (name, value) in &palette {
            let r = value
                .as_str()
                .with_context(|| format!("palette.{name}: expected a color string"))?;
            let color = resolve(r, &BTreeMap::new(), "[palette] entries must be hex")
                .with_context(|| format!("palette.{name}"))?;
            palette_colors.insert(name.clone(), color);
        }

        let variants = variants
            .into_iter()
            .enumerate()
            .map(|(i, v)| {
                let Value::Table(v) = v else {
                    bail!("variants[{i}] must be a table")
                };
                let label = v
                    .get("name")
                    .and_then(Value::as_str)
                    .map(str::to_owned)
                    .unwrap_or_else(|| format!("variants[{i}]"));
                load_variant(v, &palette_colors, &syntax)
                    .with_context(|| format!("variant {label:?}"))
            })
            .collect::<Result<Vec<_>>>()?;

        let mut seen = std::collections::HashSet::new();
        for v in &variants {
            if !seen.insert(&v.name) {
                bail!("duplicate variant name {:?}", v.name);
            }
        }

        Ok(Family {
            name: header.name,
            author: header.author,
            formats: header.formats,
            variants,
        })
    }
}

impl Family {
    /// Syntax roles with under `min` contrast against the editor background.
    ///
    /// WCAG-AA requires 4.5 for body text.
    pub fn contrast_warnings(&self, min: f64) -> Vec<String> {
        let mut out = Vec::new();
        for v in &self.variants {
            let bg = v.ui.surface.background;
            for (role, style) in v.syntax.roles() {
                let ratio = style.color.contrast(bg);
                if ratio < min {
                    out.push(format!(
                        "{:?}: syntax.{role} {} is {ratio:.2}:1 against {} (< {min}:1)",
                        v.name,
                        style.color.hex_rgb(),
                        bg.hex_rgb()
                    ));
                }
            }
        }
        out
    }
}

fn load_variant(
    mut v: Table,
    palette: &BTreeMap<String, Color>,
    syntax: &Table,
) -> Result<Variant> {
    // In scope: the shared palette and this variant's own named colors.
    let mut scope = palette.clone();
    for (name, value) in take_table(&mut v, "colors")?.unwrap_or_default() {
        let r = value
            .as_str()
            .with_context(|| format!("colors.{name}: expected a color string"))?;
        if scope.contains_key(&name) {
            bail!("colors.{name}: already defined in [palette]; pick a distinct name");
        }
        let hint = "[colors] entries can use hex or [palette] names, not each other";
        let color = resolve(r, palette, hint).with_context(|| format!("colors.{name}"))?;
        scope.insert(name, color);
    }

    // Syntax roles are shared; each variant resolves their color names itself.
    let mut syntax = Value::Table(syntax.clone());
    resolve_tree(&mut syntax, &scope, "syntax")?;
    v.insert("syntax".into(), syntax);
    for section in ["ui", "terminal"] {
        if let Some(tree) = v.get_mut(section) {
            resolve_tree(tree, &scope, section)?;
        }
    }
    deserialize(v)
}

/// Replace every string leaf under `value` with the hex of the color it names.
fn resolve_tree(value: &mut Value, scope: &BTreeMap<String, Color>, path: &str) -> Result<()> {
    match value {
        Value::String(s) => {
            *s = resolve(s, scope, "not in [palette] or this variant's [colors]")
                .with_context(|| path.to_owned())?
                .hex_rgba();
        }
        Value::Table(t) => {
            for (k, v) in t.iter_mut() {
                resolve_tree(v, scope, &format!("{path}.{k}"))?;
            }
        }
        Value::Array(a) => {
            for (i, v) in a.iter_mut().enumerate() {
                resolve_tree(v, scope, &format!("{path}[{i}]"))?;
            }
        }
        _ => {}
    }
    Ok(())
}

/// Resolve a color reference: `#hex` or a name, with optional `/AA` alpha.
///
/// If the lookup fails, `hint` tells the user which names are allowed here.
fn resolve(r: &str, scope: &BTreeMap<String, Color>, hint: &str) -> Result<Color> {
    let (base, alpha) = match r.rsplit_once('/') {
        Some((base, alpha)) => (base, Some(parse_alpha(alpha)?)),
        None => (r, None),
    };
    let color = if base.starts_with('#') {
        base.parse()?
    } else {
        *scope
            .get(base)
            .ok_or_else(|| anyhow!("unknown color {base:?} ({hint})"))?
    };
    Ok(alpha.map(|a| color.with_alpha(a)).unwrap_or(color))
}

fn take_table(t: &mut Table, key: &str) -> Result<Option<Table>> {
    match t.remove(key) {
        None => Ok(None),
        Some(Value::Table(t)) => Ok(Some(t)),
        Some(_) => bail!("`{key}` must be a table"),
    }
}

/// Deserialize, reporting errors as `path.to.field: message`.
fn deserialize<T: DeserializeOwned>(t: Table) -> Result<T> {
    T::deserialize(Value::Table(t)).map_err(|e| {
        // toml renders key context as a trailing "\nin `a.b`" line.
        let msg = e.to_string();
        match msg.trim_end().rsplit_once("\nin `") {
            Some((msg, path)) => anyhow!("{}: {msg}", path.trim_end_matches('`')),
            None => anyhow!("{}", msg.trim_end()),
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const MINIMAL: &str = r##"
        name = "T"
        author = "a"
        [palette]
        p = "#101010"
        [syntax]
        ROLES
        [[variants]]
        name = "V"
        appearance = "dark"
        [variants.colors]
        fg = "#EEEEEE"
        [variants.ui]
        accent = "fg"
        surface = { background = "p", chrome = "p", panel = "p", hover = "p", selected = "p", active_line = "p", selection = "p", drop_target = "p", search_match = "p", debugger_line = "p", document_highlight = "p" }
        border = { default = "p", variant = "p", transparent = "p/00", faint = "p" }
        text = { default = "fg", muted = "fg", placeholder = "fg", disabled = "fg", icon_muted = "fg" }
        editor = { line_number = "fg", line_number_active = "fg", invisible = "fg", indent_guide = "fg" }
        scrollbar = { thumb = "p", thumb_hover = "p", thumb_active = "p", thumb_border = "p" }
        status = { error = "fg", warning = "fg", success = "fg", info = "fg", hint = "fg", predictive = "fg", hidden = "fg" }
        vcs = { added = "fg", modified = "fg", deleted = "fg", renamed = "fg", conflict = "fg", ignored = "fg", conflict_ours = "fg", conflict_theirs = "fg" }
        [variants.terminal]
        background = "p"
        foreground = "fg"
        bright_foreground = "fg"
        dim_foreground = "fg"
        normal = { black = "p", red = "fg", green = "fg", yellow = "fg", blue = "fg", magenta = "fg", cyan = "fg", white = "fg" }
        bright = { black = "p", red = "fg", green = "fg", yellow = "fg", blue = "fg", magenta = "fg", cyan = "fg", white = "fg" }
    "##;

    fn spec(edit: impl FnOnce(String) -> String) -> String {
        let roles = [
            "comment",
            "keyword",
            "keyword_control",
            "tag",
            "constructor",
            "variable_special",
            "function",
            "string_special",
            "type",
            "namespace",
            "string",
            "constant",
            "variable",
            "property",
            "attribute",
            "operator",
            "punctuation",
            "markup_heading",
            "markup_emphasis",
            "markup_strong",
            "markup_raw",
            "markup_link_text",
            "markup_link_url",
        ]
        .map(|r| format!("{r} = {{ color = \"fg\" }}"))
        .join("\n");
        edit(MINIMAL.replace("ROLES", &roles))
    }

    fn err(src: &str) -> String {
        format!("{:#}", Family::parse(src).unwrap_err())
    }

    #[test]
    fn slugs_are_file_friendly() {
        assert_eq!(slug("Norde"), "norde");
        assert_eq!(slug("Norde Light"), "norde_light");
        assert_eq!(slug(" Nord  Deep (v2) "), "nord_deep_v2");
    }

    #[test]
    fn resolves_references() {
        let scope = BTreeMap::from([("nord1".to_owned(), "#3B4252".parse().unwrap())]);
        assert_eq!(
            resolve("nord1", &scope, "").unwrap().hex_rgba(),
            "#3B4252FF"
        );
        assert_eq!(
            resolve("nord1/00", &scope, "").unwrap().hex_rgba(),
            "#3B425200"
        );
        assert_eq!(
            resolve("#CBD2DE/80", &scope, "").unwrap().hex_rgba(),
            "#CBD2DE80"
        );
        assert!(
            resolve("nord99", &scope, "")
                .unwrap_err()
                .to_string()
                .contains("unknown color")
        );
        assert!(resolve("nord1/8", &scope, "").is_err());
    }

    #[test]
    fn minimal_spec_loads() {
        let f = Family::parse(&spec(|s| s)).unwrap();
        let v = &f.variants[0];
        assert_eq!(v.syntax.keyword.color.hex_rgb(), "#EEEEEE");
        assert_eq!(v.ui.border.transparent.hex_rgba(), "#10101000");
        // `dim` is omitted, so it falls back to `normal`.
        assert_eq!(v.terminal.dim().red, v.terminal.normal.red);
    }

    #[test]
    fn unknown_color_names_its_location() {
        let e = err(&spec(|s| {
            s.replace(r#"accent = "fg""#, r#"accent = "nope""#)
        }));
        assert!(
            e.contains("ui.accent") && e.contains("unknown color \"nope\""),
            "{e}"
        );
    }

    #[test]
    fn missing_slot_names_its_location() {
        let e = err(&spec(|s| s.replace(r#"chrome = "p", "#, "")));
        assert!(e.contains("ui.surface") && e.contains("chrome"), "{e}");
    }

    #[test]
    fn typo_in_slot_is_rejected() {
        let e = err(&spec(|s| s.replace("hover = ", "hovr = ")));
        assert!(e.contains("hovr"), "{e}");
    }

    #[test]
    fn variant_colors_cannot_shadow_palette() {
        let e = err(&spec(|s| {
            s.replace(r##"fg = "#EEEEEE""##, "fg = \"#EEEEEE\"\np = \"#000000\"")
        }));
        assert!(
            e.contains("colors.p") && e.contains("already defined"),
            "{e}"
        );
    }

    #[test]
    fn variant_colors_cannot_refer_to_each_other() {
        let e = err(&spec(|s| {
            s.replace(r##"fg = "#EEEEEE""##, "fg = \"#EEEEEE\"\nalias = \"fg\"")
        }));
        assert!(
            e.contains("colors.alias") && e.contains("not each other"),
            "{e}"
        );
    }

    #[test]
    fn low_contrast_is_reported() {
        let f = Family::parse(&spec(|s| {
            s.replace(
                r#"comment = { color = "fg" }"#,
                r#"comment = { color = "p" }"#,
            )
        }))
        .unwrap();
        let w = f.contrast_warnings(4.5);
        assert_eq!(w.len(), 1);
        assert!(w[0].contains("syntax.comment"), "{w:?}");
    }
}
