extern crate sdl2;
use crate::emulator::Emulator;

pub struct Window {
    title: String,
    width: u32,
    height: u32,
    emulator: Emulator,
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32, emulator: Emulator) -> Window {
        Window {
            title: title.to_string(),
            width: width,
            height: height,
            emulator: emulator
        }
    }

    pub fn run(&mut self) {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();
        let window = video_subsystem
            .window(&self.title, self.width, self.height)
            .resizable()
            .build()
            .unwrap();
     
        let mut event_pump = sdl.event_pump().unwrap();

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit {..} => break 'running,
                    _ => {},
                }
            }

            // self.render();
            // self.update();
        }
    }

    fn render(&self) {
        
    }

    fn update(&mut self) {
        // self.emulator.emulate_program();
    }
}

