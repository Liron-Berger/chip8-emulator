extern crate sdl2;
extern crate gl;

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

use crate::emulator::Emulator;

pub struct Video {
    title: String,
    width: u32,
    height: u32,
    emulator: Emulator,
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
        Video {
            title: title.to_string(),
            width: width,
            height: height,
            emulator: emulator,
        }
    }

    pub fn initialize_window(&self) -> (Canvas<Window>, EventPump) {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();

        let window = video_subsystem
            .window(&self.title, self.width, self.height)
            .opengl()
            .resizable()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas()
            .index(find_sdl_gl_driver().unwrap())
            .build()
            .unwrap();
        gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
        canvas.window().gl_set_context_to_current(); 
        let mut event_pump = sdl.event_pump().unwrap();
        (canvas, event_pump)
    }

    pub fn run(&mut self) {
        let (mut canvas, mut event_pump) = self.initialize_window();
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit {..} => break 'running,
                    _ => {},
                }
            }

            self.render();
            canvas.present();
            self.update();
        }
    }

    fn render(&self) {
        unsafe {
            gl::ClearColor(0.6, 0.0, 0.8, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    fn update(&mut self) {
        // self.emulator.emulate_program();
    }
}

