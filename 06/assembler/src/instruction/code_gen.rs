//! Generate binary machine code for an instruction.

use anyhow::Result;

use super::{AInstr, CInstr, Comp, Dest, Instr, InstrInner, Jump};
use crate::symbol_table::SymbolTable;

impl Instr {
    /// Unknown symbols are assumed to be new variables, and we generate new
    /// symbol-table entries accordingly.
    pub fn code_gen(self, symbol_table: &mut SymbolTable) -> Result<u16> {
        match self.inner {
            InstrInner::AInstr(a) => a.code_gen(symbol_table),
            InstrInner::CInstr(c) => Ok(c.code_gen()),
        }
    }
}

impl AInstr {
    fn code_gen(self, symbol_table: &mut SymbolTable) -> Result<u16> {
        match self {
            AInstr::Literal(value) => Ok(value),
            AInstr::Symbol(symbol) => match symbol_table.lookup_symbol(&symbol) {
                Some(value) => Ok(value),
                None => symbol_table.new_variable(symbol),
            },
        }
    }
}

impl CInstr {
    fn code_gen(&self) -> u16 {
        let mut code = 0;

        code <<= 3;
        code |= 0b111;

        code <<= 7;
        code |= self.comp.code_gen();

        code <<= 3;
        code |= self.dest.code_gen();

        code <<= 3;
        code |= self.jump.code_gen();

        code
    }
}

impl Comp {
    fn code_gen(&self) -> u16 {
        let mut code = 0;

        code <<= 1;
        code |= self.a_bit as u16;

        code <<= 6;
        code |= bits_to_u16(self.c_bits);

        code
    }
}

fn bits_to_u16<const N: usize>(bits: [bool; N]) -> u16 {
    let mut code = 0;

    for bit in bits {
        code <<= 1;
        code |= bit as u16;
    }

    code
}

impl Dest {
    fn code_gen(&self) -> u16 {
        bits_to_u16([self.a, self.d, self.m])
    }
}

impl Jump {
    fn code_gen(&self) -> u16 {
        let bits = match self {
            Jump::Never => [0, 0, 0],
            Jump::Greater => [0, 0, 1],
            Jump::Equal => [0, 1, 0],
            Jump::GreaterEqual => [0, 1, 1],
            Jump::Less => [1, 0, 0],
            Jump::NotEqual => [1, 0, 1],
            Jump::LessEqual => [1, 1, 0],
            Jump::Always => [1, 1, 1],
        };
        let bits = bits.map(|bit| bit != 0);

        bits_to_u16(bits)
    }
}
