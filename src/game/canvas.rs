use std::collections::VecDeque;

use crate::rendering::glsl_types::Resolution;

use super::{
    dot::{CanvasDot, CollisionReport, Dot, DotModification},
    material::Material,
    Vec2,
};

#[derive(Debug)]
pub enum TriDirection {
    Vertical,
    Horizontal,
    Diagonal,
}

#[derive(Debug)]
pub enum CanvasError {
    CoordOutOfBounds,
}

#[derive(Debug)]
pub struct RayPoint<'a> {
    pub coord: Vec2<isize>,
    pub dot: Option<&'a CanvasDot>,
}

pub struct Canvas {
    pub resolution: Resolution,
    grid: Vec<Vec<Option<CanvasDot>>>,
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

    pub fn get(&self, coord: Vec2<isize>) -> Result<&Option<CanvasDot>, CanvasError> {
        let coord = if coord.x < 0 || coord.y < 0 {
            return Err(CanvasError::CoordOutOfBounds);
        } else {
            Vec2::new(coord.x as usize, coord.y as usize)
        };
        Ok(self
            .grid
            .get(coord.y)
            .ok_or(CanvasError::CoordOutOfBounds)?
            .get(coord.x)
            .ok_or(CanvasError::CoordOutOfBounds)?)
    }

    pub fn get_mut(&mut self, coord: Vec2<isize>) -> Result<&mut Option<CanvasDot>, CanvasError> {
        let coord = if coord.x < 0 || coord.y < 0 {
            return Err(CanvasError::CoordOutOfBounds);
        } else {
            Vec2::new(coord.x as usize, coord.y as usize)
        };
        Ok(self
            .grid
            .get_mut(coord.y)
            .ok_or(CanvasError::CoordOutOfBounds)?
            .get_mut(coord.x)
            .ok_or(CanvasError::CoordOutOfBounds)?)
    }

    pub fn clear(&mut self) {
        for maybe_dot in self.grid.iter_mut().flatten() {
            *maybe_dot = None;
        }
    }

    pub fn cast_ray_to_edge(
        &self,
        ray_start: Vec2<f64>,
        direction_in_degrees: f64,
    ) -> VecDeque<RayPoint> {
        let ray_end = Vec2::new(
            direction_in_degrees.to_radians().cos() * self.resolution.width as f64 + ray_start.x,
            direction_in_degrees.to_radians().sin() * self.resolution.height as f64 + ray_start.y,
        );
        self.cast_ray(ray_start, ray_end).0
    }

    /// Casts a ray and captures every point in the path of the ray in order.
    /// Start of ray is exclusive, end is inclusive. Casts a ray and creates a new ray cast object.
    /// Returns a tuple where the first value is the ray and the second value is the direction of the first collision, with either a dot or wall.
    pub fn cast_ray(
        &self,
        ray_start: Vec2<f64>,
        ray_end: Vec2<f64>,
    ) -> (VecDeque<RayPoint>, Option<TriDirection>) {
        let mut path = VecDeque::new();

        let diff = ray_end - ray_start;
        let direction = diff / diff.abs();

        let find_next_coord = |cur_coord: Vec2<isize>| {
            // candidates for next coordinate
            let horizontal_coord =
                Vec2::new((cur_coord.x as f64 + direction.x) as isize, cur_coord.y);
            let vertical_coord =
                Vec2::new(cur_coord.x, (cur_coord.y as f64 + direction.y) as isize);
            let diagonal = Vec2::new(
                (cur_coord.x as f64 + direction.x) as isize,
                (cur_coord.y as f64 + direction.y) as isize,
            );

            // measure distance of candidates from start and end of ray
            let horizontal_to_start = (ray_start.to_rounded_isize() - horizontal_coord)
                .into_f64()
                .pythagorean_theorem()
                .abs();
            let horizontal_to_end = (ray_end.to_rounded_isize() - horizontal_coord)
                .into_f64()
                .pythagorean_theorem()
                .abs();
            let vertical_to_start = (ray_start.to_rounded_isize() - vertical_coord)
                .into_f64()
                .pythagorean_theorem()
                .abs();
            let vertical_to_end = (ray_end.to_rounded_isize() - vertical_coord)
                .into_f64()
                .pythagorean_theorem()
                .abs();

            // find coord that is closest to both start and end points
            let horizontal_sum = horizontal_to_start + horizontal_to_end;
            let vertical_sum = vertical_to_start + vertical_to_end;
            if horizontal_sum < vertical_sum {
                (horizontal_coord, TriDirection::Horizontal)
            } else if horizontal_sum > vertical_sum {
                (vertical_coord, TriDirection::Vertical)
            } else {
                (diagonal, TriDirection::Diagonal)
            }
        };

        let mut next_coord = Some(find_next_coord(Vec2::new(
            ray_start.x.round() as isize,
            ray_start.y.round() as isize,
        )));
        let mut collision_direction = None;

        while let Some((coord, direction)) = next_coord.take() {
            if coord.x < 0 {
                return (path, Some(TriDirection::Horizontal));
            }
            if coord.y < 0 {
                return (path, Some(TriDirection::Vertical));
            }
            match self.get(coord.into()) {
                Ok(dot_maybe) => {
                    if let None = collision_direction {
                        if let Some(_) = dot_maybe {
                            collision_direction = Some(direction);
                        }
                    }
                    path.push_back(RayPoint {
                        coord: coord.into(),
                        dot: dot_maybe.as_ref(),
                    });
                }
                Err(CanvasError::CoordOutOfBounds) => {
                    if coord.x >= self.resolution.width as isize {
                        return (path, Some(TriDirection::Horizontal));
                    }
                    return (path, Some(TriDirection::Vertical));
                }
            }
            let end_of_ray_reached =
                coord == Vec2::new(ray_end.x.round() as isize, ray_end.y.round() as isize);
            if !end_of_ray_reached {
                next_coord = Some(find_next_coord(coord));
            }
        }

        (path, collision_direction)
    }

    pub fn check_for_dot_collision(
        &self,
        this_dot: &Dot,
        next_pos: Vec2<f64>,
    ) -> Option<CollisionReport> {
        let (ray, ray_collision_direction) = self.cast_ray(this_dot.position, next_pos);
        let this_dot_coord = this_dot.position.to_rounded_isize();
        let mut prev_point = &RayPoint {
            coord: this_dot_coord,
            dot: self.get(this_dot_coord).unwrap().as_ref(),
        };
        let find_delta_velocity = |ray_collision_direction: &TriDirection| -> Vec2<f64> {
            let dv = match ray_collision_direction {
                TriDirection::Vertical => Vec2::new(0., this_dot.velocity.y),
                TriDirection::Horizontal => Vec2::new(this_dot.velocity.x, 0.),
                TriDirection::Diagonal => this_dot.velocity,
            }
            .to_negative();
            dv + dv * this_dot.material.properties().bounce
        };
        for point in ray.iter() {
            if let Some(target_dot) = point.dot.as_ref() {
                let has_gaps = self
                    .cast_ray_to_edge(
                        prev_point.coord.into_f64(),
                        (next_pos - this_dot.position).angle_in_degrees(),
                    )
                    .iter()
                    .any(|p| p.dot.is_none());
                if has_gaps
                    && this_dot.velocity.pythagorean_theorem()
                        > target_dot.velocity.pythagorean_theorem()
                {
                    let diff = this_dot.velocity - target_dot.velocity;
                    return Some(CollisionReport {
                        this: DotModification {
                            id: this_dot.id,
                            delta_velocity: Some(diff.to_negative()),
                            delta_position: Some(prev_point.coord.into_f64() - this_dot.position),
                        },
                        other: Some(DotModification {
                            id: target_dot.id,
                            delta_velocity: Some(diff),
                            delta_position: None,
                        }),
                    });
                }
                let delta_velocity = find_delta_velocity(ray_collision_direction.as_ref().unwrap());
                return Some(CollisionReport {
                    this: DotModification {
                        id: this_dot.id,
                        delta_velocity: Some(delta_velocity),
                        delta_position: Some(prev_point.coord.into_f64() - this_dot.position),
                    },
                    other: None,
                });
            }
            prev_point = point;
        }

        // calculate delta velocity IF wall collision happened
        ray_collision_direction.map(|direction| {
            let delta_velocity = find_delta_velocity(&direction);
            CollisionReport {
                this: DotModification {
                    id: this_dot.id,
                    delta_velocity: Some(delta_velocity),
                    delta_position: Some(prev_point.coord.into_f64() - this_dot.position),
                },
                other: None,
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        game::{
            canvas::{Canvas, CanvasError},
            dot::Dot,
            id_generator::IdGenerator,
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
        let mut dot_id_generator = IdGenerator::new();
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
