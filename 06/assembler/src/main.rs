//! An assembler for the hack assembly language.
//!
//! Translates high-level assembly code into binary machine instructions.

mod symbol_table;
mod instruction;

use std::{
    env,
    ffi::OsStr,
    fs::{self, File},
    io::{prelude::*, BufReader},
    path::{Path, PathBuf},
};

use anyhow::{ensure, Context, Result};
use instruction::Line;
use itertools::Itertools;

use crate::{instruction::ADDRESS_LIMIT, symbol_table::SymbolTable};

/// Expects one argmuent: the name of an assembly source file, with a `.asm`
/// extension.
///
/// The output file will be in the same directory as the input file, and have
/// the same name, except ending in `.hack` instead of `.asm`.
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

    // If translation fails, clean up the output file.
    if result.is_err() {
        if let Err(rm_err) = fs::remove_file(&out_path) {
            eprintln!(
                "failed to clean up output file {}: {rm_err}",
                out_path.display()
            );
        }
    }

    result
}

/// Convert `path/to/filename.asm` to `path/to/filename.hack`.
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

/// Translate assembly into binary format.
fn translate(mut in_file: File, out_file: File) -> Result<()> {
    let mut symbol_table = SymbolTable::new();
    first_pass(&mut in_file, &mut symbol_table)?;
    second_pass(in_file, out_file, &mut symbol_table)
}

/// Read labels, of the form `(LABEL)`, and add them to the symbol table.
fn first_pass(in_file: &mut File, symbol_table: &mut SymbolTable) -> Result<()> {
    debug_assert_eq!(in_file.stream_position()?, 0);

    let lines = BufReader::new(in_file)
        .lines()
        .map(|r| r.map_err(Into::into));

    let mut num_instructions = 0;

    for line in remove_comments(lines) {
        match Line::parse(&line?)? {
            Line::Label(symbol) => {
                symbol_table.new_label(symbol, num_instructions)?;
            }
            Line::Instr(_) => {
                ensure!(
                    num_instructions < ADDRESS_LIMIT,
                    "can't emit more than {ADDRESS_LIMIT} instructions"
                );

                num_instructions += 1;
            }
        }
    }

    Ok(())
}

/// Does the actual code-generation.
///
/// Unknown symbols are assumed to be new variables, and we generate new
/// symbol-table entries accordingly.
fn second_pass(
    mut in_file: File,
    mut out_file: File,
    symbol_table: &mut SymbolTable,
) -> Result<()> {
    in_file.rewind()?;

    let lines = BufReader::new(in_file)
        .lines()
        .map(|r| r.map_err(Into::into));

    for line in remove_comments(lines) {
        match Line::parse(&line?)? {
            Line::Label(_) => (),
            Line::Instr(instr) => {
                let code = instr.code_gen(symbol_table)?;
                writeln!(out_file, "{code:0>16b}")?;
            }
        }
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
