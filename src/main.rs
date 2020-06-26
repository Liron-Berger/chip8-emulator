mod cpu;

use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;

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

fn read_program(file_name: &String) -> Vec<u8> {
    let mut file = File::open(&file_name).expect("File not found");
    let metadata = fs::metadata(&file_name).expect("Unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    file.read(&mut buffer).expect("Buffer overflow");
    buffer
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name: &String = parse_args(&args);
    let program = read_program(file_name);

    let mut chip = Cpu::new();
    chip.load_program(program);
    println!("{:?}", chip);

}
