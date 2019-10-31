extern crate rustyline;
extern crate termion;

use crate::cpu::CPU;
use crate::memory::{MainMemory, ROM};
use crate::bus::Bus;
use crate::specs::{PROGRAM_BEGIN, Address};
use crate::asm::{decode_instruction, InstructionData};
use crate::watcher::Watcher;

use rustyline::Editor;
use std::collections::HashMap;

static PROMPT: &str = "(chip8-debug)";

pub struct Debugger {
    cpu: CPU,
    bus: Bus,
    must_exit: bool,
    editor: Editor<()>,
    current_pc: Address,
    need_input: bool,
    breakpoints: Vec<Address>,
    variables: HashMap<String, i32>,
}

enum DebuggerCommand {
    Break(Address),
    Set(String, i32),
    Continue,
    Run,
    Status,
    Next,
    Ctx,
    Screen,
    Empty,
    Quit,
    Dump,
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
                Ok(line.trim_end().to_owned())
            },
            Err(_) => Err(())
        }
    }

    fn parse_input(line: &str) -> Result<DebuggerCommand, String> {
        use DebuggerCommand::*;

        let mut tokens = line.split(" ").into_iter();

        if let Some(tok) = tokens.next() {
            Ok(match tok {
                "ctx" => Ctx,
                "c" | "continue" => Continue,
                "next" | "n" => Next,
                "run" | "r" => Run,
                "dump" => Dump,
                "screen" => Screen,
                "status" => Status,
                "quit" | "exit" | "q" => Quit,
                "set" => {
                    if let Some(tok) = tokens.next() {
                        let key = tok.to_owned();
                        if let Some(tok) = tokens.next() {
                            let value = i32::from_str_radix(tok, 10);
                            if let Ok(val) = value {
                                Set(key, val)
                            } else {
                                return Err("Unable to parse value.".to_owned())
                            }
                        } else {
                            return Err("Missing value after key.".to_owned())
                        }
                    } else {
                        return Err("Missing key after set".to_owned())
                    }
                },
                "break" | "b" => {
                    if let Some(tok) = tokens.next() {
                        let processed = tok.trim_start_matches("0x");
                        let parsed = u16::from_str_radix(processed, 16);
                        if let Ok(addr) = parsed {
                            Break(addr)
                        } else {
                            return Err("Error while parsing address.".to_owned())
                        }
                    } else {
                        return Err("Missing argument after break".to_owned())
                    }
                },
                "" => Empty,
                _ => return Err(format!("Unknown command: {}", tok)),
            })
        } else {
            Err("Unknown error".to_owned())
        }
    }

    fn get_execution_context(&self) -> Vec<(Address, InstructionData)> {
        let mut instrs: Vec<(Address, InstructionData)> = Vec::new();
        let size = self.variables["context_span"] as u16;

        let mut addr = if size * 2 > self.current_pc {
            0
        } else {
            self.current_pc - (size * 2)
        };
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

        let instrs = self.get_execution_context();
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
        use DebuggerCommand::*;

        let res = Debugger::parse_input(input);
        match res {
            Ok(cmd) => {
                match cmd {
                    Empty => {},
                    Status => println!("{}", self.cpu),
                    Run => {
                        self.need_input = false;
                        self.cpu.reset();
                    },
                    Ctx => self.show_context(),
                    Dump => println!("{}", self.bus.get_ram()),
                    Next => {
                        self.current_pc = self.cpu.tick(&mut self.bus);
                        self.show_context();
                    },
                    Screen => println!("{}", self.bus.get_frame_buffer()),
                    Continue => self.need_input = false,
                    Quit => {
                        self.must_exit = true;
                    },
                    Break(addr) => {
                        println!("Setting breakpoint at {:#05x}.", addr);
                        self.breakpoints.push(addr);
                    },
                    Set(key, value) => {
                        self.variables.insert(key, value);
                    },
                }
            },
            Err(err) => {
                println!("{}", err);
            }
        }
    }

    pub fn new(rom: ROM, watcher: Watcher) -> Self {
        let mem = MainMemory::with_rom(rom);
        let mut variables = HashMap::new();

        variables.insert("context_span".to_owned(), 2);

        Debugger {
            cpu: CPU::new(watcher),
            editor: Editor::<()>::new(),
            must_exit: false,
            bus: Bus::new(mem),
            need_input: true,
            current_pc: PROGRAM_BEGIN as u16,
            breakpoints: Vec::new(),
            variables
        }
    }

    pub fn run(&mut self) {
        self.show_context();
        loop {
            if self.need_input {
                if let Ok(line) = self.prompt() {
                    self.process_input(&line);
                }
            } else {
                self.current_pc = self.cpu.tick(&mut self.bus);
                if self.breakpoints.contains(&self.current_pc) {
                    self.need_input = true;
                    self.show_context();
                    println!("Stopped on breakpoint at {:#05X}.", self.current_pc);
                }

            }

            if self.must_exit {
                break
            }
        }
    }
}
