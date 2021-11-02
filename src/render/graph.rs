use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;

const BAR_HEIGHT: u32 = 10;

pub struct Graph {
    pub data: Vec<(Color, f32)>,
    pub max: f32,
    pub bar_max: i32,
    pub bar_x: i32,
}

impl Graph {
    pub fn new(data: Vec<(Color, f32)>, max: f32, bar_max: i32, bar_x: i32) -> Graph {
        Graph {
            data,
            max,
            bar_max,
            bar_x
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        let mut y: i32 = 0;

        for data in &self.data {
            canvas.set_draw_color(data.0);

            let bar_len: f32 = (data.1 / self.max) * self.bar_max as f32;

            if bar_len > 0.0 {
                canvas.fill_rect(Rect::new(self.bar_x, y, bar_len as u32, BAR_HEIGHT)).unwrap();
            } else {
                canvas.fill_rect(Rect::new(self.bar_x + bar_len as i32, y, bar_len.abs() as u32, BAR_HEIGHT)).unwrap();
            }

            y += BAR_HEIGHT as i32;
        }
    }
}