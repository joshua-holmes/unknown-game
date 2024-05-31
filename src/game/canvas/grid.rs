use std::collections::VecDeque;

use crate::{game::{material::Material, math::Vec2}, rendering::glsl_types::Resolution};

use super::{dot::{CanvasDot, CollisionReport, DotModification}, CanvasError, Dot, RayPoint, TriDirection};

pub struct Grid(Vec<Vec<Option<CanvasDot>>>);
impl Grid {
    pub fn new_from(inner_grid: Vec<Vec<Option<CanvasDot>>>) -> Self {
        Self(inner_grid)
    }

    pub fn iter_materials_as_bytes<'a>(&'a self) -> impl Iterator<Item = u8> + 'a {
        self.0
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
            .0
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
            .0
            .get_mut(coord.y)
            .ok_or(CanvasError::CoordOutOfBounds)?
            .get_mut(coord.x)
            .ok_or(CanvasError::CoordOutOfBounds)?)
    }

    pub fn clear(&mut self) {
        for maybe_dot in self.0.iter_mut().flatten() {
            *maybe_dot = None;
        }
    }

    fn cast_ray_to_edge(
        &self,
        ray_start: Vec2<f64>,
        direction_in_degrees: f64,
        resolution: Resolution,
    ) -> VecDeque<RayPoint> {
        let ray_end = Vec2::new(
            direction_in_degrees.to_radians().cos() * resolution.width as f64 + ray_start.x,
            direction_in_degrees.to_radians().sin() * resolution.height as f64 + ray_start.y,
        );
        self.cast_ray(ray_start, ray_end, resolution).0
    }

    /// Casts a ray and captures every point in the path of the ray in order.
    /// Start of ray is exclusive, end is inclusive. Casts a ray and creates a new ray cast object.
    /// Returns a tuple where the first value is the ray and the second value is the direction of the first collision, with either a dot or wall.
    fn cast_ray(
        &self,
        ray_start: Vec2<f64>,
        ray_end: Vec2<f64>,
        resolution: Resolution,
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
                    if coord.x >= resolution.width as isize {
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
        resolution: Resolution,
    ) -> Option<CollisionReport> {
        let (ray, ray_collision_direction) = self.cast_ray(this_dot.position, next_pos, resolution);
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
                        resolution
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
