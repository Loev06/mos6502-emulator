mod flags;
mod opcodes;

use crate::memory::Memory;
use flags::Flags;
use opcodes::OPCODES;

use anyhow::{Result, anyhow};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InstructionType {
    Load(Register, AddressingMode),
    Store(Register, AddressingMode),
    Transfer(Register, Register),
    PushStack(Register),
    PullStack(Register),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Register {
    SP,
    A,
    X,
    Y,
    Flags,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressingMode {
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndexedIndirect,
    IndirectIndexed,
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
        let mut cpu = Cpu {
            mem,
            pc: 0,
            sp: 0xFF,
            reg_a: 0,
            reg_x: 0,
            reg_y: 0,
            flags: Flags::new(),
            cycles: 0,
        };
        cpu.reset();
        cpu
    }

    pub fn reset(&mut self) {
        // A nice resource for the reset sequence and interrupt requests:
        // https://www.pagetable.com/?p=410
        let pc_low = self.mem.read_byte(0xFFFC) as u16;
        let pc_high = self.mem.read_byte(0xFFFD) as u16;
        self.pc = (pc_high << 8) | pc_low;
        self.sp = self.sp.wrapping_sub(3); // Stack pointer is decremented by 3 on reset
    }

    fn read_byte(&mut self, addr: u16) -> u8 {
        let byte = self.mem.read_byte(addr);
        self.cycles += 1;
        byte
    }

    fn write_byte(&mut self, addr: u16, value: u8) {
        self.mem.write_byte(addr, value);
        self.cycles += 1;
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

    fn get_reg(&self, reg: Register) -> u8 {
        match reg {
            Register::SP => self.sp,
            Register::A => self.reg_a,
            Register::X => self.reg_x,
            Register::Y => self.reg_y,
            Register::Flags => self.flags.to_byte(),
        }
    }

    fn set_reg(&mut self, reg: Register, value: u8) {
        if reg != Register::SP {
            self.flags.zero = value == 0;
            self.flags.negative = (value & 0b1000_0000) != 0;
        }
        match reg {
            Register::SP => self.sp = value,
            Register::A => self.reg_a = value,
            Register::X => self.reg_x = value,
            Register::Y => self.reg_y = value,
            Register::Flags => self.flags = Flags::from_byte(value),
        }
    }

    fn get_addressed_pointer(&mut self, addressing_mode: AddressingMode) -> Result<u16> {
        Ok(match addressing_mode {
            AddressingMode::ZeroPage => self.fetch_byte() as u16,
            AddressingMode::ZeroPageX => self.fetch_byte().wrapping_add(self.reg_x) as u16,
            AddressingMode::ZeroPageY => self.fetch_byte().wrapping_add(self.reg_y) as u16,
            AddressingMode::Absolute => self.fetch_u16(),
            AddressingMode::AbsoluteX => self.fetch_u16().wrapping_add(self.reg_x as u16),
            AddressingMode::AbsoluteY => self.fetch_u16().wrapping_add(self.reg_y as u16),
            AddressingMode::IndexedIndirect => {
                let zero_addr = self.fetch_byte().wrapping_add(self.reg_x);
                self.cycles += 1; // Moving data to address bus costs 1 cycle
                let low = self.read_byte(zero_addr as u16) as u16;
                let high = self.read_byte(zero_addr.wrapping_add(1) as u16) as u16;
                (high << 8) | low
            }
            AddressingMode::IndirectIndexed => {
                let zero_addr = self.fetch_byte();
                self.cycles += 1; // Moving data to address bus costs 1 cycle
                let low = self.read_byte(zero_addr as u16) as u16;
                let high = self.read_byte(zero_addr.wrapping_add(1) as u16) as u16;
                ((high << 8) | low).wrapping_add(self.reg_y as u16)
            }
            _ => {
                return Err(anyhow!(
                    "Addressing mode {:?} cannot be used to get a pointer",
                    addressing_mode
                ));
            }
        })
    }

    fn get_addressing_value(&mut self, addressing_mode: AddressingMode) -> Result<u8> {
        Ok(match addressing_mode {
            AddressingMode::Immediate => self.fetch_byte(),
            AddressingMode::ZeroPage
            | AddressingMode::ZeroPageX
            | AddressingMode::ZeroPageY
            | AddressingMode::Absolute
            | AddressingMode::IndexedIndirect
            | AddressingMode::IndirectIndexed => {
                let addr = self
                    .get_addressed_pointer(addressing_mode)
                    .expect("Valid addressing mode should return a pointer");
                self.read_byte(addr)
            }
            AddressingMode::AbsoluteX | AddressingMode::AbsoluteY => {
                let addr = self
                    .get_addressed_pointer(addressing_mode)
                    .expect("Valid addressing mode should return a pointer");
                // TODO: Handle page crossing for cycle counting
                self.read_byte(addr)
            }
            _ => {
                return Err(anyhow!(
                    "Addressing mode {:?} cannot be used to get a value",
                    addressing_mode
                ));
            }
        })
    }

    fn load_reg(&mut self, reg: Register, addressing_mode: AddressingMode) {
        // Note that not all addressing modes are valid for registers X and Y.
        // Since a valid command is assumed, we do not need to distinguish between them here.
        let value = self
            .get_addressing_value(addressing_mode)
            .expect("Valid addressing mode should return a value");
        self.set_reg(reg, value);
    }

    fn store_reg(&mut self, reg: Register, addressing_mode: AddressingMode) {
        let value = self.get_reg(reg);
        let addr = self
            .get_addressed_pointer(addressing_mode)
            .expect("Valid addressing mode should return a pointer");
        self.write_byte(addr, value);
    }

    fn transfer_reg(&mut self, reg_from: Register, reg_to: Register) {
        let value = self.get_reg(reg_from);
        self.set_reg(reg_to, value);
    }

    fn push_stack(&mut self, reg: Register) {
        let value = self.get_reg(reg);
        let addr = 0x0100 | (self.sp as u16);
        self.cycles += 1; // According to datasheet A.5.1., there is an additional cycle before writing to the stack
        self.write_byte(addr, value);
        self.sp = self.sp.wrapping_sub(1);
    }

    fn pull_stack(&mut self, reg: Register) {
        self.sp = self.sp.wrapping_add(1);
        let addr = 0x0100 | (self.sp as u16);
        self.cycles += 2; // According to datasheet A.5.2., there are two additional cycles before reading from the stack
        let value = self.read_byte(addr);
        self.set_reg(reg, value);
    }

    pub fn execute(&mut self) -> Result<()> {
        let opcode = self.fetch_byte();
        if let Some(instruction) = OPCODES[opcode as usize] {
            match instruction {
                InstructionType::Load(reg, addressing_mode) => {
                    self.load_reg(reg, addressing_mode);
                }
                InstructionType::Store(reg, addressing_mode) => {
                    self.store_reg(reg, addressing_mode);
                }
                InstructionType::Transfer(reg_from, reg_to) => {
                    self.transfer_reg(reg_from, reg_to);
                }
                InstructionType::PushStack(reg) => {
                    self.push_stack(reg);
                }
                InstructionType::PullStack(reg) => {
                    self.pull_stack(reg);
                }
            }
        } else {
            return Err(anyhow!("Unknown opcode: 0x{:02X}", opcode));
        }
        Ok(())
    }
}
