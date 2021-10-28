use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;

use super::super::render::camera::Camera;
use super::body::Body;

// Set of bodies and other objects that interact with each other
pub struct System {
    pub bodies: Vec<Body>,
    pub angle: f32,
    pub gravity: f32,
    pub surface_y: i32,
}

impl System {
    pub fn new(bodies: Vec<Body>, angle: f32, gravity: f32, surface_y: i32) -> System {
        System {
            bodies,
            angle,
            gravity,
            surface_y,
        }
    }

    pub fn update(&mut self, dt: Duration) {
        for body in self.bodies.iter_mut() {
            body.force = -self.gravity * body.mass * self.angle.sin();
            body.update(dt);
            body.force = 0.0;
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, camera: &Camera) {
        for body in self.bodies.iter() {
            body.draw(canvas, camera, self.surface_y);
        }
    }

    pub fn draw_guide_lines(
        &self,
        canvas: &mut Canvas<Window>,
        camera: &Camera,
        screen_width: u32,
        screen_height: u32,
    ) {
        let (x0, surface_canvas_y) = camera.transform(0.0, self.surface_y as f32);

        canvas.set_draw_color(Color::GRAY);
        canvas
            .draw_line(
                Point::new(x0 as i32, surface_canvas_y as i32),
                Point::new(
                    screen_width as i32,
                    surface_canvas_y as i32 - (-self.angle.sin() * (camera.x + screen_width as f32).abs()) as i32,
                ),
            )
            .unwrap();

        canvas.set_draw_color(Color::WHITE);
        canvas
            .draw_line(
                Point::new(0, surface_canvas_y as i32),
                Point::new(screen_width as i32, surface_canvas_y as i32),
            )
            .unwrap();
        canvas
            .draw_line(
                Point::new(x0 as i32, 0),
                Point::new(x0 as i32, screen_height as i32),
            )
            .unwrap();
    }
}
