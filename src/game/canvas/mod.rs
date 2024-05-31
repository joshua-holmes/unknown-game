use std::collections::{hash_map::OccupiedError, HashMap, VecDeque};

use crate::rendering::glsl_types::Resolution;
pub mod dot;
mod grid;
mod dot_id_gen;

use self::dot_id_gen::DotIdGen;
use super::{material::Material, Vec2};
pub use dot::Dot;
pub use grid::Grid;
use dot::CanvasDot;
pub use dot_id_gen::DotId;
mod physics;

#[derive(Debug)]
enum TriDirection {
    Vertical,
    Horizontal,
    Diagonal,
}

#[derive(Debug)]
pub enum CanvasError {
    CoordOutOfBounds,
}

#[derive(Debug)]
struct RayPoint<'a> {
    pub coord: Vec2<isize>,
    pub dot: Option<&'a CanvasDot>,
}

pub struct Canvas {
    pub resolution: Resolution,
    pub grid: Grid,
    palette: HashMap<DotId, Dot>,
    dot_id_gen: DotIdGen,
}
impl Canvas {
    pub fn new(resolution: Resolution) -> Self {
        let Resolution { height, width } = resolution;
        Self {
            resolution,
            grid: Grid::new_from((0..height)
                .map(|_| (0..width).map(|_| None).collect())
                .collect()),
            palette: HashMap::new(),
            dot_id_gen: DotIdGen::new(),
        }
    }

    pub fn spawn_dot(
        &mut self,
        material: Material,
        position: Vec2<f64>,
        velocity: Vec2<f64>,
    ) -> Result<(), OccupiedError<DotId, Dot>> {
        let id = self.dot_id_gen.new_id().expect("Ran out of ids");
        let dot = Dot {
            id,
            material,
            velocity,
            position,
        };
        self.palette.try_insert(id, dot)?;

        Ok(())
    }

    pub fn spawn_circle_of_dots(&mut self, radius: f64, coord: Vec2<f64>) {
        let top_left = (coord - radius)
            .clamp_to_resolution(self.resolution)
            .to_rounded_isize();
        let bottom_right = (coord + radius)
            .clamp_to_resolution(self.resolution)
            .to_rounded_isize();
        for x in top_left.x..=bottom_right.x {
            for y in top_left.y..=bottom_right.y {
                let point = Vec2::new(x, y);
                let distance = (point.into_f64() - coord).pythagorean_theorem();

                // dot can only spawn within radius of CURSOR_SIZE
                if distance > radius {
                    continue;
                }

                // dot can't spawn if another dot is already there
                if self.grid.get(point).unwrap().is_some() {
                    continue;
                }

                // add new dot to palette, error if dot is already there
                self.spawn_dot(Material::Sand, point.into_f64(), Vec2::new(0., 0.))
                    .expect(format!("Dot already exists in pos: {:?}", point.into_f64()).as_str());
            }
        }
    }

    pub fn write_dots_to_grid(&mut self) {
        self.grid.clear();
        for (dot, pos) in self
            .palette
            .values()
            .map(|d| (CanvasDot::from(d), d.position))
        {
            match self.grid.get_mut(pos.clamp_to_resolution(self.resolution).to_rounded_isize()) {
                Ok(maybe_canvas_dot) => {
                    *maybe_canvas_dot = Some(dot);
                }
                Err(CanvasError::CoordOutOfBounds) => println!(
                    "WARNING: Tried to write a dot to canvas that was out of bounds:\n{:?}",
                    dot
                ),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        game::{
            canvas::{Canvas, CanvasError},
            dot::Dot,
            id_generator::DotIdGen,
            material::Material,
            vec2::Vec2,
        },
        rendering::glsl_types::Resolution,
    };

    fn setup_canvas(test_data: Vec<Dot>) -> Result<Canvas, CanvasError> {
        const WIDTH: usize = 10;
        const HEIGHT: usize = 10;

        let mut canvas = Canvas::new(Resolution {
            height: HEIGHT as i32,
            width: WIDTH as i32,
        });

        for dot in test_data.iter() {
            let coord = Vec2::new(
                dot.position.x.round() as usize,
                dot.position.y.round() as usize,
            );
            if coord.x > WIDTH || coord.y > HEIGHT {
                panic!("Test data was not setup correctly, dot placed out of bounds.");
            }

            let canvas_dot = canvas
                .grid
                .get_mut(coord.y)
                .ok_or(CanvasError::CoordOutOfBounds)?
                .get_mut(coord.x)
                .ok_or(CanvasError::CoordOutOfBounds)?;

            *canvas_dot = Some(dot.into());
        }

        Ok(canvas)
    }

    #[test]
    fn test_cast_finds_end_point() {
        let ray_start = Vec2::new(5., 5.);
        let ray_end = Vec2::new(6., 5.);
        let mut dot_id_generator = DotIdGen::new();
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

        let canvas = setup_canvas(test_data).unwrap();
        let path = canvas.cast_ray(ray_start, ray_end).0;

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
        let mut dot_id_generator = DotIdGen::new();
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

        let canvas = setup_canvas(test_data).unwrap();
        let path = canvas.cast_ray(ray_start, ray_end).0;

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
        let mut dot_id_generator = DotIdGen::new();
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

        let canvas = setup_canvas(test_data).unwrap();
        let path = canvas.cast_ray(ray_start, ray_end).0;

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
        let mut dot_id_generator = DotIdGen::new();
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

        let canvas = setup_canvas(test_data).unwrap();
        let path = canvas.cast_ray(ray_start, ray_end).0;

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

    #[test]
    fn test_cast_to_edge() {
        // n = nothing (null)
        // d = dot
        // * = starting point
        // # = wall
        // *------> = 0 degrees
        //
        // # # # # #
        // # n d
        // #   *
        let ray_start = Vec2::new(1., 1.);
        let direction_in_degrees = 247.5;
        let mut dot_id_generator = DotIdGen::new();
        let test_data = vec![Dot::new(
            &mut dot_id_generator,
            Material::Sand,
            Vec2::new(1., 0.),
            Vec2::new(0., 0.),
        )];

        let canvas = setup_canvas(test_data).unwrap();
        let path = canvas.cast_ray_to_edge(ray_start, direction_in_degrees);

        assert_eq!(2, path.len(), "PATH: {:?}", path);
        assert_eq!(
            Vec2::new(1, 0),
            path[0].coord,
            "First dot point had incorrect coordinates:\n{:?}",
            path[0].dot
        );
        assert!(
            path[1].dot.is_none(),
            "Ray cast should have captured the dot as the first point in the ray, not the second:\n{:?}",
            path[1]
        );
    }
}
