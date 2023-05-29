//! Parse a line of text into a VM [`Command`].

use std::str::FromStr;

use anyhow::{bail, Context, Result};
use itertools::Itertools;

use crate::vm_command::{Command, Segment};

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> Result<Self> {
        let mut words = line.split_whitespace();
        let first_word = words.by_ref().next().context("line must not be empty")?;

        let command = match first_word {
            "add" => Self::Add,
            "sub" => Self::Sub,
            "neg" => Self::Neg,
            "eq" => Self::Eq,
            "gt" => Self::Gt,
            "lt" => Self::Lt,
            "and" => Self::And,
            "or" => Self::Or,
            "not" => Self::Not,
            "push" | "pop" => {
                let (segment, index) = words
                    .collect_tuple()
                    .with_context(|| format!("push/pop command expects 2 argments: {line:?}"))?;

                let segment = segment.parse()?;
                let index = index.parse().context("invalid index")?;

                if first_word == "push" {
                    Self::Push { segment, index }
                } else {
                    Self::Pop { segment, index }
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
            "argument" => Self::Argument,
            "local" => Self::Local,
            "static" => Self::Static,
            "constant" => Self::Constant,
            "this" => Self::This,
            "that" => Self::That,
            "pointer" => Self::Pointer,
            "temp" => Self::Temp,
            _ => bail!("unrecognized segment: {word:?}"),
        };

        Ok(segment)
    }
}
