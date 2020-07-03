use crate::instructions::get_opcode_func;
use crate::opcode::Opcode;
use crate::display::Display;


pub struct Cpu {
    pub registers: [u8; Cpu::REGISTERS_SIZE],
    pub ram: [u8; Cpu::RAM_SIZE as usize],
    pub stack: [u16; Cpu::STACK_SIZE],    
    pub pc: u16,
    pub sp: u8,
    pub i: u16,
    pub display: Display,
    pub keyboard: [u16; Cpu::KEYBOARD_SIZE],
    pub dt: u8,
    pub st: u8,
    pub wait_key: bool,
}

impl Cpu {
    pub const RAM_SIZE: u16 = 0x1000;
    pub const REGISTERS_SIZE: usize = 0x10;
    pub const STACK_SIZE: usize = 0x10;
    pub const KEYBOARD_SIZE: usize = 0x10;

    pub const VF: u8 = 0xf;
    pub const V0: u8 = 0x0;

    pub const PROGRAM_OFFSET: usize = 0x200;



    pub fn new() -> Cpu {
        let mut cpu = Cpu {
            registers: [0; Cpu::REGISTERS_SIZE],
            ram: [0; Cpu::RAM_SIZE as usize],
            stack: [0; Cpu::STACK_SIZE],
            pc: 0x200,
            sp: 0,
            i: 0,
            display: Display::new(),
            keyboard: [0; Cpu::KEYBOARD_SIZE],
            dt: 0,
            st: 0,
            wait_key: false,
        };
        cpu.add_sprites();
        cpu
    }

    pub fn run_opcode(&mut self, opcode: u16) -> bool {
        use std::{thread, time};
        let op = Opcode::new(opcode);
        type OpcodeFunc = fn(&mut Cpu, Opcode);
        let func: OpcodeFunc = get_opcode_func(&op);
         
        func(self, op);
        if self.dt > 0 {
            self.dt -= 1;
        }
        println!("{:x}, {:?}", opcode, self);
        self.pc == Cpu::RAM_SIZE
    }

    pub fn advance_pc(&mut self) {
        self.pc += 2;
    }

    pub fn jump_pc(&mut self, addr: u16) {
        self.pc = addr;
    }

    pub fn get_v(&self, index: u8) -> u8 {
        self.registers[index as usize]
    }

    pub fn set_v(&mut self, index: u8, value: u8) {
        if (index == 0xf) {
        println!("{:x}, {}", index, value);
        }
        self.registers[index as usize] = value;
    }

    pub fn check_keypress(&self) -> i16 {
        for i in 0..15 {
            if self.keyboard[i] != 0 {
                println!("{:?}", self.keyboard);
                return i as i16;
            }
        }
        -1
    }

    pub fn get_u8_lsb(value: u8) -> u8 {
        value << 7 >> 7
    }

    pub fn get_u8_msb(value: u8) -> u8 {
        value >> 7
    }

    pub fn add_sprites(&mut self) {
        const CHIP8_FONTSET: [u8; 80] = [ 
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ];
        for i in 0..CHIP8_FONTSET.len() {
            self.ram[i] = CHIP8_FONTSET[i];
        }
    }
}

impl std::fmt::Debug for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:x}\n\tregisters: {:?}\n\tstack: {:?}\n\tkeyboard: {:?}\n\tsp: {:x}\n\tI: {:x}\n\tdt: {:x}", self.pc, self.registers, self.stack, self.keyboard, self.sp, self.i, self.dt)
    }
}
