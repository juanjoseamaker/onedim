use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;

use super::super::render::camera::Camera;

// Physical body
pub struct Body {
    pub position: f32,
    pub velocity: f32,

    pub force: f32,
    pub mass: f32,

    pub size: u32,

    pub color: Color,
}

impl Body {
    pub fn new(position: f32, velocity: f32, mass: f32, size: u32, color: Color) -> Body {
        Body {
            position,
            velocity,
            mass,
            size,
            color,
            force: 0.0,
        }
    }

    pub fn update(&mut self, dt: Duration) {
        self.velocity += (self.force / self.mass) * dt.as_secs_f32();
        self.position += self.velocity * dt.as_secs_f32();
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, camera: &Camera, surface_y: i32) {
        let center: (f32, f32) =
            camera.transform(self.position, surface_y as f32 - self.size as f32 / 2.0);
        let rect = Rect::from_center(
            Point::new(center.0 as i32, center.1 as i32),
            self.size,
            self.size,
        );
        canvas.set_draw_color(self.color);
        canvas.fill_rect(rect).unwrap();
    }
}
