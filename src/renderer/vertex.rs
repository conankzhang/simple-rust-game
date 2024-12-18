use vulkanalia::vk::{self, HasBuilder};

use crate::math::vector::Vector;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    position : Vector,
    color: Vector
}

impl Vertex {
    pub const fn new(position: Vector, color: Vector) -> Self {
        Self {position, color}
    }

    pub fn binding_description() -> vk::VertexInputBindingDescription {
        vk::VertexInputBindingDescription::builder()
            .binding(0)
            .stride(size_of::<Vertex>() as u32)
            .input_rate(vk::VertexInputRate::VERTEX)
            .build()
    }

    pub fn attribute_descriptions() -> [vk::VertexInputAttributeDescription; 2] {
        let position = vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .location(0)
            .format(vk::Format::R32G32B32A32_SFLOAT)
            .offset(0)
            .build();

        let color = vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .location(1)
            .format(vk::Format::R32G32B32A32_SFLOAT)
            .offset(size_of::<Vector>() as u32)
            .build();

        [position, color]
    }
}
