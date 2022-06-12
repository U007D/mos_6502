use assert2::assert;

use super::*;
use crate::{error::execution::Error as ExecutionError, Memory};

#[test]
fn invalid_instruction_returns_error() {
    // Given
    // Source: https://sites.google.com/site/6502asembly/6502-instruction-set/op-codes-table
    let invalid_instruction = 0xff;
    let expected_res = Err(ExecutionError::InstructionDecode(invalid_instruction));
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
    assert!(result == expected_res);
}

#[test]
fn fetch_lda_imm_instruction_sets_expected_mode() {
    // Given
    // Source: https://web.archive.org/web/20150726112225/http://www.obelisk.demon.co.uk/6502/reference.html#LDA
    let expected_res = Ok(Mode::AFetchImmediateOperand);
    let capacity = 1;
    let cycle_budget = 1;
    let memory = {
        let mut tmp = Memory::new(capacity).unwrap();
        tmp[Address::from(0)] = Opcode::LdaImm.into();
        tmp
    };
    let mut cpu = Cpu::new(memory);

    // When
    let result = cpu.execute(cycle_budget, Address::from(0));

    // Then
    assert!(result == expected_res);
}

#[test]
fn fetch_lda_zp_instruction_sets_expected_mode() {
    // Given
    // Source: https://web.archive.org/web/20150726112225/http://www.obelisk.demon.co.uk/6502/reference.html#LDA
    let capacity = 1;
    let expected_res = Ok(Mode::AFetchZeroPageOperand);
    let cycle_budget = 1;
    let memory = {
        let mut tmp = Memory::new(capacity).unwrap();
        tmp[Address::from(0)] = Opcode::LdaZp.into();
        tmp
    };
    let mut cpu = Cpu::new(memory);

    // When
    let result = cpu.execute(cycle_budget, Address::from(0));

    // Then
    assert!(result == expected_res);
}

#[test]
fn fetch_lda_zp_operand_reads_expected_zero_page_address_operand() {
    // Given
    // Source: https://web.archive.org/web/20150726112225/http://www.obelisk.demon.co.uk/6502/reference.html#LDA
    let capacity = 2;
    let data_addr = ZeroPageAddress::from(99);
    let expected_res = Ok(Mode::ADerefZeroPageAddr(data_addr));
    let cycle_budget = 2;
    let memory = {
        let mut tmp = Memory::new(capacity).unwrap();
        tmp[Address::from(0)] = Opcode::LdaZp.into();
        tmp[Address::from(1)] = *data_addr;
        tmp
    };
    let mut cpu = Cpu::new(memory);

    // When
    let result = cpu.execute(cycle_budget, Address::from(0));

    // Then
    assert!(result == expected_res);
}

#[test]
fn execute_lda_zp_instruction_reads_expected_data() {
    // Given
    // Source: https://web.archive.org/web/20150726112225/http://www.obelisk.demon.co.uk/6502/reference.html#LDA
    let capacity = 100;
    let data_addr = ZeroPageAddress::from(99);
    let expected_res = Ok(Mode::FetchInstruction);
    let expected_value = 42;
    let cycle_budget = 3;
    let memory = {
        let mut tmp = Memory::new(capacity).unwrap();
        tmp[Address::from(0)] = Opcode::LdaZp.into();
        tmp[Address::from(1)] = *data_addr;
        tmp[data_addr] = expected_value;
        tmp
    };
    let mut cpu = Cpu::new(memory);

    // When
    let result = cpu.execute(cycle_budget, Address::from(0));

    // Then
    assert!(result == expected_res);
    assert!(cpu.a() == expected_value);
}

