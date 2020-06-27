use crate::instructions::get_opcode_func;

#[derive(Copy, Clone)]
pub struct Cpu {
    pub registers: [u8; Cpu::REGISTERS_SIZE],
    pub ram: [u8; Cpu::RAM_SIZE as usize],
    pub stack: [u16; Cpu::STACK_SIZE],    
    pub pc: u16,
    pub sp: u8,
}

impl Cpu {
    pub const RAM_SIZE: u16 = 0x1000;
    pub const REGISTERS_SIZE: usize = 0x10;
    pub const STACK_SIZE: usize = 0x10;

    pub const PROGRAM_OFFSET: usize = 0x200;


    pub fn new() -> Cpu {
        Cpu {
            registers: [0; Cpu::REGISTERS_SIZE],
            ram: [0; Cpu::RAM_SIZE as usize],
            stack: [0; Cpu::STACK_SIZE],
            pc: 0x200,
            sp: 0,
        }
    }

    pub fn run_opcode(&mut self, opcode: u16) -> bool {
        self.pc += 2;
        type Opcode = fn(&mut Cpu, u16);
        let func: Opcode = get_opcode_func(opcode);

        func(self, opcode);
        self.pc == Cpu::RAM_SIZE
    }

}

impl std::fmt::Debug for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.registers)
    }
}
