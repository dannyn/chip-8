
#[derive(Debug)]
pub enum Opcode {
    Halt, 
    Unimplemented,
    SeVX{x: u8, b: u8},
    AddVX{x: u8, b: u8},
    LdXY{x: u8, y: u8},
    AddXY{x: u8, y: u8},
    SubXY{x: u8, y: u8},
}

pub fn decode (opcode: u16) -> Opcode {

    let op_minor = (opcode & 0x000F) as u8;
    let x =        ((opcode & 0x0F00) >> 8) as u8;
    let y =        ((opcode & 0x00F0) >> 4) as u8;
    let byte =     (opcode & 0x00FF) as u8;
    let addr =     opcode & 0x0FFF;

    match opcode {
        0x0000 => Opcode::Halt,
        0x6000..=0x6FFF => Opcode::SeVX{x: x, b: byte},
        0x7000..=0x7FFF => Opcode::AddVX{x: x, b: byte},
        0x8000..=0x8FFF => 
            match op_minor {
                0x0 => Opcode::LdXY{x: x, y: y},
                0x4 => Opcode::AddXY{x: x, y: y},
                0x5 => Opcode::SubXY{x: x, y: y},
                _ => Opcode::Unimplemented,
            },
        _ => Opcode::Unimplemented,
    }
}