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
        // Dynamic
        let mut changes: Vec<(usize, f32)> = vec![]; // Change (index, new_velocity)

        for i1 in 0..self.bodies.len() {
            for i2 in 0..self.bodies.len() {
                if i1 != i2
                    && colliding(
                        self.bodies[i1].position,
                        self.bodies[i1].size as f32,
                        self.bodies[i2].position,
                        self.bodies[i2].size as f32,
                    )
                {
                    changes.push((
                        i1,
                        ((self.bodies[i1].mass - self.bodies[i2].mass)
                            / (self.bodies[i1].mass + self.bodies[i2].mass))
                            * self.bodies[i1].velocity
                            + ((2.0 * self.bodies[i2].mass)
                                / (self.bodies[i1].mass + self.bodies[i2].mass))
                                * self.bodies[i2].velocity,
                    ));
                }
            }
        }

        for change in changes {
            self.bodies[change.0].velocity = change.1;
        }

        // Kinematics
        for body in self.bodies.iter_mut() {
            body.force += -self.gravity * body.mass * self.angle.sin();
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
                    surface_canvas_y as i32
                        - (-self.angle.sin() * (camera.x + screen_width as f32).abs()) as i32,
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

fn colliding(x1: f32, size1: f32, x2: f32, size2: f32) -> bool {
    size1 / 2.0 + size2 / 2.0 > (x1 - x2).abs()
}
