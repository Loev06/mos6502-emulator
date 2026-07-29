mod flags;
use crate::memory::Memory;
use flags::Flags;

enum Register {
    A,
    X,
    Y,
}

pub struct Cpu {
    pc: u16,   // Program Counter
    sp: u8,    // Stack Pointer
    reg_a: u8, // Accumulator
    reg_x: u8, // X Register
    reg_y: u8, // Y Register
    flags: Flags,

    cycles: u64, // Total number of cycles executed
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            pc: 0,
            sp: 0xFF,
            reg_a: 0,
            reg_x: 0,
            reg_y: 0,
            flags: Flags::new(),
            cycles: 0,
        }
    }

    fn fetch_byte(&mut self, mem: &Memory) -> u8 {
        let byte = mem.read_byte(self.pc);
        self.pc += 1;
        self.cycles += 1;
        byte
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
}
