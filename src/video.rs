extern crate sdl2;
extern crate gl;

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::emulator::Emulator;

pub struct Video {
    emulator: Emulator,
    canvas: Canvas<Window>,
    event_pump: EventPump
}

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

impl Video {
    pub fn new(title: &str, width: u32, height: u32, emulator: Emulator) -> Video {
        let (canvas, event_pump) = Video::initialize_window(title, width, height);

        Video {
            emulator: emulator,
            canvas: canvas,
            event_pump: event_pump
        }
    }

    pub fn initialize_window(title: &str, width: u32, height: u32) -> (Canvas<Window>, EventPump) {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();

        let window = video_subsystem
            .window(title, width, height)
            .opengl()
            .resizable()
            .build()
            .unwrap();
        let canvas = window.into_canvas()
            .index(find_sdl_gl_driver().unwrap())
            .build()
            .unwrap();

        gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
        canvas.window().gl_set_context_to_current().unwrap(); 
        let event_pump = sdl.event_pump().unwrap();
        (canvas, event_pump)
    }

    pub fn run(&mut self) {
        'running: loop {
            if self.check_keyboard() {
                break 'running
            }

            self.render();
            if self.update() {
                break 'running
            }
        }
    }

    pub fn check_keyboard(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} => { return true; },
                Event::KeyDown { keycode: Some(Keycode::Num1), .. } => { self.emulator.cpu.keyboard[0] = 1; }
                Event::KeyDown { keycode: Some(Keycode::Num2), .. } => { self.emulator.cpu.keyboard[1] = 1; }
                Event::KeyDown { keycode: Some(Keycode::Num3), .. } => { self.emulator.cpu.keyboard[2] = 1; }
                Event::KeyDown { keycode: Some(Keycode::Num4), .. } => { self.emulator.cpu.keyboard[3] = 1; }
                Event::KeyDown { keycode: Some(Keycode::Q), .. } => { self.emulator.cpu.keyboard[4] = 1; }
                Event::KeyDown { keycode: Some(Keycode::W), .. } => { self.emulator.cpu.keyboard[5] = 1; }
                Event::KeyDown { keycode: Some(Keycode::E), .. } => { self.emulator.cpu.keyboard[6] = 1; }
                Event::KeyDown { keycode: Some(Keycode::R), .. } => { self.emulator.cpu.keyboard[7] = 1; }
                Event::KeyDown { keycode: Some(Keycode::A), .. } => { self.emulator.cpu.keyboard[8] = 1; }
                Event::KeyDown { keycode: Some(Keycode::S), .. } => { self.emulator.cpu.keyboard[9] = 1; }
                Event::KeyDown { keycode: Some(Keycode::D), .. } => { self.emulator.cpu.keyboard[10] = 1; }
                Event::KeyDown { keycode: Some(Keycode::F), .. } => { self.emulator.cpu.keyboard[11] = 1; }
                Event::KeyDown { keycode: Some(Keycode::Z), .. } => { self.emulator.cpu.keyboard[12] = 1; }
                Event::KeyDown { keycode: Some(Keycode::X), .. } => { self.emulator.cpu.keyboard[13] = 1; }
                Event::KeyDown { keycode: Some(Keycode::C), .. } => { self.emulator.cpu.keyboard[14] = 1; }
                Event::KeyDown { keycode: Some(Keycode::V), .. } => { self.emulator.cpu.keyboard[15] = 1; }
                Event::KeyUp { keycode: Some(Keycode::Num1), .. } => { self.emulator.cpu.keyboard[0] = 0; }
                Event::KeyUp { keycode: Some(Keycode::Num2), .. } => { self.emulator.cpu.keyboard[1] = 0; }
                Event::KeyUp { keycode: Some(Keycode::Num3), .. } => { self.emulator.cpu.keyboard[2] = 0; }
                Event::KeyUp { keycode: Some(Keycode::Num4), .. } => { self.emulator.cpu.keyboard[3] = 0; }
                Event::KeyUp { keycode: Some(Keycode::Q), .. } => { self.emulator.cpu.keyboard[4] = 0; }
                Event::KeyUp { keycode: Some(Keycode::W), .. } => { self.emulator.cpu.keyboard[5] = 0; }
                Event::KeyUp { keycode: Some(Keycode::E), .. } => { self.emulator.cpu.keyboard[6] = 0; }
                Event::KeyUp { keycode: Some(Keycode::R), .. } => { self.emulator.cpu.keyboard[7] = 0; }
                Event::KeyUp { keycode: Some(Keycode::A), .. } => { self.emulator.cpu.keyboard[8] = 0; }
                Event::KeyUp { keycode: Some(Keycode::S), .. } => { self.emulator.cpu.keyboard[9] = 0; }
                Event::KeyUp { keycode: Some(Keycode::D), .. } => { self.emulator.cpu.keyboard[10] = 0; }
                Event::KeyUp { keycode: Some(Keycode::F), .. } => { self.emulator.cpu.keyboard[11] = 0; }
                Event::KeyUp { keycode: Some(Keycode::Z), .. } => { self.emulator.cpu.keyboard[12] = 0; }
                Event::KeyUp { keycode: Some(Keycode::X), .. } => { self.emulator.cpu.keyboard[13] = 0; }
                Event::KeyUp { keycode: Some(Keycode::C), .. } => { self.emulator.cpu.keyboard[14] = 0; }
                Event::KeyUp { keycode: Some(Keycode::V), .. } => { self.emulator.cpu.keyboard[15] = 0; }
                _ => {},
            }
        }
        false
    }

    fn render(&mut self) {
        for i in 0..15 {
            if self.emulator.cpu.keyboard[i] != 0 {
                println!("{:?}", self.emulator.cpu.keyboard);
                break;
            }
        }
        self.canvas.present();
    }

    fn update(&mut self) -> bool {
        self.emulator.update()
    }
}

