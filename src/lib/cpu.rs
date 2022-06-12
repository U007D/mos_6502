#![allow(clippy::indexing_slicing)]
mod mode;
mod opcode;
mod stack;
mod status;
#[cfg(test)]
mod unit_tests;
mod vector_table;

use crate::{error::execution::Result as ExecutionResult, memory::Address, Memory};
pub use opcode::Opcode;
pub use stack::Stack;
pub use status::Status;
pub use vector_table::VectorTable;

use crate::{cpu::mode::Mode::Halt, memory::ZeroPageAddress};
use mode::Mode;
use opcode::U8OpcodeExt;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Cpu {
    memory: Memory,
    pc:     Address,
    sp:     u8,

    a: u8,
    x: u8,
    y: u8,

    status: Status,
}

impl Cpu {
    #[must_use]
    pub fn new(memory: Memory) -> Self {
        Self {
            memory,
            pc: Address::default(),
            sp: u8::default(),
            a: u8::default(),
            x: u8::default(),
            y: u8::default(),
            status: Status::default(),
        }
    }

    // TODO: Hide direct register access to ensure use of accessors to guarantee side-effectful register writes
    #[must_use]
    pub const fn a(&self) -> u8 { self.a }

    pub fn execute(&mut self, clock_cycles: usize, start: Address) -> ExecutionResult<Mode<'_>> {
        self.pc = start;
        (0..clock_cycles).try_fold(Mode::FetchInstruction, |mode, _cycle| match mode {
            Mode::AFetchImmediateOperand => {
                let value = self.fetch_immediate_operand();
                self.set_a(value);
                Ok(Mode::FetchInstruction)
            },

            Mode::AFetchZeroPageOperand => {
                let address = self.fetch_zero_page_address_operand();
                Ok(Mode::ADerefZeroPageAddr(address))
            },

            Mode::ADerefZeroPageAddr(address) => {
                self.set_a(self.memory[address]);
                Ok(Mode::FetchInstruction)
            },

            Mode::FetchInstruction => {
                use Opcode::*;

                self.fetch_instruction().map(|instruction| match instruction {
                    LdaImm => Mode::AFetchImmediateOperand,
                    LdaZp => Mode::AFetchZeroPageOperand,
                })
            },

            Mode::Halt(cpu) => Ok(Halt(cpu)),
        })
    }

    fn fetch_immediate_operand(&mut self) -> u8 { self.fetch_byte() }

    fn fetch_instruction(&mut self) -> ExecutionResult<Opcode> {
        #[allow(clippy::indexing_slicing)]
        let res_opcode = self.fetch_byte().to_opcode();
        res_opcode
    }

    fn fetch_zero_page_address_operand(&mut self) -> ZeroPageAddress { self.fetch_byte().into() }

    fn fetch_byte(&mut self) -> u8 {
        #[allow(clippy::indexing_slicing)]
        let byte = self.read_byte();
        self.pc.inc();
        byte
    }

    pub fn read_byte(&mut self) -> u8 { self.memory[self.pc] }

    pub fn set_a(&mut self, value: u8) -> &mut Self {
        self.a = value;
        match value {
            0 => {
                self.status.set_z();
            },
            n if n & 0b1000_0000 != 0 => {
                self.status.set_n();
            },
            _ => (),
        };
        self
    }

    #[must_use]
    pub const fn status(&self) -> &Status { &self.status }

    #[must_use]
    pub const fn x(&self) -> u8 { self.x }

    #[must_use]
    pub const fn y(&self) -> u8 { self.y }
}
