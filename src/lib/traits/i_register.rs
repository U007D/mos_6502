use std::ops::{Deref, DerefMut};

pub trait IRegister: Deref + DerefMut {
    type Target;
}