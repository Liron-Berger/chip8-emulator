use sdl2::Sdl;

use crate::cpu::Cpu;
use crate::graphics_driver::GraphicsDriver;
use crate::keyboard_driver::KeyboardDriver;


pub struct Emulator {
    cpu: Cpu,

    graphics_driver: GraphicsDriver,
    keyboard_driver: KeyboardDriver,
}

impl Emulator {

    pub fn new() -> Emulator {
        let mut sdl_context = sdl2::init().unwrap();

        Emulator {
            cpu: Cpu::new(),
            graphics_driver: GraphicsDriver::new(&sdl_context),
            keyboard_driver: KeyboardDriver::new(&sdl_context),
        }
    }

    pub fn load_program(&mut self, program_buffer: Vec<u8>) {
        for (i, byte) in program_buffer.iter().enumerate() {
            self.cpu.ram[Cpu::PROGRAM_OFFSET + i] = *byte;
        }
    }

    fn get_opcode(&self) -> u16 {
       (self.cpu.ram[self.cpu.pc as usize] as u16) << 8 | self.cpu.ram[(self.cpu.pc + 1) as usize] as u16
    }

    pub fn run(&mut self) {
        loop {
            self.update();

            println!("{:?}", self.keyboard_driver.get_keyboard_state());
        }
    }

    pub fn update(&mut self) {
        let opcode = self.get_opcode();
        self.cpu.run_opcode(opcode);
    }
}
