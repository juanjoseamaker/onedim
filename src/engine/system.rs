use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::error::Error;
use std::fs;
use std::time::Duration;

use super::super::render::camera::Camera;
use super::body::Body;

const ANGLE_GUIDE_LINE_LEN: f32 = 1000.0;

// Set of bodies and other objects that interact with each other
pub struct System {
    pub bodies: Vec<Body>,
    pub angle: f32,
    pub gravity: f32,
    pub kfriction_coefficient: f32,
    pub sfriction_coefficient: f32,
    pub surface_y: i32,
    pub elastic_collisions: bool,
    pub size: i32,
}

impl System {
    pub fn new(
        bodies: Vec<Body>,
        angle: f32,
        gravity: f32,
        kfriction_coefficient: f32,
        sfriction_coefficient: f32,
        surface_y: i32,
        elastic_collisions: bool,
        size: i32,
    ) -> System {
        System {
            bodies,
            angle,
            gravity,
            kfriction_coefficient,
            sfriction_coefficient,
            surface_y,
            elastic_collisions,
            size,
        }
    }

    pub fn update(&mut self, dt: Duration) {
        // Dynamics
        let mut changes: Vec<(usize, f32, f32)> = vec![]; // Change (index, new_velocity, new_position)

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
                    if self.elastic_collisions {
                        changes.push((
                            i1,
                            ((self.bodies[i1].mass - self.bodies[i2].mass)
                                / (self.bodies[i1].mass + self.bodies[i2].mass))
                                * self.bodies[i1].velocity
                                + ((2.0 * self.bodies[i2].mass)
                                    / (self.bodies[i1].mass + self.bodies[i2].mass))
                                    * self.bodies[i2].velocity,
                            self.bodies[i1].position,
                        ));

                        dbg!("Calculating Collision");
                    } else {
                        changes.push((
                            i1,
                            (self.bodies[i1].velocity * self.bodies[i1].mass
                                + self.bodies[i2].velocity * self.bodies[i2].mass)
                                / (self.bodies[i1].mass + self.bodies[i2].mass),
                            self.bodies[i1].position
                                + (self.bodies[i1].position - self.bodies[i2].position)
                                    / (1000.0
                                        - self.bodies[i1].size as f32
                                        - self.bodies[i2].size as f32)
                                        .abs(), // NOTE: solution to: inelastic collisions can overlap bodies
                        ));

                        dbg!("Calculating Collision");
                    }
                }
            }
        }

        for change in changes {
            self.bodies[change.0].velocity = change.1;
            self.bodies[change.0].position = change.2;
        }

        // Kinematics
        for body in self.bodies.iter_mut() {
            // Gravity
            body.force += -self.gravity * body.mass * self.angle.sin();

            // Friction
            if body.velocity.abs() > 1.0 {
                body.force -=
                    (self.kfriction_coefficient * self.gravity * body.mass * self.angle.cos())
                        .copysign(body.velocity);
            } else {
                if body.force.abs()
                    > (self.sfriction_coefficient * self.gravity * body.mass * self.angle.cos())
                        .abs()
                {
                    body.force -=
                        (self.sfriction_coefficient * self.gravity * body.mass * self.angle.cos())
                            .copysign(body.force);
                } else {
                    body.force = 0.0;
                }
            }

            if body.position < -self.size as f32 {
                body.position = -self.size as f32;
                if self.elastic_collisions {
                    body.velocity = -body.velocity;
                    body.force = 0.0;
                } else {
                    body.velocity = 0.0;
                    body.force = 0.0;
                }
            } else if body.position > self.size as f32 {
                body.position = self.size as f32;
                if self.elastic_collisions {
                    body.velocity = -body.velocity;
                    body.force = 0.0;
                } else {
                    body.velocity = 0.0;
                    body.force = 0.0;
                }
            }

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
                    x0 as i32 + (ANGLE_GUIDE_LINE_LEN * self.angle.cos()) as i32,
                    surface_canvas_y as i32 + (ANGLE_GUIDE_LINE_LEN * self.angle.sin()) as i32,
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
        canvas
            .draw_line(
                Point::new(x0 as i32 + self.size, 0),
                Point::new(x0 as i32 + self.size, screen_height as i32),
            )
            .unwrap();
        canvas
            .draw_line(
                Point::new(x0 as i32 - self.size, 0),
                Point::new(x0 as i32 - self.size, screen_height as i32),
            )
            .unwrap();
    }

    pub fn energy(&self) -> (f32, f32) {
        let mut result: (f32, f32) = (0.0, 0.0);
        for body in self.bodies.iter() {
            result.0 += 0.5 * body.mass * body.velocity * body.velocity;
            result.1 +=
                body.mass * self.gravity * (body.position + self.size as f32) * self.angle.sin();
        }

        result
    }

    pub fn run_command(&mut self, raw: &str) -> Result<(), &str> {
        let mut cmd = raw.split_whitespace();

        match cmd.next() {
            Some("set") => match cmd.next() {
                Some("angle") => {
                    self.angle = match cmd.next() {
                        Some(raw) => match raw.parse() {
                            Ok(result) => result,
                            _ => return Err("Unexpected argument"),
                        },
                        _ => return Err("Unexpected argument"),
                    };
                }
                Some("gravity") => {
                    self.gravity = match cmd.next() {
                        Some(raw) => match raw.parse() {
                            Ok(result) => result,
                            _ => return Err("Unexpected argument"),
                        },
                        _ => return Err("Unexpected argument"),
                    };
                }
                Some("kfriction") => {
                    self.kfriction_coefficient = match cmd.next() {
                        Some(raw) => match raw.parse() {
                            Ok(result) => result,
                            _ => return Err("Unexpected argument"),
                        },
                        _ => return Err("Unexpected argument"),
                    };
                }
                Some("sfriction") => {
                    self.sfriction_coefficient = match cmd.next() {
                        Some(raw) => match raw.parse() {
                            Ok(result) => result,
                            _ => return Err("Unexpected argument"),
                        },
                        _ => return Err("Unexpected argument"),
                    };
                }
                Some("elastic_collision") => {
                    self.elastic_collisions = match cmd.next() {
                        Some(raw) => match raw.parse() {
                            Ok(result) => result,
                            _ => return Err("Unexpected argument"),
                        },
                        _ => return Err("Unexpected argument"),
                    };
                }
                Some("size") => {
                    self.size = match cmd.next() {
                        Some(raw) => match raw.parse() {
                            Ok(result) => result,
                            _ => return Err("Unexpected argument"),
                        },
                        _ => return Err("Unexpected argument"),
                    };
                }
                _ => return Err("Unexpected argument"),
            },
            Some("new") => self.bodies.push(Body::new(
                match cmd.next() {
                    Some(raw) => match raw.parse() {
                        Ok(result) => result,
                        _ => return Err("Unexpected argument"),
                    },
                    _ => return Err("Unexpected argument"),
                },
                match cmd.next() {
                    Some(raw) => match raw.parse() {
                        Ok(result) => result,
                        _ => return Err("Unexpected argument"),
                    },
                    _ => return Err("Unexpected argument"),
                },
                match cmd.next() {
                    Some(raw) => match raw.parse() {
                        Ok(result) => result,
                        _ => return Err("Unexpected argument"),
                    },
                    _ => return Err("Unexpected argument"),
                },
                match cmd.next() {
                    Some(raw) => match raw.parse() {
                        Ok(result) => result,
                        _ => return Err("Unexpected argument"),
                    },
                    _ => return Err("Unexpected argument"),
                },
                Color::from((
                    match cmd.next() {
                        Some(raw) => match raw.parse() {
                            Ok(result) => result,
                            _ => return Err("Unexpected argument"),
                        },
                        _ => return Err("Unexpected argument"),
                    },
                    match cmd.next() {
                        Some(raw) => match raw.parse() {
                            Ok(result) => result,
                            _ => return Err("Unexpected argument"),
                        },
                        _ => return Err("Unexpected argument"),
                    },
                    match cmd.next() {
                        Some(raw) => match raw.parse() {
                            Ok(result) => result,
                            _ => return Err("Unexpected argument"),
                        },
                        _ => return Err("Unexpected argument"),
                    },
                )),
            )),
            _ => return Err("Unexpected command"),
        }

        Ok(())
    }

    pub fn run_script(&mut self, filename: &str) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(filename)?;

        for line in contents.lines() {
            self.run_command(line)?;
        }

        Ok(())
    }
}

fn colliding(x1: f32, size1: f32, x2: f32, size2: f32) -> bool {
    size1 / 2.0 + size2 / 2.0 > (x1 - x2).abs()
}
