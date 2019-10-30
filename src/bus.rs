use crate::memory::{MainMemory, Memory, merge_bytes};
use crate::specs::{Address, Instruction, Byte};
use crate::display::FrameBuffer;
use crate::input::Keyboard;

pub struct Bus {
    memory: MainMemory,
    frame_buffer: FrameBuffer,
    keyboard: Keyboard,
}

impl Bus {
    pub fn new(memory: MainMemory) -> Self {
        Bus {
            memory,
            frame_buffer: FrameBuffer::new(),
            keyboard: Keyboard::new(),
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

    pub fn write_bytes(&mut self, address: Address, bytes: &[Byte]) {
        let mut addr = address;
        for byte in bytes {
            self.memory.write(addr, *byte);
            addr += 1;
        }
    }

    pub fn clear_screen(&mut self) {
        self.frame_buffer.clear_screen();
    }

    pub fn press_key(&mut self, key: Option<u8>) {
        self.keyboard.set_key_pressed(key);
    }

    pub fn get_key_pressed(&self) -> Option<u8> {
        self.keyboard.get_key_pressed()
    }

    pub fn display_sprite(&mut self, pos: (usize, usize), sprite: &[u8]) -> bool {
        self.frame_buffer.write_bytes(pos, sprite)
    }

    pub fn get_frame_buffer(&self) -> &FrameBuffer {
        &self.frame_buffer
    }

    pub fn get_ram(&self) -> &MainMemory {
        &self.memory
    }
}
