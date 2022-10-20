use assert2::assert;

use super::*;

#[test]
fn default_value_is_clear() {
    // Given
    let expected = false;
    let status_register = Status::new();

    // When
    let result = status_register.b();

    // Then
    assert!(result == expected);
}

#[test]
fn set_flag_returns_expected_value() {
    // Given
    let expected = true;
    let mut status_register = Status::new();

    // When
    let _result = status_register.set_b();

    // Then
    assert!(status_register.b() == expected);
}

#[test]
fn clear_flag_returns_expected_value() {
    // Given
    let expected = false;
    let mut status_register = Status::new();
    status_register.set_b().set_c().set_d().set_i().set_n().set_v().set_z();

    // When
    let _result = status_register.clear_b();

    // Then
    assert!(status_register.b() == expected);
}
