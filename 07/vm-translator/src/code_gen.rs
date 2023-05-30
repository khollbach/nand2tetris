use std::io::Write;

use anyhow::{Result, ensure};

use crate::vm_command::{Command, Segment};

impl Command {
    pub fn code_gen(self, mut out: impl Write) -> Result<()> {
        match self {
            Command::Push { segment, index } => {
                writeln!(out, "// *** {self:?} ***")?;
                writeln!(out)?;
                segment.read_into_d_register(index, &mut out)?;
                writeln!(out)?;
                writeln!(out, "// write D to top-of-stack")?;
                writeln!(out, "@SP")?;
                writeln!(out, "A=M  // dereference")?;
                writeln!(out, "M=D")?;
                writeln!(out)?;
                writeln!(out, "// incr SP")?;
                writeln!(out, "@SP")?;
                writeln!(out, "M=M+1")?;
                writeln!(out)?;
                writeln!(out)?;
                writeln!(out)?;
            }

            Command::Pop { segment, index } => todo!(),
            Command::Add => todo!(),
            Command::Sub => todo!(),
            Command::Neg => todo!(),
            Command::Eq => todo!(),
            Command::Gt => todo!(),
            Command::Lt => todo!(),
            Command::And => todo!(),
            Command::Or => todo!(),
            Command::Not => todo!(),
        }

        Ok(())
    }
}

impl Segment {
    /// Generate assembly code to read data from this segment, into the D
    /// register.
    /// 
    /// Note that the generated code may clobber the A register.
    fn read_into_d_register(self, offset: u16, mut out: impl Write) -> Result<()> {
        writeln!(out, "// read {self:?}[{offset}] into D")?;

        match self {
            Self::Constant => {
                ensure!(offset < 2u16.pow(15), "constants must be less than 2^15");
                writeln!(out, "@{offset}")?;
                writeln!(out, "D=A")?;
            }

            Self::Argument => todo!(),
            Self::Local => todo!(),
            Self::Static => todo!(),
            Self::This => todo!(),
            Self::That => todo!(),
            Self::Pointer => todo!(),
            Self::Temp => todo!(),
        }

        Ok(())
    }
}
