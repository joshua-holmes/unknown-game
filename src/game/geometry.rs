use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use winit::dpi::{PhysicalPosition, PhysicalSize};

use crate::rendering::glsl_types::{Resolution, Vertex};

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
    where
        T: Copy + PartialOrd,
    {
        let mut copy_self = self.clone();
        if let Some(min) = min {
            copy_self.x = if min.x > copy_self.x {
                min.x
            } else {
                copy_self.x
            };
            copy_self.y = if min.y > copy_self.y {
                min.y
            } else {
                copy_self.y
            };
        }
        if let Some(max) = max {
            copy_self.x = if max.x < copy_self.x {
                max.x
            } else {
                copy_self.x
            };
            copy_self.y = if max.y < copy_self.y {
                max.y
            } else {
                copy_self.y
            };
        }
        copy_self
    }
}
impl Vec2<f64> {
    pub fn pythagorean_theorem(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn new_from_direction(direction_in_degrees: f64, length: f64) -> Self {
        Vec2 {
            x: direction_in_degrees.to_radians().cos() * length,
            y: direction_in_degrees.to_radians().sin() * length,
        }
    }

    pub fn to_negative(self) -> Self {
        Self::new(-self.x, -self.y)
    }
}
impl<T> From<PhysicalPosition<T>> for Vec2<T> {
    fn from(value: PhysicalPosition<T>) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}
impl<T> From<&PhysicalPosition<T>> for Vec2<T>
where
    T: Copy,
{
    fn from(value: &PhysicalPosition<T>) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}
impl<T> From<PhysicalSize<T>> for Vec2<T> {
    fn from(value: PhysicalSize<T>) -> Self {
        Self {
            x: value.width,
            y: value.height,
        }
    }
}
impl<T> From<&PhysicalSize<T>> for Vec2<T>
where
    T: Copy,
{
    fn from(value: &PhysicalSize<T>) -> Self {
        Self {
            x: value.width,
            y: value.height,
        }
    }
}
impl<T> From<Resolution> for Vec2<T>
where
    T: From<i32>,
{
    fn from(value: Resolution) -> Self {
        Self {
            x: T::from(value.width),
            y: T::from(value.height),
        }
    }
}
impl<T> From<&Resolution> for Vec2<T>
where
    T: From<i32>,
{
    fn from(value: &Resolution) -> Self {
        Self {
            x: T::from(value.width),
            y: T::from(value.height),
        }
    }
}
impl<T> Mul<T> for Vec2<T>
where
    T: Mul + Copy,
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
where
    T: Mul + Copy,
{
    type Output = Vec2<T::Output>;
    fn mul(self, rhs: T) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl<T> Div<T> for Vec2<T>
where
    T: Div + Copy,
{
    type Output = Vec2<T::Output>;
    fn div(self, rhs: T) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
impl<T> Div<T> for &Vec2<T>
where
    T: Div + Copy,
{
    type Output = Vec2<T::Output>;
    fn div(self, rhs: T) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
impl<T> Add for Vec2<T>
where
    T: Add,
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
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl<'a, T> Add for &'a Vec2<T>
where
    T: Add + Copy,
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
where
    T: AddAssign + Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl<T> Div for Vec2<T>
where
    T: Div,
{
    type Output = Vec2<T::Output>;
    fn div(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}
impl<T> DivAssign for Vec2<T>
where
    T: DivAssign,
{
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}
impl<'a, T> Div for &'a Vec2<T>
where
    T: Div + Copy,
{
    type Output = Vec2<T::Output>;
    fn div(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}
impl<T> DivAssign for &mut Vec2<T>
where
    T: DivAssign + Copy,
{
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}
impl<T> Sub for Vec2<T>
where
    T: Sub,
{
    type Output = Vec2<T::Output>;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl<T> SubAssign for Vec2<T>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
impl<'a, T> Sub for &'a Vec2<T>
where
    T: Sub + Copy,
{
    type Output = Vec2<T::Output>;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl<T> SubAssign for &mut Vec2<T>
where
    T: SubAssign + Copy,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
impl<T> Mul for Vec2<T>
where
    T: Mul,
{
    type Output = Vec2<T::Output>;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}
impl<T> MulAssign for Vec2<T>
where
    T: MulAssign,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}
impl<'a, T> Mul for &'a Vec2<T>
where
    T: Mul + Copy,
{
    type Output = Vec2<T::Output>;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}
impl<T> MulAssign for &mut Vec2<T>
where
    T: MulAssign + Copy,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}
