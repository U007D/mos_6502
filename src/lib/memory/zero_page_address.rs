use std::ops::Deref;
use crate::IAddress;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ZeroPageAddress(u8);

impl ZeroPageAddress {
    #[allow(clippy::integer_arithmetic)]
    pub fn inc(&mut self) -> &mut Self {
        self.0 = self.0.wrapping_add(1);
        self
    }
}

impl Deref for ZeroPageAddress {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl const From<u8> for ZeroPageAddress {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl IAddress for ZeroPageAddress {}
