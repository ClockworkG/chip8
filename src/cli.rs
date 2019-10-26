use std::path::{Path};
use std::io::Write;

use crate::asm;
use crate::assembler;

use crate::memory;
use crate::debugger;
use crate::window;
use crate::context::Context;

use std::fs;

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
    } else {
        let mut window = window::Window::new(rom);
        window.run();
    }

    Ok(())
}

pub fn assemble(path: &Path, output: &Path) -> Result<(), error::CLIError> {
    let source = fs::read_to_string(path)?;
    let bytecode = assembler::source_to_bytecode(source.as_str());

    match bytecode {
        Ok(bytecode) => {
            let mut output_file = fs::File::create(output)?;
            output_file.write(&bytecode[..])?;
        },
        Err(err) => {
            eprintln!("{}", err);
        },
    };

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
