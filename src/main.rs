pub mod config;
pub mod emulator;
pub mod interfaces;

use std::{env, fs, time::SystemTime};

use config::{CPU_TIME_PER_INSTRUCTION, DISPLAY_TIME_PER_UPDATE};
use emulator::Emulator;
use interfaces::display::Display;
use interfaces::input::Input;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        dbg!(args);
        println!("Expected usage: ./rust-chip8 romfile.ch8");
        return;
    }

    let rom = fs::read(&args[1]).unwrap();

    let sdl_context = sdl2::init().unwrap();
    let mut emulator = Emulator::init(rom);
    let mut display = Display::init(&sdl_context);
    let mut input = Input::init(&sdl_context);

    let mut time_since_last_instruction = SystemTime::now();
    let mut time_since_last_frame_update = SystemTime::now();

    loop {
        let current_time = SystemTime::now();

        if input.handle_inputs(&mut emulator).unwrap() {
            break;
        }

        if current_time > time_since_last_instruction + CPU_TIME_PER_INSTRUCTION {
            emulator.next_instruction();
            time_since_last_instruction = current_time;
        }

        if current_time > time_since_last_frame_update + DISPLAY_TIME_PER_UPDATE {
            display.draw_frame(&emulator);
            emulator.decrement_delay_timer();
            time_since_last_frame_update = current_time;
        }
    }
}
