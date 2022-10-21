use assert2::assert;

use super::*;

#[test]
fn default_initialized_register_returns_expected_value() {
    // Given
    let expected = u8::default();
    let capacity = 1;
    let memory = Memory::new(capacity).unwrap();
    let cpu = Cpu::new(memory);

    // When
    let result = cpu.a();

    // Then
    assert!(result == expected);
}

#[test]
fn modified_register_returns_expected_value_in_expected_register_only() {
    // Given
    let expected_value_x = 42;
    let expected_value_ay = u8::default();
    let capacity = 2;
    let cycle_budget = 2.into();
    let memory = {
        let mut tmp = Memory::new(capacity).unwrap();
        // LDA #42
        tmp[Address::from(0)] = Opcode::LdxImm.into();
        tmp[Address::from(1)] = expected_value_x;
        tmp
    };
    let mut cpu = Cpu::new(memory);
    cpu.execute(Address::from(0), cycle_budget).unwrap();

    // When
    let (result_a, result_x, result_y) = (cpu.a(), cpu.x(), cpu.y());

    // Then
    assert!(result_x == expected_value_x);
    assert!(result_a == expected_value_ay);
    assert!(result_y == expected_value_ay);
}
