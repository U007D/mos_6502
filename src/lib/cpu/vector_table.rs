use crate::memory::Address;

#[repr(u16)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum VectorTable {
    Reset = 0xfffc,
}

impl const From<VectorTable> for u16 {
    fn from(vector: VectorTable) -> Self {
        vector as Self
    }
}

impl const From<VectorTable> for Address {
    fn from(vector: VectorTable) -> Self { Self::from(u16::from(vector)) }
}
