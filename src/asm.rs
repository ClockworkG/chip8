use crate::specs::{
    Instruction,
    Nibble,
    Byte,
    Address,
};

pub enum InstructionData {
    Sys(Address),
    Cls,
    Ret,
    Jp(Address),
    Call(Address),
    Se(Nibble, Byte),
    Sne(Nibble, Byte),
    SeReg(Nibble, Nibble),
    Ld(Nibble, Byte),
    Add(Nibble, Byte),
    LdReg(Nibble, Nibble),
    Or(Nibble, Nibble),
    And(Nibble, Nibble),
    Xor(Nibble, Nibble),
    AddReg(Nibble, Nibble),
    SubReg(Nibble, Nibble),
    Shr(Nibble, Nibble),
    SubN(Nibble, Nibble),
    Shl(Nibble, Nibble),
    SneReg(Nibble, Nibble),
    LdI(Address),
    JpV0(Address),
    Rnd(Nibble, Byte),
    Drw(Nibble, Nibble, Nibble),
    Skp(Nibble),
    Sknp(Nibble),
    LdRegDt(Nibble),
    LdK(Nibble),
    LdDtReg(Nibble),
    LdSt(Nibble),
    AddI(Nibble),
    LdF(Nibble),
    LdB(Nibble),
    LdIMem(Nibble),
    LdVx(Nibble),
    Unknown,
}

pub fn decode_instruction(instruction: Instruction) -> InstructionData {
    let x = ((instruction & 0x0F00) >> 8) as Nibble;
    let y = ((instruction & 0x00F0) >> 4) as Nibble;
    let n = instruction & 0x0FFF;

    match (instruction & 0xF000) >> 12 {
        0x0 => {
            match n {
                0x0E0 => InstructionData::Cls,
                0x0EE => InstructionData::Ret,
                n => InstructionData::Sys(n),
            }
        },

        0x1 => InstructionData::Jp(n),
        0x2 => InstructionData::Call(n),
        0x3 => InstructionData::Se(x, truncate_2_bytes(n)),
        0x4 => InstructionData::Sne(x, truncate_2_bytes(n)),
        0x5 => {
            match n & 0x000F {
                0x0 => InstructionData::SeReg(x, y),
                _ => InstructionData::Unknown,
            }
        },
        0x6 => InstructionData::Ld(x, truncate_2_bytes(n)),
        0x7 => InstructionData::Add(x, truncate_2_bytes(n)),
        0x8 => {
            match n & 0x000F {
                0x0 => InstructionData::LdReg(x, y),
                0x1 => InstructionData::Or(x, y),
                0x2 => InstructionData::And(x, y),
                0x3 => InstructionData::Xor(x, y),
                0x4 => InstructionData::AddReg(x, y),
                0x5 => InstructionData::SubReg(x, y),
                0x6 => InstructionData::Shr(x, y),
                0x7 => InstructionData::SubN(x, y),
                0xE => InstructionData::Shl(x, y),
                _ => InstructionData::Unknown,
            }
        },
        _ => InstructionData::Unknown,
    }
}

fn truncate_2_bytes(n: u16) -> u8 {
    (n & 0x00FF) as u8
}

fn truncate_1_byte(n: u16) -> u8 {
    (n & 0x000F) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_simple() {
        match decode_instruction(0x00E0) {
            InstructionData::Cls => {},
            _ => assert!(false, "Decoded instruction is not CLS")
        }
    }

    #[test]
    fn decode_with_data() {
        if let InstructionData::Sys(data) = decode_instruction(0x0314) {
            assert_eq!(data, 0x314);
        } else {
            assert!(false, "Decoded instruction is not SYS");
        }
    }

    #[test]
    fn truncation_2_bytes() {
        assert_eq!(truncate_2_bytes(0xCAFE), 0xFE);
    }

    #[test]
    fn truncation_1_byte() {
        assert_eq!(truncate_1_byte(0xCAFE), 0xE);
    }
}
