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
    assert!(result.status().is_b() == expected_status);
    assert!(result.status().is_c() == expected_status);
    assert!(result.status().is_d() == expected_status);
    assert!(result.status().is_i() == expected_status);
    assert!(result.status().is_n() == expected_status);
    assert!(result.status().is_v() == expected_status);
    assert!(result.status().is_z() == expected_status);
}
