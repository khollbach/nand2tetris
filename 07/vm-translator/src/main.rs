//! Translates nand2tetris' Virtual Machine language to Hack assembly language.

mod parse;

use std::{
    env,
    ffi::OsStr,
    fs::{self, File},
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

use anyhow::{ensure, Context, Result};
use itertools::Itertools;

use crate::parse::Command;

/// Expects one argmuent: a filename with a `.vm` extension.
///
/// Generates an output file in the same directory as the input file, with the
/// same filename, except ending in `.asm` instead of `.vm`.
fn main() -> Result<()> {
    let (_prog_name, in_path) = env::args().collect_tuple().with_context(|| {
        let n = env::args().count().saturating_sub(1);
        format!("expected one argument, got {n}")
    })?;
    let out_path = out_path(&in_path)?;

    let in_file =
        File::open(&in_path).with_context(|| format!("couldn't open file {in_path:?}"))?;
    let out_file =
        File::create(&out_path).with_context(|| format!("couldn't create file {out_path:?}"))?;

    let result = translate(in_file, out_file);

    // If translation fails, clean up the output file.
    if result.is_err() {
        if let Err(rm_err) = fs::remove_file(&out_path) {
            eprintln!("failed to clean up output file {out_path:?}: {rm_err}");
        }
    }

    result
}

/// Convert `path/to/filename.vm` to `path/to/filename.asm`.
fn out_path(path: impl AsRef<Path>) -> Result<PathBuf> {
    let path = path.as_ref();

    let name = path
        .file_name()
        .and_then(OsStr::to_str)
        .with_context(|| format!("expected unicode file name, got {path:?}"))?;

    let starts_upper = name.starts_with(char::is_uppercase);
    ensure!(starts_upper, "file name must start uppercase: {path:?}");

    let ext = path.extension().and_then(OsStr::to_str);
    ensure!(ext == Some("vm"), "must have .vm extension: {path:?}");

    // These should succeed, since `.extension()` suceeded.
    let dir = path.parent().unwrap();
    let mut out_name = path.file_stem().unwrap().to_owned();
    out_name.push(".asm");

    let out_path = dir.join(out_name);
    Ok(out_path)
}

/// Translate VM language into assembly language.
fn translate(in_file: File, _out_file: File) -> Result<()> {
    let lines = BufReader::new(in_file)
        .lines()
        .map(|r| r.map_err(Into::into));

    for line in remove_comments_and_blanks(lines) {
        let command: Command = line?.parse()?;
        dbg!(command);
    }

    Ok(())
}

/// Trim comments, and then remove any blank lines.
fn remove_comments_and_blanks(
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
