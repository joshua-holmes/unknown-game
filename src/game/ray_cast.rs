use std::collections::VecDeque;

use super::{dot::Dot, geometry::Vec2, canvas::{Canvas, CanvasError}};

enum RayCastError {
    RayOutOfBounds,
}

#[derive(Debug)]
pub struct RayPoint {
    pub coord: Vec2<usize>,
    pub dot: Option<Dot>,
}

/// Casts a ray and captures every point in the path of the ray in order.
pub struct RayCast<'a> {
    pub path: VecDeque<RayPoint>,
    ray_start: Vec2<f64>,
    ray_end: Vec2<f64>,
    canvas: &'a Canvas,
}
impl<'a> RayCast<'a> {
    /// Start of ray is exclusive, end is inclusive. Casts a ray and creates a new ray cast object.
    pub fn new(canvas: &'a Canvas, ray_start: Vec2<f64>, ray_end: Vec2<f64>) -> Self {
        let mut ray_cast = Self {
            path: VecDeque::new(),
            ray_start,
            ray_end,
            canvas,
        };

        ray_cast.cast();

        ray_cast
    }

    fn cast(&mut self) {
        // Hardcoded ray width, if this needs to change, the algorithm will need to be reworked to handle different ray widths.
        let ray_width = 1.;
        self.path.clear();

        if self.ray_start.x == self.ray_end.x || self.ray_start.y == self.ray_end.y {
            self.cast_flat(self.ray_start, self.ray_end);
        }
    }

    fn cast_flat(&mut self, ray_start: Vec2<f64>, ray_end: Vec2<f64>) {
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
            match self.canvas.get(coord) {
                Ok(dot_maybe) => self.path.push_back(RayPoint {
                    coord,
                    dot: dot_maybe,
                }),
                Err(CanvasError::CoordOutOfBounds) => {
                    return println!("Ray cast blast got cast where it won't last! Some features may not function properly.");
                }
            }
            let end_of_ray_reached =
                coord == Vec2::new(ray_end.x.round() as usize, ray_end.y.round() as usize);
            if !end_of_ray_reached {
                next_coord = Some(find_next_coord(coord));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use crate::{game::{
        dot::Dot, geometry::Vec2, id_generator::IdGenerator,
        material::Material, canvas::Canvas,
    }, rendering::glsl_types::Resolution};

    use super::RayCast;

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

    fn setup_ray_cast<'a>(
        canvas: &'a Canvas,
        ray_start: Vec2<f64>,
        ray_end: Vec2<f64>,
    ) -> RayCast<'a> {
        RayCast {
            canvas,
            path: VecDeque::new(),
            ray_start,
            ray_end,
        }
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
        let mut ray_cast = setup_ray_cast(&canvas, ray_start, ray_end);

        ray_cast.cast();

        assert_eq!(
            1,
            ray_cast.path.len(),
            "Ray cast did not stop at end point:\n{:?}",
            ray_cast.path
        );
        assert_eq!(
            expected_id,
            ray_cast.path[0].dot.unwrap().id,
            "Could not find correct ray end dot"
        );
        assert_eq!(
            Vec2::new(6, 5),
            ray_cast.path[0].coord,
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
        let mut ray_cast = setup_ray_cast(&canvas, ray_start, ray_end);

        ray_cast.cast();

        assert_eq!(3, ray_cast.path.len());
        assert!(
            ray_cast.path[0].dot.is_none(),
            "First point was not empty:\n{:?}",
            ray_cast.path[0]
        );
        assert!(
            ray_cast.path[1].dot.is_none(),
            "Second point was not empty:\n{:?}",
            ray_cast.path[1]
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
        let mut ray_cast = setup_ray_cast(&canvas, ray_start, ray_end);

        ray_cast.cast();

        assert_eq!(3, ray_cast.path.len());
        assert_eq!(
            middle_dot_id_1,
            ray_cast.path[0].dot.unwrap().id,
            "First middle dot had incorrect id:\n{:?}",
            ray_cast.path[0].dot
        );
        assert_eq!(
            Vec2::new(5, 5),
            ray_cast.path[0].coord,
            "First middle point had incorrect coordinates:\n{:?}",
            ray_cast.path[0].dot
        );
        assert_eq!(
            middle_dot_id_2,
            ray_cast.path[1].dot.unwrap().id,
            "Second middle dot had incorrect id:\n{:?}",
            ray_cast.path[1].dot
        );
        assert_eq!(
            Vec2::new(6, 5),
            ray_cast.path[1].coord,
            "Second middle point had incorrect coordinates:\n{:?}",
            ray_cast.path[1].dot
        );
        assert!(
            ray_cast.path[2].dot.is_none(),
            "End point was not empty:\n{:?}",
            ray_cast.path[2]
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
        let mut ray_cast = setup_ray_cast(&canvas, ray_start, ray_end);

        ray_cast.cast();

        assert_eq!(2, ray_cast.path.len());
        assert!(
            ray_cast.path[0].dot.is_none(),
            "Ray cast found dot between start and end point when it should have found empty space:\n{:?}",
            ray_cast.path[0]
        );
        assert!(
            ray_cast.path[0].dot.is_none(),
            "Ray cast found dot at end point when it should have found empty space:\n{:?}",
            ray_cast.path[0]
        );
    }
}
