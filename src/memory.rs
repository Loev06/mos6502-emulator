pub struct Memory {
    data: [u8; 1024 * 64], // 64KB of memory
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            data: [0; 1024 * 64],
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        self.data[addr as usize] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_size() {
        let mem = Memory::new();
        assert_eq!(mem.data.len(), 1024 * 64);
        mem.read_byte(u16::MAX);
    }
}