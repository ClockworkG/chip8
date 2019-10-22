use crate::specs::{Byte, MEMORY_SIZE};

pub trait Memory {
    type Address;

    fn read(&self, address: Self::Address) -> Byte;
    fn write(&mut self, address: Self::Address, value: Byte);
}

pub struct MainMemory {
    mem: [Byte; MEMORY_SIZE]
}

impl MainMemory {
    pub fn new() -> Self {
        MainMemory {
            mem: [0x0; MEMORY_SIZE],
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
