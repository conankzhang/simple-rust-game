use anyhow::{anyhow, Result};
use buffer::{create_index_buffer, create_vertex_buffer};
use cgmath::{Deg, Point3};
use command::{create_command_buffers, create_command_pool};
use descriptor::{create_descriptor_pool, create_descriptor_set_layout, create_descriptor_sets, create_uniform_buffers, Mat4, UniformBufferObject};
use device::{create_logical_device, pick_physical_device};
use image::{create_color_objects, create_depth_objects, create_texture_image, create_texture_image_view, create_texture_sampler};
use instance::{create_instance, create_sync_objects, load_model, VALIDATION_ENABLED};
use pipeline::{create_pipeline, create_render_pass};
use std::mem::size_of;
use std::ptr::copy_nonoverlapping as memcpy;
use std::u64;
use swapchain::{create_framebuffers, create_swapchain, create_swapchain_image_views};
use vertex::Vertex;
use vk::ImageView;
use vulkanalia::loader::{LibloadingLoader, LIBRARY};
use vulkanalia::prelude::v1_0::*;
use vulkanalia::window;
use vulkanalia::vk::ExtDebugUtilsExtension;
use vulkanalia::vk::KhrSurfaceExtension;
use vulkanalia::vk::KhrSwapchainExtension;
use winit::window::Window;

use crate::Character;

mod buffer;
mod command;
mod descriptor;
mod device;
mod image;
mod instance;
mod pipeline;
mod swapchain;
mod vertex;

type Vec3 = cgmath::Vector3<f32>;

pub const MAX_FRAMES_IN_FLIGHT: usize = 2;

#[derive(Debug)]
pub struct Renderer {
    entry: Entry,
    data: RenderData,
    instance: Instance,
    device: Device,
}

#[derive(Clone, Debug, Default)]
struct RenderData {
    surface: vk::SurfaceKHR,
    messenger: vk::DebugUtilsMessengerEXT,
    physical_device: vk::PhysicalDevice,
    msaa_samples: vk::SampleCountFlags,
    graphics_queue: vk::Queue,
    present_queue: vk::Queue,

    swapchain_format: vk::Format,
    swapchain_extent: vk::Extent2D,
    swapchain: vk::SwapchainKHR,
    swapchain_images: Vec<vk::Image>,
    swapchain_image_views: Vec<ImageView>,

    // Pipeline
    render_pass: vk::RenderPass,
    descriptor_set_layout: vk::DescriptorSetLayout,
    pipeline_layout: vk::PipelineLayout,
    pipeline: vk::Pipeline,

    framebuffers : Vec<vk::Framebuffer>,
    command_pool: vk::CommandPool,
    command_buffers: Vec<vk::CommandBuffer>,

    image_available_semaphores: Vec<vk::Semaphore>,
    render_finished_semaphores: Vec<vk::Semaphore>,
    in_flight_fences: Vec<vk::Fence>,
    images_in_flight: Vec<vk::Fence>,

    // Vertex Buffers
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    vertex_buffer: vk::Buffer,
    vertex_buffer_memory: vk::DeviceMemory,
    index_buffer: vk::Buffer,
    index_buffer_memory: vk::DeviceMemory,

    // Uniform Buffers
    uniform_buffers: Vec<vk::Buffer>,
    uniform_buffers_memory: Vec<vk::DeviceMemory>,
    descriptor_pool : vk::DescriptorPool,
    descriptor_sets : Vec<vk::DescriptorSet>,

    // Texture Sampling
    mip_levels: u32,
    texture_image : vk::Image,
    texture_image_memory: vk::DeviceMemory,
    texture_image_view: vk::ImageView,
    texture_sampler: vk::Sampler,

    // Depth Buffering
    depth_image: vk::Image,
    depth_image_memory: vk::DeviceMemory,
    depth_image_view: vk::ImageView,

    // Multisampling
    color_image : vk::Image,
    color_image_memory : vk::DeviceMemory,
    color_image_view: vk::ImageView,
}

impl Renderer {
    pub unsafe fn create(window: &Window) -> Result<Self>
    {
        let loader = LibloadingLoader::new(LIBRARY)?;
        let entry = Entry::new(loader).map_err(|b| anyhow!("{}", b))?;
        let mut data = RenderData::default();
        let instance = create_instance(window, &entry, &mut data)?;
        data.surface = window::create_surface(&instance, &window, &window)?;
        pick_physical_device(&instance, &mut data)?;

        let device = create_logical_device(&entry, &instance, &mut data)?;
        create_swapchain(window, &instance, &device, &mut data)?;
        create_swapchain_image_views(&device, &mut data)?;

        create_render_pass(&instance, &device, &mut data)?;
        create_descriptor_set_layout(&device, &mut data)?;
        create_pipeline(&device, &mut data)?;

        create_color_objects(&instance, &device, &mut data)?;
        create_depth_objects(&instance, &device, &mut data)?;
        create_framebuffers(&device, &mut data)?;
        create_command_pool(&instance, &device, &mut data)?;
        create_texture_image(&instance, &device, &mut data)?;
        create_texture_image_view(&device, &mut data)?;
        create_texture_sampler(&device, &mut data)?;

        load_model(&mut data)?;
        create_vertex_buffer(&instance, &device, &mut data)?;
        create_index_buffer(&instance, &device, &mut data)?;
        create_uniform_buffers(&instance, &device, &mut data)?;

        create_descriptor_pool(&device, &mut data)?;
        create_descriptor_sets(&device, &mut data)?;

        create_command_buffers(&device, &mut data)?;
        create_sync_objects(&device, &mut data)?;

        Ok(Self{entry, data, instance, device})
    }

    unsafe fn update_uniform_buffer(&self, character: &Character, image_index: usize) -> Result<()>
    {
        let mut model = Mat4::from_axis_angle(
            Vec3{x: 0.0, y:0.0, z:1.0},
            Deg(0.0)
        );

        let translation = Vec3{x: character.position.x, y: character.position.y, z: character.position.z};
        let position = Point3{x: character.position.x, y: character.position.y, z: character.position.z};
        let transformation = Mat4::from_translation(translation);

        model = model * transformation;

        let view_angle = character.position - character.view_angle.to_vector() * 1.0;
        let eye = Point3{x: view_angle.x, y: view_angle.y, z: view_angle.z};

        let view = Mat4::look_at_rh(
            eye,
            Point3{x: 0.0, y: 0.0, z: 0.0},
            Vec3{x: 0.0, y: 0.0, z: 1.0},
        );

        /*
            cgmath uses OpenGL range of -1.0 - 1.0 while Vulkan uses 0.0 - 1.0
            We also need to flip the Y-axis due to OpenGL coordinates
            Lastly, this matrix is transposed because cgmath constructs in column-major order
         */
        let correction = Mat4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, -1.0, 0.0, 0.0,
            0.0, 0.0, 1.0/2.0, 0.0,
            0.0, 0.0, 1.0/2.0, 1.0,
        );

        let projection = correction * cgmath::perspective(
            Deg(90.0),
            self.data.swapchain_extent.width as f32 / self.data.swapchain_extent.height as f32,
             0.1,
              1000.0);

        let ubo = UniformBufferObject{model, view, projection};

        let memory = self.device.map_memory(
            self.data.uniform_buffers_memory[image_index],
            0,
            size_of::<UniformBufferObject>() as u64,
            vk::MemoryMapFlags::empty()
        )?;

        memcpy(&ubo, memory.cast(), 1);
        self.device.unmap_memory(self.data.uniform_buffers_memory[image_index]);

        Ok(())
    }

    pub unsafe fn render(&mut self, frame: usize, resized : bool, character: &Character, window: &Window) -> Result<()>
    {
        self.device.wait_for_fences(&[self.data.in_flight_fences[frame]], true, u64::MAX, )?;

        let result = self
            .device
            .acquire_next_image_khr(
                self.data.swapchain,
                u64::MAX,
                self.data.image_available_semaphores[frame],
                vk::Fence::null());

        let image_index = match result {
            Ok((image_index, _)) => image_index as usize,
            Err(vk::ErrorCode::OUT_OF_DATE_KHR) => return self.recreate_swapchain(window),
            Err(e) => return Err(anyhow!(e)),
        };

        if !self.data.images_in_flight[image_index as usize].is_null() {
            self.device.wait_for_fences(
                &[self.data.images_in_flight[image_index as usize]],
                true,
                u64::MAX
            )?;
        }

        self.data.images_in_flight[image_index as usize] = self.data.in_flight_fences[frame];

        self.update_uniform_buffer(character, image_index)?;

        let wait_semaphores = &[self.data.image_available_semaphores[frame]];
        let wait_stages = &[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let command_buffers = &[self.data.command_buffers[image_index as usize]];
        let signal_semaphores = &[self.data.render_finished_semaphores[frame]];
        let submit_info = vk::SubmitInfo::builder()
            .wait_semaphores(wait_semaphores)
            .wait_dst_stage_mask(wait_stages)
            .command_buffers(command_buffers)
            .signal_semaphores(signal_semaphores);

        self.device.reset_fences(&[self.data.in_flight_fences[frame]])?;
        self.device.queue_submit(self.data.graphics_queue, &[submit_info], self.data.in_flight_fences[frame])?;

        let swapchains = &[self.data.swapchain];
        let image_indices = &[image_index as u32];
        let present_info = vk::PresentInfoKHR::builder()
            .wait_semaphores(signal_semaphores)
            .swapchains(swapchains)
            .image_indices(image_indices);

        let result = self.device.queue_present_khr(self.data.present_queue, &present_info);
        let changed = result == Ok(vk::SuccessCode::SUBOPTIMAL_KHR) || result == Err(vk::ErrorCode::OUT_OF_DATE_KHR);

        if resized || changed {
            self.recreate_swapchain(window)?;
        }
        else if let Err(e) = result {
            return Err(anyhow!(e));
        }

        Ok(())
    }

    unsafe fn recreate_swapchain(&mut self, window: &Window) -> Result<()> {
        self.device.device_wait_idle()?;
        self.destroy_swapchain();

        create_swapchain(window, &self.instance, &self.device, &mut self.data)?;
        create_swapchain_image_views(&self.device, &mut self.data)?;

        create_render_pass(&self.instance, &self.device, &mut self.data)?;
        create_pipeline(&self.device, &mut self.data)?;
        create_color_objects(&self.instance, &self.device, &mut self.data)?;
        create_depth_objects(&self.instance, &self.device, &mut self.data)?;

        create_framebuffers(&self.device, &mut self.data)?;
        create_uniform_buffers(&self.instance, &self.device, &mut self.data)?;
        create_descriptor_pool(&self.device, &mut self.data)?;
        create_descriptor_sets(&self.device, &mut self.data)?;
        create_command_buffers(&self.device, &mut self.data)?;

        self.data.images_in_flight.resize(self.data.swapchain_images.len(), vk::Fence::null());

        Ok(())
    }

    unsafe fn destroy_swapchain(&mut self) {
        self.device.destroy_image_view(self.data.color_image_view, None);
        self.device.free_memory(self.data.color_image_memory, None);
        self.device.destroy_image(self.data.color_image, None);

        self.device.destroy_image_view(self.data.depth_image_view, None);
        self.device.free_memory(self.data.depth_image_memory, None);
        self.device.destroy_image(self.data.depth_image, None);

        self.device.destroy_descriptor_pool(self.data.descriptor_pool, None);
        self.data.uniform_buffers
            .iter()
            .for_each(|b| self.device.destroy_buffer(*b, None));
        self.data.uniform_buffers_memory
            .iter()
            .for_each(|m| self.device.free_memory(*m, None));

        self.data.framebuffers
            .iter()
            .for_each(|f| self.device.destroy_framebuffer(*f, None));

        self.device.free_command_buffers(self.data.command_pool, &self.data.command_buffers);
        self.device.destroy_pipeline(self.data.pipeline, None);
        self.device.destroy_pipeline_layout(self.data.pipeline_layout, None);
        self.device.destroy_render_pass(self.data.render_pass, None);

        self.data.swapchain_image_views
            .iter()
            .for_each(|v| self.device.destroy_image_view(*v, None));

        self.device.destroy_swapchain_khr(self.data.swapchain, None);
    }

    pub unsafe fn destroy(&mut self) {
        self.device.device_wait_idle().unwrap();

        self.destroy_swapchain();

        self.device.destroy_sampler(self.data.texture_sampler, None);
        self.device.destroy_image_view(self.data.texture_image_view, None);
        self.device.destroy_image(self.data.texture_image, None);
        self.device.free_memory(self.data.texture_image_memory, None);

        self.device.destroy_descriptor_set_layout(self.data.descriptor_set_layout, None);

        self.device.destroy_buffer(self.data.index_buffer, None);
        self.device.free_memory(self.data.index_buffer_memory, None);

        self.device.destroy_buffer(self.data.vertex_buffer, None);
        self.device.free_memory(self.data.vertex_buffer_memory, None);

        self.data.in_flight_fences
            .iter()
            .for_each(|f| self.device.destroy_fence(*f, None));
        self.data.render_finished_semaphores
            .iter()
            .for_each(|s| self.device.destroy_semaphore(*s, None));
        self.data.image_available_semaphores
            .iter()
            .for_each(|s| self.device.destroy_semaphore(*s, None));

        self.device.destroy_command_pool(self.data.command_pool, None);
        self.device.destroy_device(None);
        self.instance.destroy_surface_khr(self.data.surface, None);

        if VALIDATION_ENABLED
        {
            self.instance.destroy_debug_utils_messenger_ext(self.data.messenger, None);
        }

        self.instance.destroy_instance(None);
    }
}
