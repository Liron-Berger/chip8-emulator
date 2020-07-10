use crate::cpu::Cpu;
use crate::drivers::keyboard_driver::KeyboardDriver;
use crate::drivers::graphics_driver::GraphicsDriver;

pub struct Emulator {
    cpu: Cpu,

    graphics_driver: GraphicsDriver,
    keyboard_driver: KeyboardDriver,
}

impl Emulator {

    pub fn new() -> Emulator {
        let sdl_context = sdl2::init().unwrap();

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
            let keyboard_state = self.keyboard_driver.get_keyboard_state();

            self.update();

            self.cpu.keyboard = keyboard_state;
        }
    }

    pub fn update(&mut self) {
        let opcode = self.get_opcode();
        self.cpu.run_opcode(opcode);

        self.graphics_driver.render(&self.cpu.display);
    }
}
