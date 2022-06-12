use super::*;
use crate::catch_unwind_silent;
use assert2::assert;

#[test]
fn write_valid_memory_locations_returns_expected_value() {
    // Given
    let capacity = 3000_usize;
    let new_value = 42;
    let mut memory = Memory::new(capacity).unwrap();

    // When
    (0..capacity).for_each(|index| memory[Address::from(u16::try_from(index).unwrap())] = new_value);

    // Then
    assert!((0..capacity).all(|index| memory[Address::from(u16::try_from(index).unwrap())] == new_value));
}

#[test]
fn write_invalid_memory_location_panics() {
    // Given
    let capacity = 300_usize;
    let new_value = 99;
    // Only [0..=299] are valid addresses
    let invalid_address = Address::from(u16::try_from(capacity).unwrap());

    // When
    let result = catch_unwind_silent(|| {
        // E0277
        let mut memory = Memory::new(capacity).unwrap();
        memory[invalid_address] = new_value;
    });

    // Then
    assert!(result.is_err());
}

#[test]
fn write_valid_zero_page_memory_locations_returns_expected_value() {
    // Given
    let capacity = 200_usize;
    let new_value = 42;
    let mut memory = Memory::new(capacity).unwrap();

    // When
    (0..capacity).for_each(|index| memory[ZeroPageAddress::from(u8::try_from(index).unwrap())] = new_value);

    // Then
    assert!((0..capacity).all(|index| memory[ZeroPageAddress::from(u8::try_from(index).unwrap())] == new_value));
}

#[test]
fn write_zero_page_invalid_memory_location_panics() {
    // Given
    let capacity = 200_usize;
    let new_value = 99;
    // Only [0..=199] are valid addresses
    let invalid_address = ZeroPageAddress::from(u8::try_from(capacity).unwrap());

    // When
    let result = catch_unwind_silent(|| {
        // E0277
        let mut memory = Memory::new(capacity).unwrap();
        memory[invalid_address] = new_value;
    });

    // Then
    assert!(result.is_err());
}
