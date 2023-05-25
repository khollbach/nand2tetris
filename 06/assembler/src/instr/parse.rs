use anyhow::{bail, ensure, Context, Result};

use super::{AInstr, CInstr, Comp, Dest, Instr, InstrInner, Jump};

impl Instr {
    pub fn parse(line: &str) -> Result<Self> {
        let line = line.trim();

        let inner = if line.starts_with('@') {
            InstrInner::AInstr(AInstr::parse(line)?)
        } else {
            InstrInner::CInstr(CInstr::parse(line)?)
        };

        Ok(Instr { inner })
    }
}

impl AInstr {
    /// `line` must already be trimmed.
    fn parse(line: &str) -> Result<Self> {
        debug_assert_eq!(line, line.trim());
        ensure!(
            line.starts_with('@'),
            "A-instruction must start with '@': {line:?}"
        );

        let value: u16 = line[1..]
            .parse()
            .with_context(|| format!("failed to parse A-instruction as u16: {line:?}"))?;

        let limit = 2u16.pow(15);
        ensure!(
            value < limit,
            "A-instruction value must be less than limit: {value} vs {limit}"
        );

        Ok(AInstr { value })
    }
}

impl CInstr {
    /// `line` must already be trimmed.
    fn parse(mut line: &str) -> Result<Self> {
        debug_assert_eq!(line, line.trim());

        let dest = Dest::parse(&mut line)?;
        let jump = Jump::parse(&mut line)?;
        let comp = Comp::parse(&line)?;

        Ok(CInstr { dest, comp, jump })
    }
}

impl Dest {
    /// `line` must already be trimmed.
    ///
    /// Consume the `dest=` prefix of `line`, and parse it into a `Dest`.
    ///
    /// If the line doesn't start with `dest=`, return `Dest::default()`.
    fn parse(line: &mut &str) -> Result<Self> {
        debug_assert_eq!(*line, line.trim());

        let Some((dest, rest)) = line.split_once('=') else {
            return Ok(Dest::default());
        };

        ensure!(!dest.is_empty(), "empty dest field in line {line:?}");
        for c in dest.chars() {
            if !"ADM".contains(c) {
                bail!("invalid dest char {c:?} in line {line:?}");
            }
        }
        ensure!(
            dest.len() <= 3,
            "repeated char in dest field {dest:?}. line: {line:?}"
        );

        let a = dest.contains('A');
        let d = dest.contains('D');
        let m = dest.contains('M');

        *line = rest;
        Ok(Dest { a, d, m })
    }
}

impl Jump {
    /// `line` must already be trimmed.
    ///
    /// Consume the `;jump` suffix of `line`, and parse it into a `Jump`.
    ///
    /// If the line doesn't end with `;jump`, return `Jump::Never`.
    fn parse(line: &mut &str) -> Result<Self> {
        debug_assert_eq!(*line, line.trim());

        let Some((rest, jump)) = line.split_once(';') else {
            return Ok(Jump::Never);
        };

        let jump = match jump {
            "JGT" => Jump::Greater,
            "JEQ" => Jump::Equal,
            "JGE" => Jump::GreaterEqual,
            "JLT" => Jump::Less,
            "JNE" => Jump::NotEqual,
            "JLE" => Jump::LessEqual,
            "JMP" => Jump::Always,
            _ => bail!("jump must be one of {{JGT, JEQ, JGE, JLT, JNE, JLE, JMP}}; got: {jump:?}"),
        };

        *line = rest;
        Ok(jump)
    }
}

impl Comp {
    /// Parse `expr` as a `comp` field.
    ///
    /// You must first strip the optional `dest=` and `;jump` before calling
    /// this function.
    fn parse(expr: &str) -> Result<Self> {
        let (a_bit, c_bits) = match expr {
            "0" => (0, [1, 0, 1, 0, 1, 0]),
            "1" => (0, [1, 1, 1, 1, 1, 1]),
            "-1" => (0, [1, 1, 1, 0, 1, 0]),

            "D" => (0, [0, 0, 1, 1, 0, 0]),
            "A" => (0, [1, 1, 0, 0, 0, 0]),
            "M" => (1, [1, 1, 0, 0, 0, 0]),
            "!D" => (0, [0, 0, 1, 1, 0, 1]),
            "!A" => (0, [1, 1, 0, 0, 0, 1]),
            "!M" => (1, [1, 1, 0, 0, 0, 1]),
            "-D" => (0, [0, 0, 1, 1, 1, 1]),
            "-A" => (0, [1, 1, 0, 0, 1, 1]),
            "-M" => (1, [1, 1, 0, 0, 1, 1]),

            "D+1" => (0, [0, 1, 1, 1, 1, 1]),
            "A+1" => (0, [1, 1, 0, 1, 1, 1]),
            "M+1" => (1, [1, 1, 0, 1, 1, 1]),
            "D-1" => (0, [0, 0, 1, 1, 1, 0]),
            "A-1" => (0, [1, 1, 0, 0, 1, 0]),
            "M-1" => (1, [1, 1, 0, 0, 1, 0]),

            "D+A" => (0, [0, 0, 0, 0, 1, 0]),
            "D+M" => (1, [0, 0, 0, 0, 1, 0]),
            "D-A" => (0, [0, 1, 0, 0, 1, 1]),
            "D-M" => (1, [0, 1, 0, 0, 1, 1]),
            "A-D" => (0, [0, 0, 0, 1, 1, 1]),
            "M-D" => (1, [0, 0, 0, 1, 1, 1]),

            "D&A" => (0, [0, 0, 0, 0, 0, 0]),
            "D&M" => (1, [0, 0, 0, 0, 0, 0]),
            "D|A" => (0, [0, 1, 0, 1, 0, 1]),
            "D|M" => (1, [0, 1, 0, 1, 0, 1]),

            _ => bail!("unrecognized comp expresion {expr:?} (note: arguments *must* appear in alphabetical order)"),
        };

        let a_bit = a_bit != 0;
        let c_bits = c_bits.map(|bit| bit != 0);

        Ok(Comp { a_bit, c_bits })
    }
}
