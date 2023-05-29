//! Data types for representing a single command in the VM language.

use std::fmt::{self, Debug};

/// A command in the VM language.
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

/// One of the virtual memory segments of the nand2tetris VM.
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

impl Debug for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Push { segment, index } => write!(f, "push {segment:?} {index:?}"),
            Self::Pop { segment, index } => write!(f, "push {segment:?} {index:?}"),
            Self::Add => write!(f, "add"),
            Self::Sub => write!(f, "sub"),
            Self::Neg => write!(f, "neg"),
            Self::Eq => write!(f, "eq"),
            Self::Gt => write!(f, "gt"),
            Self::Lt => write!(f, "lt"),
            Self::And => write!(f, "and"),
            Self::Or => write!(f, "or"),
            Self::Not => write!(f, "not"),
        }
    }
}

impl Debug for Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Argument => write!(f, "argument"),
            Self::Local => write!(f, "local"),
            Self::Static => write!(f, "static"),
            Self::Constant => write!(f, "constant"),
            Self::This => write!(f, "this"),
            Self::That => write!(f, "that"),
            Self::Pointer => write!(f, "pointer"),
            Self::Temp => write!(f, "temp"),
        }
    }
}
