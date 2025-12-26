use sdl2::{Sdl, pixels::Color};

pub struct Display {
    pub tile_size: u32,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl Display {
    pub fn new(width: u32, height: u32, tile_size: u32,sdl_ctx: &Sdl) -> Display {
        let video_subsystem = sdl_ctx.video().unwrap();
        let window = video_subsystem
            .window("CHIP-8 Emulator", width, height)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();

        Display {
            tile_size,
            canvas: canvas,
        }
    }

    pub fn draw(&mut self, io: &crate::io::IOData) {
        self.canvas.set_draw_color(Color::BLACK);

        for (y, row) in io.display.iter().enumerate() {
            for (x, &pixel) in row.iter().enumerate() {
                let rect = sdl2::rect::Rect::new(
                    (x as u32 * self.tile_size) as i32,
                    (y as u32 * self.tile_size) as i32,
                    self.tile_size,
                    self.tile_size,
                );
                if pixel != 0 {
                    self.canvas.set_draw_color(Color::WHITE);
                }
                else {
                    self.canvas.set_draw_color(Color::BLACK);
                }
                self.canvas.fill_rect(rect).unwrap();
            }
        }

        self.canvas.present();
    }
}
