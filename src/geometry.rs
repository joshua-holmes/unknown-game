use vulkano::{buffer::BufferContents, pipeline::graphics::vertex_input::Vertex as VertexMacro};

#[derive(BufferContents, VertexMacro)]
#[repr(C)]
pub struct Vertex {
    #[format(R32G32_SFLOAT)]
    pub position: [f32; 2],
}

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

