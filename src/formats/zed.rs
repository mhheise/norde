//! Zed theme family JSON
//!
//! See: <https://zed.dev/docs/extensions/themes>

use std::collections::BTreeMap;

use anyhow::Result;
use serde_json::{Map, Value, json};

use super::Renderer;
use crate::color::Color;
use crate::theme::{Family, Style, Variant};

const SCHEMA: &str = "https://zed.dev/schema/themes/v0.2.0.json";

pub struct Zed;

impl Renderer for Zed {
    fn name(&self) -> &'static str {
        "zed"
    }

    fn render(&self, family: &Family, variants: &[Variant]) -> Result<String> {
        let themes: Vec<Value> = variants
            .iter()
            .map(|v| json!({ "name": v.name, "appearance": v.appearance.as_str(), "style": style(v) }))
            .collect();
        let doc = json!({
            "$schema": SCHEMA,
            "name": family.name,
            "author": family.author,
            "themes": themes,
        });
        let mut out = serde_json::to_string_pretty(&doc)?;
        out.push('\n');
        Ok(out)
    }
}

fn style(v: &Variant) -> Value {
    let ui = &v.ui;
    let (s, b, t, e, st, vc, term) = (
        &ui.surface,
        &ui.border,
        &ui.text,
        &ui.editor,
        &ui.status,
        &ui.vcs,
        &v.terminal,
    );
    let bg = s.background;

    let mut out = Map::new();
    let mut put = |key: &str, c: Color| {
        out.insert(key.to_owned(), Value::String(c.hex_rgba()));
    };

    put("border", b.default);
    put("border.variant", b.variant);
    put("border.focused", ui.accent);
    put("border.selected", ui.accent);
    put("border.transparent", b.transparent);
    put("border.disabled", b.variant);
    put("elevated_surface.background", bg);
    put("surface.background", bg);
    put("background", bg);
    put("element.background", bg);
    put("element.hover", s.hover);
    put("element.active", s.hover);
    put("element.selected", s.selected);
    put("element.disabled", t.disabled.with_alpha(0x00));
    put("element.selection_background", s.selection.with_alpha(0x80));
    put("drop_target.background", s.drop_target.with_alpha(0x80));
    put("drop_target.border", s.drop_target);
    put("ghost_element.background", Color::TRANSPARENT);
    put("ghost_element.hover", s.hover);
    put("ghost_element.active", s.hover);
    put("ghost_element.selected", s.selected);
    put("ghost_element.disabled", t.disabled.with_alpha(0x00));
    put("text", t.default);
    put("text.muted", t.muted);
    put("text.placeholder", t.placeholder);
    put("text.disabled", t.disabled);
    put("text.accent", ui.accent);
    put("icon", t.muted);
    put("icon.muted", t.icon_muted);
    put("icon.disabled", t.disabled);
    put("icon.accent", ui.accent);
    put("status_bar.background", s.chrome);
    put("title_bar.background", s.chrome);
    put("title_bar.inactive_background", s.chrome);
    put("toolbar.background", bg);
    put("tab_bar.background", s.panel);
    put("tab.inactive_background", s.panel);
    put("tab.active_background", bg);
    put("search.match_background", s.search_match);
    put("panel.background", s.panel);
    put("panel.focused_border", ui.accent);
    put("panel.indent_guide", e.indent_guide);
    put("panel.indent_guide_active", e.line_number);
    put("panel.indent_guide_hover", e.line_number);
    put("panel.overlay_background", bg);
    put("panel.overlay_hover", bg);
    put("pane.focused_border", ui.accent);
    for prefix in ["scrollbar", "minimap"] {
        let sb = &ui.scrollbar;
        put(&format!("{prefix}.thumb.background"), sb.thumb);
        put(&format!("{prefix}.thumb.hover_background"), sb.thumb_hover);
        put(
            &format!("{prefix}.thumb.active_background"),
            sb.thumb_active,
        );
        put(&format!("{prefix}.thumb.border"), sb.thumb_border);
        if prefix == "scrollbar" {
            put("scrollbar.track.background", s.chrome);
            put("scrollbar.track.border", b.faint);
        }
    }
    put("editor.foreground", t.default);
    put("editor.background", bg);
    put("editor.gutter.background", bg);
    put("editor.subheader.background", bg);
    put("editor.active_line.background", s.active_line);
    put("editor.highlighted_line.background", s.active_line);
    put("editor.debugger_active_line.background", s.debugger_line);
    put("editor.line_number", e.line_number);
    put("editor.active_line_number", e.line_number_active);
    put("editor.hover_line_number", e.line_number_active);
    put("editor.invisible", e.invisible);
    put("editor.wrap_guide", e.invisible);
    put("editor.active_wrap_guide", b.faint);
    put("editor.indent_guide", e.indent_guide);
    put("editor.indent_guide_active", e.line_number);
    put(
        "editor.document_highlight.bracket_background",
        s.document_highlight,
    );
    put(
        "editor.document_highlight.read_background",
        s.document_highlight,
    );
    put(
        "editor.document_highlight.write_background",
        s.document_highlight,
    );
    put("terminal.background", term.background);
    put("terminal.foreground", term.foreground);
    put("terminal.bright_foreground", term.bright_foreground);
    put("terminal.dim_foreground", term.dim_foreground);
    let (normal, bright, dim) = (
        term.normal.colors(),
        term.bright.colors(),
        term.dim().colors(),
    );
    for (i, name) in crate::theme::Ansi::NAMES.iter().enumerate() {
        put(&format!("terminal.ansi.{name}"), normal[i]);
        put(&format!("terminal.ansi.bright_{name}"), bright[i]);
        put(&format!("terminal.ansi.dim_{name}"), dim[i]);
    }
    put("link_text.hover", ui.accent);
    // Each status color gets a foreground, a plain background, and a border.
    for (key, c) in [
        ("conflict", vc.conflict),
        ("created", vc.added),
        ("deleted", vc.deleted),
        ("error", st.error),
        ("hidden", st.hidden),
        ("hint", st.hint),
        ("ignored", vc.ignored),
        ("info", st.info),
        ("modified", vc.modified),
        ("predictive", st.predictive),
        ("renamed", vc.renamed),
        ("success", st.success),
        ("unreachable", st.hidden),
        ("warning", st.warning),
    ] {
        put(key, c);
        put(&format!("{key}.background"), bg);
        put(&format!("{key}.border"), c);
    }
    put("version_control.added", vc.added);
    put("version_control.modified", vc.modified);
    put("version_control.deleted", vc.deleted);
    put("version_control.renamed", vc.renamed);
    put("version_control.conflict", vc.conflict);
    put("version_control.ignored", vc.ignored);
    put("version_control.conflict_marker.theirs", vc.conflict_theirs);
    put("version_control.conflict_marker.ours", vc.conflict_ours);

    out.insert(
        "players".into(),
        json!([{
            "cursor": ui.accent.hex_rgba(),
            "background": ui.accent.hex_rgba(),
            "selection": s.selection.hex_rgba(),
        }]),
    );
    out.insert("syntax".into(), syntax(v));
    Value::Object(out)
}

/// Map tree-sitter capture names to neutral roles.
fn syntax(v: &Variant) -> Value {
    let x = &v.syntax;
    let captures: &[(&str, &Style)] = &[
        ("attribute", &x.attribute),
        ("boolean", &x.constant),
        ("comment", &x.comment),
        ("comment.doc", &x.comment),
        ("constant", &x.constant),
        ("constructor", &x.constructor),
        ("embedded", &x.variable),
        ("emphasis", &x.markup_emphasis),
        ("emphasis.strong", &x.markup_strong),
        ("enum", &x.ty),
        ("function", &x.function),
        ("hint", &x.keyword),
        ("keyword", &x.keyword),
        // Control-flow captures go bold; grammars that only emit plain
        // `keyword` degrade gracefully to regular weight. Both the `.control.`
        // and bare naming conventions are covered.
        ("keyword.control", &x.keyword_control),
        ("keyword.conditional", &x.keyword_control),
        ("keyword.repeat", &x.keyword_control),
        ("keyword.return", &x.keyword_control),
        ("keyword.exception", &x.keyword_control),
        ("keyword.coroutine", &x.keyword_control),
        ("keyword.control.import", &x.keyword), // keep use/import regular
        ("label", &x.variable),
        ("link_text", &x.markup_link_text),
        ("link_uri", &x.markup_link_url),
        ("namespace", &x.namespace),
        ("number", &x.constant),
        ("operator", &x.operator),
        ("predictive", &x.variable),
        ("preproc", &x.keyword),
        ("primary", &x.keyword),
        ("property", &x.property),
        ("punctuation", &x.punctuation),
        ("punctuation.bracket", &x.punctuation),
        ("punctuation.delimiter", &x.punctuation),
        ("punctuation.list_marker", &x.punctuation),
        ("punctuation.markup", &x.punctuation),
        ("punctuation.special", &x.punctuation),
        ("selector", &x.property),
        ("selector.pseudo", &x.property),
        ("string", &x.string),
        ("string.escape", &x.string_special),
        ("string.regex", &x.string_special),
        ("string.special", &x.string_special),
        ("string.special.symbol", &x.string_special),
        ("tag", &x.tag),
        ("text.literal", &x.markup_raw),
        ("title", &x.markup_heading),
        ("type", &x.ty),
        ("variable", &x.variable),
        ("variable.special", &x.variable_special),
        ("variant", &x.constructor),
    ];
    let sorted: BTreeMap<&str, Value> = captures
        .iter()
        .map(|&(name, st)| {
            (
                name,
                json!({
                    "color": st.color.hex_rgba(),
                    "font_style": st.italic.then_some("italic"),
                    "font_weight": st.bold.then_some(700),
                }),
            )
        })
        .collect();
    Value::Object(sorted.into_iter().map(|(k, v)| (k.to_owned(), v)).collect())
}
