use crate::specs::{Register, Byte, Address, REGISTERS_COUNT, STACK_SIZE};
use crate::memory::MainMemory;

pub struct CPU {
    i: Register<Address>,
    pc: Register<Address>,
    registers: [Register<Byte>; REGISTERS_COUNT],

    stack: [Address; STACK_SIZE],
    sp: Register<Byte>,

    memory: MainMemory,

    delay_timer: Byte,
    sound_timer: Byte,
}

impl CPU {
    pub fn new(memory: MainMemory) -> Self {
        CPU {
            i: 0x0,
            pc: 0x0,
            registers: [0x0; REGISTERS_COUNT],
            stack: [0x0; STACK_SIZE],
            sp: 0x0,
            memory: memory,
            delay_timer: 0,
            sound_timer: 0,
        }
    }
}
