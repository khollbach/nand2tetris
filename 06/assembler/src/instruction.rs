//! Data types to represent assembly instructions.

mod parse;
mod code_gen;

/// All memory addresses must be strictly less than this limit.
///
/// This applies to both RAM (data memory) and ROM (instruction memory).
pub const ADDRESS_LIMIT: u16 = 2u16.pow(15);

/// Either an instruction, or a "label" pseudo-instruction.
#[derive(Debug)]
pub enum Line {
    Instr(Instr),
    Label(String),
}

/// An A-instruction or a C-instruction.
#[derive(Debug)]
pub struct Instr {
    // "private" enum
    inner: InstrInner,
}

#[derive(Debug)]
enum InstrInner {
    AInstr(AInstr),
    CInstr(CInstr),
}

#[derive(Debug)]
enum AInstr {
    Symbol(String),

    /// The highest bit should never be set.
    /// I.e., the max value is 2^15 - 1.
    Literal(u16),
}

#[derive(Debug)]
struct CInstr {
    dest: Dest,
    comp: Comp,
    jump: Jump,
}

#[derive(Debug, Default)]
struct Dest {
    a: bool,
    d: bool,
    m: bool,
}

#[derive(Debug)]
struct Comp {
    a_bit: bool,

    /// Note: only certain combinations of (a_bit, c_bits) are valid.
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
