use std::collections::HashMap;

use once_cell::sync::Lazy;

use super::*;
use crate::{
    error::execution::{Error, Result},
    Sealed,
};

static U8_TO_OPCODE_LOOKUP: Lazy<HashMap<u8, Opcode>> = Lazy::new(|| {
    let mut lut = HashMap::new();
    lut.insert(Opcode::LdaImmediate as u8, Opcode::LdaImmediate);
    lut
});

pub trait U8OpcodeExt: Sealed
where
    Self: Copy, {
    fn to_opcode(self) -> Result<Opcode>;
}

impl U8OpcodeExt for u8 {
    fn to_opcode(self) -> Result<Opcode> {
        U8_TO_OPCODE_LOOKUP.get(&self).copied().ok_or(Error::InstructionDecode(self))
    }
}

impl Sealed for u8 {}
