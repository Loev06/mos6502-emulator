mod addressing;
mod flags;

use crate::memory::Memory;
use addressing::AddressingMode;
use flags::Flags;

enum Register {
    A,
    X,
    Y,
}

pub struct Cpu<'a> {
    mem: &'a mut Memory,
    pc: u16,   // Program Counter
    sp: u8,    // Stack Pointer
    reg_a: u8, // Accumulator
    reg_x: u8, // X Register
    reg_y: u8, // Y Register
    flags: Flags,

    cycles: u64, // Total number of cycles executed
}

impl<'a> Cpu<'a> {
    pub fn new(mem: &'a mut Memory) -> Self {
        Cpu {
            mem,
            pc: 0,
            sp: 0xFF,
            reg_a: 0,
            reg_x: 0,
            reg_y: 0,
            flags: Flags::new(),
            cycles: 0,
        }
    }

    fn read_byte(&mut self, addr: u16) -> u8 {
        let byte = self.mem.read_byte(addr);
        self.cycles += 1;
        byte
    }

    fn fetch_byte(&mut self) -> u8 {
        let byte = self.read_byte(self.pc);
        self.pc = self.pc.wrapping_add(1);
        byte
    }

    fn fetch_u16(&mut self) -> u16 {
        let low = self.fetch_byte() as u16;
        let high = self.fetch_byte() as u16;
        (high << 8) | low
    }

    fn set_reg(&mut self, reg: Register, value: u8) {
        self.flags.zero = value == 0;
        self.flags.negative = (value & 0b1000_0000) != 0;
        match reg {
            Register::A => self.reg_a = value,
            Register::X => self.reg_x = value,
            Register::Y => self.reg_y = value,
        }
    }

    fn get_addressing_value(&mut self, addressing_mode: AddressingMode) -> u8 {
        match addressing_mode {
            AddressingMode::Immediate => self.fetch_byte(),
            AddressingMode::ZeroPage => {
                let addr = self.fetch_byte() as u16;
                self.read_byte(addr)
            }
            AddressingMode::ZeroPageX => {
                let addr = (self.fetch_byte().wrapping_add(self.reg_x)) as u16;
                self.read_byte(addr)
            }
            AddressingMode::ZeroPageY => {
                let addr = (self.fetch_byte().wrapping_add(self.reg_y)) as u16;
                self.read_byte(addr)
            }
            AddressingMode::Absolute => {
                let addr = self.fetch_u16();
                self.read_byte(addr)
            }
            AddressingMode::AbsoluteX => {
                let addr = (self.fetch_u16()).wrapping_add(self.reg_x as u16);
                // TODO: Handle page crossing for cycle counting
                self.read_byte(addr)
            }
            AddressingMode::AbsoluteY => {
                let addr = (self.fetch_u16()).wrapping_add(self.reg_y as u16);
                // TODO: Handle page crossing for cycle counting
                self.read_byte(addr)
            }
            AddressingMode::IndexedIndirect => {
                let zero_addr = self.fetch_byte().wrapping_add(self.reg_x);
                self.cycles += 1; // Moving data to address bus costs 1 cycle
                let low = self.read_byte(zero_addr as u16) as u16;
                let high = self.read_byte(zero_addr.wrapping_add(1) as u16) as u16;
                let addr = (high << 8) | low;
                self.read_byte(addr)
            }
            AddressingMode::IndirectIndexed => {
                let zero_addr = self.fetch_byte();
                self.cycles += 1; // Moving data to address bus costs 1 cycle
                let low = self.read_byte(zero_addr as u16) as u16;
                let high = self.read_byte(zero_addr.wrapping_add(1) as u16) as u16;
                let addr = ((high << 8) | low).wrapping_add(self.reg_y as u16);
                self.read_byte(addr)
            }
            _ => panic!("Addressing mode {:?} cannot be used to get a value", addressing_mode)
        }
    }
}
