#![allow(
    dead_code,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

mod math;
mod renderer;

use anyhow::{Result};

use math::approach;
use math::euler::Euler;
use math::vector::Vector;

use renderer::{Renderer, MAX_FRAMES_IN_FLIGHT};

use std::result::Result::Ok;
use std::time::Instant;

use winit::dpi::{LogicalSize, PhysicalPosition};
use winit::event::{ElementState, Event, KeyEvent, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowBuilder};

fn main() -> Result<()> {
    pretty_env_logger::init();

    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new()
        .with_title("Simple Rust Game")
        .with_inner_size(LogicalSize::new(1024, 768))
        .build(&event_loop)?;

    let mut current_time = Instant::now();

    let mut game = unsafe {Game::create(&window)}.unwrap();

    event_loop.run(move |event, elwt| {
        match event {
            Event::AboutToWait => window.request_redraw(),
            Event::WindowEvent { event, ..} => match event {
                WindowEvent::RedrawRequested if !elwt.exiting() && !game.minimized => unsafe {
                    let previous_time = current_time;
                    current_time = Instant::now();

                    let delta_time = current_time.duration_since(previous_time);

                    game.update(delta_time.as_secs_f32());

                    let resized = game.resized;
                    let frame = game.frame;

                    game.resized = false;
                    game.frame = (game.frame + 1) % MAX_FRAMES_IN_FLIGHT;

                    game.renderer.render(frame, resized, &game.character, &window)
                }.unwrap(),
                WindowEvent::Resized(size) => {
                    if size.width == 0 || size.height == 0 {
                        game.minimized = true;
                    } else {
                        game.minimized = false;
                        game.resized = true;
                    }
                },
                WindowEvent::CursorMoved{device_id, position} => {
                    game.handle_cursor_movement(position);
                },
                WindowEvent::KeyboardInput {event, ..} => {
                    game.handle_keyboard_event(event);
                },
                WindowEvent::CloseRequested => {
                    elwt.exit();
                    unsafe{game.renderer.destroy(); }
                }
                _ => {}
            }
            _ => {}
        }
    })?;

    Ok(())
}

#[derive(Clone, Debug)]
struct Character
{
    position : Vector,
    velocity: Vector,
    velocity_goal: Vector,
    view_angle: Euler,
}

#[derive(Debug)]
struct Game{
    renderer: Renderer,
    frame: usize,
    resized: bool,
    minimized: bool,
    start: Instant,
    character: Character,
    last_mouse: PhysicalPosition<f64>,
}

impl Game {
    unsafe fn create(window: &Window) -> Result<Self> {
        Ok(Self{
            renderer : Renderer::create(&window)?,
            frame: 0,
            resized: false,
            minimized: false,
            start: Instant::now(),
            character: Character{
                position: Vector{x:0.0, y:0.0, z: 0.0, w:0.0},
                velocity: Vector{x:0.0, y:0.0, z:0.0, w:0.0},
                velocity_goal: Vector{x:0.0, y:0.0,z:0.0,w:0.0},
                view_angle: Euler{pitch: 0.0, yaw: 0.0, roll: 0.0},
            },
            last_mouse: PhysicalPosition{x: 0.0, y: 0.0},
        })
    }

    fn update(&mut self, delta_time : f32)
    {
        let speed = delta_time * 80.0;
        self.character.velocity.x = approach(self.character.velocity_goal.x, self.character.velocity.x, speed);
        self.character.velocity.y = approach(self.character.velocity_goal.y, self.character.velocity.y, speed);

        self.character.position = self.character.position + self.character.velocity * delta_time;
    }

    fn handle_cursor_movement(&mut self, position: PhysicalPosition<f64>)
    {
        let delta_x = (position.x - self.last_mouse.x) as f32;
        let delta_y = (position.y - self.last_mouse.y) as f32;

        let sensitivity = 0.01;

        self.character.view_angle.pitch += delta_y * sensitivity;
        self.character.view_angle.yaw += delta_x * sensitivity;
        self.character.view_angle.normalize();

        self.last_mouse = position;
    }

    fn handle_keyboard_event(&mut self, event: KeyEvent)
    {
        if event.state == ElementState::Pressed {
            match event.physical_key {
                PhysicalKey::Code(KeyCode::ArrowLeft) => {
                    self.character.velocity_goal.x = -10.0;
                },
                PhysicalKey::Code(KeyCode::ArrowRight) => {
                    self.character.velocity_goal.x = 10.0;
                },
                PhysicalKey::Code(KeyCode::ArrowUp) => {
                    self.character.velocity_goal.y = 10.0;
                },
                PhysicalKey::Code(KeyCode::ArrowDown) => {
                    self.character.velocity_goal.y = -10.0;
                },
                _ => {}
            }
        }
        else if event.state == ElementState::Released {
            match event.physical_key {
                PhysicalKey::Code(KeyCode::ArrowLeft) => {
                    self.character.velocity_goal.x = 0.0;
                },
                PhysicalKey::Code(KeyCode::ArrowRight) => {
                    self.character.velocity_goal.x = 0.0;
                },
                PhysicalKey::Code(KeyCode::ArrowUp) => {
                    self.character.velocity_goal.y = 0.0;
                },
                PhysicalKey::Code(KeyCode::ArrowDown) => {
                    self.character.velocity_goal.y = 0.0;
                },
                _ => {}
            }
        }
    }
}
