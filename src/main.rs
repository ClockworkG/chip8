mod asm;
mod specs;
mod memory;
mod cpu;
mod cli;
mod context;
mod debugger;
mod bus;
mod display;

extern crate clap;

use clap::{App, Arg};
use std::path::Path;

fn main() {
    let matches = App::new("Chip8")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Robin Le Bihan")
        .about("Tools to manipulate and emulate Chip8")
        .subcommand(
            App::new("vm")
                .about("runs the Chip8 virtual machine")
                .arg(Arg::from_usage("-g, --debug 'enables debugging mode'"))
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
                .arg(Arg::from_usage("-n 'enables display of addresses'"))
        )
        .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("vm") {
        let path = Path::new(matches.value_of("rom").unwrap());
        let debug_mode = matches.is_present("debug");

        cli::emulate(&path, debug_mode).unwrap();
    } else if let Some(ref _matches) = matches.subcommand_matches("asm") {
        // FIXME...
    } else if let Some(ref matches) = matches.subcommand_matches("dis") {
        let path = Path::new(matches.value_of("rom").unwrap());
        let display_address = matches.is_present("n");

        cli::disassemble(&path, display_address).unwrap();
    }
}
