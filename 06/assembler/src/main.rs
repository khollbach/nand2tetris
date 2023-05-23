mod parse;

use std::{
    env,
    ffi::OsStr,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

use anyhow::{ensure, Context, Result};
use itertools::Itertools;

use crate::parse::{remove_comments, Line};

fn main() -> Result<()> {
    let (mut in_file, _out_file) = open_files()?;

    let lines = BufReader::new(&mut in_file)
        .lines()
        .map(|r| r.map_err(Into::into));

    for line in remove_comments(lines) {
        dbg!(Line::parse(dbg!(&line?))?);
    }

    // // second pass
    // in_file.seek(SeekFrom::Start(0))?;
    // for line in BufReader::new(in_file).lines() {
    //     dbg!(line?);
    // }

    Ok(())
}

fn open_files() -> Result<(File, File)> {
    let (_prog_name, path) = env::args().collect_tuple().with_context(|| {
        let n = env::args().count().saturating_sub(1);
        format!("expected one argument, got {n}")
    })?;
    let path: &Path = path.as_ref();

    let ext = path.extension().and_then(OsStr::to_str);
    ensure!(
        ext == Some("asm"),
        "must have .asm extension: {}",
        path.display()
    );

    let in_file =
        File::open(path).with_context(|| format!("couldn't open file {}", path.display()))?;

    // These should succeed, since `.extension()` suceeded.
    let dir = path.parent().unwrap();
    let mut out_name = path.file_stem().unwrap().to_owned();
    out_name.push(".hack");

    let out_path = dir.join(out_name);
    let out_file = File::create(&out_path)
        .with_context(|| format!("couldn't create file {}", out_path.display()))?;

    Ok((in_file, out_file))
}
