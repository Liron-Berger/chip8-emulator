use sdl2::Sdl;
use sdl2::EventPump;
use std::collections::HashSet;
use sdl2::keyboard::Scancode;
use sdl2::event::Event;
use std::ptr::null;

pub struct KeyboardDriver {
    event_pump: sdl2::EventPump,
}

impl KeyboardDriver {
    pub fn new(sdl_context: &Sdl) -> KeyboardDriver {
        KeyboardDriver {
            event_pump: sdl_context.event_pump().unwrap(),
        }
    }

    pub fn pressed_scancode_set(&self) -> HashSet<Scancode> {
        self.event_pump.keyboard_state().pressed_scancodes().collect()
    }

    pub fn get_keyboard_state(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => panic!("Exit!!"),
                _ => {}
            }
        }
        let pressed: HashSet<Scancode> = self.event_pump.keyboard_state().pressed_scancodes().collect();
        println!("{:?}", pressed);
    }
}

