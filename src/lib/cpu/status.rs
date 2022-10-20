mod flag;
#[cfg(test)]
mod unit_tests;

use core::fmt::{Binary, Debug};

pub use flag::StatusFlag;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Status(u8);

// `const` impl not derivable at the time of writing
#[allow(clippy::derivable_impls)]
impl const Default for Status {
    fn default() -> Self { Self(u8::default()) }
}

#[allow(clippy::inline_always)]
impl Status {
    #[must_use]
    pub const fn new() -> Self { Self::default() }

    #[inline(always)]
    #[must_use]
    pub const fn b(self) -> bool { (self.0 & u8::from(StatusFlag::B)) != 0 }

    #[inline(always)]
    #[must_use]
    pub const fn c(self) -> bool { (self.0 & u8::from(StatusFlag::C)) != 0 }

    #[inline(always)]
    pub fn clear_b(&mut self) -> &mut Self {
        self.0 &= !u8::from(StatusFlag::B);
        self
    }

    #[inline(always)]
    pub fn clear_c(&mut self) -> &mut Self {
        self.0 &= !u8::from(StatusFlag::C);
        self
    }

    #[inline(always)]
    pub fn clear_d(&mut self) -> &mut Self {
        self.0 &= !u8::from(StatusFlag::D);
        self
    }

    #[inline(always)]
    pub fn clear_i(&mut self) -> &mut Self {
        self.0 &= !u8::from(StatusFlag::I);
        self
    }

    #[inline(always)]
    pub fn clear_n(&mut self) -> &mut Self {
        self.0 &= !u8::from(StatusFlag::N);
        self
    }

    #[inline(always)]
    pub fn clear_v(&mut self) -> &mut Self {
        self.0 &= !u8::from(StatusFlag::V);
        self
    }

    #[inline(always)]
    pub fn clear_z(&mut self) -> &mut Self {
        self.0 &= !u8::from(StatusFlag::Z);
        self
    }

    #[inline(always)]
    #[must_use]
    pub const fn d(self) -> bool { (self.0 & u8::from(StatusFlag::D)) != 0 }

    #[inline(always)]
    #[must_use]
    pub const fn i(self) -> bool { (self.0 & u8::from(StatusFlag::I)) != 0 }

    #[inline(always)]
    #[must_use]
    pub const fn n(self) -> bool { (self.0 & u8::from(StatusFlag::N)) != 0 }

    #[inline(always)]
    pub fn set_b(&mut self) -> &mut Self {
        self.0 |= u8::from(StatusFlag::B);
        self
    }

    #[inline(always)]
    pub fn set_c(&mut self) -> &mut Self {
        self.0 |= u8::from(StatusFlag::C);
        self
    }

    #[inline(always)]
    pub fn set_d(&mut self) -> &mut Self {
        self.0 |= u8::from(StatusFlag::D);
        self
    }

    #[inline(always)]
    pub fn set_i(&mut self) -> &mut Self {
        self.0 |= u8::from(StatusFlag::I);
        self
    }

    #[inline(always)]
    pub fn set_n(&mut self) -> &mut Self {
        self.0 |= u8::from(StatusFlag::N);
        self
    }

    #[inline(always)]
    pub fn set_v(&mut self) -> &mut Self {
        self.0 |= u8::from(StatusFlag::V);
        self
    }

    #[inline(always)]
    pub fn set_z(&mut self) -> &mut Self {
        self.0 |= u8::from(StatusFlag::Z);
        self
    }

    #[inline(always)]
    #[must_use]
    pub const fn v(self) -> bool { (self.0 & u8::from(StatusFlag::V)) != 0 }

    #[inline(always)]
    #[must_use]
    pub const fn value(self) -> u8 { self.0 }

    #[inline(always)]
    #[must_use]
    pub const fn z(self) -> bool { (self.0 & u8::from(StatusFlag::Z)) != 0 }
}

impl Binary for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:b}", self.0)
    }
}

impl Debug for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(NV-B_DIZC) {:04b}_{:04b}/{:#04x}", self.0 >> 4, self.0 & 0x0f, self.0)
    }
}