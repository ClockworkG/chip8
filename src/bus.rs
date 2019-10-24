use crate::memory::{MainMemory, Memory, merge_bytes};
use crate::specs::{Address, Instruction, Byte};
use crate::display::FrameBuffer;

pub struct Bus {
    memory: MainMemory,
    frame_buffer: FrameBuffer,
}

impl Bus {
    pub fn new(memory: MainMemory) -> Self {
        Bus {
            memory,
            frame_buffer: FrameBuffer::new()
        }
    }

    pub fn read_instruction(&self, address: Address) -> Instruction {
        let left = self.memory.read(address);
        let right = self.memory.read(address + 1);
        merge_bytes(left, right)
    }

    pub fn read_bytes(&self, address: Address, offset: Address) -> Vec<Byte> {
        self.memory.read_bytes(address, offset)
    }

    pub fn display_sprite(&mut self, pos: (usize, usize), sprite: &[u8]) {
        self.frame_buffer.write_bytes(pos, sprite);
    }

    pub fn get_frame_buffer(&self) -> &FrameBuffer {
        &self.frame_buffer
    }

    pub fn get_ram(&self) -> &MainMemory {
        &self.memory
    }
}
