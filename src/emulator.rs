use crate::cpu::Cpu;

#[derive(Copy, Clone)]
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
            self.cpu.ram[Cpu::PROGRAM_OFFSET + i] = *byte;
        }
    }

    fn get_opcode(&self) -> u16 {
       (self.cpu.ram[self.cpu.pc] as u16) << 8 | self.cpu.ram[self.cpu.pc + 1] as u16
    }

    pub fn update(&mut self) -> bool {
        let opcode = self.get_opcode();
        self.cpu.run_opcode(opcode)
    }
}
