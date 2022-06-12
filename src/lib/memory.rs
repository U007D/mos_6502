mod address;
#[cfg(test)]
mod unit_tests;
mod zero_page_address;

use std::ops::{Index, IndexMut};

pub use address::Address;
pub use zero_page_address::ZeroPageAddress;

const MAX_6502_MEMORY_CAPACITY: usize = 0x1 << u16::BITS; // 65536 bytes

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Memory(Box<[u8]>);

impl Memory {
    #[must_use]
    pub fn new(capacity: usize) -> Option<Self> {
        (capacity <= MAX_6502_MEMORY_CAPACITY).then(|| Self(vec![0; capacity].into_boxed_slice()))
    }

    pub fn clear(&mut self) -> &mut Self {
        self.0.fill(0);
        self
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool { self.capacity() == 0 }

    #[must_use]
    pub const fn capacity(&self) -> usize { self.0.len() }
}

impl Default for Memory {
    fn default() -> Self { Self(vec![0_u8; MAX_6502_MEMORY_CAPACITY].into_boxed_slice()) }
}

impl Index<Address> for Memory {
    type Output = u8;

    fn index(&self, index: Address) -> &Self::Output {
        let address = usize::from(*index);
        self.0.get(address).unwrap_or_else(|| {
            panic!("Fault: Attempt to read invalid memory address.  No memory present at this location: {:x}", address)
        })
    }
}

impl Index<ZeroPageAddress> for Memory {
    type Output = u8;

    fn index(&self, index: ZeroPageAddress) -> &Self::Output {
        let address = usize::from(*index);
        self.0.get(address).unwrap_or_else(|| {
            panic!(
                "Fault: Attempt to read invalid zero page memory address.  No memory present at this location: {:x}",
                address
            )
        })
    }
}

impl IndexMut<Address> for Memory {
    fn index_mut(&mut self, index: Address) -> &mut Self::Output {
        let address = usize::from(*index);
        self.0.get_mut(address).unwrap_or_else(|| {
            panic!("Fault: Attempt to write invalid memory address.  No memory present at this location: {:x}", address)
        })
    }
}

impl IndexMut<ZeroPageAddress> for Memory {
    fn index_mut(&mut self, index: ZeroPageAddress) -> &mut Self::Output {
        let address = usize::from(*index);
        self.0.get_mut(address).unwrap_or_else(|| {
            panic!(
                "Fault: Attempt to write invalid zero page memory address.  No memory present at this location: {:x}",
                address
            )
        })
    }
}
