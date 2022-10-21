use std::ops::{Deref, DerefMut};

use crate::traits::IRegister;

#[derive(Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Y(u8);

impl const Deref for Y {
    type Target = <Self as IRegister>::Target;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Y {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IRegister for Y {
    type Target = u8;
}