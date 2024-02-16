use vulkano::{pipeline::graphics::vertex_input::Vertex as VertexMacro, buffer::BufferContents};
use winit::dpi::PhysicalSize;


#[derive(BufferContents, Clone, Debug)]
#[repr(C)]
pub struct Dot {
    pub dot_value: u32,
}

#[derive(BufferContents, VertexMacro, Debug)]
#[repr(C)]
pub struct Vertex {
    #[format(R32G32_SFLOAT)]
    pub position: [f32; 2],
}

#[derive(BufferContents, Clone, Debug)]
#[repr(C)]
pub struct Resolution {
    pub x: i32,
    pub y: i32,
}
impl Resolution {
    pub fn new(width: i32, height: i32) -> Self {
        Self { x: width, y: height }
    }
    pub fn update_from(&mut self, value: PhysicalSize<u32>) {
        self.x = value.width as i32;
        self.y = value.height as i32;
    }
}
impl From<PhysicalSize<u32>> for Resolution {
    fn from(value: PhysicalSize<u32>) -> Self {
        Self { x: value.width as i32, y: value.height as i32 }
    }
}
impl From<&PhysicalSize<u32>> for Resolution {
    fn from(value: &PhysicalSize<u32>) -> Self {
        Self { x: value.width as i32, y: value.height as i32 }
    }
}
