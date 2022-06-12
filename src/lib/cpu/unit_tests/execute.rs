use super::*;
use crate::{error::execution::Error as ExecutionError, Memory};

#[test]
fn executing_invalid_instruction_returns_error() {
    // Given
    // Source: https://sites.google.com/site/6502asembly/6502-instruction-set/op-codes-table
    let invalid_instruction = 0xff;
    let expected_res = ExecutionError::InstructionDecode(invalid_instruction);
    let capacity = 1;
    let cycle_budget = 1;
    let memory = {
        let mut tmp = Memory::new(capacity).unwrap();
        tmp[Address::from(0)] = invalid_instruction;
        tmp
    };
    let mut cpu = Cpu::new(memory);

    // When
    let result = cpu.execute(cycle_budget, Address::from(0));
    
    // Then
    assert!(result.unwrap_err() == expected_res);
}
