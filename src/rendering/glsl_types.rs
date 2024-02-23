use vulkano::{pipeline::graphics::vertex_input::Vertex as VertexMacro, buffer::BufferContents};
use winit::dpi::PhysicalSize;

#[derive(BufferContents, VertexMacro, Debug)]
#[repr(C)]
pub struct Vertex {
    #[format(R32G32_SFLOAT)]
    pub position: [f32; 2],
}

#[derive(BufferContents, Clone, Debug)]
#[repr(C)]
pub struct Resolution {
    pub width: i32,
    pub height: i32,
}
impl Resolution {
    pub fn update_from(&mut self, value: PhysicalSize<u32>) {
        self.width = value.width as i32;
        self.height = value.height as i32;
    }
}
impl From<PhysicalSize<u32>> for Resolution {
    fn from(value: PhysicalSize<u32>) -> Self {
        Self { width: value.width as i32, height: value.height as i32 }
    }
}
impl From<&PhysicalSize<u32>> for Resolution {
    fn from(value: &PhysicalSize<u32>) -> Self {
        Self { width: value.width as i32, height: value.height as i32 }
    }
}
