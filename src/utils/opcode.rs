pub struct Opcode {
    pub opcode: u16,
    pub nnn: u16,
    pub kk: u8,
    pub n: u8,
    pub x: u8,
    pub y: u8,
}

impl Opcode {
    pub fn new(opcode: u16) -> Opcode {
        let (nnn, kk, n, x, y) = Opcode::get_opcode_parts(opcode);
        Opcode {
            opcode: opcode,
            nnn: nnn,
            kk: kk,
            n: n,
            x: x,
            y: y,
        }
    }

    fn get_opcode_parts(opcode: u16) -> (u16, u8, u8, u8, u8) {
        let nnn = opcode & 0x0fff;
        let kk = (opcode & 0xff) as u8;
        let n = (opcode & 0xf) as u8;
        let x = ((opcode & 0xf00) >> 8) as u8;
        let y = ((opcode & 0xf0) >> 4) as u8;
        (nnn, kk, n, x, y)
    }
}
