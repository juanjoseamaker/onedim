extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

mod render;
use render::camera::Camera;

mod engine;
use engine::body::Body;
use engine::system::System;

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 800;
const FPS: u32 = 60;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("onedim engine", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut main_camera = Camera::new(0.0, 0.0);
    let mut main_camera_velocity: (f32, f32) = (0.0, 0.0);

    let mut system = System::new(
        vec![
            Body::new(0.0, 500.0, 10.0, 100, Color::RED),
        ],
        0.1,
        98.0,
        1.0,
        2.0,
        (HEIGHT / 2) as i32,
        true
    );

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown { keycode, .. } => match keycode {
                    Some(Keycode::Up) => main_camera_velocity.1 = -5.0,
                    Some(Keycode::Down) => main_camera_velocity.1 = 5.0,
                    Some(Keycode::Right) => main_camera_velocity.0 = 5.0,
                    Some(Keycode::Left) => main_camera_velocity.0 = -5.0,
                    _ => {}
                },
                Event::KeyUp { keycode, .. } => match keycode {
                    Some(Keycode::Up) => main_camera_velocity.1 = 0.0,
                    Some(Keycode::Down) => main_camera_velocity.1 = 0.0,
                    Some(Keycode::Right) => main_camera_velocity.0 = 0.0,
                    Some(Keycode::Left) => main_camera_velocity.0 = 0.0,
                    _ => {}
                },
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Game Loop
        main_camera.displace(main_camera_velocity.0, main_camera_velocity.1);
        system.update(Duration::new(0, 1_000_000_000u32 / FPS));
        system.draw(&mut canvas, &main_camera);
        system.draw_guide_lines(&mut canvas, &main_camera, WIDTH, HEIGHT);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
    }
}
