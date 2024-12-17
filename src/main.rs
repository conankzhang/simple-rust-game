#![allow(
    dead_code,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

use anyhow::{anyhow, Result};
use log::*;

use std::collections::HashSet;
use std::ffi::CStr;
use std::os::raw::c_void;

use vulkanalia::loader::{LibloadingLoader, LIBRARY};
use vulkanalia::prelude::v1_0::*;
use vulkanalia::Version;
use vulkanalia::window as vk_window;
use vulkanalia::vk::ExtDebugUtilsExtension;

use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

mod math;
use std::time::Instant;

const PORTABILITY_MACOS_VERSION: Version = Version::new(1, 3, 216);
const VALIDATION_ENABLED: bool = cfg!(debug_assertions);
const VALIDATION_LAYER: vk::ExtensionName = vk::ExtensionName::from_bytes(b"VK_LAYER_KHRONOS_validation");

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

unsafe fn create_instance(window: &Window, entry: &Entry, data: &mut AppData) -> Result<Instance> {
    let application_info = vk::ApplicationInfo::builder()
        .application_name(b"Simple Rust Game\0")
        .application_version(vk::make_version(1, 0, 0))
        .engine_name(b"Oxide\0")
        .engine_version(vk::make_version(1, 0, 0))
        .api_version(vk::make_version(1, 0, 0));

    let available_layers = entry
        .enumerate_instance_layer_properties()?
        .iter()
        .map(|l| l.layer_name)
        .collect::<HashSet<_>>();

    if VALIDATION_ENABLED && !available_layers.contains(&VALIDATION_LAYER){
        return Err(anyhow!("Validation layer requested but not supported."));
    }

    let layers = if VALIDATION_ENABLED {
        vec![VALIDATION_LAYER.as_ptr()]
    } else {
        Vec::new()
    };

    let mut extensions = vk_window::get_required_instance_extensions(window)
        .iter()
        .map(|e| e.as_ptr())
        .collect::<Vec<_>>();

    if VALIDATION_ENABLED
    {
        extensions.push(vk::EXT_DEBUG_UTILS_EXTENSION.name.as_ptr());
    }

    let flags = if
        cfg!(target_os = "macos") &&
        entry.version()? >= PORTABILITY_MACOS_VERSION
    {
        info!("Enabling extension for macOS portability.");
        extensions.push(vk::KHR_GET_PHYSICAL_DEVICE_PROPERTIES2_EXTENSION.name.as_ptr());
        extensions.push(vk::KHR_PORTABILITY_ENUMERATION_EXTENSION.name.as_ptr());
        vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR

    } else {
        vk::InstanceCreateFlags::empty()
    };

    let mut info = vk::InstanceCreateInfo::builder()
        .application_info(&application_info)
        .enabled_layer_names(&layers)
        .enabled_extension_names(&extensions)
        .flags(flags);

    let mut debug_info = vk::DebugUtilsMessengerCreateInfoEXT::builder()
        .message_severity(vk::DebugUtilsMessageSeverityFlagsEXT::all())
        .message_type(
            vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
            | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
            | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
        )
        .user_callback(Some(debug_callback));

    if VALIDATION_ENABLED
    {
        info = info.push_next(&mut debug_info);
    }

    let instance = entry.create_instance(&info, None)?;

    if VALIDATION_ENABLED
    {
        let debug_info = vk::DebugUtilsMessengerCreateInfoEXT::builder()
            .message_severity(vk::DebugUtilsMessageSeverityFlagsEXT::all())
            .message_type(
                vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
            )
            .user_callback(Some(debug_callback));

        data.messenger = instance.create_debug_utils_messenger_ext(&debug_info, None)?;
    }

    Ok(instance)
}

extern "system" fn debug_callback(severity: vk::DebugUtilsMessageSeverityFlagsEXT, type_: vk::DebugUtilsMessageTypeFlagsEXT, data: *const vk::DebugUtilsMessengerCallbackDataEXT, _: *mut c_void) -> vk::Bool32 {
    let data = unsafe {*data};
    let message = unsafe {CStr::from_ptr(data.message)}.to_string_lossy();

    if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
    {
        error!("({:?}) {}", type_, message);
    }
    else if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
    {
        warn!("({:?}) {}", type_, message);
    }
    else if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::INFO
    {
        debug!("({:?}) {}", type_, message);
    }
    else {
        trace!("({:?}) {}", type_, message);
    }

    vk::FALSE
}

#[derive(Clone, Debug)]
struct App{
    entry: Entry,
    instance: Instance,
    data: AppData
}

impl App {
    unsafe fn create(window: &Window) -> Result<Self> {
        let loader = LibloadingLoader::new(LIBRARY)?;
        let entry = Entry::new(loader).map_err(|b| anyhow!("{}", b))?;
        let mut data = AppData::default();
        let instance = create_instance(window, &entry, &mut data)?;
        Ok(Self{entry, instance, data })
    }

    fn update(&mut self, delta_time : f32, character : & mut Character)
    {
        character.position = &character.position + &(&character.velocity * delta_time);
        character.velocity = &character.velocity + &(&character.gravity * delta_time);
    }

    unsafe fn render(&mut self, window: &Window) -> Result<()>
    {
        Ok(())
    }

    unsafe fn destroy(&mut self) {
        if VALIDATION_ENABLED
        {
            self.instance.destroy_debug_utils_messenger_ext(self.data.messenger, None);
        }

        self.instance.destroy_instance(None);
    }

}

#[derive(Clone, Debug, Default)]
struct AppData {
    messenger: vk::DebugUtilsMessengerEXT
}
