
mod opcodes;

struct CPU {
    registers: [u8; 16],
    position_in_memory: usize,
    memory: [u8; 4096],
    stack: [u16; 16],
    stack_pointer: usize,
    screen: [[bool; 64]; 32],
}

impl CPU {
    
    fn new() -> CPU {
        CPU {
            registers: [0; 16],
            memory: [0; 4096],
            position_in_memory: 0,
            stack: [0; 16],
            stack_pointer: 0,
            screen: [[false; 64]; 32],
        }
    }

    fn reset(&mut self) {
        self.registers = [0; 16];
        self.memory = [0; 4096];
        self.position_in_memory = 0;
        self.stack = [0; 16];
        self.stack_pointer = 0;
    }

    fn load(&mut self, program: &[u8]) {
        self.memory[0..program.len()].clone_from_slice(program);
    }

    fn run(&mut self) {
        loop {
            let op_byte1 = self.memory[self.position_in_memory] as u16;
            let op_byte2 = self.memory[self.position_in_memory + 1] as u16;
            let opcode = op_byte1 << 8 | op_byte2;

            let decoded = opcodes::decode(opcode);

            println!("{:?}", decoded);
            self.position_in_memory += 2;
            match decoded {
                opcodes::Opcode::Halt => return,
                opcodes::Opcode::Unimplemented => unimplemented!(),
                opcodes::Opcode::Cls => self.cls(),
                opcodes::Opcode::SeVX{x,b} => self.se_vx(x ,b),
                opcodes::Opcode::AddVX{x,b} => self.add_vx(x ,b),
                opcodes::Opcode::LdXY{x,y} => self.ld_xy(x, y),
                opcodes::Opcode::AddXY{x,y} => self.add_xy(x, y),
                opcodes::Opcode::SubXY{x,y} => self.sub_xy(x, y),
                _ => unimplemented!(),
            };
        }
    }

    fn cls(&mut self) {
        self.screen = [[false; 64]; 32];
    }

    fn se_vx(&mut self, x: u8, byte: u8) {
        self.registers[x as usize] = byte;
    }

    fn add_vx(&mut self, x: u8, byte: u8) {
        let (sum, _) = self.registers[x as usize].overflowing_add(byte);
        self.registers[x as usize] = sum;
    }

    fn ld_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] = self.registers[y as usize];
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let (sum, overflow) = self.registers[x as usize].overflowing_add(self.registers[y as usize]);

        if overflow {
            self.registers[15] = 1;
        }
        self.registers[x as usize] = sum;
    }

    fn sub_xy(&mut self, x: u8, y: u8) {
        let (sum, overflow) = self.registers[x as usize].overflowing_sub(self.registers[y as usize]);

        if overflow {
            self.registers[15] = 1;
        }
        self.registers[x as usize] = sum;

    }
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_load() {
        let program : [u8; 4] = [
            0x01, 0x02, 0x03, 0x04
        ];

        let mut cpu = CPU::new();

        cpu.load(&program);

        assert_eq!(cpu.memory[0], 0x01);
        assert_eq!(cpu.memory[1], 0x02);
        assert_eq!(cpu.memory[2], 0x03);
        assert_eq!(cpu.memory[3], 0x04);
        assert_eq!(cpu.memory[4], 0x00);
    }

    #[test]
    fn test_cls() {
        let mut cpu = CPU::new();
        let program : [u8; 2] = [0x00, 0xE0];
        cpu.load(&program);

        // turn on a few pixels
        cpu.screen[10][10] = true;
        cpu.screen[11][11] = true;
        cpu.screen[12][12] = true;

        cpu.run();
        assert_eq!(cpu.screen, [[false; 64]; 32]);
    }

    #[test]
    fn test_ld() {
        let program : [u8; 2] = [0x80, 0x10];

        let mut cpu = CPU::new();
        cpu.load(&program);

        cpu.registers[0] = 1;
        cpu.registers[1] = 2;

        cpu.run();

        assert_eq!(cpu.registers[0], 2);
    }

    #[test]
    fn test_se_vx() {
        let mut cpu = CPU::new();
        cpu.registers[0] = 5;

        let program : [u8; 2] = [0x6C, 0x01];
        cpu.load(&program); 
        cpu.run();

        assert_eq!(cpu.registers[0xC], 0x01);
    }

    #[test]
    fn test_add_vx() {
        let mut cpu = CPU::new();
        cpu.registers[0] = 5;

        let program : [u8; 4] = [
            0x70, 0x01,
            0x70, 0x02];
        cpu.load(&program); 
        cpu.run();

        assert_eq!(cpu.registers[0], 0x08);

        // test overflow
        cpu.reset();
        cpu.registers[0] = 0x01;
        let program : [u8; 2] = [0x70, 0xFF];
        cpu.load(&program);
        cpu.run();    

        assert_eq!(cpu.registers[0], 0x00);
    }

    #[test]
    fn test_add_xy() {
        let mut cpu = CPU::new();
        cpu.registers[0] = 5;
        cpu.registers[1] = 10;
        cpu.registers[2] = 10;
        cpu.registers[3] = 10;

        let program : [u8; 6] = [
            0x80, 0x14,
            0x80, 0x24,
            0x80, 0x34];
        cpu.load(&program); 
        cpu.run();

        assert_eq!(cpu.registers[0], 35);

        // check overflow
        cpu.reset();
        cpu.registers[0] = 5;
        cpu.registers[1] = 10;
        cpu.registers[2] = 10;
        cpu.registers[3] = 235;

        cpu.load(&program); 
        cpu.run();

        assert_eq!(cpu.registers[0], 4);
        assert_eq!(cpu.registers[15], 1);
    }

    #[test]
    fn test_sub_xy() {
        let mut cpu = CPU::new();
        cpu.registers[0] = 100;
        cpu.registers[1] = 10;
        cpu.registers[2] = 10;
        cpu.registers[3] = 10;

        let program : [u8; 6] = [
            0x80, 0x15,
            0x80, 0x25,
            0x80, 0x35];
        cpu.load(&program); 
        cpu.run();

        assert_eq!(cpu.registers[0], 70);

        // check underflow
        cpu.reset();
        cpu.registers[0] = 100;
        cpu.registers[1] = 90;
        cpu.registers[2] = 5;
        cpu.registers[3] = 6;

        let program : [u8; 6] = [
            0x80, 0x15,
            0x80, 0x25,
            0x80, 0x35];
        cpu.load(&program); 
        cpu.run();

        assert_eq!(cpu.registers[0], 255);
        assert_eq!(cpu.registers[15], 1);

    }

}