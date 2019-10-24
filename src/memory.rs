use std::path::Path;
use std::fs::File;
use std::io;
use std::io::Read;

use crate::specs::{Byte, MEMORY_SIZE, PROGRAM_BEGIN, Address};

pub trait Memory {
    type Address;
    type Value;

    fn read(&self, address: Self::Address) -> Self::Value;
    fn write(&mut self, address: Self::Address, value: Self::Value);
}

pub struct MainMemory {
    mem: [Byte; MEMORY_SIZE]
}

pub struct ROM {
    data: Vec<Byte>,
}

impl MainMemory {
    pub fn new() -> Self {
        MainMemory {
            mem: [0x0; MEMORY_SIZE]
        }
    }

    pub fn with_rom(rom: ROM) -> Self {
        let mut mem = [0x0; MEMORY_SIZE];

        let inf_bound = PROGRAM_BEGIN;
        let sup_bound = PROGRAM_BEGIN + rom.size();
        &mem[inf_bound..sup_bound].copy_from_slice(rom.bytes());

        let mut mem = MainMemory { mem };
        mem.load_fontset();
        mem
    }

    pub fn read_bytes(&self, address: Address, offset: Address) -> Vec<Byte> {
        self.mem[
            (address as usize)..(address as usize + offset as usize)
        ].to_vec()
    }

    fn load_fontset(&mut self) {
        // 0
        &self.mem[0..5].copy_from_slice(&[0xF0, 0x90, 0x90, 0x90, 0xF0]);

        // 1
        &self.mem[5..10].copy_from_slice(&[0x20, 0x60, 0x20, 0x20, 0x70]);

        // 2
        &self.mem[10..15].copy_from_slice(&[0xF0, 0x10, 0xF0, 0x80, 0xF0]);

        // 3
        &self.mem[15..20].copy_from_slice(&[0xF0, 0x10, 0xF0, 0x10, 0xF0]);

        // 4
        &self.mem[15..20].copy_from_slice(&[0xF0, 0x10, 0xF0, 0x10, 0xF0]);

        // 5
        &self.mem[20..25].copy_from_slice(&[0xF0, 0x80, 0xF0, 0x10, 0xF0]);

        // 6
        &self.mem[25..30].copy_from_slice(&[0xF0, 0x80, 0xF0, 0x90, 0xF0]);

        // 7
        &self.mem[30..35].copy_from_slice(&[0xF0, 0x10, 0x20, 0x40, 0x40]);

        // 8
        &self.mem[35..40].copy_from_slice(&[0xF0, 0x90, 0xF0, 0x90, 0xF0]);

        // 9
        &self.mem[40..45].copy_from_slice(&[0xF0, 0x90, 0xF0, 0x10, 0xF0]);

        // A
        &self.mem[45..50].copy_from_slice(&[0xF0, 0x90, 0xF0, 0x90, 0x90]);

        // B
        &self.mem[50..55].copy_from_slice(&[0xE0, 0x90, 0xE0, 0x90, 0xE0]);

        // C
        &self.mem[55..60].copy_from_slice(&[0xF0, 0x80, 0x80, 0x80, 0xF0]);

        // D
        &self.mem[60..65].copy_from_slice(&[0xE0, 0x90, 0x90, 0x90, 0xE0]);

        // E
        &self.mem[65..70].copy_from_slice(&[0xF0, 0x80, 0xF0, 0x80, 0xF0]);

        // F
        &self.mem[70..75].copy_from_slice(&[0xF0, 0x80, 0xF0, 0x80, 0x80]);
    }
}

impl Memory for MainMemory {
    type Address = Address;
    type Value = Byte;

    fn read(&self, address: Self::Address) -> Self::Value {
        let real_address = address as usize;

        assert!(real_address < MEMORY_SIZE, "Address out of memory space.");
        self.mem[real_address]
    }

    fn write(&mut self, address: Self::Address, value: Self::Value) {
        let real_address = address as usize;

        assert!(real_address < MEMORY_SIZE, "Address out of memory space.");
        self.mem[real_address] = value;
    }
}

pub fn merge_bytes(left: u8, right: u8) -> u16 {
    ((left as u16) << 8) | (right as u16)
}

impl ROM {
    pub fn from_file(path: &Path) -> Result<Self, io::Error> {
        let mut bytes = Vec::new();
        let mut file = File::open(path)?;

        file.read_to_end(&mut bytes)?;

        Ok(ROM {
            data: bytes,
        })
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn bytes(&self) -> &[u8] {
        &self.data
    }

    pub fn instructions(&self) -> Vec<u16> {
        self.data.chunks(2)
                 .map(|word| {
                    let (left, right) = (word[0], word[1]);
                    merge_bytes(left, right)
                 })
                 .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() {
        let mut mem = MainMemory::new();
        mem.mem[0x563] = 200;
        assert_eq!(mem.read(0x563), 200);
    }

    #[test]
    fn test_write() {
        let mut mem = MainMemory::new();
        mem.write(0x345, 23);
        assert_eq!(mem.mem[0x345], 23);
    }

    #[test]
    #[should_panic]
    fn test_write_out_of_bound() {
        let mut mem = MainMemory::new();
        mem.write(0x1345, 34);
    }

    #[test]
    #[should_panic]
    fn test_read_out_of_bound() {
        let mem = MainMemory::new();
        mem.read(0x6721);
    }
}
