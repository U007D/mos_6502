use std::{fmt::Debug, ops::Deref};

use crate::traits::IAddress;

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ZeroPageAddress(u8);

impl ZeroPageAddress {
    #[allow(clippy::integer_arithmetic)]
    pub fn inc(&mut self) -> &mut Self {
        self.0 = self.0.wrapping_add(1);
        self
    }
}

impl Debug for ZeroPageAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ZeroPageAddress({:#04x}/{})", self.0, self.0)
    }
}

// `const` trait impl not derivable at time of writing
#[allow(clippy::derivable_impls)]
impl const Default for ZeroPageAddress {
    fn default() -> Self { Self(u8::default()) }
}

impl Deref for ZeroPageAddress {
    type Target = u8;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl const From<u8> for ZeroPageAddress {
    fn from(value: u8) -> Self { Self(value) }
}

impl IAddress for ZeroPageAddress {}
