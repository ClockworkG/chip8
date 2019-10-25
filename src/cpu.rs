extern crate termion;

use crate::specs::{
    Register,
    Byte,
    Address,
    Nibble,
    Instruction,
    REGISTERS_COUNT,
    STACK_SIZE,
    PROGRAM_BEGIN,
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

    pub fn reset(&mut self) {
        self.i = 0x0;
        self.pc = PROGRAM_BEGIN as Address;
        self.registers = [0x0; REGISTERS_COUNT];
        self.stack = [0x0; STACK_SIZE];
        self.sp = 0x0;
        self.delay_timer = 0;
        self.sound_timer = 0;
    }

    fn get_reg(&self, index: Nibble) -> Byte {
        self.registers[index as usize]
    }

    fn set_reg(&mut self, index: Nibble, value: Byte) {
        self.registers[index as usize] = value;
    }

    fn fetch(&mut self, bus: &mut Bus) -> Instruction {
        let instr = bus.read_instruction(self.pc);
        self.pc += 2;
        instr
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
            Se(x, n) => {
                if self.get_reg(x) == n {
                    self.pc += 2;
                }
            },
            Sne(x, n) => {
                if self.get_reg(x) != n {
                    self.pc += 2;
                }
            },
            SeReg(x, y) => {
                if self.get_reg(x) == self.get_reg(y) {
                    self.pc += 2;
                }
            },
            Ld(x, n) => self.registers[x as usize] = n,
            Add(x, n) => {
                self.set_reg(x, self.get_reg(x) + n);
            },
            LdReg(x, y) => {
                self.set_reg(x, self.get_reg(y));
            },
            Or(x, y) => {
                self.set_reg(x, self.get_reg(x) | self.get_reg(y));
            },
            And(x, y) => {
                self.set_reg(x, self.get_reg(x) & self.get_reg(y));
            },
            Xor(x, y) => {
                self.set_reg(x, self.get_reg(x) ^ self.get_reg(y));
            },
            AddReg(x, y) => {
                let sum: u16 = self.get_reg(x) as u16 + self.get_reg(y) as u16;
                let carry = sum % 256;
                self.set_reg(x, (sum - carry) as u8);
                self.set_reg(0xF, carry as u8);
            },
            SubReg(x, y) => {
                let x_val = self.get_reg(x);
                let y_val = self.get_reg(y);
                if x_val > y_val {
                    self.set_reg(0xF, 1);
                }
                self.set_reg(x, x_val - y_val);
            },
            Shr(x, _) => {
                let x_val = self.get_reg(x);
                if x_val & 0b1 == 1 {
                    self.set_reg(0xF, 1);
                }
                self.set_reg(x, x_val / 2);
            },
            SubN(x, y) => {
                let x_val = self.get_reg(x);
                let y_val = self.get_reg(y);
                if y_val > x_val {
                    self.set_reg(0xF, 1);
                }
                self.set_reg(x, y_val - x_val);
            },
            Shl(x, _) => {
                let x_val = self.get_reg(x);
                if x_val & 0b1000000 == 0b1000000 {
                    self.set_reg(0xF, 1);
                }
                self.set_reg(x, x_val * 2);
            },
            SneReg(x, y) => {
                if self.get_reg(x) != self.get_reg(y) {
                    self.pc += 2;
                }
            },
            LdI(n) => self.i = n,
            JpV0(n) => {
                self.pc = self.get_reg(0) as u16 + n;
            },
            Rnd(x, n) => {
                let random: u16 = self.random_device.gen_range(0, 256);
                self.set_reg(x, (random as u8) & n);
            },
            Drw(x, y, n) => {
                let x_val = self.get_reg(x) as usize;
                let y_val = self.get_reg(y) as usize;
                let bytes = bus.read_bytes(self.i, n as Address);
                bus.display_sprite((x_val, y_val), &bytes[..]);
            },
            Skp(x) => {
                // FIXME
            },
            Sknp(x) => {
                // FIXME
            },
            LdF(x) => {
                let font_index: Byte = self.get_reg(x);
                self.i = (font_index * 5) as u16;
            },
            LdRegDt(x) => {
                self.set_reg(x, self.delay_timer);
            },
            LdK(n) => {
                // FIXME
            },
            LdDtReg(x) => {
                self.delay_timer = self.get_reg(x);
            },
            LdSt(x) => {
                self.sound_timer = self.get_reg(x);
            },
            AddI(x) => {
                self.i = self.i + self.get_reg(x) as u16;
            },
            LdB(n) => {
                // FIXME
            },
            LdIMem(x) => {
                let loc = self.i;
                bus.write_bytes(loc, &self.registers[0..x as usize]);
            },
            LdVx(x) => {
                let loc = self.i;
                let bytes = bus.read_bytes(loc, x as Address);
                for (idx, byte) in bytes.iter().enumerate() {
                    self.set_reg(idx as u8, *byte);
                }
            },
            Unknown => panic!("Illegal instruction, aborting..."),
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
