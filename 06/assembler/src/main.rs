mod instr;

use std::{
    env,
    ffi::OsStr,
    fs::{self, File},
    io::{prelude::*, BufReader},
    path::{Path, PathBuf},
};

use anyhow::{ensure, Context, Result};
use itertools::Itertools;

use crate::instr::Instr;

fn main() -> Result<()> {
    let (_prog_name, in_path) = env::args().collect_tuple().with_context(|| {
        let n = env::args().count().saturating_sub(1);
        format!("expected one argument, got {n}")
    })?;
    let out_path = out_path(&in_path)?;

    let in_file = File::open(&in_path).with_context(|| format!("couldn't open file {in_path}"))?;
    let out_file = File::create(&out_path)
        .with_context(|| format!("couldn't create file {}", out_path.display()))?;

    let result = translate(in_file, out_file);

    // If unsucessful, clean up the output file.
    if result.is_err() {
        if let Err(e) = fs::remove_file(&out_path) {
            eprintln!("failed to clean up output file {}: {e}", out_path.display());
        }
        result?;
    };

    Ok(())
}

fn out_path(path: impl AsRef<Path>) -> Result<PathBuf> {
    let path = path.as_ref();

    let ext = path.extension().and_then(OsStr::to_str);
    ensure!(
        ext == Some("asm"),
        "must have .asm extension: {}",
        path.display()
    );

    // These should succeed, since `.extension()` suceeded.
    let dir = path.parent().unwrap();
    let mut out_name = path.file_stem().unwrap().to_owned();
    out_name.push(".hack");

    let out_path = dir.join(out_name);
    Ok(out_path)
}

fn translate(in_file: File, mut out_file: File) -> Result<()> {
    let lines = BufReader::new(in_file)
        .lines()
        .map(|r| r.map_err(Into::into));

    for line in remove_comments(lines) {
        let instr = Instr::parse(&line?)?;
        let code = instr.code_gen();
        writeln!(out_file, "{code:0>16b}")?;
    }

    Ok(())
}

/// Remove comments and blank lines.
fn remove_comments(
    lines: impl Iterator<Item = Result<String>>,
) -> impl Iterator<Item = Result<String>> {
    lines.filter_map_ok(|mut line| {
        // Remove everything after the first "//".
        if let Some(idx) = line.find("//") {
            line.truncate(idx);
        }

        // If this line is blank, filter it out.
        if line.trim().is_empty() {
            None
        } else {
            Some(line)
        }
    })
}
