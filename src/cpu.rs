extern crate termion;

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
    merge_bytes,
    Memory,
};
use crate::asm::{
    InstructionData,
    decode_instruction,
};
use crate::bus::Bus;
use std::fmt;
use rand::Rng;
use rand::prelude::ThreadRng;

pub struct CPU {
    i: Register<Address>,
    pc: Register<Address>,
    registers: [Register<Byte>; REGISTERS_COUNT],

    stack: [Address; STACK_SIZE],
    sp: Register<Byte>,

    delay_timer: Byte,
    sound_timer: Byte,

    random_device: ThreadRng,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            i: 0x0,
            pc: PROGRAM_BEGIN as Address,
            registers: [0x0; REGISTERS_COUNT],
            stack: [0x0; STACK_SIZE],
            sp: 0x0,
            delay_timer: 0,
            sound_timer: 0,
            random_device: rand::thread_rng(),
        }
    }

    fn fetch(&mut self, bus: &mut Bus) -> Instruction {
        let left = bus.read(self.pc);
        let right = bus.read(self.pc + 1);
        self.pc += 2;

        merge_bytes(left, right)
    }

    fn decode(&mut self, instr: Instruction) -> InstructionData {
        decode_instruction(instr)
    }

    fn execute(&mut self, data: InstructionData, bus: &mut Bus) -> Address {
        use InstructionData::*;

        match data {
            Cls => (),
            Ret => {
                self.pc = self.stack[self.sp as usize];
                self.sp -= 1;
            },
            Jp(n) => self.pc = n,
            Call(n) => {
                self.sp += 1;
                self.stack[self.sp as usize] = self.pc;
                self.pc = n;
            },
            LdI(n) => self.i = n,
            Rnd(x, n) => {
                let random: u16 = self.random_device.gen_range(0, 256);
                self.registers[x as usize] = (random as u8) & n;
            },
            Se(x, n) => {
                if self.registers[x as usize] == n {
                    self.pc += 2;
                }
            },
            _ => {}
        }

        self.pc
    }

    pub fn tick(&mut self, bus: &mut Bus) -> Address {
        let instr = self.fetch(bus);
        let data = self.decode(instr);
        self.execute(data, bus)
    }
}

impl fmt::Display for CPU {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use termion::{color, style};

        write!(f, "I = {}{:#05x}{}, PC = {}{:#05x}{}\n",
               style::Bold, self.i, style::Reset,
               style::Bold, self.pc, style::Reset)?;

        write!(f, "DT = {}{:#04x}{}, ST = {}{:#04x}{}\n",
               style::Bold, self.delay_timer, style::Reset,
               style::Bold, self.sound_timer, style::Reset)?;
        write!(f, "\n")?;

        for (idx, regs) in self.registers.chunks(2).enumerate() {
            let (left, right) = (regs[0], regs[1]);
            write!(f, "V{:1X} = {}{:#04x}{}, V{:1X} = {}{:#04x}{}\n",
                   idx * 2, style::Bold, left, style::Reset,
                   idx * 2 + 1, style::Bold, right, style::Reset)?;
        }
        write!(f, "\nStack: ")?;

        for (idx, val) in self.stack.iter().enumerate() {
            if (idx as u8) == self.sp {
                write!(f, "[ {}{:#05x}{} ]",
                       style::Bold, val, style::Reset)?;
            } else {
                if (idx as u8) < self.sp {
                    write!(f, "{}", color::Fg(color::Green))?;
                } else {
                    write!(f, "{}", color::Fg(color::Red))?;
                }
                write!(f, " {:#05x}{} ", val, style::Reset)?;
            }
        }

        Ok(())
    }
}
