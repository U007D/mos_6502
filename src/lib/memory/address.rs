use std::ops::Deref;
use crate::IAddress;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Address(u16);

impl Address {
    #[allow(clippy::integer_arithmetic)]
    pub fn inc(&mut self) -> &mut Self {
        self.0 = self.0.wrapping_add(1);
        self
    }
}

impl Deref for Address {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl const From<u16> for Address {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl IAddress for Address {}
