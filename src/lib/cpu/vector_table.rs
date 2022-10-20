use crate::memory::Address;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u16)]
pub enum VectorTable {
    Reset = 0xfffc,
}

impl const From<VectorTable> for u16 {
    fn from(vector: VectorTable) -> Self { vector as Self }
}

impl const From<VectorTable> for Address {
    fn from(vector: VectorTable) -> Self { Self::from(u16::from(vector)) }
}
