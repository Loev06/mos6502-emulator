pub struct Flags {
    pub carry: bool,
    pub zero: bool,
    pub interrupt_disable: bool,
    pub decimal_mode: bool,
    pub break_command: bool,
    pub overflow: bool,
    pub negative: bool,
}

impl Flags {
    pub fn new() -> Self {
        Flags {
            carry: false,
            zero: false,
            interrupt_disable: false,
            decimal_mode: false,
            break_command: false,
            overflow: false,
            negative: false,
        }
    }

    pub fn to_byte(&self) -> u8 {
        (self.carry as u8)
            | ((self.zero as u8) << 1)
            | ((self.interrupt_disable as u8) << 2)
            | ((self.decimal_mode as u8) << 3)
            | ((self.break_command as u8) << 4)
            | ((self.overflow as u8) << 6) // Bit 5 is unused
            | ((self.negative as u8) << 7)
    }

    pub fn from_byte(byte: u8) -> Self {
        Flags {
            carry: (byte & 0b0000_0001) != 0,
            zero: (byte & 0b0000_0010) != 0,
            interrupt_disable: (byte & 0b0000_0100) != 0,
            decimal_mode: (byte & 0b0000_1000) != 0,
            break_command: (byte & 0b0001_0000) != 0,
            overflow: (byte & 0b0100_0000) != 0,
            negative: (byte & 0b1000_0000) != 0,
        }
    }
}
