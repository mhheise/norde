//! Sublime Text color scheme
//!
//! See: <https://www.sublimetext.com/docs/color_schemes.html>
//!
//! One scheme per file. The scope rules and global settings are shared with
//! the `.tmTheme` format; see [`super::textmate`].

use anyhow::{Result, bail};
use serde::Serialize;
use serde_json::{Map, Value, json};

use super::Renderer;
use super::textmate;
use crate::theme::{Family, Variant};

pub struct Sublime;

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
    let g: Map<String, Value> = textmate::globals(v)
        .into_iter()
        .map(|g| (g.sublime.to_owned(), json!(g.value.to_text())))
        .collect();
    Value::Object(g)
}

fn rules(v: &Variant) -> Vec<Value> {
    textmate::rules(v)
        .into_iter()
        .map(|r| {
            let mut rule = Map::new();
            rule.insert("name".into(), json!(r.name));
            rule.insert("scope".into(), json!(r.scope));
            if let Some(c) = r.foreground {
                rule.insert("foreground".into(), json!(c.hex_rgb()));
            }
            if let Some(f) = r.font_style {
                rule.insert("font_style".into(), json!(f));
            }
            Value::Object(rule)
        })
        .collect()
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

    #[test]
    fn macros_use_the_macro_role() {
        let spec = Path::new(env!("CARGO_MANIFEST_DIR")).join("norde.toml");
        let family = Family::load(&spec).unwrap();
        for (s, v) in schemes().iter().zip(&family.variants) {
            let r = rule(s, "Macro");
            assert_eq!(r["scope"], "support.macro, entity.name.macro");
            assert_eq!(r["foreground"], v.syntax.r#macro.color.hex_rgb());
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
