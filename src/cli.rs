use std::path::{Path};

use crate::asm;
use crate::cpu;
use crate::memory;
use crate::debugger;
use crate::context::Context;

pub mod error {
    use std::io;

    #[derive(Debug)]
    pub enum CLIError {
        IOError(io::Error),
    }

    impl From<io::Error> for CLIError {
        fn from(err: io::Error) -> Self {
            CLIError::IOError(err)
        }
    }
}

pub fn emulate(path: &Path, debug: bool) -> Result<(), error::CLIError> {
    let rom = memory::ROM::from_file(path)?;

    if debug {
        let mut debugger = debugger::Debugger::new(rom);
        debugger.run();
    }

    Ok(())
}

pub fn disassemble(path: &Path, address: bool) -> Result<(), error::CLIError> {
    let rom = memory::ROM::from_file(path)?;

    println!("{:-^20}", path.file_name().unwrap().to_str().unwrap());
    for (addr, instr) in rom.instructions().iter().enumerate() {
        let decoded = asm::decode_instruction(*instr);

        if address {
            print!("{:#05X} ", addr * 2);
        }
        println!("{}", decoded);
    }
    println!("{:-^20}", "");
    Ok(())
}
