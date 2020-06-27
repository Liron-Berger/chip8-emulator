extern crate sdl2;
extern crate gl;

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

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
            for event in self.event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit {..} => break 'running,
                    _ => {},
                }
            }

            self.render();
            if self.update() {
                break 'running
            }
        }
    }

    fn render(&mut self) {
        unsafe {
            gl::ClearColor(0.6, 0.0, 0.8, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        self.canvas.present();
    }

    fn update(&mut self) -> bool {
        self.emulator.update()
    }
}

