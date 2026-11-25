//! Static HTML preview: a tokenized code sample rendered in each variant, so
//! color and bold/italic choices can be eyeballed without loading an editor.

use std::fmt::Write;

use anyhow::Result;

use super::Renderer;
use crate::theme::{Family, Style, Syntax, Variant};

pub struct Preview;

/// Which role colors each demo token.
#[derive(Clone, Copy)]
enum Tok {
    Kw,
    Kwc,
    Fn,
    Ty,
    Str,
    Num,
    Cmt,
    Var,
    Op,
    Ns,
    Punct,
}

impl Tok {
    fn style(self, s: &Syntax) -> &Style {
        match self {
            Kw => &s.keyword,
            Kwc => &s.keyword_control,
            Fn => &s.function,
            Ty => &s.ty,
            Str => &s.string,
            Num => &s.constant,
            Cmt => &s.comment,
            Var => &s.variable,
            Op => &s.operator,
            Ns => &s.namespace,
            Punct => &s.punctuation,
        }
    }
}

use Tok::{Cmt, Fn, Kw, Kwc, Ns, Num, Op, Punct, Str, Ty, Var};

/// Pre-tokenized sample; `None` is the default foreground.
const LINES: &[&[(Option<Tok>, &str)]] = &[
    &[(Some(Cmt), "// Load config, retrying on failure.")],
    &[
        (Some(Kw), "use"),
        (None, " "),
        (Some(Ns), "std"),
        (Some(Op), "::"),
        (Some(Ns), "fs"),
        (Some(Punct), ";"),
    ],
    &[],
    &[
        (Some(Kw), "const"),
        (None, " "),
        (Some(Num), "MAX_RETRIES"),
        (Some(Op), ": "),
        (Some(Ty), "u32"),
        (None, " "),
        (Some(Op), "= "),
        (Some(Num), "5"),
        (Some(Punct), ";"),
    ],
    &[],
    &[
        (Some(Kw), "pub fn"),
        (None, " "),
        (Some(Fn), "load_config"),
        (Some(Punct), "("),
        (Some(Var), "path"),
        (Some(Op), ": &"),
        (Some(Ty), "str"),
        (Some(Punct), ")"),
        (None, " "),
        (Some(Op), "-> "),
        (Some(Ty), "Config"),
        (None, " "),
        (Some(Punct), "{"),
    ],
    &[
        (None, "    "),
        (Some(Kw), "let mut"),
        (None, " "),
        (Some(Var), "file"),
        (None, " "),
        (Some(Op), "= "),
        (Some(Ns), "fs"),
        (Some(Op), "::"),
        (Some(Fn), "read_to_string"),
        (Some(Punct), "("),
        (Some(Var), "path"),
        (Some(Punct), ");"),
    ],
    &[
        (None, "    "),
        (Some(Kw), "let"),
        (None, " "),
        (Some(Var), "ready"),
        (None, " "),
        (Some(Op), "= "),
        (Some(Num), "true"),
        (Some(Punct), ";"),
    ],
    &[
        (None, "    "),
        (Some(Kwc), "if"),
        (None, " "),
        (Some(Var), "ready"),
        (None, " "),
        (Some(Punct), "{"),
    ],
    &[
        (None, "        "),
        (Some(Kwc), "return"),
        (None, " "),
        (Some(Ty), "Config"),
        (Some(Op), "::"),
        (Some(Fn), "default"),
        (Some(Punct), "();"),
    ],
    &[(None, "    "), (Some(Punct), "}")],
    &[
        (None, "    "),
        (Some(Fn), "println!"),
        (Some(Punct), "("),
        (Some(Str), "\"loaded {}\""),
        (Some(Punct), ", "),
        (Some(Var), "path"),
        (Some(Punct), ");"),
    ],
    &[(Some(Punct), "}")],
];

/// 1-based line drawn with the active-line background.
const ACTIVE_LINE: usize = 7;

impl Renderer for Preview {
    fn name(&self) -> &'static str {
        "preview"
    }

    fn render(&self, family: &Family, variants: &[Variant]) -> Result<String> {
        let title = escape(&family.name);
        let panes: String = variants.iter().map(pane).collect();
        let columns = vec!["1fr"; variants.len()].join(" ");
        Ok(format!(
            r#"<!doctype html><html><head><meta charset="utf-8"><title>{title} preview</title>
<style>
  body {{ margin:0; background:#0f1115; font-family:-apple-system,system-ui,sans-serif; }}
  .wrap {{ display:grid; grid-template-columns:{columns}; gap:1px; }}
  .pane {{ padding:20px 22px 26px; }}
  .cap {{ font:600 12px/1 -apple-system,system-ui; letter-spacing:.04em; text-transform:uppercase; margin-bottom:14px; opacity:.8; }}
  .ln {{ font-family:"SF Mono",ui-monospace,Menlo,monospace; font-size:13.5px; line-height:1.65; white-space:pre; padding:0 6px; border-radius:3px; }}
  .g {{ display:inline-block; width:22px; margin-right:16px; text-align:right; user-select:none; opacity:.7; }}
  .note {{ color:#9aa3b2; font:13px/1.5 -apple-system,system-ui; padding:14px 22px; }}
</style></head>
<body>
  <div class="note"><b>Font styles:</b> only <b>control-flow</b> keywords bold (<b>if</b>, <b>return</b>); plain keywords (<code>use</code>, <code>const</code>, <code>let</code>, <code>fn</code>) stay regular; comments <i>italic</i>. Mutable-underline is a settings recipe (semantic tokens), not shown here.</div>
  <div class="wrap">{panes}</div>
</body></html>
"#
        ))
    }
}

fn pane(v: &Variant) -> String {
    let fg = v.ui.text.default.hex_rgb();
    let bg = v.ui.surface.background.hex_rgb();
    let gutter = v.ui.editor.line_number.hex_rgb();
    let active = v.ui.surface.active_line.hex_rgb();

    let mut rows = String::new();
    for (i, line) in LINES.iter().enumerate().map(|(i, l)| (i + 1, l)) {
        let mut spans = String::new();
        for &(tok, text) in line.iter() {
            let text = escape(text);
            match tok {
                None => write!(spans, r#"<span style="color:{fg}">{text}</span>"#).unwrap(),
                Some(tok) => {
                    let st = tok.style(&v.syntax);
                    let mut css = format!("color:{}", st.color.hex_rgb());
                    if st.italic {
                        css.push_str(";font-style:italic");
                    }
                    if st.bold {
                        css.push_str(";font-weight:700");
                    }
                    write!(spans, r#"<span style="{css}">{text}</span>"#).unwrap();
                }
            }
        }
        let line_bg = if i == ACTIVE_LINE {
            format!("background:{active}")
        } else {
            String::new()
        };
        let spans = if spans.is_empty() {
            "&nbsp;".to_owned()
        } else {
            spans
        };
        write!(rows, r#"<div class="ln" style="{line_bg}"><span class="g" style="color:{gutter}">{i:>2}</span>{spans}</div>"#)
            .unwrap();
    }
    format!(
        r#"<div class="pane" style="background:{bg}"><div class="cap" style="color:{gutter}">{}: {bg}</div>{rows}</div>"#,
        escape(&caption(v))
    )
}

/// "Norde (dark)", but not "Norde Light (light)".
fn caption(v: &Variant) -> String {
    let kind = v.appearance.as_str();
    if v.name.to_lowercase().contains(kind) {
        v.name.clone()
    } else {
        format!("{} ({kind})", v.name)
    }
}

fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}
