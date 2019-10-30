use std::path::{Path};

use crate::asm;

use crate::memory;
use crate::debugger;
use crate::window;
use crate::watcher;

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

pub fn emulate(path: &Path, debug: bool, verbose: bool) -> Result<(), error::CLIError> {
    let rom = memory::ROM::from_file(path)?;
    let mut watcher = watcher::Watcher::new();

    watcher.verbose = verbose;

    if debug {
        let mut debugger = debugger::Debugger::new(rom, watcher);
        debugger.run();
    } else {
        let mut window = window::Window::new(rom, watcher);
        window.run();
    }

    Ok(())
}

pub fn disassemble(path: &Path, address: bool) -> Result<(), error::CLIError> {
    let rom = memory::ROM::from_file(path)?;

    println!("{:-^20}", path.file_name().unwrap().to_str().unwrap());
    for (addr, instr) in rom.instructions().iter().enumerate() {
        let decoded = asm::decode_instruction(*instr);

        if address {
            print!("{:#05X} ", 0x200 + addr * 2);
        }
        println!("{}", decoded);
    }
    println!("{:-^20}", "");
    Ok(())
}
