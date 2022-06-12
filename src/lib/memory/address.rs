#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Address(u16);

impl Address {

    pub const fn as_u16(self) -> u16 { self.0 }

    #[allow(clippy::integer_arithmetic)]
    pub fn inc(&mut self) -> &mut Self {
        self.0 = self.0.wrapping_add(1);
        self
    }
}

impl const From<u16> for Address {
    fn from(value: u16) -> Self {
        Self(value)
    }
}
