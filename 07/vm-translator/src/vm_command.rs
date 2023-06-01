//! Data types for representing a single command in the VM language.

use std::fmt::{self, Debug};

/// A command in the VM language.
#[derive(Clone, Copy)]
pub enum Command {
    /// Push to the stack.
    Push {
        source: VirtualMemoryAddr,
    },

    /// Pop from the stack.
    Pop {
        /// `segment` must not be `Constant`.
        dest: VirtualMemoryAddr,
    },

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

/// A location in the nand2tetris VM's virtual memory.
#[derive(Clone, Copy)]
pub struct VirtualMemoryAddr {
    pub segment: Segment,
    pub index: u16,
}

/// One of the virtual memory segments of the nand2tetris VM.
#[derive(Clone, Copy)]
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
            Self::Push { source } => write!(f, "push {source:?}"),
            Self::Pop { dest } => write!(f, "pop {dest:?}"),
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

impl Debug for VirtualMemoryAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {}", self.segment, self.index)
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
