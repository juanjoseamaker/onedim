extern crate sdl2;

use std::env;
use std::process;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

mod render;
use render::camera::Camera;
use render::graph::Graph;

mod engine;
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
        vec![],
        0.0,
        0.0,
        0.0,
        0.0,
        (HEIGHT / 2) as i32,
        false,
        1000,
    );

    let mut args = env::args();
    let program_name_arg = args.next();
    system.run_script(args.next().unwrap_or_else(|| {
        eprintln!("Usage: {} [script filename]", program_name_arg.unwrap());
        process::exit(1);
    }).as_str()).unwrap_or_else(|err| {
        eprintln!("Failed to run the script: {}", err);
        process::exit(1);
    });

    let energy = system.energy();
    let total_energy = energy.0 + energy.1;

    let mut graph = Graph::new(
        vec![
            (Color::WHITE, total_energy),
            (Color::YELLOW, 0.0),
            (Color::BLUE, 0.0),
            (Color::GRAY, 0.0),
        ],
        total_energy,
        WIDTH as i32/4,
        WIDTH as i32/2,
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

        let energy = system.energy();
        graph.data[1].1 = energy.0;
        graph.data[2].1 = energy.1;
        graph.data[3].1 = total_energy - energy.0 - energy.1;

        system.draw(&mut canvas, &main_camera);
        system.draw_guide_lines(&mut canvas, &main_camera, WIDTH, HEIGHT);
        graph.draw(&mut canvas);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
    }
}
