//! TextMate-style scope rules and global settings, shared by the formats that
//! use TextMate scopes (Sublime Text's `.sublime-color-scheme` and `.tmTheme`).
//!
//! Scopes follow Sublime's scope naming guide, which advises against styling
//! `meta.*` scopes: they cover whole regions, such as a function call's
//! arguments.

use crate::color::Color;
use crate::theme::{Style, Variant};

/// One scope rule. A missing `foreground` or `font_style` leaves that property
/// to less specific rules.
pub struct Rule {
    pub name: &'static str,
    pub scope: &'static str,
    pub foreground: Option<Color>,
    pub font_style: Option<String>,
}

/// One global setting, named in both formats' conventions.
pub struct Global {
    /// `.sublime-color-scheme` key (snake case).
    pub sublime: &'static str,
    /// `.tmTheme` key (camel case), if the format has one.
    pub tmtheme: Option<&'static str>,
    pub value: GlobalValue,
}

pub enum GlobalValue {
    Color(Color),
    Text(&'static str),
}

impl GlobalValue {
    /// `#RRGGBB`, or `#RRGGBBAA` for translucent colors.
    pub fn to_text(&self) -> String {
        match self {
            GlobalValue::Color(c) if c.a == 0xFF => c.hex_rgb(),
            GlobalValue::Color(c) => c.hex_rgba(),
            GlobalValue::Text(t) => (*t).to_owned(),
        }
    }
}

/// What a rule sets: a role's color and font style, only its font style (so
/// the color comes from the surrounding scope), or only a color.
enum Paint<'a> {
    Role(&'a Style),
    FontOnly(&'a Style),
    Color(Color),
}

pub fn globals(v: &Variant) -> Vec<Global> {
    let (ui, s, e, t, vcs) = (&v.ui, &v.ui.surface, &v.ui.editor, &v.ui.text, &v.ui.vcs);
    let c = GlobalValue::Color;
    let text = GlobalValue::Text;
    [
        ("background", Some("background"), c(s.background)),
        ("foreground", Some("foreground"), c(t.default)),
        ("caret", Some("caret"), c(ui.accent)),
        ("line_highlight", Some("lineHighlight"), c(s.active_line)),
        ("selection", Some("selection"), c(s.selection)),
        ("selection_corner_style", None, text("round")),
        (
            "gutter_foreground",
            Some("gutterForeground"),
            c(e.line_number),
        ),
        ("gutter_foreground_highlight", None, c(e.line_number_active)),
        ("find_highlight", Some("findHighlight"), c(s.search_match)),
        (
            "find_highlight_foreground",
            Some("findHighlightForeground"),
            c(t.default),
        ),
        ("guide", Some("guide"), c(e.indent_guide)),
        ("active_guide", Some("activeGuide"), c(e.line_number)),
        ("stack_guide", Some("stackGuide"), c(e.indent_guide)),
        ("invisibles", Some("invisibles"), c(e.invisible)),
        ("accent", Some("accent"), c(ui.accent)),
        ("brackets_foreground", None, c(ui.accent)),
        ("brackets_options", None, text("underline")),
        ("bracket_contents_foreground", None, c(ui.accent)),
        ("bracket_contents_options", None, text("underline")),
        ("tags_options", None, text("stippled_underline")),
        ("misspelling", Some("misspelling"), c(ui.status.error)),
        ("shadow", None, c(Color::TRANSPARENT.with_alpha(0x33))),
        ("line_diff_added", None, c(vcs.added)),
        ("line_diff_modified", None, c(vcs.modified)),
        ("line_diff_deleted", None, c(vcs.deleted)),
    ]
    .into_iter()
    .map(|(sublime, tmtheme, value)| Global {
        sublime,
        tmtheme,
        value,
    })
    .collect()
}

/// TextMate scopes -> neutral roles and UI slots.
pub fn rules(v: &Variant) -> Vec<Rule> {
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
            "Macro",
            "support.macro, entity.name.macro",
            Paint::Role(&x.r#macro),
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
            let (foreground, font_style) = match paint {
                // Always set the font style, so a regular rule overrides a
                // bolder, less specific one (e.g. imports under
                // `keyword.control`).
                Paint::Role(style) => (Some(style.color), Some(font_style(style))),
                Paint::FontOnly(style) => (None, Some(font_style(style))),
                Paint::Color(c) => (Some(*c), None),
            };
            Rule {
                name,
                scope,
                foreground,
                font_style,
            }
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
