use std::collections::HashSet;

use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Scancode;
use sdl2::Sdl;

pub struct KeyboardDriver {
    event_pump: EventPump,
}


impl KeyboardDriver {
    pub fn new(sdl_context: &Sdl) -> KeyboardDriver {
        KeyboardDriver {
            event_pump: sdl_context.event_pump().unwrap(),
        }
    }

    fn get_keyboard_mapping(button: Scancode) -> usize {
        match button {
            Scancode::X { .. } => 0,
            Scancode::Num1 { .. } => 1,
            Scancode::Num2 { .. } => 2,
            Scancode::Num3 { .. } => 3,
            Scancode::Q { .. } => 4,
            Scancode::W { .. } => 5,
            Scancode::E { .. } => 6,
            Scancode::A { .. } => 7,
            Scancode::S { .. } => 8,
            Scancode::D { .. } => 9,
            Scancode::Z { .. } => 0xA,
            Scancode::C { .. } => 0xB,
            Scancode::Num4 { .. } => 0xC,
            Scancode::R { .. } => 0xD,
            Scancode::F { .. } => 0xE,
            Scancode::V { .. } => 0xF,
            _ => panic!("Unrecognized chip8 scan code!")
        }
    }

    pub fn get_keyboard_state(&mut self) -> [bool; 16] {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => panic!("Exit!!"),
                _ => {}
            }
        }
        let keyboard_state: HashSet<Scancode> = self.event_pump.keyboard_state().pressed_scancodes().collect();

        let mut pressed: [bool; 16] = [false; 16];
        for button in keyboard_state {
            pressed[KeyboardDriver::get_keyboard_mapping(button)] = true;
        }
        pressed
    }
}

