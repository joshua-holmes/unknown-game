use std::{collections::HashSet, time::Duration};

use crate::game::FRICTION;

use super::{dot::DotModification, Canvas};

impl Canvas {
    pub fn calculate_physics(&mut self, delta_time: Duration) {
        // find velocity
        for dot in self.palette.values_mut() {
            dot.velocity = dot.find_next_velocity(delta_time);
        }

        // find position & handle collisions
        let mut dots_to_modify = Vec::new();
        let mut visited_collisions = HashSet::new();
        for dot in self.palette.values_mut() {
            // let offset_from_drag = dot.find_pos_offset_from_drag();
            let next_pos = dot.find_next_position(delta_time);
            if next_pos.to_rounded_isize() != dot.position.to_rounded_isize() {
                let collision_check = self.grid.check_for_dot_collision(&dot, next_pos, self.resolution);
                if let Some(collided_dots) = collision_check {
                    if let Some(other) = collided_dots.other {
                        let mut ids = [collided_dots.this.id, other.id];
                        ids.sort();
                        let key = ids
                            .iter()
                            .map(|id| id.to_string())
                            .collect::<Vec<_>>()
                            .join("|");
                        if !visited_collisions.insert(key) {
                            continue;
                        }
                        dots_to_modify.push(other);
                    }
                    dots_to_modify.push(collided_dots.this);
                    continue;
                }
            }
            dots_to_modify.push(DotModification {
                id: dot.id,
                delta_velocity: None,
                delta_position: Some(next_pos - dot.position),
            });
        }

        // apply position & collision changes
        for dot_to_modify in dots_to_modify {
            let dot = self.palette.get_mut(&dot_to_modify.id).unwrap();
            if let Some(del_vel) = dot_to_modify.delta_velocity {
                dot.velocity += del_vel * (1.0 - FRICTION);
            }
            if let Some(del_pos) = dot_to_modify.delta_position {
                dot.position += del_pos;
            }
        }

        // TODO: check for dots that are trying to go to the same place
        // let mut dot_positions: HashMap<Vec2<usize>, HashMap<DotId, &mut Dot>> = HashMap::new();
        // for (id, dot) in self.palette.iter_mut() {
        //     let dots = dot_positions
        //         .entry(dot.position.to_rounded_usize())
        //         .or_insert(HashMap::new());
        //     dots.insert(*id, dot);
        // }
        // for dots in dot_positions.into_values() {
        //     if dots.len() <= 1 {
        //         continue;
        //     }
        //     let vel_sum = dots
        //         .values()
        //         .fold(Vec2::new(0., 0.), |acc, dot| acc + dot.velocity);
        //     for dot in dots.into_values() {
        //         dot.velocity = vel_sum - dot.velocity;
        //     }
        // }
    }
}
