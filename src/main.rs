#![allow(
    dead_code,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

use anyhow::Result;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

mod math;
use std::time::Instant;

struct Character
{
    position : math::Vector,
    velocity: math::Vector,
    gravity: math::Vector
}

fn main() -> Result<()> {
    pretty_env_logger::init();

    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new()
        .with_title("Simple Rust Game")
        .with_inner_size(LogicalSize::new(1024, 768))
        .build(&event_loop)?;

    let mut character = Character
    {
        position: math::Vector{x: 0.0, y: 0.0},
        velocity: math::Vector{x: 2.0, y: 2.0},
        gravity: math::Vector{x: 0.0, y: -2.0},
    };

    let mut current_time = Instant::now();

    let mut app = unsafe {App::create(&window)}.unwrap();
    event_loop.run(move |event, elwt| {
        match event {
            Event::AboutToWait => window.request_redraw(),
            Event::WindowEvent { event, ..} => match event {
                WindowEvent::RedrawRequested if !elwt.exiting() => unsafe {
                    let previous_time = current_time;
                    current_time = Instant::now();

                    let delta_time = current_time.duration_since(previous_time);

                    app.update(delta_time.as_secs_f32(), &mut character);
                    app.render(&window)
                }.unwrap(),
                WindowEvent::CloseRequested => {
                    elwt.exit();
                    unsafe {app.destroy(); }
                }
                _ => {}
            }
            _ => {}
        }
    })?;

    Ok(())
}

#[derive(Clone, Debug)]
struct App{}

impl App {
    unsafe fn create(window: &Window) -> Result<Self> {
        Ok(Self{})
    }

    fn update(&mut self, delta_time : f32, character : & mut Character)
    {
        character.position = &character.position + &(&character.velocity * delta_time);
        character.velocity = &character.velocity + &(&character.gravity * delta_time);

        println!("{0},{1}", character.position.x, character.position.y);
    }

    unsafe fn render(&mut self, window: &Window) -> Result<()>
    {
        Ok(())
    }

    unsafe fn destroy(&mut self) {}
}

#[derive(Clone, Debug, Default)]
struct AppData {}
