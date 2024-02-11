use vulkano::{buffer::BufferContents, pipeline::graphics::vertex_input::Vertex as VertexMacro};
use winit::dpi::PhysicalSize;

#[derive(BufferContents, VertexMacro, Clone, Debug)]
#[repr(C)]
pub struct Dot {
    #[format(R32_UINT)]
    pub dot_value: u32,
}

pub struct Canvas {
    pub grid: Vec<Vec<Dot>>,
}
#[allow(dead_code)]
impl Canvas {
    pub fn new(resolution: PhysicalSize<u32>) -> Self {
        let grid = (0..resolution.height).map(|j| {
            (0..resolution.width).map(|i| {
                Dot { dot_value: if (i + j % 2) % 2 == 0 { 9 } else { 0 } }
                // Dot { dot_value: 9 }
            }).collect()
        }).collect();
        Self {
            grid,
        }
    }

    pub fn to_vec_of_dots(&self) -> Vec<Dot> {
        self.grid.iter().flatten().cloned().collect()
    }

    pub fn resolution(&self) -> PhysicalSize<u32> {
        PhysicalSize {
            height: self.grid.len() as u32,
            width: self.grid[0].len() as u32
        }
    }
}

#[derive(BufferContents, VertexMacro, Debug)]
#[repr(C)]
pub struct Vertex {
    #[format(R32G32_SFLOAT)]
    pub position: [f32; 2],
}

#[derive(Debug)]
pub struct Triangle(Vertex, Vertex, Vertex);
#[allow(dead_code)]
impl Triangle {
    pub fn new(point_1: [f32; 2], point_2: [f32; 2], point_3: [f32; 2]) -> Self {
        Self(
            Vertex { position: point_1 },
            Vertex { position: point_2 },
            Vertex { position: point_3 },
        )
    }

    pub fn into_vec_of_verticies(self) -> Vec<Vertex> {
        vec![self.0, self.1, self.2]
    }
}

#[derive(Debug)]
pub struct Model(Vec<Triangle>);
#[allow(dead_code)]
impl Model {
    pub fn new(mut triangles: impl Iterator<Item = Triangle>) -> Self {
        let mut model_triangles = Vec::new();
        while let Some(t) = triangles.next() {
            model_triangles.push(t);
        }
        Self(model_triangles)
    }

    pub fn into_vec_of_verticies(self) -> Vec<Vertex> {
        let mut v = Vec::with_capacity(self.0.len() * 3);
        for triangle in self.0 {
            for vertex in triangle.into_vec_of_verticies() {
                v.push(vertex);
            }
        }
        v
    }

    pub fn count_verticies(&self) -> u32 {
        // TODO: count number of verticies in triangle dynamically instead of hard coded. probably will involve a macro
        let verticies_per_triangle = 3;
        self.0.len() as u32 * verticies_per_triangle
    }
}

#[derive(BufferContents, VertexMacro, Clone, Debug)]
#[repr(C)]
pub struct Vec2 {
    #[format(R32_SINT)]
    pub x: i32,
    #[format(R32_SINT)]
    pub y: i32,
}
impl Vec2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
impl From<PhysicalSize<u32>> for Vec2 {
    fn from(value: PhysicalSize<u32>) -> Self {
        Self { x: value.width as i32, y: value.height as i32 }
    }
}
impl From<&PhysicalSize<u32>> for Vec2 {
    fn from(value: &PhysicalSize<u32>) -> Self {
        Self { x: value.width as i32, y: value.height as i32 }
    }
}

