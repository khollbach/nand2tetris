//! Parse a line of text into a VM [`Command`].

use std::str::FromStr;

use anyhow::{bail, ensure, Context, Result};

use crate::vm_command::{Command, Segment, VirtualMemoryAddr};

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> Result<Self> {
        let line = line.trim();

        let command = match line.split_once(char::is_whitespace) {
            Some((push_pop, vm_addr)) => {
                let vm_addr = vm_addr
                    .parse()
                    .with_context(|| format!("failed to parse command: {line:?}"))?;

                match push_pop {
                    "push" => Self::Push { source: vm_addr },
                    "pop" => {
                        ensure!(
                            !matches!(vm_addr.segment, Segment::Constant),
                            "cannot pop a constant: {line:?}"
                        );
                        Self::Pop { dest: vm_addr }
                    }
                    _ => bail!("invalid command: {line:?}"),
                }
            }

            None => match line {
                "add" => Command::Add,
                "sub" => Command::Sub,
                "neg" => Command::Neg,
                "eq" => Command::Eq,
                "gt" => Command::Gt,
                "lt" => Command::Lt,
                "and" => Command::And,
                "or" => Command::Or,
                "not" => Command::Not,
                _ => bail!("unrecognized command: {line:?}"),
            },
        };

        Ok(command)
    }
}

impl FromStr for VirtualMemoryAddr {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (segment, index) = s.split_once(char::is_whitespace).with_context(|| {
            format!("failed to parse virtual memory address. expected two words, got: {s:?}")
        })?;

        let segment = segment.parse()?;
        let index = index.parse().with_context(|| {
            format!("failed to parse index field of virtual memory address: {index:?}")
        })?;
        ensure!(
            index < 2u16.pow(15),
            "virtual memory address index must be less than 2^15: {index:?}"
        );

        Ok(Self { segment, index })
    }
}

impl FromStr for Segment {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let segment = match s {
            "argument" => Self::Argument,
            "local" => Self::Local,
            "static" => Self::Static,
            "constant" => Self::Constant,
            "this" => Self::This,
            "that" => Self::That,
            "pointer" => Self::Pointer,
            "temp" => Self::Temp,
            _ => bail!("unrecognized segment: {s:?}"),
        };

        Ok(segment)
    }
}
