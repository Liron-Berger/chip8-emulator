use crate::cpu::Cpu;


pub struct Emulator {
   pub cpu: Cpu,
}

impl Emulator { 
    pub fn new() -> Emulator {
        Emulator {
            cpu: Cpu::new(),
        }
    }

    pub fn load_program(&mut self, program_buffer: Vec<u8>) {
        for (i, byte) in program_buffer.iter().enumerate() {
            self.cpu.memory[Cpu::PROGRAM_OFFSET + i] = *byte;
        }
    }
}
