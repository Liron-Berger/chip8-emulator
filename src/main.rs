use std::fs;
use std::fs::File;
use std::io::Read;
use clap::{Arg, App};

use emulator::Emulator;

mod cpu;
mod emulator;
mod instructions;
mod display;

mod drivers;
mod utils;

fn parse_args() -> (String, bool) {
    let matches = App::new("My Test Program")
        .version("0.1.0")
        .author("Liron-Berger <lironberger13@gmail.com>")
        .about("An emulator for Chip8 written in Rust")
        .arg(Arg::with_name("rom")
            .short("r")
            .long("rom")
            .takes_value(true)
            .required(true)
            .help("The chip8 rom to emulate"))
        .arg(Arg::with_name("debug")
            .default_value("false")
            .short("d")
            .long("debug")
            .possible_values(&["true", "false"])
            .help("Whether to run in debug mode - print CPU state to console every tick"))
        .get_matches();

    let rom = matches.value_of("rom").unwrap();
    let debug: bool = matches.value_of("debug").unwrap().parse().unwrap();
    (String::from(rom), debug)
}

fn read_program(file_name: &String) -> Vec<u8> {
    let mut file = File::open(&file_name).expect("File not found");
    let metadata = fs::metadata(&file_name).expect("Unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    file.read(&mut buffer).expect("Buffer overflow");
    buffer
}

fn main() {
    let (rom, debug) = parse_args();
    let program = read_program(&rom);
    let mut emulator = Emulator::new();
    emulator.load_program(program);
    emulator.run(debug);
}
