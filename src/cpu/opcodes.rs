use super::{InstructionType::{self,*}, Register::*, AddressingMode::*};

pub const OPCODES: [Option<InstructionType>; 256] = generate_opcodes();

const fn generate_opcodes() -> [Option<InstructionType>; 256] {
    let mut opcodes: [Option<InstructionType>; 256] = [None; 256];
    opcodes[0xA9] = Some(Load(A, Immediate));
    opcodes[0xA5] = Some(Load(A, ZeroPage));
    opcodes[0xB5] = Some(Load(A, ZeroPageX));
    opcodes[0xAD] = Some(Load(A, Absolute));
    opcodes[0xBD] = Some(Load(A, AbsoluteX));
    opcodes[0xB9] = Some(Load(A, AbsoluteY));
    opcodes[0xA1] = Some(Load(A, IndexedIndirect));
    opcodes[0xB1] = Some(Load(A, IndirectIndexed));

    opcodes[0xA2] = Some(Load(X, Immediate));
    opcodes[0xA6] = Some(Load(X, ZeroPage));
    opcodes[0xB6] = Some(Load(X, ZeroPageY));
    opcodes[0xAE] = Some(Load(X, Absolute));
    opcodes[0xBE] = Some(Load(X, AbsoluteY));

    opcodes[0xA0] = Some(Load(Y, Immediate));
    opcodes[0xA4] = Some(Load(Y, ZeroPage));
    opcodes[0xB4] = Some(Load(Y, ZeroPageX));
    opcodes[0xAC] = Some(Load(Y, Absolute));
    opcodes[0xBC] = Some(Load(Y, AbsoluteX));

    opcodes
}