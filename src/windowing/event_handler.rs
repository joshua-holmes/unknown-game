use winit::{event::{Event, WindowEvent}, event_loop::ControlFlow};

use crate::{windowing::vulkan::VulkanGraphicsPipeline, geometry};

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
            while let Err(e) = pipeline.vertex_buffer.write() {
                println!("cannot get vertex buffer lock, {}", e);
            }
            pipeline
                .vertex_buffer
                .write()
                .unwrap()
                .last_mut()
                .unwrap()
            .position[1] += 0.1;
            pipeline.display_next_frame();
        }
        _ => (),
    }
}
