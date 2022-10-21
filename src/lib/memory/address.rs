use std::{fmt::Debug, ops::Deref};
use crate::traits::IAddress;

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Address(u16);

impl Address {
    #[allow(clippy::integer_arithmetic)]
    pub fn inc(&mut self) -> &mut Self {
        self.0 = self.0.wrapping_add(1);
        self
    }
}

impl Debug for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Address({:#06x}/{})", self.0, self.0)
    }
}

// `const` trait impl not derivable at time of writing
#[allow(clippy::derivable_impls)]
impl const Default for Address {
    fn default() -> Self { Self(u16::default()) }
}

impl Deref for Address {
    type Target = u16;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl const From<u16> for Address {
    fn from(value: u16) -> Self { Self(value) }
}

impl IAddress for Address {}
