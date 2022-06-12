use crate::Cpu;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Mode<'cpu> {
    FetchInstruction,
    FetchAImmediateOperand,
    Halted(&'cpu Cpu)
}
