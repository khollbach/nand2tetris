use anyhow::{ensure, Context, Result};
use itertools::Itertools;

/// Remove comments and blank lines.
///
/// Only handles comments that appear on their own line. E.g., you're not
/// allowed to write:
/// ```no_run
/// A=D+M // this is an end-of-line comment, but that's not allowed
/// ```
//
// todo: could be a nice improvement (and not too hard) to correctly handle
// end-of-line comments.
pub fn remove_comments(
    lines: impl Iterator<Item = Result<String>>,
) -> impl Iterator<Item = Result<String>> {
    lines.filter_ok(|line| {
        let line = line.trim();
        !(line.is_empty() || line.starts_with("//"))
    })
}

#[derive(Debug)]
pub enum Line {
    // Label(&str),
    AInstr(AInstr),
    CInstr(CInstr),
}

#[derive(Debug)]
pub struct AInstr {
    /// The highest bit should never be set.
    /// I.e., the max value is 2^15 - 1.
    value: u16,
}

#[derive(Debug)]
pub struct CInstr {
    dest: Dest,
    comp: Comp,
    jump: Jump,
}

#[derive(Debug)]
struct Dest {
    a: bool,
    d: bool,
    m: bool,
}

#[derive(Debug)]
struct Comp {
    /// Note that only certain combinations are valid.
    c_bits: [bool; 6],
}

#[derive(Debug)]
enum Jump {
    Never,
    Greater,
    Equal,
    GreaterEqual,
    Less,
    NotEqual,
    LessEqual,
    Always,
}

impl Line {
    pub fn parse(line: &str) -> Result<Self> {
        let line = line.trim();

        if line.starts_with('@') {
            Ok(Line::AInstr(AInstr::parse(line)?))
        } else {
            Ok(Line::CInstr(CInstr::parse(line)?))
        }
    }
}

impl AInstr {
    fn parse(line: &str) -> Result<Self> {
        let line = line.trim();
        ensure!(
            line.starts_with('@'),
            "A-instruction must start with '@': {line:?}"
        );

        let value: u16 = line[1..]
            .parse()
            .with_context(|| format!("failed to parse A-instruction as u16: {line:?}"))?;

        let limit = 2u16.pow(15);
        ensure!(
            value < limit,
            "A-instruction value must be less than limit: {value} vs {limit}"
        );

        Ok(AInstr { value })
    }
}

impl CInstr {
    fn parse(line: &str) -> Result<Self> {
        let line = line.trim();

        todo!()
    }
}
