use winit::{event::{Event, WindowEvent}, event_loop::ControlFlow};

use crate::windowing::vulkan::VulkanGraphicsPipeline;

pub fn handle_event(event: Event<()>, control_flow: &mut ControlFlow, pipeline: &mut VulkanGraphicsPipeline) {
    match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            println!("User requested window to be closed");
            control_flow.set_exit();
        }
        Event::WindowEvent {
            event: WindowEvent::Resized(_),
            ..
        } => {
            pipeline.recreate_swapchain_and_resize_window();
        }
        Event::MainEventsCleared => {
            pipeline.display_next_frame();
        }
        _ => (),
    }
}
