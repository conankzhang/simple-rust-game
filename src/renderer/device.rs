use std::collections::HashSet;

use anyhow::{anyhow, Result};
use log::{info, warn};
use thiserror::Error;
use vulkanalia::{vk::{self, DeviceQueueCreateInfo, DeviceV1_0, HasBuilder, InstanceV1_0}, Device, Entry, Instance};

use crate::renderer::VALIDATION_LAYER;

use super::{swapchain::SwapchainSupport, QueueFamilyIndices, RenderData, PORTABILITY_MACOS_VERSION, VALIDATION_ENABLED};

const DEVICE_EXTENSIONS: &[vk::ExtensionName] = &[vk::KHR_SWAPCHAIN_EXTENSION.name];

#[derive(Debug, Error)]
#[error("Missing {0}.")]
pub struct SuitabilityError(pub &'static str);

pub unsafe fn pick_physical_device(instance: &Instance, data: &mut RenderData) ->Result<()> {

    for physical_device in instance.enumerate_physical_devices()? {
        let properties = instance.get_physical_device_properties(physical_device);

        if let Err(error) = check_physical_device(instance,  data, physical_device) {
            warn!("Skipping physical device (`{}`): {}", properties.device_name, error);
        }
        else {
            info!("Selected physical device (`{}`)", properties.device_name);
            data.physical_device = physical_device;
            return Ok(())
        }
    }

    Err(anyhow!("Failed to find suitable physical device."))
}

pub unsafe fn check_physical_device(instance: &Instance, data: & RenderData, physical_device : vk::PhysicalDevice) ->Result<()> {
    QueueFamilyIndices::get(instance, data, physical_device)?;
    check_physical_device_extensions(instance, physical_device)?;

    let support = SwapchainSupport::get(instance, data, physical_device)?;
    if support.formats.is_empty() || support.present_modes.is_empty() {
        return Err(anyhow!(SuitabilityError("Insufficient swapchain support.")))
    }

    let features = instance.get_physical_device_features(physical_device);
    if features.sampler_anisotropy != vk::TRUE {
        return Err(anyhow!(SuitabilityError("No sampler anisotropy.")));
    }

    Ok(())
}

pub unsafe fn check_physical_device_extensions(instance: &Instance, physical_device : vk::PhysicalDevice) ->Result<()> {
    let extensions = instance.enumerate_device_extension_properties(physical_device, None)?
        .iter()
        .map(|e| e.extension_name)
        .collect::<HashSet<_>>();

    if DEVICE_EXTENSIONS.iter().all(|e| extensions.contains(e)) {
        Ok(())
    }
    else {
        Err(anyhow!(SuitabilityError("Missing required device extensions.")))
    }
}

pub unsafe fn create_logical_device(entry: &Entry, instance: &Instance, data: &mut RenderData) -> Result<Device> {
    let indices = QueueFamilyIndices::get(instance, data, data.physical_device)?;

    let mut unique_indices = HashSet::new();
    unique_indices.insert(indices.graphics);
    unique_indices.insert(indices.present);

    let queue_priorities = &[1.0];
    let queue_infos = unique_indices
        .iter()
        .map(|i| {
            vk::DeviceQueueCreateInfo::builder()
                .queue_family_index(*i)
                .queue_priorities(queue_priorities).build()
        })
        .collect::<Vec<DeviceQueueCreateInfo>>();

    let layers = if VALIDATION_ENABLED {
        vec![VALIDATION_LAYER.as_ptr()]
    } else {
        vec![]
    };

    let mut extensions = DEVICE_EXTENSIONS
        .iter()
        .map(|n| n.as_ptr())
        .collect::<Vec<_>>();

    if cfg!(target_os = "macos") && entry.version()? >= PORTABILITY_MACOS_VERSION
    {
        extensions.push(vk::KHR_PORTABILITY_SUBSET_EXTENSION.name.as_ptr());
    }

    let features  = vk::PhysicalDeviceFeatures::builder()
        .sampler_anisotropy(true);

    let info = vk::DeviceCreateInfo::builder()
        .queue_create_infos(&queue_infos)
        .enabled_layer_names(&layers)
        .enabled_extension_names(&extensions)
        .enabled_features(&features);

    let device = instance.create_device(data.physical_device, &info, None)?;
    data.graphics_queue = device.get_device_queue(indices.graphics, 0);
    data.present_queue = device.get_device_queue(indices.present, 0);

    Ok(device)
}
