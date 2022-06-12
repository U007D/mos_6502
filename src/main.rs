use dbg_hex::dbg_hex;
use mos_6502_lib::{Address, Cpu, Memory, Result, VectorTable};

fn main() -> Result<()> {
    let memory = load_program();
    let mut cpu = Cpu::new(memory);
    let mode = cpu.execute(2, VectorTable::Reset.into())?;
    dbg_hex!(mode);
    dbg_hex!(cpu.status());
    Ok(())
}

fn load_program() -> Memory {
    let mut memory = Memory::default();
    let mut address = Address::from(VectorTable::Reset);
    memory[address] = 0xa9; // LDA immediate
    address.inc();
    memory[address] = 0x42; // immediate operand
    memory
}
