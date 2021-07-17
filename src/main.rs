mod cpu;
mod opcodes;

fn main() {
    let mut cpu = cpu::CPU::new();

    let program : [u8; 6] = [
        0x80, 0x14,
        0x80, 0x24,
        0x80, 0x34];
    cpu.load(&program); 
    cpu.run();


}