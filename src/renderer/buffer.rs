use anyhow::Result;
use std::ptr::copy_nonoverlapping as memcpy;
use vulkanalia::{vk::{self, DeviceV1_0, HasBuilder}, Device, Instance};

use super::{command::{begin_single_time_commands, end_single_time_commands}, device::get_memory_type_index, vertex::Vertex, RenderData, INDICES, VERTICES};

pub unsafe fn create_buffer(instance: &Instance, device: &Device, data: &RenderData, size: vk::DeviceSize, usage: vk::BufferUsageFlags, properties: vk::MemoryPropertyFlags) ->Result<(vk::Buffer, vk::DeviceMemory)>
{
    let buffer_info = vk::BufferCreateInfo::builder()
        .size(size)
        .usage(usage)
        .sharing_mode(vk::SharingMode::EXCLUSIVE)
        .flags(vk::BufferCreateFlags::empty());

    let buffer = device.create_buffer(&buffer_info, None)?;

    let requirements = device.get_buffer_memory_requirements(buffer);
    let memory_info = vk::MemoryAllocateInfo::builder()
        .allocation_size(requirements.size)
        .memory_type_index(get_memory_type_index(
            instance,
            data,
            properties,
            requirements
        )?);

    let buffer_memory = device.allocate_memory(&memory_info, None)?;
    device.bind_buffer_memory(buffer, buffer_memory, 0)?;

    Ok((buffer, buffer_memory))
}

pub unsafe fn copy_buffer(device: &Device, data: &RenderData, source: vk::Buffer, destination: vk::Buffer, size: vk::DeviceSize) ->Result<()>
{
    let command_buffer = begin_single_time_commands(device, data)?;

    let regions = vk::BufferCopy::builder().size(size);
    device.cmd_copy_buffer(command_buffer, source, destination, &[regions]);

    end_single_time_commands(device, data, command_buffer)?;

    Ok(())
}

pub unsafe fn create_vertex_buffer(instance: &Instance, device: &Device, data: &mut RenderData) ->Result<()>
{
    let size = (size_of::<Vertex>() * VERTICES.len()) as u64;

    let (staging_buffer, staging_buffer_memory) = create_buffer(
        instance,
        device,
        data,
        size,
        vk::BufferUsageFlags::TRANSFER_SRC,
        vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
    )?;

    let memory = device.map_memory(staging_buffer_memory, 0, size, vk::MemoryMapFlags::empty())?;
    memcpy(VERTICES.as_ptr(), memory.cast(), VERTICES.len());
    device.unmap_memory(staging_buffer_memory);

    let (vertex_buffer, vertex_buffer_memory) = create_buffer(
        instance,
        device,
        data,
        size,
        vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::VERTEX_BUFFER,
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
    )?;

    data.vertex_buffer = vertex_buffer;
    data.vertex_buffer_memory = vertex_buffer_memory;

    copy_buffer(device, data, staging_buffer, vertex_buffer, size)?;

    device.destroy_buffer(staging_buffer, None);
    device.free_memory(staging_buffer_memory, None);

    Ok(())
}

pub unsafe fn create_index_buffer(instance: &Instance, device: &Device, data: &mut RenderData) ->Result<()>
{
    let size = (size_of::<u32>() * INDICES.len()) as u64;

    let (staging_buffer, staging_buffer_memory) = create_buffer(
        instance,
        device,
        data,
        size,
        vk::BufferUsageFlags::TRANSFER_SRC,
        vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
    )?;

    let memory = device.map_memory(staging_buffer_memory, 0, size, vk::MemoryMapFlags::empty())?;
    memcpy(INDICES.as_ptr(), memory.cast(), INDICES.len());
    device.unmap_memory(staging_buffer_memory);

    let (index_buffer, index_buffer_memory) = create_buffer(
        instance,
        device,
        data,
        size,
        vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::INDEX_BUFFER,
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
    )?;

    data.index_buffer = index_buffer;
    data.index_buffer_memory = index_buffer_memory;

    copy_buffer(device, data, staging_buffer, index_buffer, size)?;

    device.destroy_buffer(staging_buffer, None);
    device.free_memory(staging_buffer_memory, None);

    Ok(())
}
