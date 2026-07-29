mod cpu;
mod memory;
use std::println;

use cpu::Cpu;

use anyhow::Result;

fn main() -> Result<()> {
    let mut memory = memory::Memory::new();
    memory.write_byte(0x0000, 0xA9);
    memory.write_byte(0x0001, 0x42);
    let mut cpu = Cpu::new(&mut memory);
    cpu.execute()?;
    Ok(())
}
