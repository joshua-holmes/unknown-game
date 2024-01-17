use std::{error::Error, sync::Arc};

use vulkano::{
    instance::{Instance, InstanceCreateInfo},
    swapchain::Surface,
    Version, VulkanLibrary,
};
use winit::{event_loop::EventLoop, window::Window};

type MyVulkanError = Box<dyn Error>;

fn init_vulkan_and_window(
    event_loop: &EventLoop<()>,
    window: Arc<Window>,
) -> Result<(Arc<Instance>, Arc<Surface>), MyVulkanError> {
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

pub fn init(event_loop: &EventLoop<()>, window: Arc<Window>) -> Result<(), MyVulkanError> {
    // init vulkan and window
    let (vk_instance, vk_surface) = init_vulkan_and_window(event_loop, window.clone())?;

    // setup graphics device

    // create swapchain

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
