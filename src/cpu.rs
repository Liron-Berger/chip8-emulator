pub struct Cpu {
    pub registers: [u8; 16],
    pub memory: [u8; 4096],
    
}

impl Cpu {
    pub const PROGRAM_OFFSET: usize = 0x200;

    pub fn new() -> Cpu {
        Cpu {
            registers: [0; 16],
            memory: [0; 4096],
        }
    }
}

impl std::fmt::Debug for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.registers)
    }
}
