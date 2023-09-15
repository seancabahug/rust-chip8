use sdl2::{event::Event, Error, EventPump, Sdl};

use crate::emulator::Emulator;

pub struct Input {
    event_pump: EventPump,
}

impl Input {
    pub fn init(sdl_context: &Sdl) -> Input {
        Input {
            event_pump: sdl_context.event_pump().unwrap(),
        }
    }

    pub fn handle_inputs(&mut self, emulator: &mut Emulator) -> Result<bool, Error> {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return Ok(true),
                Event::KeyDown {
                    keycode: Some(key), ..
                } => match key {
                    _ => {}
                },
                Event::KeyUp {
                    keycode: Some(key), ..
                } => match key {
                    _ => {}
                },
                _ => {}
            }
        }

        Ok(false)
    }
}
