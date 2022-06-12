use crate::memory::Address;

#[allow(clippy::unwrap_used)]
const BASE: Address = Address::from(0x100);

#[repr(u8)]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Stack {
    Start = u8::MAX,
    End   = u8::MIN,
}

impl From<Stack> for u8 {
    fn from(offset: Stack) -> Self { offset as Self }
}

impl From<Stack> for u16 {
    fn from(offset: Stack) -> Self { Self::from(u8::from(offset)) }
}
