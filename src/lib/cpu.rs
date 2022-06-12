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

use crate::cpu::mode::Mode::Halted;
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

    pub fn execute(&mut self, clock_cycles: usize, start: Address) -> ExecutionResult<Mode<'_>> {
        self.pc = start;
        (0..clock_cycles).try_fold(Mode::FetchInstruction, |mode, _cycle| match mode {
            Mode::FetchInstruction => {
                use Opcode::*;

                self.fetch_instruction().map(|instruction| match instruction {
                    LdaImmediate => Mode::FetchAImmediateOperand,
                })
            },
            Mode::FetchAImmediateOperand => {
                self.a = self.fetch_immediate_operand();
                match self.a {
                    0 => {
                        self.status.set_z();
                    },
                    value if value & 0b1000_0000 != 0 => {
                        self.status.set_n();
                    },
                    _ => (),
                }
                Ok(Mode::FetchInstruction)
            },
            Mode::Halted(cpu) => Ok(Halted(cpu)),
        })
    }

    pub fn fetch_immediate_operand(&mut self) -> u8 {
        #[allow(clippy::indexing_slicing)]
        let data = self.memory[self.pc];
        self.pc.inc();
        data
    }

    pub fn fetch_instruction(&mut self) -> ExecutionResult<Opcode> {
        #[allow(clippy::indexing_slicing)]
        let res_opcode = self.memory[self.pc].to_opcode();
        self.pc.inc();
        res_opcode
    }

    #[must_use]
    pub const fn status(&self) -> &Status { &self.status }
}
