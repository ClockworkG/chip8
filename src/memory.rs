use std::path::Path;
use std::fs::File;
use std::io;
use std::io::Read;

use crate::specs::{Byte, MEMORY_SIZE, PROGRAM_BEGIN};

pub trait Memory {
    type Address;

    fn read(&self, address: Self::Address) -> Byte;
    fn write(&mut self, address: Self::Address, value: Byte);
}

pub struct MainMemory {
    mem: [Byte; MEMORY_SIZE]
}

pub struct ROM {
    data: Vec<Byte>,
}

impl MainMemory {
    pub fn with_rom(rom: ROM) -> Self {
        let mut mem = [0x0; MEMORY_SIZE];

        let inf_bound = PROGRAM_BEGIN;
        let sup_bound = PROGRAM_BEGIN + rom.size();
        &mem[inf_bound..sup_bound].copy_from_slice(rom.bytes());

        MainMemory {
            mem
        }
    }
}

impl Memory for MainMemory {
    type Address = u16;

    fn read(&self, address: Self::Address) -> Byte {
        let real_address = address as usize;

        assert!(real_address < MEMORY_SIZE, "Address out of memory space.");
        self.mem[real_address]
    }

    fn write(&mut self, address: Self::Address, value: Byte) {
        let real_address = address as usize;

        assert!(real_address < MEMORY_SIZE, "Address out of memory space.");
        self.mem[real_address] = value;
    }
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

    pub fn from_bytes(bytes: &[u8]) -> Self {
        ROM {
            data: bytes.to_vec()
        }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn bytes(&self) -> &[u8] {
        &self.data
    }

    pub fn instructions(&self) -> Vec<u16> {
        fn merge_bytes(left: u8, right: u8) -> u16 {
            ((left as u16) << 8) | (right as u16)
        }

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
