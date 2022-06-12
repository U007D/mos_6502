use crate::Cpu;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Mode<'cpu> {
    AFetchImmediateOperand,
    AFetchZeroPageOperand,
    FetchInstruction,
    Halt(&'cpu Cpu)
}
