//! TextMate `.tmTheme` color scheme, as used by bat and delta
//!
//! See: <https://www.sublimetext.com/docs/color_schemes_tmtheme.html>
//!
//! One theme per file; bat names the theme after the file, so configure the
//! path with `{name}`. The scope rules and global settings are shared with the
//! Sublime Text format; see [`super::textmate`].

use std::fmt::Write;

use anyhow::{Result, bail};

use super::Renderer;
use super::textmate;
use crate::theme::{Family, Variant};

pub struct TmTheme;

impl Renderer for TmTheme {
    fn name(&self) -> &'static str {
        "tmtheme"
    }

    fn render(&self, family: &Family, variants: &[Variant]) -> Result<String> {
        let [v] = variants else {
            bail!("a .tmTheme holds one variant; put `{{name}}` in its output path");
        };
        let mut out = String::from(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
             <!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \
             \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">\n\
             <plist version=\"1.0\">\n\
             <dict>\n",
        );
        entry(&mut out, 1, "name", &v.name)?;
        entry(&mut out, 1, "author", &family.author)?;
        writeln!(out, "    <key>settings</key>\n    <array>")?;

        // The first, scope-less entry holds the global settings.
        writeln!(
            out,
            "        <dict>\n            <key>settings</key>\n            <dict>"
        )?;
        for g in textmate::globals(v) {
            if let Some(key) = g.tmtheme {
                entry(&mut out, 4, key, &g.value.to_text())?;
            }
        }
        writeln!(out, "            </dict>\n        </dict>")?;

        for r in textmate::rules(v) {
            writeln!(out, "        <dict>")?;
            entry(&mut out, 3, "name", r.name)?;
            entry(&mut out, 3, "scope", r.scope)?;
            writeln!(out, "            <key>settings</key>\n            <dict>")?;
            if let Some(c) = r.foreground {
                entry(&mut out, 4, "foreground", &c.hex_rgb())?;
            }
            if let Some(f) = &r.font_style {
                entry(&mut out, 4, "fontStyle", f)?;
            }
            writeln!(out, "            </dict>\n        </dict>")?;
        }
        writeln!(out, "    </array>\n</dict>\n</plist>")?;
        Ok(out)
    }
}

/// A `<key>`/`<string>` pair, indented `depth` levels of four spaces.
fn entry(out: &mut String, depth: usize, key: &str, value: &str) -> std::fmt::Result {
    let pad = "    ".repeat(depth);
    writeln!(out, "{pad}<key>{}</key>", escape(key))?;
    writeln!(out, "{pad}<string>{}</string>", escape(value))
}

fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    fn themes() -> Vec<String> {
        let spec = Path::new(env!("CARGO_MANIFEST_DIR")).join("norde.toml");
        let family = Family::load(&spec).unwrap();
        family
            .variants
            .iter()
            .map(|v| TmTheme.render(&family, std::slice::from_ref(v)).unwrap())
            .collect()
    }

    /// The author contains `<` and `>`, which must be escaped in XML.
    #[test]
    fn markup_characters_are_escaped() {
        for t in themes() {
            assert!(t.contains("&lt;mhheise@pm.me&gt;"), "{t}");
            assert!(!t.contains("<mhheise@pm.me>"));
        }
    }

    /// Every rule from the shared table appears, with the same scope.
    #[test]
    fn has_every_shared_rule() {
        let spec = Path::new(env!("CARGO_MANIFEST_DIR")).join("norde.toml");
        let family = Family::load(&spec).unwrap();
        for (t, v) in themes().iter().zip(&family.variants) {
            for r in textmate::rules(v) {
                let scope = format!("<string>{}</string>", escape(r.scope));
                assert!(t.contains(&scope), "{}: missing {}", v.name, r.name);
            }
        }
    }
}
