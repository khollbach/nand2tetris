use std::{
    collections::{hash_map::Entry, HashMap},
    iter::zip,
};

use anyhow::{bail, ensure, Result};

use crate::instruction::ADDRESS_LIMIT;

/// A mapping from symbols to the memory addresses they correspond to.
pub struct SymbolTable {
    mapping: HashMap<String, u16>,

    /// How many distict _variables_ have been assigned?
    ///
    /// This doesn't count predefined symbols, or labels.
    num_variables: u16,
}

impl SymbolTable {
    /// Create a new symbol table, including all pre-defined symbols.
    pub fn new() -> Self {
        let registers = (0..16).map(|i| (format!("R{i}"), i));
        let aliases = zip(["SP", "LCL", "ARG", "THIS", "THAT"].map(String::from), 0..);
        let hardware = [
            ("SCREEN".into(), 0b_0100_0000_0000_0000),
            ("KBD".into(), 0b_0110_0000_0000_0000),
        ];

        Self {
            mapping: registers.chain(aliases).chain(hardware).collect(),
            num_variables: 0,
        }
    }

    pub fn lookup_symbol(&self, symbol: &str) -> Option<u16> {
        self.mapping.get(symbol).copied()
    }

    /// Variables are assigned increasing memory addresses, starting from 16.
    pub fn new_variable(&mut self, symbol: String) -> Result<u16> {
        let address = 16 + self.num_variables;

        ensure!(
            address < ADDRESS_LIMIT,
            "can't allocate more than {ADDRESS_LIMIT} variables"
        );

        self.try_insert(symbol, address)?;

        self.num_variables += 1;
        Ok(address)
    }

    pub fn new_label(&mut self, symbol: String, instruction_offset: u16) -> Result<()> {
        self.try_insert(symbol, instruction_offset)
    }

    /// Fails if the symbol already exists.
    fn try_insert(&mut self, symbol: String, value: u16) -> Result<()> {
        // The `Entry` API lets us avoid cloning `symbol` in the happy path.
        match self.mapping.entry(symbol) {
            Entry::Vacant(e) => {
                e.insert(value);
            }
            Entry::Occupied(e) => {
                let symbol = e.key();
                let prev_val = e.get();
                bail!("attempt to re-define symbol {symbol:?}. previous value: {prev_val}, new value: {value}");
            }
        }

        Ok(())
    }
}
