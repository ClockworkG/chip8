use crate::specs::{
    Instruction,
    Nibble,
    Byte,
    Address,
};

#[derive(Debug)]
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
                0x000 => InstructionData::Unknown,
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
        0x9 => {
            match n & 0x000F {
                0x0 => InstructionData::SneReg(x, y),
                _ => InstructionData::Unknown,
            }
        },
        0xA => InstructionData::LdI(n),
        0xB => InstructionData::JpV0(n),
        0xC => InstructionData::Rnd(x, truncate_2_bytes(n)),
        0xD => InstructionData::Drw(x, y, truncate_1_byte(n)),
        0xE => {
            match n & 0x00FF {
                0x9E => InstructionData::Skp(x),
                0xA1 => InstructionData::Sknp(x),
                _ => InstructionData::Unknown,
            }
        },
        0xF => {
            match n & 0x00FF {
                0x07 => InstructionData::LdRegDt(x),
                0x0A => InstructionData::LdK(x),
                0x15 => InstructionData::LdDtReg(x),
                0x18 => InstructionData::LdSt(x),
                0x1E => InstructionData::AddI(x),
                0x29 => InstructionData::LdF(x),
                0x33 => InstructionData::LdB(x),
                0x55 => InstructionData::LdIMem(x),
                0x65 => InstructionData::LdVx(x),
                _ => InstructionData::Unknown,
            }
        },
        _ => InstructionData::Unknown,
    }
}

impl std::fmt::Display for InstructionData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use InstructionData::*;

        match self {
            Sys(n) => write!(f, "{:<5} {:#05X}", "SYS", n),
            Cls => write!(f, "{:<5}", "CLS"),
            Ret => write!(f, "{:<5}", "RET"),
            Jp(n) => write!(f, "{:<5} {:#05X}", "JP", n),
            Call(n) => write!(f, "{:<5} {:#05X}", "CALL", n),
            Se(x, n) => write!(f, "{:<5} V{:1X}, {:#04X}", "SE", x, n),
            Sne(x, n) => write!(f, "{:<5} V{:1X}, {:#04X}", "SNE", x, n),
            SeReg(x, y) => write!(f, "{:<5} V{:1X}, V{:1X}", "SE", x, y),
            Ld(x, n) => write!(f, "{:<5} V{:1X}, {:#04X}", "LD", x, n),
            Add(x, n) => write!(f, "{:<5} V{:1X}, {:#04X}", "ADD", x, n),
            LdReg(x, y) => write!(f, "{:<5} V{:1X}, V{:1X}", "LD", x, y),
            Or(x, y) => write!(f, "{:<5} V{:1X}, V{:1X}", "OR", x, y),
            And(x, y) => write!(f, "{:<5} V{:1X}, V{:1X}", "AND", x, y),
            Xor(x, y) => write!(f, "{:<5} V{:1X}, V{:1X}", "XOR", x, y),
            AddReg(x, y) => write!(f, "{:<5} V{:1X}, V{:1X}", "ADD", x, y),
            SubReg(x, y) => write!(f, "{:<5} V{:1X}, V{:1X}", "SUB", x, y),
            Shr(x, y) => write!(f, "{:<5} V{:1X}, V{:1X}", "SHR", x, y),
            SubN(x, y) => write!(f, "{:<5} V{:1X}, V{:1X}", "SUBN", x, y),
            Shl(x, y) => write!(f, "{:<5} V{:1X}, V{:1X}", "SHL", x, y),
            SneReg(x, y) => write!(f, "{:<5} V{:1X}, V{:1X}", "SNE", x, y),
            LdI(n) => write!(f, "{:<5} I, {:#05X}", "LD", n),
            JpV0(n) => write!(f, "{:<5} V0, {:#05X}", "JP", n),
            Rnd(x, n) => write!(f, "{:<5} V{:1X}, {:#04X}", "RND", x, n),
            Drw(x, y, n) => write!(f, "{:<5} V{:1X}, V{:1X}, {:#03X}", "DRW", x, y, n),
            Skp(x) => write!(f, "{:<5} V{:1X}", "SKP", x),
            Sknp(x) => write!(f, "{:<5} V{:1X}", "SKNP", x),
            LdRegDt(x) => write!(f, "{:<5} V{:1X}, DT", "LD", x),
            LdK(x) => write!(f, "{:<5} V{:1X}, K", "LD", x),
            LdDtReg(x) => write!(f, "{:<5} DT, V{:1X}", "LD", x),
            LdSt(x) => write!(f, "{:<5} V{:1X}, ST", "LD", x),
            AddI(x) => write!(f, "{:<5} I, V{:1X}", "ADD", x),
            LdF(x) => write!(f, "{:<5} F, V{:1X}", "LD", x),
            LdB(x) => write!(f, "{:<5} B, V{:1X}", "LD", x),
            LdIMem(x) => write!(f, "{:<5} [I], V{:1X}", "LD", x),
            LdVx(x) => write!(f, "{:<5} V{:1X}, [I]", "LD", x),
            _ => write!(f, "XXXXXXXXXXXXXX")
        }
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
