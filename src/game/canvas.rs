use crate::rendering::glsl_types::Resolution;

use super::{dot::Dot, geometry::Vec2, material::Material};

#[derive(Debug)]
pub enum CanvasError {
    CoordOutOfBounds,
}

pub struct Canvas {
    pub resolution: Resolution,
    grid: Vec<Vec<Option<Dot>>>,
}
impl Canvas {
    pub fn new(resolution: Resolution) -> Self {
        let Resolution { height, width } = resolution;
        Self {
            resolution,
            grid: (0..height)
                .map(|_| (0..width).map(|_| None).collect())
                .collect(),
        }
    }

    pub fn iter_materials_as_bytes<'a>(&'a self) -> impl Iterator<Item = u8> + 'a {
        self.grid
            .iter()
            .flatten()
            .map(|maybe_dot| maybe_dot.map_or(Material::EmptySpace as u8, |dot| dot.material as u8))
    }

    pub fn get(&self, coord: Vec2<usize>) -> Result<Option<Dot>, CanvasError> {
        Ok(self
            .grid
            .get(coord.y)
            .ok_or(CanvasError::CoordOutOfBounds)?
            .get(coord.x)
            .ok_or(CanvasError::CoordOutOfBounds)?
            .clone())
    }

    pub fn set(&mut self, coord: Vec2<usize>, value: Option<Dot>) -> Result<(), CanvasError> {
        let dot = self
            .grid
            .get_mut(coord.y)
            .ok_or(CanvasError::CoordOutOfBounds)?
            .get_mut(coord.x)
            .ok_or(CanvasError::CoordOutOfBounds)?;

        *dot = value;

        Ok(())
    }

    pub fn clear(&mut self) {
        for maybe_dot in self.grid.iter_mut().flatten() {
            *maybe_dot = None;
        }
    }
}
