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
fn modified_register_returns_expected_value() {
    // Given
    let expected_value = 42;
    let capacity = 2;
    let cycle_budget = 2;
    let memory = {
        let mut tmp = Memory::new(capacity).unwrap();
        // LDA #42
        tmp[Address::from(0)] = Opcode::LdaImm.into();
        tmp[Address::from(1)] = expected_value;
        tmp
    };
    let mut cpu = Cpu::new(memory);
    cpu.execute(cycle_budget, 0.into()).unwrap();

    // When
    let result = cpu.a();

    // Then
    assert!(result == expected_value);
}
