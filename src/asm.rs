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
    Unknown,
}

pub fn decode_instruction(instruction: Instruction) -> InstructionData {
    let x = (instruction & 0x0F00) >> 8;
    let y = (instruction & 0x00F0) >> 4;
    let n = instruction & 0x0FFF;

    match (instruction & 0xF000) >> 12 {
        0x0 => {
            match n {
                0x0E0 => InstructionData::Cls,
                0x0EE => InstructionData::Ret,
                n => InstructionData::Sys(n),
            }
        },

        0x1 => {
            InstructionData::Jp(n)
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
