extern crate sdl2;

pub struct Window {
    // sdl: sdl2::Sdl,
    // window: sdl2::video::Window,
    event_pump: sdl2::EventPump,
}

impl Window {
    pub fn new() -> Window {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();
        let window = video_subsystem
            .window("Game", 900, 700)
            .resizable()
            .build()
            .unwrap();
     
        let mut event_pump = sdl.event_pump().unwrap();
        Window {
            // sdl: sdl,
            // window: window,
            event_pump: event_pump,
        }
    }

    pub fn run(&mut self) {
        'running: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit {..} => break 'running,
                    _ => {},
                }
            }
        }
    }

    pub fn render() {
        
    }

    pub fn update() {
        
    }
}

