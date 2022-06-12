mod u8_opcode_ext;

pub use u8_opcode_ext::U8OpcodeExt;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Opcode {
    LdaImmediate = 0xa9,
}
