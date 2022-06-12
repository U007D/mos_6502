use super::*;
use crate::{error::execution::Error as ExecutionError, Memory};

#[test]
fn executing_zero_instruction_returns_error() {
    // Given
    let expected_res = ExecutionError::InstructionDecode(u8::default());
    let memory_capacity = 1;
    let cycle_budget = 1;
    let mut cpu = Cpu::new(Memory::new(memory_capacity).unwrap());

    // When
    let result = cpu.execute(cycle_budget, Address::from(0));
    
    // Then
    assert!(result.unwrap_err() == expected_res);
}
