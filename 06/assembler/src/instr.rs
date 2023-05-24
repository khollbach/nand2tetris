mod parse;
mod code_gen;

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
struct AInstr {
    /// The highest bit should never be set.
    /// I.e., the max value is 2^15 - 1.
    value: u16,
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
