use std::sync::Arc;

use winit::{event_loop::EventLoop, window::Window, error::OsError};

mod event_handler;

use super::rendering::VulkanGraphicsPipeline;

enum WindowingError {
    OsError(OsError),
}
impl From<OsError> for WindowingError {
    fn from(os_error: OsError) -> Self {
        WindowingError::OsError(os_error)
    }
}

pub fn init_window() {
    let event_loop = EventLoop::new();
    let window = {
        let window = Window::new(&event_loop).unwrap();
        Arc::new(window)
    };

    let mut pipeline = VulkanGraphicsPipeline::new(&event_loop, window.clone());

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();
        event_handler::handle_event(event, control_flow, &mut pipeline); // TODO: handle error appropriately
    });
}
