//! Sublime Text color scheme
//!
//! See: <https://www.sublimetext.com/docs/color_schemes.html>
//!
//! One scheme per file. Scopes follow Sublime's scope naming guide, which
//! advises against styling `meta.*` scopes: they cover whole regions, such as
//! a function call's arguments.

use anyhow::{Result, bail};
use serde::Serialize;
use serde_json::{Map, Value, json};

use super::Renderer;
use crate::color::Color;
use crate::theme::{Family, Style, Variant};

pub struct Sublime;

/// What a rule sets: a role's color and font style, only its font style (so
/// the color comes from the surrounding scope), or only a color.
enum Paint<'a> {
    Role(&'a Style),
    FontOnly(&'a Style),
    Color(Color),
}

impl Renderer for Sublime {
    fn name(&self) -> &'static str {
        "sublime"
    }

    fn render(&self, family: &Family, variants: &[Variant]) -> Result<String> {
        let [v] = variants else {
            bail!("a Sublime color scheme holds one variant; put `{{name}}` in its output path");
        };
        let doc = json!({
            "name": v.name,
            "author": family.author,
            "globals": globals(v),
            "rules": rules(v),
        });
        let mut out = Vec::new();
        let fmt = serde_json::ser::PrettyFormatter::with_indent(b"    ");
        doc.serialize(&mut serde_json::Serializer::with_formatter(&mut out, fmt))?;
        out.push(b'\n');
        Ok(String::from_utf8(out)?)
    }
}

fn globals(v: &Variant) -> Value {
    let (ui, s, e, t, vcs) = (&v.ui, &v.ui.surface, &v.ui.editor, &v.ui.text, &v.ui.vcs);
    let mut g = Map::new();
    let mut put = |key: &str, value: Value| {
        g.insert(key.to_owned(), value);
    };
    let hex = |c: Color| Value::String(c.hex_rgb());
    put("background", hex(s.background));
    put("foreground", hex(t.default));
    put("caret", hex(ui.accent));
    put("line_highlight", hex(s.active_line));
    put("selection", hex(s.selection));
    put("selection_corner_style", json!("round"));
    put("gutter_foreground", hex(e.line_number));
    put("gutter_foreground_highlight", hex(e.line_number_active));
    put("find_highlight", hex(s.search_match));
    put("find_highlight_foreground", hex(t.default));
    put("guide", hex(e.indent_guide));
    put("active_guide", hex(e.line_number));
    put("stack_guide", hex(e.indent_guide));
    put("invisibles", hex(e.invisible));
    put("accent", hex(ui.accent));
    put("brackets_foreground", hex(ui.accent));
    put("brackets_options", json!("underline"));
    put("bracket_contents_foreground", hex(ui.accent));
    put("bracket_contents_options", json!("underline"));
    put("tags_options", json!("stippled_underline"));
    put("misspelling", hex(ui.status.error));
    put(
        "shadow",
        json!(Color::TRANSPARENT.with_alpha(0x33).hex_rgba()),
    );
    put("line_diff_added", hex(vcs.added));
    put("line_diff_modified", hex(vcs.modified));
    put("line_diff_deleted", hex(vcs.deleted));
    Value::Object(g)
}

/// Sublime scopes -> neutral roles and UI slots.
fn rules(v: &Variant) -> Vec<Value> {
    let x = &v.syntax;
    let (st, vcs) = (&v.ui.status, &v.ui.vcs);
    let table: &[(&str, &str, Paint<'_>)] = &[
        (
            "Comment",
            "comment, punctuation.definition.comment",
            Paint::Role(&x.comment),
        ),
        (
            "Keyword",
            "keyword, storage, storage.type, storage.modifier",
            Paint::Role(&x.keyword),
        ),
        (
            "Control flow",
            "keyword.control, keyword.control.conditional, keyword.control.loop, \
             keyword.control.flow, keyword.control.exception",
            Paint::Role(&x.keyword_control),
        ),
        (
            "Import and export",
            "keyword.control.import, keyword.control.export",
            Paint::Role(&x.keyword),
        ),
        ("Operator", "keyword.operator", Paint::Role(&x.operator)),
        ("Punctuation", "punctuation", Paint::Role(&x.punctuation)),
        (
            "Function",
            "entity.name.function, support.function, variable.function",
            Paint::Role(&x.function),
        ),
        (
            "Type",
            "entity.name.type, entity.name.class, entity.name.struct, entity.name.enum, \
             entity.name.trait, entity.name.interface, entity.other.inherited-class, \
             support.type, support.class",
            Paint::Role(&x.ty),
        ),
        (
            "Namespace",
            "entity.name.namespace, entity.name.module",
            Paint::Role(&x.namespace),
        ),
        (
            "String",
            "string, punctuation.definition.string",
            Paint::Role(&x.string),
        ),
        (
            "Escape, regex, and placeholder",
            "constant.character.escape, string.regexp, constant.other.placeholder",
            Paint::Role(&x.string_special),
        ),
        (
            "Constant",
            "constant, constant.numeric, constant.language, support.constant",
            Paint::Role(&x.constant),
        ),
        (
            "Variable",
            "variable, variable.parameter, variable.other",
            Paint::Role(&x.variable),
        ),
        (
            "Language variable",
            "variable.language",
            Paint::Role(&x.variable_special),
        ),
        ("Tag", "entity.name.tag", Paint::Role(&x.tag)),
        (
            "Attribute",
            "entity.other.attribute-name",
            Paint::Role(&x.attribute),
        ),
        (
            "Annotation",
            "variable.annotation, punctuation.definition.annotation, \
             entity.name.function.decorator",
            Paint::Role(&x.keyword),
        ),
        (
            "Heading",
            "markup.heading, markup.heading entity.name",
            Paint::Role(&x.markup_heading),
        ),
        ("Bold", "markup.bold", Paint::FontOnly(&x.markup_strong)),
        (
            "Italic",
            "markup.italic",
            Paint::FontOnly(&x.markup_emphasis),
        ),
        (
            "Link",
            "markup.underline.link, markup.link",
            Paint::Role(&x.markup_link_url),
        ),
        ("Raw", "markup.raw", Paint::Role(&x.markup_raw)),
        ("Quote", "markup.quote", Paint::Role(&x.comment)),
        (
            "Inserted",
            "markup.inserted, diff.inserted",
            Paint::Color(vcs.added),
        ),
        (
            "Deleted",
            "markup.deleted, diff.deleted",
            Paint::Color(vcs.deleted),
        ),
        (
            "Changed",
            "markup.changed, diff.changed",
            Paint::Color(vcs.modified),
        ),
        (
            "Invalid",
            "invalid, invalid.illegal",
            Paint::Color(st.error),
        ),
        ("Deprecated", "invalid.deprecated", Paint::Color(st.warning)),
    ];
    table
        .iter()
        .map(|(name, scope, paint)| {
            let mut rule = Map::new();
            rule.insert("name".into(), json!(name));
            rule.insert("scope".into(), json!(scope));
            match paint {
                Paint::Role(style) => {
                    rule.insert("foreground".into(), json!(style.color.hex_rgb()));
                    // Always set, so a regular rule overrides a bolder, less
                    // specific one (e.g. imports under `keyword.control`).
                    rule.insert("font_style".into(), json!(font_style(style)));
                }
                Paint::FontOnly(style) => {
                    rule.insert("font_style".into(), json!(font_style(style)));
                }
                Paint::Color(c) => {
                    rule.insert("foreground".into(), json!(c.hex_rgb()));
                }
            }
            Value::Object(rule)
        })
        .collect()
}

fn font_style(s: &Style) -> String {
    [(s.bold, "bold"), (s.italic, "italic")]
        .iter()
        .filter(|(on, _)| *on)
        .map(|(_, name)| *name)
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    fn schemes() -> Vec<Value> {
        let spec = Path::new(env!("CARGO_MANIFEST_DIR")).join("norde.toml");
        let family = Family::load(&spec).unwrap();
        family
            .variants
            .iter()
            .map(|v| {
                let out = Sublime.render(&family, std::slice::from_ref(v)).unwrap();
                serde_json::from_str(&out).unwrap()
            })
            .collect()
    }

    fn rule<'a>(scheme: &'a Value, name: &str) -> &'a Value {
        scheme["rules"]
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["name"] == name)
            .unwrap()
    }

    #[test]
    fn control_flow_is_bold_but_imports_are_not() {
        for s in schemes() {
            assert_eq!(rule(&s, "Control flow")["font_style"], "bold");
            assert_eq!(rule(&s, "Import and export")["font_style"], "");
        }
    }

    /// Sublime's scope naming guide says color schemes shouldn't style these.
    #[test]
    fn no_meta_scopes() {
        for s in schemes() {
            for r in s["rules"].as_array().unwrap() {
                let scope = r["scope"].as_str().unwrap();
                assert!(!scope.contains("meta."), "{}: {scope}", r["name"]);
            }
        }
    }
}
