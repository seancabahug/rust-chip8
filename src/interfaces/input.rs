use sdl2::{event::Event, keyboard::Keycode, Error, EventPump, Sdl};

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
                    Keycode::Escape => return Ok(true),
                    Keycode::X => emulator.set_key_state(0, true),
                    Keycode::Num1 => emulator.set_key_state(1, true),
                    Keycode::Num2 => emulator.set_key_state(2, true),
                    Keycode::Num3 => emulator.set_key_state(3, true),
                    Keycode::Q => emulator.set_key_state(4, true),
                    Keycode::W => emulator.set_key_state(5, true),
                    Keycode::E => emulator.set_key_state(6, true),
                    Keycode::A => emulator.set_key_state(7, true),
                    Keycode::S => emulator.set_key_state(8, true),
                    Keycode::D => emulator.set_key_state(9, true),
                    Keycode::Z => emulator.set_key_state(10, true),
                    Keycode::C => emulator.set_key_state(11, true),
                    Keycode::Num4 => emulator.set_key_state(12, true),
                    Keycode::R => emulator.set_key_state(13, true),
                    Keycode::F => emulator.set_key_state(14, true),
                    Keycode::V => emulator.set_key_state(15, true),
                    _ => {}
                },
                Event::KeyUp {
                    keycode: Some(key), ..
                } => match key {
                    Keycode::X => emulator.set_key_state(0, false),
                    Keycode::Num1 => emulator.set_key_state(1, false),
                    Keycode::Num2 => emulator.set_key_state(2, false),
                    Keycode::Num3 => emulator.set_key_state(3, false),
                    Keycode::Q => emulator.set_key_state(4, false),
                    Keycode::W => emulator.set_key_state(5, false),
                    Keycode::E => emulator.set_key_state(6, false),
                    Keycode::A => emulator.set_key_state(7, false),
                    Keycode::S => emulator.set_key_state(8, false),
                    Keycode::D => emulator.set_key_state(9, false),
                    Keycode::Z => emulator.set_key_state(10, false),
                    Keycode::C => emulator.set_key_state(11, false),
                    Keycode::Num4 => emulator.set_key_state(12, false),
                    Keycode::R => emulator.set_key_state(13, false),
                    Keycode::F => emulator.set_key_state(14, false),
                    Keycode::V => emulator.set_key_state(15, false),
                    _ => {}
                },
                _ => {}
            }
        }

        Ok(false)
    }
}
