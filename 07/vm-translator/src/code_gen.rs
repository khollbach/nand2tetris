// possible todo: refactor this code nicely to re-use push/pop impl's in arith codegen.
// The generated code won't be minimal, but it'll probably clean up the VM translator impl quite a bit.

use std::io::Write;

use anyhow::{bail, Result};

use crate::vm_command::{Command, Segment, VirtualMemoryAddr};

pub fn halt(mut out: impl Write) -> Result<()> {
    write!(
        out,
        "\
// Halt.
(INFINITE_LOOP)
@INFINITE_LOOP
0;JMP
"
    )?;
    Ok(())
}

impl Command {
    pub fn code_gen(self, mut out: impl Write) -> Result<()> {
        write!(out, "// *** {self:?} ***\n\n")?;

        match self {
            Self::Push { source } => push(source, &mut out)?,
            Self::Pop { dest } => pop(dest, &mut out)?,
            Self::Add => add(&mut out)?,
            Self::Sub => sub(&mut out)?,
            Self::Neg => neg(&mut out)?,

            // todo: impl these, ideally without too-too much code dup
            // See also the codegen in the unit test for Eq
            Self::Eq => todo!(),
            Self::Gt => todo!(),
            Self::Lt => todo!(),

            Self::And => and(&mut out)?,
            Self::Or => or(&mut out)?,
            Self::Not => not(&mut out)?,
        }

        write!(out, "\n\n\n")?;
        Ok(())
    }
}

fn push(source: VirtualMemoryAddr, mut out: impl Write) -> Result<()> {
    let VirtualMemoryAddr { segment, index } = source;

    match segment {
        Segment::Constant => {
            write!(
                out,
                "\
// D := {index}
@{index}
D=A

"
            )?;
        }

        Segment::Pointer => {
            let this_that = match index {
                0 => "THIS",
                1 => "THAT",
                i @ _ => bail!("invalid index into `pointer` virtual memory segment: {i}"),
            };

            write!(
                out,
                "\
// D := {this_that}
@{this_that}
D=M

"
            )?;
        }

        s @ (Segment::Local | Segment::Argument | Segment::This | Segment::That) => {
            let base = match s {
                Segment::Local => "LCL",
                Segment::Argument => "ARG",
                Segment::This => "THIS",
                Segment::That => "THAT",
                _ => unreachable!(),
            };

            write!(
                out,
                "\
// D := *({base} + {index})
@{base}
D=M
@{index}
A=D+A
D=M

"
            )?;
        }

        Segment::Temp => todo!(),
        Segment::Static => todo!(),
    }

    write!(
        out,
        "\
// *SP := D
@SP
A=M
M=D

// SP++
@SP
M=M+1
"
    )?;
    Ok(())
}

fn pop(dest: VirtualMemoryAddr, mut out: impl Write) -> Result<()> {
    let base = match dest.segment {
        Segment::Constant => panic!("can't write to constant segment"),
        Segment::Local => "LCL",
        Segment::Argument => "ARG",
        Segment::This => "THIS",
        Segment::That => "THAT",
        Segment::Temp => todo!(),
        Segment::Pointer => todo!(),
        Segment::Static => todo!(),
    };

    let index = dest.index;
    write!(
        out,
        "\
// SP--
@SP
M=M-1

// R13 := {base} + {index}
@{base}
D=M
@{index}
D=D+A
@R13
M=D

// *R13 := *SP
@SP
A=M
D=M
@R13
A=M
M=D
"
    )?;

    Ok(())
}

fn add(mut out: impl Write) -> Result<()> {
    write!(
        out,
        "\
// SP--
@SP
M=M-1

// D := *SP
A=M
D=M

// SP--
@SP
M=M-1

// *SP += D
A=M
M=M+D

// SP++
@SP
M=M+1
"
    )?;
    Ok(())
}

// todo: reduce code dup b/w Add and Sub
fn sub(mut out: impl Write) -> Result<()> {
    write!(
        out,
        "\
// SP--
@SP
M=M-1

// D := *SP
A=M
D=M

// SP--
@SP
M=M-1

// *SP -= D
A=M
M=M-D

// SP++
@SP
M=M+1
"
    )?;
    Ok(())
}

fn neg(mut out: impl Write) -> Result<()> {
    write!(
        out,
        "\
// SP--
@SP
M=M-1

// *SP := -*SP
A=M
M=-M

// SP++
@SP
M=M+1
"
    )?;
    Ok(())
}

// todo: reduce code dup b/w these (And/Or/Not) and Add/Sub/Neg
fn and(mut out: impl Write) -> Result<()> {
    write!(
        out,
        "\
// SP--
@SP
M=M-1

// D := *SP
A=M
D=M

// SP--
@SP
M=M-1

// *SP &= D
A=M
M=M&D

// SP++
@SP
M=M+1
"
    )?;
    Ok(())
}

fn or(mut out: impl Write) -> Result<()> {
    write!(
        out,
        "\
// SP--
@SP
M=M-1

// D := *SP
A=M
D=M

// SP--
@SP
M=M-1

// *SP |= D
A=M
M=M|D

// SP++
@SP
M=M+1
"
    )?;
    Ok(())
}

fn not(mut out: impl Write) -> Result<()> {
    write!(
        out,
        "\
// SP--
@SP
M=M-1

// *SP := !*SP
A=M
M=!M

// SP++
@SP
M=M+1
"
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::str;

    use super::*;

    fn assert_code_gen(command: Command, expected_code: &str) -> Result<()> {
        let mut generated = vec![];
        command.code_gen(&mut generated)?;
        assert_eq!(str::from_utf8(&generated)?, expected_code);
        Ok(())
    }

    // todo: figure out how this generalizes to Lt, Gt.
    #[test]
    fn eq() -> Result<()> {
        let command = Command::Eq;

        // todo: come back to this, and figure out what to do about labels
        // * option 1: generate unique labels using the filename and a seq-number
        // * option 2: keep track of the current instruction address during codegen,
        //   and emit those as jump targets
        let code = "\
// *** eq ***

// SP--
@SP
M=M-1

// D := *SP
A=M
D=M

// SP--
@SP
M=M-1

// D := *SP - D
A=M
D=M-D

// jump to 'true' case
@TRUExxx
D;JEQ

// *SP := false
@SP
M=0

// jump to end of comparison
@ENDxxx
0;JMP

(TRUExxx)

// *SP  := true
@SP
M=-1

(ENDxxx)

// SP++
@SP
M=M+1



";

        assert_code_gen(command, code)
    }
}
