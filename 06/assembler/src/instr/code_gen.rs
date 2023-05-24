use super::{AInstr, CInstr, Comp, Dest, Instr, InstrInner, Jump};

impl Instr {
    pub fn code_gen(&self) -> u16 {
        match &self.inner {
            InstrInner::AInstr(inner) => inner.code_gen(),
            InstrInner::CInstr(inner) => inner.code_gen(),
        }
    }
}

impl AInstr {
    fn code_gen(&self) -> u16 {
        self.value
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
