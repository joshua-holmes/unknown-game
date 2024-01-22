use std::{sync::Arc, error::Error};

use winit::{event_loop::EventLoop, window::Window};

mod vulkan;

type WindowingError = Box<dyn Error>;

pub fn init_window() -> Result<(), WindowingError> {
    let event_loop = EventLoop::new();
    let window = {
        let window = Window::new(&event_loop)?;
        Arc::new(window)
    };

    vulkan::init(&event_loop, window.clone());

    Ok(())
}
