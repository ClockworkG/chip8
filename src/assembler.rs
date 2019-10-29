use std::fmt;
use crate::specs::{Address};

pub type Bytecode = Vec<u8>;

#[derive(Debug)]
pub enum AssemblerError {
    ExpectedInstruction,
    ExpectedAddress,
}

impl fmt::Display for AssemblerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            AssemblerError::ExpectedInstruction => {
                write!(f, "An expression was expected.")?
            },
            AssemblerError::ExpectedAddress => {
                write!(f, "An address was expected.")?
            },
        }
        Ok(())
    }
}

#[derive(Debug)]
enum Mnemonic {
    SYS,
    CLS,
    RET,
    JP,
    CALL,
    SE,
    SNE,
    LD,
    ADD,
    OR,
    AND,
    XOR,
    SUB,
    SHR,
    SUBN,
    SHL,
    RND,
    DRW,
    SKP,
    SKNP,
}

#[derive(Debug)]
enum Token {
    Instruction(Mnemonic),
    Literal(u16),
    Register(u8),
    RegisterF,
    RegisterI,
    DerefRegisterI,
    RegisterB,
    RegisterST,
    RegisterDT,
    Unknown,
}

fn word_to_token(word: &str) -> Token {
    use Token::*;

    match word.to_lowercase().as_str() {
        "ld" => Instruction(Mnemonic::LD),
        "sys" => Instruction(Mnemonic::SYS),
        "cls" => Instruction(Mnemonic::CLS),
        "ret" => Instruction(Mnemonic::RET),
        "jp" => Instruction(Mnemonic::JP),
        "call" => Instruction(Mnemonic::CALL),
        "se" => Instruction(Mnemonic::SE),
        "sne" => Instruction(Mnemonic::SNE),
        "add" => Instruction(Mnemonic::ADD),
        "or" => Instruction(Mnemonic::OR),
        "and" => Instruction(Mnemonic::AND),
        "xor" => Instruction(Mnemonic::XOR),
        "sub" => Instruction(Mnemonic::SUB),
        "shr" => Instruction(Mnemonic::SHR),
        "subn" => Instruction(Mnemonic::SUBN),
        "shl" => Instruction(Mnemonic::SHL),
        "rnd" => Instruction(Mnemonic::RND),
        "drw" => Instruction(Mnemonic::DRW),
        "skp" => Instruction(Mnemonic::SKP),
        "sknp" => Instruction(Mnemonic::SKNP),
        "i" => RegisterI,
        "f" => RegisterF,
        "[i]" => DerefRegisterI,
        "b" => RegisterB,
        "st" => RegisterST,
        "dt" => RegisterDT,
        tok_word => {
            use regex::Regex;
            let re = Regex::new(r"v(?P<id>[0-9a-fA-F])").unwrap();
            if re.is_match(tok_word) {
                let captures = re.captures(tok_word).unwrap();
                let id = u8::from_str_radix(&captures["id"], 16).unwrap();
                return Register(id)
            }

            let lit_re = Regex::new(r"0x(?P<num>([0-9a-fA-F])+)").unwrap();
            if lit_re.is_match(tok_word) {
                let captures = lit_re.captures(tok_word).unwrap();
                let num = u16::from_str_radix(&captures["num"], 16).unwrap();
                return Literal(num)
            }

            return Unknown
        },
    }
}

fn fetch_addr(iter: &mut std::slice::Iter<'_, Token>) -> Result<Address, AssemblerError> {
    match iter.next() {
        None => Err(AssemblerError::ExpectedAddress),
        Some(tok) => {
            match tok {
                Token::Literal(lit) => Ok(*lit & 0x0FFF),
                _ => Err(AssemblerError::ExpectedAddress),
            }
        }
    }
}

fn tokens_to_bytecode(tokens: &[Token]) -> Result<Bytecode, AssemblerError> {
    use Token::*;

    let mut iter = tokens.iter();
    match iter.next().unwrap() {
        Instruction(mnem) => {
            use Mnemonic::*;

            fn push_bytes(bytes: &mut Bytecode, value: u16) {
                bytes.push(((value & 0xFF00) >> 8) as u8);
                bytes.push((value & 0x00FF) as u8);
            }

            let mut bytes = Bytecode::new();
            match mnem {
                RET => push_bytes(&mut bytes, 0x00EE),
                CLS => push_bytes(&mut bytes, 0x00E0),
                SYS => {
                    let addr = fetch_addr(&mut iter)?;
                    push_bytes(&mut bytes, addr);
                },
                JP => {
                    let addr = fetch_addr(&mut iter)? | 0x1000;
                    push_bytes(&mut bytes, addr);
                },
                CALL => {
                    let addr = fetch_addr(&mut iter)? | 0x2000;
                    push_bytes(&mut bytes, addr);
                },
                _ => {},
            };
            Ok(bytes)
        },
        _ => {
            Err(AssemblerError::ExpectedInstruction)
        }
    }
}

pub fn source_to_bytecode(source: &str) -> Result<Bytecode, AssemblerError> {
    let mut bytecode = Bytecode::new();
    for line in source.split("\n") {
        if line.is_empty() {
            continue;
        }

        let toks: Vec<Token> = line.split(" ")
                                   .map(|word| word_to_token(word))
                                   .collect();
        let mut instruction_bytecode = tokens_to_bytecode(&toks[..])?;
        bytecode.append(&mut instruction_bytecode);
    }

    Ok(bytecode)
}
