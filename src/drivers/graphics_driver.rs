extern crate gl;
extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::Sdl;
use sdl2::video::Window;

use crate::display::Display;

pub struct GraphicsDriver {
    canvas: Canvas<Window>,
}

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

impl GraphicsDriver {
    const PIXEL_FACTOR: u32 = 16;

    pub fn new(sdl_context: &Sdl) -> GraphicsDriver {
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("Chip 8", (Display::WIDTH as u32) * GraphicsDriver::PIXEL_FACTOR, (Display::HEIGHT as u32) * GraphicsDriver::PIXEL_FACTOR)
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

        GraphicsDriver {
            canvas,
        }
    }

    pub fn render(&mut self, display: &Display) {
        for (i, row) in display.pixels.clone().iter().enumerate() {
            for (j, val) in row.iter().enumerate() {
                if *val == 1 {
                    self.fill_rect(i as i32, j as i32, 255, 210, 0);
                } else {
                    self.fill_rect(i as i32, j as i32, 0, 0, 0);
                }
            }
        }
        self.canvas.present();
    }

    fn fill_rect(&mut self, i: i32, j: i32, r: u8, g: u8, b: u8) {
        self.canvas.set_draw_color(Color::RGB(r, g, b));
        self.canvas.fill_rect(Rect::new(j * GraphicsDriver::PIXEL_FACTOR as i32, i * GraphicsDriver::PIXEL_FACTOR as i32, GraphicsDriver::PIXEL_FACTOR, GraphicsDriver::PIXEL_FACTOR)).unwrap();
    }
}
