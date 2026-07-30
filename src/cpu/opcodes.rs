use super::{
    AddressingMode::*,
    InstructionType::{self, *},
    Register::*,
};

pub const OPCODES: [Option<InstructionType>; 256] = generate_opcodes();

const fn generate_opcodes() -> [Option<InstructionType>; 256] {
    let mut opcodes: [Option<InstructionType>; 256] = [None; 256];
    let mut i = 0;
    while i < 256 {
        // Ugly notation to avoid a repeated Some() for every opcode.
        // We continue; in the catch-all case to keep this a None-value.
        opcodes[i] = Some(match i {
            0xA9 => Load(A, Immediate),
            0xA5 => Load(A, ZeroPage),
            0xB5 => Load(A, ZeroPageX),
            0xAD => Load(A, Absolute),
            0xBD => Load(A, AbsoluteX),
            0xB9 => Load(A, AbsoluteY),
            0xA1 => Load(A, IndexedIndirect),
            0xB1 => Load(A, IndirectIndexed),

            0xA2 => Load(X, Immediate),
            0xA6 => Load(X, ZeroPage),
            0xB6 => Load(X, ZeroPageY),
            0xAE => Load(X, Absolute),
            0xBE => Load(X, AbsoluteY),

            0xA0 => Load(Y, Immediate),
            0xA4 => Load(Y, ZeroPage),
            0xB4 => Load(Y, ZeroPageX),
            0xAC => Load(Y, Absolute),
            0xBC => Load(Y, AbsoluteX),

            0x85 => Store(A, ZeroPage),
            0x95 => Store(A, ZeroPageX),
            0x8D => Store(A, Absolute),
            0x9D => Store(A, AbsoluteX),
            0x99 => Store(A, AbsoluteY),
            0x81 => Store(A, IndexedIndirect),
            0x91 => Store(A, IndirectIndexed),

            0x86 => Store(X, ZeroPage),
            0x96 => Store(X, ZeroPageY),
            0x8E => Store(X, Absolute),

            0x84 => Store(Y, ZeroPage),
            0x94 => Store(Y, ZeroPageX),
            0x8C => Store(Y, Absolute),

            0xAA => Transfer(A, X),
            0x8A => Transfer(X, A),
            0xA8 => Transfer(A, Y),
            0x98 => Transfer(Y, A),
            0xBA => Transfer(SP, X),
            0x9A => Transfer(X, SP),

            0x48 => PushStack(A),
            0x08 => PushStack(Flags),

            0x68 => PullStack(A),
            0x28 => PullStack(Flags),

            _ => {
                // No match, so do not modify to a Some value
                i += 1;
                continue;
            }
        });
        i += 1;
    }

    opcodes
}
