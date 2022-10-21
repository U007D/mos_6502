use crate::{Cpu, ZeroPageAddress};

use crate::cpu::RegisterId;

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Mode<'cpu> {
    AFetchImmediateOperand(RegisterId),
    AFetchZeroPageOperand,
    ADerefZeroPageAddr(ZeroPageAddress),
    FetchInstruction,
    Halt(&'cpu Cpu),
}
