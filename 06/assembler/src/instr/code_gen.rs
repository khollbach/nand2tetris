use super::{AInstr, CInstr, Instr, InstrInner};

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
        todo!()
    }
}
