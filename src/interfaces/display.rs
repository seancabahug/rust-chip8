use crate::{config::*, emulator::Emulator};
use sdl2::{pixels::Color, rect::Rect, render::WindowCanvas, Sdl};

pub struct Display {
    canvas: WindowCanvas,
}

impl Display {
    pub fn init(sdl_context: &Sdl) -> Display {
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("CHIP-8 - @seancabahug", WINDOW_WIDTH, WINDOW_HEIGHT)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        Display { canvas }
    }

    pub fn draw_frame(&mut self, emulator: &Emulator) {
        let canvas = &mut self.canvas;

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        canvas.set_draw_color(Color::WHITE);
        let framebuffer = emulator.get_framebuffer();
        for (y, row) in framebuffer.iter().enumerate() {
            for (x, &pixel) in row.iter().enumerate() {
                if pixel {
                    let x: i32 = (x as u32 * PIXEL_DENSITY).try_into().unwrap();
                    let y: i32 = (y as u32 * PIXEL_DENSITY).try_into().unwrap();
                    canvas
                        .fill_rect(Rect::new(x, y, PIXEL_DENSITY, PIXEL_DENSITY))
                        .unwrap();
                }
            }
        }

        canvas.present();
    }
}
