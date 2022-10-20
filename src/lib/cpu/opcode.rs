use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::error::execution::{self, Error, Result};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u8)]
pub enum Opcode {
    LdaImm = 0xa9,
    LdaZp  = 0xa5,
}

impl const From<Opcode> for u8 {
    fn from(opcode: Opcode) -> Self { opcode as Self }
}

impl TryFrom<u8> for Opcode {
    type Error = execution::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        U8_TO_OPCODE_LOOKUP.get(&value).copied().ok_or(Error::IllegalInstruction(value))
    }
}

static U8_TO_OPCODE_LOOKUP: Lazy<HashMap<u8, Opcode>> = Lazy::new(|| {
    let mut lut = HashMap::new();
    lut.insert(Opcode::LdaImm.into(), Opcode::LdaImm);
    lut.insert(Opcode::LdaZp.into(), Opcode::LdaZp);
    lut
});
