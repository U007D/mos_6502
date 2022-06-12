mod u8_opcode_ext;

pub use u8_opcode_ext::U8OpcodeExt;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Opcode {
    LdaImm = 0xa9,
    LdaZp  = 0xa5,
}

impl const From<Opcode> for u8 {
    fn from(opcode: Opcode) -> Self { opcode as Self }
}
