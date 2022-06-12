mod flag;
#[cfg(test)]
mod unit_tests;

pub use flag::StatusFlag;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Status(u8);

#[allow(clippy::inline_always)]
impl Status {
    pub fn new() -> Self { Self::default() }

    #[inline(always)]
    pub fn b(self) -> bool { (self.0 & u8::from(StatusFlag::B)) != 0 }

    #[inline(always)]
    pub fn c(self) -> bool { (self.0 & u8::from(StatusFlag::C)) != 0 }

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
    pub fn d(self) -> bool { (self.0 & u8::from(StatusFlag::D)) != 0 }

    #[inline(always)]
    pub fn i(self) -> bool { (self.0 & u8::from(StatusFlag::I)) != 0 }

    #[inline(always)]
    pub fn n(self) -> bool { (self.0 & u8::from(StatusFlag::N)) != 0 }

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
    pub fn v(self) -> bool { (self.0 & u8::from(StatusFlag::V)) != 0 }

    #[must_use]
    pub const fn value(self) -> u8 { self.0 }

    #[inline(always)]
    pub fn z(self) -> bool { (self.0 & u8::from(StatusFlag::Z)) != 0 }
}
