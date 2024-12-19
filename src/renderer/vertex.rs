use vulkanalia::vk::{self, HasBuilder};

use crate::math::vector::Vector3;
use crate::math::vector::Vector2;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    position : Vector3,
    color: Vector3,
    tex_coord: Vector2,
}

impl Vertex {
    pub const fn new(position: Vector3, color: Vector3, tex_coord: Vector2) -> Self {
        Self {position, color, tex_coord}
    }

    pub fn binding_description() -> vk::VertexInputBindingDescription {
        vk::VertexInputBindingDescription::builder()
            .binding(0)
            .stride(size_of::<Vertex>() as u32)
            .input_rate(vk::VertexInputRate::VERTEX)
            .build()
    }

    pub fn attribute_descriptions() -> [vk::VertexInputAttributeDescription; 3] {
        let position = vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .location(0)
            .format(vk::Format::R32G32B32_SFLOAT)
            .offset(0)
            .build();

        let color = vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .location(1)
            .format(vk::Format::R32G32B32_SFLOAT)
            .offset(size_of::<Vector3>() as u32)
            .build();

        let tex_coord = vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .location(2)
            .format(vk::Format::R32G32_SFLOAT)
            .offset((size_of::<Vector3>() + size_of::<Vector3>()) as u32)
            .build();

        [position, color, tex_coord]
    }
}
