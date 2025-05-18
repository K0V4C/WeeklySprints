use std::sync::Arc;

use vulkano::{
    device::{
        DeviceExtensions, QueueFlags,
        physical::{PhysicalDevice, PhysicalDeviceType},
    },
    instance::Instance,
    swapchain::{Surface, SurfaceCapabilities},
};

pub type QueueFamilyIndex = u32;

pub fn select_physical_device(
    instace: &Arc<Instance>,
    surface: &Arc<Surface>,
    device_extensions: &DeviceExtensions,
) -> (Arc<PhysicalDevice>, QueueFamilyIndex) {
    instace
        .enumerate_physical_devices()
        .expect("could not enumerate physical devices")
        .filter(|physical_device| {
            physical_device
                .supported_extensions()
                .contains(&device_extensions)
        })
        .filter_map(|physical_device| {
            physical_device
                .queue_family_properties()
                .iter()
                .enumerate()
                .position(|(queue_family_index, q)| {
                    q.queue_flags.contains(QueueFlags::GRAPHICS)
                        && physical_device
                            .surface_support(queue_family_index as u32, &surface)
                            .unwrap_or(false)
                })
                .map(|q| (physical_device, q as u32))
        })
        .min_by_key(
            |(physical_device, _)| match physical_device.properties().device_type {
                PhysicalDeviceType::DiscreteGpu => 0,
                PhysicalDeviceType::IntegratedGpu => 1,
                PhysicalDeviceType::VirtualGpu => 2,
                PhysicalDeviceType::Cpu => 3,
                _ => 4,
            },
        )
        .expect("no device available")
}

pub fn get_capabilities(
    physical_device: Arc<PhysicalDevice>,
    surface: Arc<Surface>,
) -> SurfaceCapabilities {
    physical_device
        .surface_capabilities(&surface, Default::default())
        .unwrap()
}
