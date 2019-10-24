use crate::memory::{MainMemory, Memory, merge_bytes};
use crate::specs::{Address, Instruction};

pub struct Bus {
    memory: MainMemory,
}

impl Bus {
    pub fn new(memory: MainMemory) -> Self {
        Bus {
            memory
        }
    }

    pub fn read_instruction(&self, address: Address) -> Instruction {
        let left = self.memory.read(address);
        let right = self.memory.read(address + 1);
        merge_bytes(left, right)
    }
}
