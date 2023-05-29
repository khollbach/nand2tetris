use std::io::Write;

use anyhow::Result;

use crate::vm_command::Command;

impl Command {
    pub fn code_gen(&self, mut out: impl Write) -> Result<()> {
        match self {
            Command::Push { segment, index } => {
                writeln!(out, "// {self:?}")?;
                writeln!(out)?;
                writeln!(out, "// todo: get the data into the D register")?;
                writeln!(out)?;
                writeln!(out, "@SP")?;
                writeln!(out, "A=M  // deref")?;
                writeln!(out, "M=D")?;
                writeln!(out)?;
                writeln!(out, "@SP")?;
                writeln!(out, "M=M+1")?;
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
