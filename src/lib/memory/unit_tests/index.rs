use super::*;
use crate::catch_unwind_silent;
use assert2::assert;

#[test]
fn read_valid_unmodified_memory_locations_returns_default_value() {
    // Given
    let capacity = 1000_usize;
    let default_value = u8::default();
    let memory = Memory::new(capacity).unwrap();

    // When
    let result = (0..capacity)
        .all(|index| memory[Address::from(u16::try_from(index).unwrap())] == default_value);

    // Then
    assert!(result);
}

#[test]
fn read_invalid_memory_location_panics() {
    // Given
    let capacity = 100_usize;
    // Only [0..=99] are valid addresses
    let invalid_address = Address::from(100);
    let memory = Memory::new(capacity).unwrap();

    // When
    let result = catch_unwind_silent(|| {
        let _ = memory[invalid_address];
    });

    // Then
    assert!(result.is_err());
}

#[test]
fn read_valid_unmodified_zero_page_memory_locations_returns_default_value() {
    // Given
    let capacity = 100_usize;
    let default_value = u8::default();
    let memory = Memory::new(capacity).unwrap();

    // When
    let result = (0..capacity)
        .all(|index| memory[ZeroPageAddress::from(u8::try_from(index).unwrap())] == default_value);

    // Then
    assert!(result);
}

#[test]
fn read_invalid_zero_page_memory_location_panics() {
    // Given
    let capacity = 100_usize;
    // Only [0..=99] are valid addresses
    let invalid_address = ZeroPageAddress::from(100);
    let memory = Memory::new(capacity).unwrap();

    // When
    let result = catch_unwind_silent(|| {
        let _ = memory[invalid_address];
    });

    // Then
    assert!(result.is_err());
}
