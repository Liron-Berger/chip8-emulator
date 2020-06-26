pub struct Cpu {
    pub registers: [u8; 16],
    pub memory: [u8; 4096],
    
}

impl Cpu {
    const PROGRAM_OFFSET: usize = 0x200;

    pub fn new() -> Cpu {
        println!("Hello");
        Cpu {
            registers: [0; 16],
            memory: [0; 4096],
        }
    }

    pub fn load_program(&mut self, program_buffer: Vec<u8>) {
        for (i, byte) in program_buffer.iter().enumerate() {
            self.memory[Cpu::PROGRAM_OFFSET + i] = *byte;
        }
    }
}

impl std::fmt::Debug for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.registers)
    }
}
