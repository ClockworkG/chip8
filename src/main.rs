mod specs;
mod memory;
mod cpu;

extern crate clap;

use clap::{App, Arg};

fn main() {
    let matches = App::new("Chip8")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Robin Le Bihan")
        .about("Tools to manipulate and emulate Chip8")
        .subcommand(
            App::new("vm")
                .about("runs the Chip8 virtual machine")
                .arg(Arg::from_usage("<rom> 'ROM file to run.'")),
        )
        .subcommand(
            App::new("asm")
                .about("assembles code to a Chip8 bytecode")
                .arg(Arg::from_usage("<code> 'Code to assemble.'")),
        )
        .subcommand(
            App::new("dis")
                .about("disassembles bytecode to code.")
                .arg(Arg::from_usage("<rom> 'ROM file to disassemble'"))
        )
        .get_matches();

    if let Some(ref _matches) = matches.subcommand_matches("vm") {
        // FIXME...
    } else if let Some(ref _matches) = matches.subcommand_matches("asm") {
        // FIXME...
    } else if let Some(ref _matches) = matches.subcommand_matches("dis") {
        // FIXME...
    }
}
