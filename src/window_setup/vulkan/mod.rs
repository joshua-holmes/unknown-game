use std::{error::Error, sync::Arc};

use vulkano::{
    instance::{Instance, InstanceCreateInfo},
    swapchain::{Surface, Swapchain, SwapchainCreateInfo},
    Version, VulkanLibrary,
    device::{DeviceExtensions, QueueFlags, Queue, physical::{PhysicalDevice, PhysicalDeviceType}, DeviceCreateInfo, QueueCreateInfo, Device}, VulkanError, image::Image
};
use winit::{event_loop::EventLoop, window::Window};

type VulkanApiError = Box<dyn Error>;

fn init_vulkan_and_window(
    event_loop: &EventLoop<()>,
    window: Arc<Window>,
) -> Result<(Arc<Instance>, Arc<Surface>), VulkanApiError> {
    let library = VulkanLibrary::new()?;
    let required_extensions = Surface::required_extensions(event_loop);
    let vk_instance = Instance::new(
        library,
        InstanceCreateInfo {
            application_name: Some("Unknown Game".to_string()),
            application_version: Version::major_minor(0, 0),
            enabled_extensions: required_extensions,
            ..Default::default()
        },
    )?;
    let vk_surface = Surface::from_window(vk_instance.clone(), window)?;

    Ok((vk_instance, vk_surface))
}

fn get_graphics_device(vk_instance: Arc<Instance>, vk_surface: Arc<Surface>) -> Result<(Arc<Device>, Arc<Queue>), VulkanApiError> {
    let device_extensions = DeviceExtensions {
        khr_swapchain: true,
        ..Default::default()
    };

    let (device, mut queues) = vk_instance
        .enumerate_physical_devices()?
        .filter(|pysical_device| pysical_device.supported_extensions().contains(&device_extensions))
        .filter_map(|pysical_device| {
            pysical_device.queue_family_properties()
                .iter()
                .enumerate()
                .position(|(i, queue_family_properties)| {
                    queue_family_properties.queue_flags.contains(QueueFlags::GRAPHICS)
                        && pysical_device.surface_support(i as u32, &vk_surface).unwrap_or(false)
                })
                .map(|q| (pysical_device, q as u32))
        })
        .min_by_key(|(pysical_device, _)| match pysical_device.properties().device_type {
            PhysicalDeviceType::DiscreteGpu => 0,
            PhysicalDeviceType::IntegratedGpu => 1,
            PhysicalDeviceType::VirtualGpu => 2,
            PhysicalDeviceType::Cpu => 3,
            _ => 4,
        })
        .ok_or(VulkanError::DeviceLost) // FIX: Use my own error because this is wrong
        .map(|(physical_device, queue_family_index)| {
            Device::new(
                physical_device.clone(),
                DeviceCreateInfo {
                    queue_create_infos: vec![QueueCreateInfo {
                        queue_family_index,
                        ..Default::default()
                    }],
                    enabled_extensions: device_extensions,
                    ..Default::default()
                }
            )
        })??;

    Ok((device, queues.next().unwrap()))
}

fn create_swapchain(device: Arc<Device>, vk_surface: Arc<Surface>, window: Arc<Window>) -> Result<(Arc<Swapchain>, Vec<Arc<Image>>), VulkanApiError> {
    let capabilities = device
        .physical_device()
        .surface_capabilities(&vk_surface, Default::default())?;
    let (image_format, image_color_space) = device.physical_device().surface_formats(&vk_surface, Default::default())?[0];
    Ok(Swapchain::new(
        device.clone(),
        vk_surface.clone(),
        SwapchainCreateInfo {
            min_image_count: capabilities.min_image_count + 1,
            image_format,
            image_color_space,
            image_extent: window.inner_size().into(),
            composite_alpha: capabilities.supported_composite_alpha.into_iter().next().unwrap(), // TODO: setup error handling
            ..Default::default()
        }
    )?)
}

pub fn init(event_loop: &EventLoop<()>, window: Arc<Window>) -> Result<(), VulkanApiError> {
    // init vulkan and window
    let (vk_instance, vk_surface) = init_vulkan_and_window(event_loop, window.clone())?;

    // get graphics device
    let (device, queue) = get_graphics_device(vk_instance.clone(), vk_surface.clone())?;

    // create swapchain
    let (mut swapchain, images) = create_swapchain(device.clone(), vk_surface.clone(), window.clone())?;

    // setup basic triangle

    // setup vertex buffer

    // setup render pass

    // create image view

    // load shaders

    // setup viewport

    // create command buffers

    // setup fences vector so CPU doesn't have to wait for GPU

    Ok(())
}
