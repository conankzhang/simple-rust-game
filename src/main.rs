#![allow(
    dead_code,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

use anyhow::{anyhow, Result};
use log::*;

use vulkanalia::loader::{LibloadingLoader, LIBRARY};
use vulkanalia::window as vk_window;
use vulkanalia::prelude::v1_0::*;
use vulkanalia::Version;

use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

mod math;
use std::time::Instant;

const PORTABILITY_MACOS_VERSION: Version = Version::new(1, 3, 216);

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

unsafe fn create_instance(window: &Window, entry: &Entry) -> Result<Instance> {
    let application_info = vk::ApplicationInfo::builder()
        .application_name(b"Simple Rust Game\0")
        .application_version(vk::make_version(1, 0, 0))
        .engine_name(b"Oxide\0")
        .engine_version(vk::make_version(1, 0, 0))
        .api_version(vk::make_version(1, 0, 0));

    let mut extensions = vk_window::get_required_instance_extensions(window)
        .iter()
        .map(|e| e.as_ptr())
        .collect::<Vec<_>>();

    let flags = if
        cfg!(target_os = "macos") &&
        entry.version()? >= PORTABILITY_MACOS_VERSION
    {
        info!("Enabling exteniosn for macOS portability.");
        extensions.push(vk::KHR_GET_PHYSICAL_DEVICE_PROPERTIES2_EXTENSION.name.as_ptr());
        extensions.push(vk::KHR_PORTABILITY_ENUMERATION_EXTENSION.name.as_ptr());
        vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR

    } else {
        vk::InstanceCreateFlags::empty()
    };

    let info = vk::InstanceCreateInfo::builder()
        .application_info(&application_info)
        .enabled_extension_names(&extensions)
        .flags(flags);

    Ok(entry.create_instance(&info, None)?)
}

#[derive(Clone, Debug)]
struct App{
    entry: Entry,
    instance: Instance
}

impl App {
    unsafe fn create(window: &Window) -> Result<Self> {
        let loader = LibloadingLoader::new(LIBRARY)?;
        let entry = Entry::new(loader).map_err(|b| anyhow!("{}", b))?;
        let instance = create_instance(window, &entry)?;
        Ok(Self{entry, instance})
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

    unsafe fn destroy(&mut self) {
        self.instance.destroy_instance(None);
    }

}

#[derive(Clone, Debug, Default)]
struct AppData {}
