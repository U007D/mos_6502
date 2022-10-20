// This trait is unreachable from outside the `mos_6502` crate and thus is used to prevent
// implementations of `mos_6502` extension traits (e.g. see `U8InstructionExt`).
pub trait ISealed {}
