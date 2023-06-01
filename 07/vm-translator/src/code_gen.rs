use std::io::Write;

use anyhow::Result;

use crate::vm_command::{Command, Segment, VirtualMemoryAddr};

impl Command {
    pub fn code_gen(self, mut out: impl Write) -> Result<()> {
        write!(out, "// *** {self:?} ***\n\n")?;

        match self {
            Self::Push { source } => {
                let VirtualMemoryAddr { segment, index } = source;

                if matches!(segment, Segment::Constant) {
                    write!(
                        out,
                        "// D := {index}\n\
                        @{index}\n\
                        D=A\n\
                        \n"
                    )?;
                } else {
                    let base = match segment {
                        Segment::Constant => unreachable!(),
                        Segment::Local => "LCL",
                        Segment::Argument => "ARG",
                        Segment::This => "THIS",
                        Segment::That => "THAT",
                        Segment::Temp => todo!(),
                        Segment::Pointer => todo!(),
                        Segment::Static => todo!(),
                    };

                    write!(
                        out,
                        "// D := *({base} + {index})\n\
                        @{base}\n\
                        D=M\n\
                        @{index}\n\
                        A=D+A\n\
                        D=M\n\
                        \n"
                    )?;
                }

                write!(
                    out,
                    "// *SP := D\n\
                    @SP\n\
                    A=M\n\
                    M=D\n\
                    \n\
                    // SP++\n\
                    @SP\n\
                    M=M+1\n"
                )?;
            }

            Self::Pop { dest } => {
                let VirtualMemoryAddr { segment, index } = dest;

                let base = match segment {
                    Segment::Constant => panic!("can't write to constant segment"),
                    Segment::Local => "LCL",
                    Segment::Argument => "ARG",
                    Segment::This => "THIS",
                    Segment::That => "THAT",
                    Segment::Temp => todo!(),
                    Segment::Pointer => todo!(),
                    Segment::Static => todo!(),
                };

                write!(
                    out,
                    "// SP--\n\
                    @SP\n\
                    M=M-1\n\
                    \n\
                    // R13 := {base} + {index}\n\
                    @{base}\n\
                    D=M\n\
                    @{index}\n\
                    D=D+A\n\
                    @R13\n\
                    M=D\n\
                    \n\
                    // *R13 := *SP\n\
                    @SP\n\
                    A=M\n\
                    D=M\n\
                    @R13\n\
                    A=M\n\
                    M=D\n"
                )?;
            }

            Self::Add => {
                write!(
                    out,
                    "// SP--\n\
                    @SP\n\
                    M=M-1\n\
                    \n\
                    // D := *SP\n\
                    A=M\n\
                    D=M\n\
                    \n\
                    // SP--\n\
                    @SP\n\
                    M=M-1\n\
                    \n\
                    // *SP += D\n\
                    A=M\n\
                    M=M+D\n\
                    \n\
                    // SP++\n\
                    @SP\n\
                    M=M+1\n"
                )?;
            }

            Self::Sub => todo!(),
            Self::Neg => todo!(),
            Self::Eq => todo!(),
            Self::Gt => todo!(),
            Self::Lt => todo!(),
            Self::And => todo!(),
            Self::Or => todo!(),
            Self::Not => todo!(),
        }

        write!(out, "\n\n\n")
    }
}

#[cfg(test)]
mod tests {
    use std::str;

    use super::*;

    #[test]
    fn push_argument_5() -> Result<()> {
        let expected = "\
// *** push argument 5 ***

// D := *(ARG + 5)
@ARG
D=M
@5
A=D+A
D=M

// *SP := D
@SP
A=M
M=D

// SP++
@SP
M=M+1



";

        let command = Command::Push {
            source: VirtualMemoryAddr {
                segment: Segment::Argument,
                index: 5,
            },
        };

        let mut actual = vec![];
        command.code_gen(&mut actual)?;

        assert_eq!(expected, str::from_utf8(&actual)?);
        Ok(())
    }

    #[test]
    fn pop_argument_5() -> Result<()> {
        let expected = "\
// *** pop argument 5 ***

// SP--
@SP
M=M-1

// R13 := ARG + 5
@ARG
D=M
@5
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



";

        let command = Command::Pop {
            dest: VirtualMemoryAddr {
                segment: Segment::Argument,
                index: 5,
            },
        };

        let mut actual = vec![];
        command.code_gen(&mut actual)?;

        assert_eq!(expected, str::from_utf8(&actual)?);
        Ok(())
    }

    #[test]
    fn add() -> Result<()> {
        let expected = "\
// *** add ***

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



";

        let command = Command::Add;

        let mut actual = vec![];
        command.code_gen(&mut actual)?;

        assert_eq!(expected, str::from_utf8(&actual)?);
        Ok(())
    }
}
