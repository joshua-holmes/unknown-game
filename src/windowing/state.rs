use std::sync::Arc;

use winit::{event_loop::EventLoop, window::Window};

pub struct WindowState {
    pub window: Arc<Window>,
    pub event_loop: EventLoop<()>,
}
