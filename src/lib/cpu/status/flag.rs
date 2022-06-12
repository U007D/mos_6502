#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum StatusFlag {
    C = 0x01,
    Z = 0x02,
    I = 0x04,
    D = 0x08,
    B = 0x10,
    V = 0x20,
    N = 0x40,
}

impl const From<StatusFlag> for u8 {
    fn from(flag: StatusFlag) -> Self { flag as Self }
}
