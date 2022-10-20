use super::*;
use crate::Memory;

#[test]
fn all_status_register_fields_of_newly_constructed_cpu_are_zero() {
    // Given
    let memory = Memory::new(0).unwrap();
    let expected_status = false;
    let new = Cpu::new;

    // When
    let result = new(memory);

    // Then
    assert!(result.status().b() == expected_status);
    assert!(result.status().c() == expected_status);
    assert!(result.status().d() == expected_status);
    assert!(result.status().i() == expected_status);
    assert!(result.status().n() == expected_status);
    assert!(result.status().v() == expected_status);
    assert!(result.status().z() == expected_status);
}
