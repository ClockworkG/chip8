use std::path::{Path};
use std::fs::File;
use std::io::Read;

use crate::asm;
use crate::cpu;
use crate::memory;

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

pub fn emulate(path: &Path, _debug: bool) -> Result<(), error::CLIError> {
    let rom = memory::ROM::from_file(path)?;
    let _memory = memory::MainMemory::with_rom(rom);
    let _cpu = cpu::CPU::new(_memory);

    // FIXME...
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
