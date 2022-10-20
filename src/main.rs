use dbg_hex::dbg_hex;
use lib::{cpu::Opcode, Address, Cpu, Memory, Result, VectorTable};

fn main() -> Result<()> {
    let memory = load_program();
    let mut cpu = Cpu::new(memory);
    let mode = cpu.execute(VectorTable::Reset.into(), 2.into())?;
    dbg_hex!(mode);
    dbg_hex!(cpu);
    Ok(())
}

fn load_program() -> Memory {
    let mut memory = Memory::default();
    let mut address = Address::from(VectorTable::Reset);

    memory[address] = Opcode::LdaImm.into();
    address.inc();
    memory[address] = 0x42; // immediate operand
    memory
}
