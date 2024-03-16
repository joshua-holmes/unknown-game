use std::collections::VecDeque;

use crate::rendering::glsl_types::Resolution;

use super::{dot::Dot, geometry::Vec2, material::Material};

#[derive(Debug)]
pub enum CanvasError {
    CoordOutOfBounds,
}

#[derive(Debug)]
pub struct RayPoint {
    pub coord: Vec2<usize>,
    pub dot: Option<Dot>,
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

    /// Casts a ray and captures every point in the path of the ray in order.
    /// Start of ray is exclusive, end is inclusive. Casts a ray and creates a new ray cast object.
    pub fn cast_ray(&self, ray_start: Vec2<f64>, ray_end: Vec2<f64>) -> VecDeque<RayPoint> {
        let mut path = VecDeque::new();

        let diff = ray_end - ray_start;
        let direction = diff / diff.abs();

        let find_next_coord = |cur_coord: Vec2<usize>| {
            let horizontal_coord =
                Vec2::new((cur_coord.x as f64 + direction.x) as usize, cur_coord.y);
            let vertical_coord =
                Vec2::new(cur_coord.x, (cur_coord.y as f64 + direction.y) as usize);
            let diagonal = Vec2::new(
                (cur_coord.x as f64 + direction.x) as usize,
                (cur_coord.y as f64 + direction.y) as usize,
            );
            let horizontal_to_end = (ray_end
                - Vec2::new(horizontal_coord.x as f64, horizontal_coord.y as f64))
            .pythagorean_theorem();
            let vertical_to_end = (ray_end
                - Vec2::new(vertical_coord.x as f64, vertical_coord.y as f64))
            .pythagorean_theorem();
            if horizontal_to_end < vertical_to_end {
                horizontal_coord
            } else if horizontal_to_end > vertical_to_end {
                vertical_coord
            } else {
                diagonal
            }
        };

        let mut next_coord = Some(find_next_coord(Vec2::new(
            ray_start.x.round() as usize,
            ray_start.y.round() as usize,
        )));

        while let Some(coord) = next_coord.take() {
            match self.get(coord) {
                Ok(dot_maybe) => path.push_back(RayPoint {
                    coord,
                    dot: dot_maybe,
                }),
                Err(CanvasError::CoordOutOfBounds) => {
                    println!("Ray cast blast got cast where it won't last! Some features may not function properly.");
                    return path;
                }
            }
            let end_of_ray_reached =
                coord == Vec2::new(ray_end.x.round() as usize, ray_end.y.round() as usize);
            if !end_of_ray_reached {
                next_coord = Some(find_next_coord(coord));
            }
        }

        path
    }
}

#[cfg(test)]
mod tests {
    use crate::{game::{
        dot::Dot, geometry::Vec2, id_generator::IdGenerator,
        material::Material, canvas::Canvas,
    }, rendering::glsl_types::Resolution};

    fn setup_canvas(test_data: Vec<Dot>) -> Canvas {
        const WIDTH: usize = 10;
        const HEIGHT: usize = 10;

        let mut canvas = Canvas::new(Resolution { height: HEIGHT as i32, width: WIDTH as i32 });

        for dot in test_data.iter() {
            let coord = Vec2::new(
                dot.position.x.round() as usize,
                dot.position.y.round() as usize,
            );
            if coord.x > WIDTH || coord.y > HEIGHT {
                panic!("Test data was not setup correctly, dot placed out of bounds.");
            }
            canvas.set(coord, Some(*dot));
        }

        canvas
    }

    #[test]
    fn test_cast_finds_end_point() {
        let ray_start = Vec2::new(5., 5.);
        let ray_end = Vec2::new(6., 5.);
        let mut dot_id_generator = IdGenerator::new();
        let test_data = vec![
            Dot::new(
                &mut dot_id_generator,
                Material::Sand,
                ray_start,
                Vec2::new(0., 0.),
            ),
            Dot::new(
                &mut dot_id_generator,
                Material::Sand,
                ray_end,
                Vec2::new(0., 0.),
            ),
        ];

        let expected_id = test_data[1].id;

        let canvas = setup_canvas(test_data);
        let path = canvas.cast_ray(ray_start, ray_end);

        assert_eq!(
            1,
            path.len(),
            "Ray cast did not stop at end point:\n{:?}",
            path
        );
        assert_eq!(
            expected_id,
            path[0].dot.unwrap().id,
            "Could not find correct ray end dot"
        );
        assert_eq!(
            Vec2::new(6, 5),
            path[0].coord,
            "End point was not the correct coordinate"
        );
    }

    #[test]
    fn test_cast_includes_empty_points() {
        let ray_start = Vec2::new(4., 5.);
        let ray_end = Vec2::new(7., 5.);
        let mut dot_id_generator = IdGenerator::new();
        let test_data = vec![
            Dot::new(
                &mut dot_id_generator,
                Material::Sand,
                ray_start,
                Vec2::new(0., 0.),
            ),
            Dot::new(
                &mut dot_id_generator,
                Material::Sand,
                ray_end,
                Vec2::new(0., 0.),
            ),
        ];

        let canvas = setup_canvas(test_data);
        let path = canvas.cast_ray(ray_start, ray_end);

        assert_eq!(3, path.len());
        assert!(
            path[0].dot.is_none(),
            "First point was not empty:\n{:?}",
            path[0]
        );
        assert!(
            path[1].dot.is_none(),
            "Second point was not empty:\n{:?}",
            path[1]
        );
    }

    #[test]
    fn test_cast_includes_dots_in_middle_of_ray_and_empty_end_point() {
        let ray_start = Vec2::new(4., 5.);
        let middle_dot_point_1 = Vec2::new(5., 5.);
        let middle_dot_point_2 = Vec2::new(6., 5.);
        let ray_end = Vec2::new(7., 5.);
        let mut dot_id_generator = IdGenerator::new();
        let test_data = vec![
            Dot::new(
                &mut dot_id_generator,
                Material::Sand,
                ray_start,
                Vec2::new(0., 0.),
            ),
            Dot::new(
                &mut dot_id_generator,
                Material::Sand,
                middle_dot_point_1,
                Vec2::new(0., 0.),
            ),
            Dot::new(
                &mut dot_id_generator,
                Material::Sand,
                middle_dot_point_2,
                Vec2::new(0., 0.),
            ),
        ];

        let middle_dot_id_1 = test_data[1].id;
        let middle_dot_id_2 = test_data[2].id;

        let canvas = setup_canvas(test_data);
        let path = canvas.cast_ray(ray_start, ray_end);

        assert_eq!(3, path.len());
        assert_eq!(
            middle_dot_id_1,
            path[0].dot.unwrap().id,
            "First middle dot had incorrect id:\n{:?}",
            path[0].dot
        );
        assert_eq!(
            Vec2::new(5, 5),
            path[0].coord,
            "First middle point had incorrect coordinates:\n{:?}",
            path[0].dot
        );
        assert_eq!(
            middle_dot_id_2,
            path[1].dot.unwrap().id,
            "Second middle dot had incorrect id:\n{:?}",
            path[1].dot
        );
        assert_eq!(
            Vec2::new(6, 5),
            path[1].coord,
            "Second middle point had incorrect coordinates:\n{:?}",
            path[1].dot
        );
        assert!(
            path[2].dot.is_none(),
            "End point was not empty:\n{:?}",
            path[2]
        );
    }

    #[test]
    fn test_cast_between_dots_doesnt_capture_them() {
        let ray_start = Vec2::new(4., 5.);
        let ray_end = Vec2::new(6., 5.);
        let mut dot_id_generator = IdGenerator::new();
        let test_data = vec![
            Dot::new(
                &mut dot_id_generator,
                Material::Sand,
                ray_start,
                Vec2::new(0., 0.),
            ),
            Dot::new(
                &mut dot_id_generator,
                Material::Sand,
                Vec2::new(5., 4.),
                Vec2::new(0., 0.),
            ),
            Dot::new(
                &mut dot_id_generator,
                Material::Sand,
                Vec2::new(5., 6.),
                Vec2::new(0., 0.),
            ),
        ];

        let canvas = setup_canvas(test_data);
        let path = canvas.cast_ray(ray_start, ray_end);

        assert_eq!(2, path.len());
        assert!(
            path[0].dot.is_none(),
            "Ray cast found dot between start and end point when it should have found empty space:\n{:?}",
            path[0]
        );
        assert!(
            path[0].dot.is_none(),
            "Ray cast found dot at end point when it should have found empty space:\n{:?}",
            path[0]
        );
    }
}
