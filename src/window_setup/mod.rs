use std::sync::Arc;

use winit::{event_loop::EventLoop, window::Window};

mod vulkan;

pub enum MyWindowError {

}

pub fn init() {
    let event_loop = EventLoop::new();
    let window = {
        let window = Window::new(&event_loop)?;
        Arc::new(window)
    };

    vulkan::init(&event_loop, window.clone());
}
