extern crate rustyline;

use crate::context::Context;
use crate::cpu::CPU;
use crate::memory::{MainMemory, ROM};
use crate::bus::Bus;

use rustyline::error::ReadlineError;
use rustyline::Editor;

static PROMPT: &str = "(chip8-debug) ";

pub struct Debugger {
    cpu: CPU,
    bus: Bus,
    must_exit: bool,
    editor: Editor<()>
}

impl Debugger {
    fn prompt(&mut self) -> Result<String, ()> {
        let readline = self.editor.readline(PROMPT);
        match readline {
            Ok(line) => {
                self.editor.add_history_entry(line.as_str());
                Ok(line)
            },
            Err(_) => Err(())
        }
    }

    fn process_input(&mut self, input: &str) {
        match input {
            "status" => println!("{}", self.cpu),
            "next" => { self.cpu.tick(&mut self.bus); },
            "exit" => self.must_exit = true,
            _ => {}
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
            bus: Bus::new(mem)
        }
    }

    fn run(&mut self) {
        while let Ok(line) = self.prompt() {
            self.process_input(&line);
            if self.must_exit {
                break
            }
        }
    }
}
