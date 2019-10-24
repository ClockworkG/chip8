use crate::memory::{MainMemory, Memory};
use crate::specs::{Address, Byte};

pub struct Bus {
    memory: MainMemory
}

impl Bus {
    pub fn new(memory: MainMemory) -> Self {
        Bus {
            memory
        }
    }
}

impl Memory for Bus {
    type Address = Address;

    fn read(&self, address: Self::Address) -> Byte {
        self.memory.read(address)
    }

    fn write(&mut self, address: Self::Address, value: Byte) {
        self.memory.write(address, value);
    }
}
