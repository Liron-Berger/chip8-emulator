mod cpu;

use std::env;

pub use cpu::Cpu;

fn parse_args(args: &Vec<String>) -> &String {
    match args.len() {
        2 => {
            return &args[1];
        },
        _ => {
            panic!("You should add a filename to run");
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();


    let chip = Cpu::new();
    let offset = 0x200;

    println!("{:?}", chip.memory[offset + 10]);
}
