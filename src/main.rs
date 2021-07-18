use hex;
use std::fs;
use structopt::StructOpt;


mod cpu;
mod opcodes;


#[derive(StructOpt)]
struct Cli {
    filename: String
}

fn read_rom_hexadecimal<'a>(filename: &str) -> Vec<u8> {
    let contents = fs::read_to_string(filename).unwrap();
    let filtered: String = contents.chars().filter(|c| c.is_digit(16)).collect();
    hex::decode(filtered).unwrap()
}

fn main() {

    let args = Cli::from_args();
    let program = read_rom_hexadecimal(&args.filename);
    println! ("{:#04X?}", program);

    let mut cpu = cpu::CPU::new();
    cpu.load(&program); 
    cpu.run();
}