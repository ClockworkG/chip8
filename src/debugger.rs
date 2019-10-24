extern crate rustyline;
extern crate termion;

use crate::context::Context;
use crate::cpu::CPU;
use crate::memory::{MainMemory, ROM};
use crate::bus::Bus;
use crate::specs::{PROGRAM_BEGIN, Address};
use crate::asm::{decode_instruction, InstructionData};

use rustyline::Editor;

static PROMPT: &str = "(chip8-debug)";

pub struct Debugger {
    cpu: CPU,
    bus: Bus,
    must_exit: bool,
    editor: Editor<()>,
    current_pc: Address,
    need_input: bool,
}

impl Debugger {
    fn prompt(&mut self) -> Result<String, ()> {
        use termion::{color, style};

        let readline = self.editor.readline(
            format!("{}{}{} ", color::Fg(color::Blue), PROMPT, style::Reset).as_str()
        );
        match readline {
            Ok(line) => {
                self.editor.add_history_entry(line.as_str());
                Ok(line)
            },
            Err(_) => Err(())
        }
    }

    fn get_execution_context(&self, size: u16) -> Vec<(Address, InstructionData)> {
        let mut instrs: Vec<(Address, InstructionData)> = Vec::new();

        let mut addr = self.current_pc - (size * 2);
        while addr <= self.current_pc + (size * 2) {
            if addr < (PROGRAM_BEGIN as u16) {
                addr += 2;
                continue;
            }

            let instr = self.bus.read_instruction(addr);
            instrs.push((addr, decode_instruction(instr)));

            addr += 2;
        }

        instrs
    }

    fn show_context(&self) {
        use termion::{color, style};

        let instrs = self.get_execution_context(1);
        for (addr, instr) in instrs {
            if addr == self.current_pc {
                print!("{:<4}{}", "->", color::Fg(color::Green));
            } else {
                print!("{:<4}", "");
            }

            println!("{:#05X}   {}{}", addr, instr, style::Reset);
        }
    }

    fn process_input(&mut self, input: &str) {
        match input {
            "status" => println!("{}", self.cpu),
            "run" => {
                self.need_input = false;
            },
            "next" => {
                self.current_pc = self.cpu.tick(&mut self.bus);
                self.show_context();
            },
            "ctx" => {
                self.show_context();
            },
            "dump" => {
                let mem = self.bus.get_ram();
                println!("{}", mem);
            },
            "screen" => {
                let buffer = self.bus.get_frame_buffer();
                println!("{}", buffer);
            },
            "exit" | "quit" => self.must_exit = true,
            _ => {
                println!("Command not found: {}", input);
            }
        }
    }
}

impl Context for Debugger {
    fn new(rom: ROM) -> Self {
        let mem = MainMemory::with_rom(rom);
        Debugger {
            cpu: CPU::new(),
            editor: Editor::<()>::new(),
            must_exit: false,
            bus: Bus::new(mem),
            need_input: true,
            current_pc: PROGRAM_BEGIN as u16
        }
    }

    fn run(&mut self) {
        self.show_context();
        loop {
            if self.need_input {
                if let Ok(line) = self.prompt() {
                    self.process_input(&line);
                }
            } else {
                self.cpu.tick(&mut self.bus);
            }

            if self.must_exit {
                break
            }
        }
    }
}
