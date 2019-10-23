use std::path::{Path};
use std::fs::File;
use std::io::Read;

use crate::asm;

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

pub fn disassemble(path: &Path, address: bool) -> Result<(), error::CLIError> {
    let mut file = File::open(path)?;
    let mut bytes = Vec::new();

    file.read_to_end(&mut bytes)?;

    fn merge_bytes(left: u8, right: u8) -> u16 {
        ((left as u16) << 8) | (right as u16)
    }

    let instructions: Vec<u16> = bytes.chunks(2)
                                      .map(|word| {
                                        let (left, right) = (word[0], word[1]);
                                        merge_bytes(left, right)
                                      })
                                      .collect();

    println!("{:-^20}", path.file_name().unwrap().to_str().unwrap());
    for (addr, instr) in instructions.iter().enumerate() {
        let decoded = asm::decode_instruction(*instr);

        if address {
            print!("{:#05X} ", addr * 2);
        }
        println!("{}", decoded);
    }
    println!("{:-^20}", "");
    Ok(())
}
