use std::sync::Arc;

use winit::{event_loop::EventLoop, window::Window};

use crate::game::canvas::Canvas;

use self::render_engine::RenderEngine;

pub mod render_engine;
pub mod glsl_types;
mod load_shaders;

pub fn init(event_loop: &EventLoop<()>, window: Arc<Window>, canvas: &Canvas) -> RenderEngine {
    RenderEngine::new(event_loop, window, canvas)
}
