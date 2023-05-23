use anyhow::Result;
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

pub fn parse_line(line: &str) -> Line {
    todo!()
}
