use std::ops::{Add, AddAssign, Mul};

use crate::rendering::glsl_types::Vertex;

#[derive(Debug)]
pub struct Triangle(Vertex, Vertex, Vertex);
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

pub enum Quadrant {
    UpperLeft,
    UpperRight,
    LowerLeft,
    LowerRight,
}

#[derive(Clone, Copy, Debug)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}
impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn clamp(&self, min: Option<Self>, max: Option<Self>) -> Self
        where T: Copy + PartialOrd
    {
        let mut copy_self = self.clone();
        if let Some(min) = min {
            copy_self.x = if min.x > copy_self.x { min.x } else { copy_self.x };
            copy_self.y = if min.y > copy_self.y { min.y } else { copy_self.y };
        }
        if let Some(max) = max {
            copy_self.x = if max.x < copy_self.x { max.x } else { copy_self.x };
            copy_self.y = if max.y < copy_self.y { max.y } else { copy_self.y };
        }
        copy_self
    }
}
impl Vec2<f64> {
    pub fn direction_in_degrees(&self) -> f64 {
        (self.x / self.y).tan().to_degrees()
    }

    pub fn quadrant(&self) -> Option<Quadrant> {
        if self.x > 0. && self.y > 0. {
            Some(Quadrant::LowerRight)
        } else if self.x > 0. && self.y < 0. {
            Some(Quadrant::UpperRight)
        } else if self.x < 0. && self.y > 0. {
            Some(Quadrant::LowerLeft)
        } else if self.x < 0. && self.y < 0. {
            Some(Quadrant::UpperLeft)
        } else {
            None
        }
    }
}
impl<T> Mul<T> for Vec2<T>
    where T: Mul + Copy
{
    type Output = Vec2<T::Output>;
    fn mul(self, rhs: T) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl<T> Mul<T> for &Vec2<T>
    where T: Mul + Copy
{
    type Output = Vec2<T::Output>;
    fn mul(self, rhs: T) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl<T> Add for Vec2<T>
    where T: Add
{
    type Output = Vec2<T::Output>;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl<T> AddAssign for Vec2<T>
    where T: AddAssign
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl<'a, T> Add for &'a Vec2<T>
    where T: Add + Copy
{
    type Output = Vec2<T::Output>;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl<T> AddAssign for &mut Vec2<T>
    where T: AddAssign + Copy
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
