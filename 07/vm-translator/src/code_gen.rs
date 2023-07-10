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

            // todo: reduce code dup b/w Add and Sub
            Self::Sub => {
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
                    // *SP -= D\n\
                    A=M\n\
                    M=M-D\n\
                    \n\
                    // SP++\n\
                    @SP\n\
                    M=M+1\n"
                )?;
            }

            Self::Neg => {
                write!(
                    out,
                    "// SP--\n\
                    @SP\n\
                    M=M-1\n\
                    \n\
                    // *SP := -*SP\n\
                    A=M\n\
                    M=-M\n\
                    \n\
                    // SP++\n\
                    @SP\n\
                    M=M+1\n"
                )?;
            }

            Self::Eq => todo!(),
            Self::Gt => todo!(),
            Self::Lt => todo!(),

            // todo: reduce code dup b/w these (And/Or/Not) and Add/Sub/Neg
            Self::And => {
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
                    // *SP &= D\n\
                    A=M\n\
                    M=M&D\n\
                    \n\
                    // SP++\n\
                    @SP\n\
                    M=M+1\n"
                )?;
            }

            Self::Or => {
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
                    // *SP |= D\n\
                    A=M\n\
                    M=M|D\n\
                    \n\
                    // SP++\n\
                    @SP\n\
                    M=M+1\n"
                )?;
            }

            Self::Not => {
                write!(
                    out,
                    "// SP--\n\
                    @SP\n\
                    M=M-1\n\
                    \n\
                    // *SP := !*SP\n\
                    A=M\n\
                    M=!M\n\
                    \n\
                    // SP++\n\
                    @SP\n\
                    M=M+1\n"
                )?;
            }
        }

        write!(out, "\n\n\n")?;
        Ok(())
    }
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

    #[test]
    fn push_argument_5() -> Result<()> {
        let command = Command::Push {
            source: VirtualMemoryAddr {
                segment: Segment::Argument,
                index: 5,
            },
        };

        let code = "\
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

        assert_code_gen(command, code)
    }

    #[test]
    fn pop_argument_5() -> Result<()> {
        let command = Command::Pop {
            dest: VirtualMemoryAddr {
                segment: Segment::Argument,
                index: 5,
            },
        };

        let code = "\
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

        assert_code_gen(command, code)
    }

    #[test]
    fn add() -> Result<()> {
        let command = Command::Add;

        let code = "\
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

        assert_code_gen(command, code)
    }

    #[test]
    fn sub() -> Result<()> {
        let command = Command::Sub;

        let code = "\
// *** sub ***

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



";

        assert_code_gen(command, code)
    }

    #[test]
    fn neg() -> Result<()> {
        let command = Command::Neg;

        let code = "\
// *** neg ***

// SP--
@SP
M=M-1

// *SP := -*SP
A=M
M=-M

// SP++
@SP
M=M+1



";

        assert_code_gen(command, code)
    }

    #[test]
    fn and() -> Result<()> {
        let command = Command::And;

        let code = "\
// *** and ***

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



";

        assert_code_gen(command, code)
    }

    #[test]
    fn or() -> Result<()> {
        let command = Command::Or;

        let code = "\
// *** or ***

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



";

        assert_code_gen(command, code)
    }

    #[test]
    fn not() -> Result<()> {
        let command = Command::Not;

        let code = "\
// *** not ***

// SP--
@SP
M=M-1

// *SP := !*SP
A=M
M=!M

// SP++
@SP
M=M+1



";

        assert_code_gen(command, code)
    }
}
