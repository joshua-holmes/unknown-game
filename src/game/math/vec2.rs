use std::{ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign}, hash::Hash};
use winit::dpi::{PhysicalPosition, PhysicalSize};
use crate::rendering::glsl_types::Resolution;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
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

    pub fn clamp_to_resolution(&self, resolution: Resolution) -> Self
    where
        T: From<i32> + Copy + PartialOrd
    {
        self.clamp(
            Some(Vec2::new(T::from(0), T::from(0))),
            Some(Vec2::new(
                T::from(resolution.width - 1),
                T::from(resolution.height - 1)
            ))
        )
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

    pub fn abs(&self) -> Self {
        Self::new(self.x.abs(), self.y.abs())
    }

    pub fn to_rounded_usize(&self) -> Vec2<usize> {
        Vec2::new(self.x.round() as usize, self.y.round() as usize)
    }

    pub fn to_rounded_isize(&self) -> Vec2<isize> {
        Vec2::new(self.x.round() as isize, self.y.round() as isize)
    }

    pub fn angle_in_degrees(&self) -> f64 {
        (self.y.atan2(self.x).to_degrees() + 360.) % 360.
    }
}
impl Vec2<usize> {
    pub fn into_f64(&self) -> Vec2<f64> {
        Vec2::new(self.x as f64, self.y as f64)
    }
}
impl Vec2<isize> {
    pub fn into_f64(&self) -> Vec2<f64> {
        Vec2::new(self.x as f64, self.y as f64)
    }
    pub fn down(&self) -> Self {
        Self::new(self.x, self.y + 1)
    }
    pub fn up(&self) -> Self {
        Self::new(self.x, self.y - 1)
    }
    pub fn left(&self) -> Self {
        Self::new(self.x - 1, self.y)
    }
    pub fn right(&self) -> Self {
        Self::new(self.x + 1, self.y)
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
impl From<Vec2<usize>> for Vec2<isize> {
    fn from(value: Vec2<usize>) -> Self {
        Self {
            x: value.x as isize,
            y: value.y as isize,
        }
    }
}
impl From<Vec2<isize>> for Vec2<usize> {
    fn from(value: Vec2<isize>) -> Self {
        Self {
            x: value.x as usize,
            y: value.y as usize,
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
impl<T> Add<T> for Vec2<T>
where
    T: Add + Copy,
{
    type Output = Vec2<T::Output>;
    fn add(self, rhs: T) -> Self::Output {
        Self::Output {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}
impl<T> Add<T> for &Vec2<T>
where
    T: Add + Copy,
{
    type Output = Vec2<T::Output>;
    fn add(self, rhs: T) -> Self::Output {
        Self::Output {
            x: self.x + rhs,
            y: self.y + rhs,
        }
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
impl<T> Sub<T> for Vec2<T>
where
    T: Sub + Copy,
{
    type Output = Vec2<T::Output>;
    fn sub(self, rhs: T) -> Self::Output {
        Self::Output {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}
impl<T> Sub<T> for &Vec2<T>
where
    T: Sub + Copy,
{
    type Output = Vec2<T::Output>;
    fn sub(self, rhs: T) -> Self::Output {
        Self::Output {
            x: self.x - rhs,
            y: self.y - rhs,
        }
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
