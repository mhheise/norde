//! Render one semantic theme spec into themes for many applications.

mod color;
mod formats;
mod theme;

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use anyhow::{Context, Result, bail};
use clap::Parser;

use theme::Family;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    /// Theme spec to render; output paths are relative to its directory.
    #[arg(default_value = "norde.toml")]
    spec: PathBuf,

    /// Don't write anything; exit non-zero if any output is missing or stale.
    #[arg(long)]
    check: bool,

    /// Only render these formats (repeatable). Default: every `[formats.*]`.
    #[arg(long = "format", short = 'f', value_name = "NAME")]
    only: Vec<String>,
}

/// WCAG-AA for body text; syntax colors below this get a warning.
const MIN_CONTRAST: f64 = 4.5;

struct Output {
    path: PathBuf,
    contents: String,
}

#[expect(clippy::print_stderr, reason = "reporting errors is the CLI's job")]
fn main() -> ExitCode {
    match run(Cli::parse()) {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::FAILURE,
        Err(e) => {
            eprintln!("error: {e:#}");
            ExitCode::FAILURE
        }
    }
}

#[expect(
    clippy::print_stdout,
    clippy::print_stderr,
    reason = "progress and warnings are the CLI's output"
)]
fn run(cli: Cli) -> Result<bool> {
    let family = Family::load(&cli.spec)?;
    for w in family.contrast_warnings(MIN_CONTRAST) {
        eprintln!("warning: {w}");
    }
    let root = cli.spec.parent().unwrap_or(Path::new(""));

    for name in &cli.only {
        if !family.formats.contains_key(name) {
            bail!(
                "--format {name:?} is not configured in {}",
                cli.spec.display()
            );
        }
    }

    let mut outputs = Vec::new();
    for (name, cfg) in &family.formats {
        if !cli.only.is_empty() && !cli.only.contains(name) {
            continue;
        }
        let renderer = formats::lookup(name).with_context(|| {
            format!(
                "[formats.{name}]: unknown format; available: {}",
                formats::names().join(", ")
            )
        })?;
        let ctx = || format!("rendering format {name:?}");
        if cfg.per_variant() {
            for v in &family.variants {
                if v.name.contains(['/', '\\']) {
                    bail!("variant name {:?} can't be used in a file path", v.name);
                }
                let contents = renderer
                    .render(&family, std::slice::from_ref(v))
                    .with_context(ctx)?;
                outputs.push(Output {
                    path: root.join(cfg.path_for(v)),
                    contents,
                });
            }
        } else {
            let contents = renderer
                .render(&family, &family.variants)
                .with_context(ctx)?;
            outputs.push(Output {
                path: root.join(&cfg.path),
                contents,
            });
        }
    }

    for (i, a) in outputs.iter().enumerate() {
        if outputs[..i].iter().any(|b| b.path == a.path) {
            bail!("two outputs would write {}", a.path.display());
        }
    }

    let mut ok = true;
    for out in &outputs {
        let current = std::fs::read_to_string(&out.path).ok();
        let fresh = current.as_deref() == Some(out.contents.as_str());
        let shown = out.path.display();
        match (cli.check, fresh) {
            (_, true) => println!("up to date  {shown}"),
            (true, false) => {
                println!(
                    "{}  {shown}",
                    if current.is_some() {
                        "stale"
                    } else {
                        "missing"
                    }
                );
                ok = false;
            }
            (false, false) => {
                if let Some(dir) = out.path.parent() {
                    std::fs::create_dir_all(dir)
                        .with_context(|| format!("creating {}", dir.display()))?;
                }
                std::fs::write(&out.path, &out.contents)
                    .with_context(|| format!("writing {shown}"))?;
                println!("wrote       {shown}");
            }
        }
    }
    if !ok {
        eprintln!("outputs are out of date; run `cargo gen` to regenerate");
    }
    Ok(ok)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn norde() -> (PathBuf, Family) {
        let spec = Path::new(env!("CARGO_MANIFEST_DIR")).join("norde.toml");
        let family = Family::load(&spec).unwrap();
        (spec, family)
    }

    /// The README promises every syntax color clears WCAG-AA in both themes.
    #[test]
    fn norde_syntax_meets_wcag_aa() {
        let (_, family) = norde();
        assert_eq!(family.contrast_warnings(MIN_CONTRAST), Vec::<String>::new());
    }

    /// In the light theme, types (green) are darkest and strings (orange) are
    /// lightest, about 6 luminance points apart, so that readers with red-green
    /// color blindness can tell them apart.
    #[test]
    fn norde_light_keeps_types_and_strings_apart() {
        let (_, family) = norde();
        let light = family
            .variants
            .iter()
            .find(|v| v.name == "Norde Light")
            .unwrap();
        let s = &light.syntax;
        let green = s.ty.color.luminance();
        let orange = s.string.color.luminance();
        for (role, style) in [
            ("keyword", &s.keyword),
            ("function", &s.function),
            ("constant", &s.constant),
        ] {
            let l = style.color.luminance();
            assert!(
                green < l,
                "type ({green:.3}) not darker than {role} ({l:.3})"
            );
            assert!(
                l < orange,
                "string ({orange:.3}) not lighter than {role} ({l:.3})"
            );
        }
        assert!(orange - green >= 0.06, "gap is only {:.3}", orange - green);
    }

    /// Terminal colors are readable as text, and black/white as a pair (status
    /// bars and other reversed text). `dim` is deliberately faint, so exempt.
    #[test]
    fn norde_terminal_colors_are_readable() {
        const MIN: f64 = 3.0;
        let (_, family) = norde();
        let mut bad = Vec::new();
        for v in &family.variants {
            let t = &v.terminal;
            for (kind, ansi) in [("normal", &t.normal), ("bright", &t.bright)] {
                for (name, color) in theme::Ansi::NAMES.iter().zip(ansi.colors()) {
                    let ratio = color.contrast(t.background);
                    if ratio < MIN {
                        bad.push(format!("{}: {kind}.{name} {ratio:.2}", v.name));
                    }
                }
            }
            let pair = t.normal.black.contrast(t.normal.white);
            if pair < MIN {
                bad.push(format!("{}: black on white {pair:.2}", v.name));
            }
        }
        assert_eq!(bad, Vec::<String>::new());
    }

    /// Committed outputs must match the spec (same as `cargo gen --check`).
    #[test]
    fn committed_outputs_are_up_to_date() {
        let (spec, _) = norde();
        assert!(
            run(Cli {
                spec,
                check: true,
                only: vec![]
            })
            .unwrap(),
            "run `cargo gen` and commit the result"
        );
    }
}
