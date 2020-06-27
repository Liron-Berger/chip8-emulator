use crate::instructions::get_opcode_func;
use crate::opcode::Opcode;

#[derive(Copy, Clone)]
pub struct Cpu {
    pub registers: [u8; Cpu::REGISTERS_SIZE],
    pub ram: [u8; Cpu::RAM_SIZE as usize],
    pub stack: [u16; Cpu::STACK_SIZE],    
    pub pc: u16,
    pub sp: u8,
    pub i: u16,
}

impl Cpu {
    pub const RAM_SIZE: u16 = 0x1000;
    pub const REGISTERS_SIZE: usize = 0x10;
    pub const STACK_SIZE: usize = 0x10;
    pub const VF: u8 = 0xf;
    pub const V0: u8 = 0x0;

    pub const PROGRAM_OFFSET: usize = 0x200;



    pub fn new() -> Cpu {
        Cpu {
            registers: [0; Cpu::REGISTERS_SIZE],
            ram: [0; Cpu::RAM_SIZE as usize],
            stack: [0; Cpu::STACK_SIZE],
            pc: 0x200,
            sp: 0,
            i: 0,
        }
    }

    pub fn run_opcode(&mut self, opcode: u16) -> bool {
        self.pc += 2;
        let op = Opcode::new(opcode);
        type OpcodeFunc = fn(&mut Cpu, Opcode);
        let func: OpcodeFunc = get_opcode_func(&op);
        
        func(self, op);
        self.pc == Cpu::RAM_SIZE
    }

    pub fn advance_pc(&mut self) {
        self.pc += 2;
    }

    pub fn get_v(&self, index: u8) -> u8 {
        self.registers[index as usize]
    }

    pub fn set_v(&mut self, index: u8, value: u8) {
        self.registers[index as usize] = value;
    }

    pub fn get_u8_lsb(value: u8) -> u8 {
        value << 7 >> 7
    }

    pub fn get_u8_msb(value: u8) -> u8 {
        value >> 7
    }
}

impl std::fmt::Debug for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.registers)
    }
}
