use std::str::FromStr;

use anyhow::{bail, Context, Result};
use itertools::Itertools;

#[derive(Debug)]
pub enum Command {
    Push { segment: Segment, index: u16 },
    Pop { segment: Segment, index: u16 },
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
}

#[derive(Debug)]
pub enum Segment {
    Argument,
    Local,
    Static,
    Constant,
    This,
    That,
    Pointer,
    Temp,
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> Result<Self> {
        let mut words = line.split_whitespace();
        let first_word = words.by_ref().next().context("line must not be empty")?;

        let command = match first_word {
            "add" => Command::Add,
            "sub" => Command::Sub,
            "neg" => Command::Neg,
            "eq" => Command::Eq,
            "gt" => Command::Gt,
            "lt" => Command::Lt,
            "and" => Command::And,
            "or" => Command::Or,
            "not" => Command::Not,
            "push" | "pop" => {
                let (segment, index) = words
                    .collect_tuple()
                    .with_context(|| format!("push/pop command expects 2 argments: {line:?}"))?;

                let segment = segment.parse()?;
                let index = index.parse().context("invalid index")?;

                if first_word == "push" {
                    Command::Push { segment, index }
                } else {
                    Command::Pop { segment, index }
                }
            }
            _ => bail!("unrecognized command type {first_word:?}. line: {line:?}"),
        };

        Ok(command)
    }
}

impl FromStr for Segment {
    type Err = anyhow::Error;

    fn from_str(word: &str) -> Result<Self> {
        let segment = match word {
            "argument" => Segment::Argument,
            "local" => Segment::Local,
            "static" => Segment::Static,
            "constant" => Segment::Constant,
            "this" => Segment::This,
            "that" => Segment::That,
            "pointer" => Segment::Pointer,
            "temp" => Segment::Temp,
            _ => bail!("unrecognized segment: {word:?}"),
        };

        Ok(segment)
    }
}
