use std::fmt;

pub type Bytecode = Vec<u8>;

#[derive(Debug)]
pub enum AssemblerError {
    ExpectedInstruction,
}

impl fmt::Display for AssemblerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            AssemblerError::ExpectedInstruction => {
                write!(f, "An expression was expected.")?
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
            let re = Regex::new(r"v(?P<id>[0-9a-f])").unwrap();
            if re.is_match(tok_word) {
                let captures = re.captures(tok_word).unwrap();
                let id = u8::from_str_radix(&captures["id"], 16).unwrap();
                Register(id)
            } else {
                Unknown
            }
        },
    }
}

fn tokens_to_bytecode(tokens: &[Token]) -> Result<Bytecode, AssemblerError> {
    use Token::*;

    let mut iter = tokens.iter();
    match iter.next().unwrap() {
        Instruction(mnem) => {
            use Mnemonic::*;

            fn push_bytes(bytes: &mut Bytecode, value: u16) {
                bytes.push((value & 0xFF00 >> 12) as u8);
                bytes.push((value & 0x00FF) as u8);
            }

            let mut bytes = Bytecode::new();
            match mnem {
                RET => push_bytes(&mut bytes, 0x00EE),
                CLS => push_bytes(&mut bytes, 0x00E0),
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
