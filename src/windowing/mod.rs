use std::{sync::Arc, error::Error};

use winit::{event_loop::EventLoop, window::Window};

mod vulkan;
mod event_handler;

use vulkan::VulkanGraphicsPipeline;

type WindowingError = Box<dyn Error>;

pub fn init_window() -> Result<(), WindowingError> {
    let event_loop = EventLoop::new();
    let window = {
        let window = Window::new(&event_loop)?;
        Arc::new(window)
    };

    let mut pipeline = VulkanGraphicsPipeline::new(&event_loop, window.clone())?;

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();
        event_handler::handle_event(event, control_flow, &mut pipeline).unwrap(); // TODO: handle error appropriately
    });
}
