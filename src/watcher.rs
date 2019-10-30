use crate::asm::InstructionData;
use crate::specs::{Address, Byte, Nibble};

pub struct Watcher {
    pub verbose: bool,
}

#[derive(Debug)]
pub enum Message {
    Execute { instr: InstructionData, new_pc: Address },
    RegisterChange { id: Nibble, old: Byte, new: Byte },
}

impl Watcher {
    pub fn new() -> Self {
        Watcher {
            verbose: false,
        }
    }

    pub fn send(&self, msg: Message) {
        use Message::*;

        match msg {
            Execute { instr, new_pc } => {
                if self.verbose {
                    println!("Executed: {}. PC is now at: {:#05X}", instr, new_pc);
                }
            },
            RegisterChange { id, old, new } => {
                if self.verbose {
                    println!("Register V{:01X} changed: {:#04X} => {:#04X}",
                             id, old, new);
                }
            },
        }
    }
}
