use std::ops::{Deref, DerefMut};

use crate::traits::IRegister;

#[derive(Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct X(u8);

impl const Deref for X {
    type Target = <Self as IRegister>::Target;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for X {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IRegister for X {
    type Target = u8;
}