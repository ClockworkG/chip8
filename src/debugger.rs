extern crate rustyline;

use crate::context::Context;
use crate::cpu::CPU;

use rustyline::error::ReadlineError;
use rustyline::Editor;

static PROMPT: &str = "(chip8-debug) ";

pub struct Debugger<'a> {
    cpu: &'a mut CPU,
    must_exit: bool,
    editor: Editor<()>
}

impl<'a> Debugger<'a> {
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
            "next" => self.cpu.tick(),
            "exit" => self.must_exit = true,
            _ => {}
        }
    }
}

impl<'a> Context<'a> for Debugger<'a> {
    fn with_cpu(cpu: &'a mut CPU) -> Self {
        Debugger {
            cpu,
            editor: Editor::<()>::new(),
            must_exit: false,
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
