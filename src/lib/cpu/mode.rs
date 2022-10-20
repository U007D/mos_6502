use crate::{Cpu, ZeroPageAddress};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Mode<'cpu> {
    AFetchImmediateOperand,
    AFetchZeroPageOperand,
    ADerefZeroPageAddr(ZeroPageAddress),
    FetchInstruction,
    Halt(&'cpu Cpu),
}
