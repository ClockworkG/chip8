use crate::specs::{
    Register,
    Byte,
    Address,
    Instruction,
    REGISTERS_COUNT,
    STACK_SIZE,
    PROGRAM_BEGIN,
};
use crate::memory::{
    Memory,
    MainMemory,
    merge_bytes,
};
use crate::asm::{
    InstructionData,
    decode_instruction,
};
use std::fmt;

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
            pc: PROGRAM_BEGIN as Address,
            registers: [0x0; REGISTERS_COUNT],
            stack: [0x0; STACK_SIZE],
            sp: 0x0,
            memory: memory,
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    fn fetch(&mut self) -> Instruction {
        let left = self.memory.read(self.pc);
        let right = self.memory.read(self.pc + 1);

        merge_bytes(left, right)
    }

    fn decode(&mut self, instr: Instruction) -> InstructionData {
        decode_instruction(instr)
    }

    fn execute(&mut self, data: InstructionData) {
        // FIXME
        self.pc += 2;
    }

    pub fn tick(&mut self) {
        let instr = self.fetch();
        let data = self.decode(instr);
        self.execute(data);
    }
}

impl fmt::Display for CPU {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "I = {:#05x}, PC = {:#05x}\n", self.i, self.pc)?;
        write!(f, "DT = {:#04x}, ST = {:#04x}\n", self.delay_timer, self.sound_timer)?;
        write!(f, "\n")?;

        for (idx, regs) in self.registers.chunks(2).enumerate() {
            let (left, right) = (regs[0], regs[1]);
            write!(f, "V{:1X} = {:#04x}, V{:1X} = {:#04x}\n",
                   idx * 2, left, idx * 2 + 1, right)?;
        }
        write!(f, "\nStack: ")?;

        for (idx, val) in self.stack.iter().enumerate() {
            if (idx as u8) == self.sp {
                write!(f, "[ {:#05x} ]", val)?;
            } else {
                write!(f, " {:#05x} ", val)?;
            }
        }

        Ok(())
    }
}
