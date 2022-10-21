#![allow(clippy::indexing_slicing)]
mod clock_cycles;
mod mode;
mod opcode;
mod register_id;
mod stack;
mod status;
#[cfg(test)]
mod unit_tests;
mod vector_table;

use std::fmt::Debug;

use crate::{error::execution::Result as ExecutionResult, memory::Address, Memory};
use clock_cycles::ClockCycles;
pub use opcode::Opcode;
pub use register_id::RegisterId;
use status::Status;
pub use vector_table::VectorTable;

use crate::{cpu::mode::Mode::Halt, memory::ZeroPageAddress};
use mode::Mode;

#[derive(Eq, Hash, PartialEq)]
pub struct Cpu {
    pc: Address,
    sp: ZeroPageAddress,

    a: u8,
    x: u8,
    y: u8,

    status: Status,
    memory: Memory,
}

impl Cpu {
    #[must_use]
    pub const fn new(memory: Memory) -> Self {
        Self {
            memory,
            pc: Address::default(),
            sp: ZeroPageAddress::default(),
            a: u8::default(),
            x: u8::default(),
            y: u8::default(),
            status: Status::default(),
        }
    }

    // TODO: Hide direct register access to ensure use of accessors to guarantee side-effectful
    // register writes
    #[must_use]
    pub const fn a(&self) -> u8 { self.a }

    pub fn execute(
        &mut self,
        start: Address,
        clock_cycles: ClockCycles,
    ) -> ExecutionResult<Mode<'_>> {
        let clock_cycles = clock_cycles.into();
        self.pc = start;
        (0_usize..clock_cycles).try_fold(Mode::FetchInstruction, |mode, _cycle| match mode {
            Mode::AFetchImmediateOperand(register_id) => {
                use RegisterId::*;

                let value = self.fetch_immediate_operand();
                // TODO: Explore dynamic dispatch (without allocation) alternate implementation
                match register_id {
                    A => self.set_a(value),
                    X => self.set_x(value),
                    Y => self.set_y(value),
                };
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
                    LdaImm => Mode::AFetchImmediateOperand(RegisterId::A),
                    LdaZp => Mode::AFetchZeroPageOperand,
                    LdxImm => Mode::AFetchImmediateOperand(RegisterId::X),
                    LdyImm => Mode::AFetchImmediateOperand(RegisterId::Y),
                })
            },

            Mode::Halt(cpu) => Ok(Halt(cpu)),
        })
    }

    fn fetch_immediate_operand(&mut self) -> u8 { self.fetch_byte() }

    fn fetch_instruction(&mut self) -> ExecutionResult<Opcode> { self.fetch_byte().try_into() }

    fn fetch_zero_page_address_operand(&mut self) -> ZeroPageAddress { self.fetch_byte().into() }

    fn fetch_byte(&mut self) -> u8 {
        #[allow(clippy::indexing_slicing)]
        let byte = self.read_byte();
        self.pc.inc();
        byte
    }

    #[must_use]
    pub const fn pc(&self) -> Address { self.pc }

    pub fn read_byte(&mut self) -> u8 { self.memory[self.pc] }

    pub fn set_a(&mut self, value: u8) -> &mut Self {
        self.a = value;
        self.set_register(value)
    }

    pub fn set_x(&mut self, value: u8) -> &mut Self {
        self.x = value;
        self.set_register(value)
    }

    pub fn set_y(&mut self, value: u8) -> &mut Self {
        self.y = value;
        self.set_register(value)
    }

    #[inline(always)]
    pub fn set_register(&mut self, value: u8) -> &mut Self {
        match value {
            0 => {
                self.status.set_z();
            },
            _n if _n & 0b1000_0000 != 0 => {
                self.status.set_n();
            },
            _ => (),
        };
        self
    }

    #[must_use]
    pub const fn sp(&self) -> ZeroPageAddress { self.sp }

    #[must_use]
    pub const fn status(&self) -> Status { self.status }

    #[must_use]
    pub const fn x(&self) -> u8 { self.x }

    #[must_use]
    pub const fn y(&self) -> u8 { self.y }
}

impl Debug for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "pc: {:?}\nsp: {:?}\na: {:#04x}/{}, x: {:#04x}/{}, y: {:#04x}/{}\nstatus: \
             {:?}\nmemory: <elided for brevity; dump `Cpu::memory` to view>",
            self.pc(),
            self.sp(),
            self.a(),
            self.a(),
            self.x(),
            self.x(),
            self.y(),
            self.y(),
            self.status()
        )
    }
}
