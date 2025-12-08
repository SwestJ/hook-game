//!Rough detection:
//! - Collect bounds from objects. Bounds could be a sorrounding box or something more granular
//! - Cross-check bounds between each object.
//!
//!Granular detection:
//! - Collect all vertices/triangles for objects. Or maybe just the "outer" ones / contour vertices
//! - Cross-check overlap between each triangle of each object.
//! - Could perform rough detection first (if it is significantly faster) and only perform granular detection on detected collisions
//!
//!Rough detection: Collisisions that affect the object as a whole. E.g. collisions with walls, which should shift the position of the whole object back
//!
//! On detected collision:
//!  - Call "collision detected" trait method.
//!  - Each object implements its own resolution, depending on collision details and the other object. Alternatively, the collision detector could handle resolution logic, and just update objects with e.g. new positions.

use crate::{
    draw::graphics::{Vertices, hook_graphics::HOOK_GRAPHICS},
    state::state_machine::{
        State,
        hook::{Contracting, Hook},
    },
};

use super::*;
use itertools::Itertools;

pub fn collisions(states: &[StateMachineEnum]) {
    let collision_boxes: Vec<CollisionBox> = states.iter().flat_map(StateMachineEnum::collision_box).collect();
    let collisions = find_collisions(&collision_boxes);

    if DEBUG_DRAW_COLLISION_BOXES {
        draw::debug_draw_collision_boxes(&collision_boxes, RED);
        draw::debug_draw_collided_boxes(&collisions, GREEN);
    }
}

pub trait Collision: Draw {
    fn collision_box(&self) -> Vec<CollisionBox>;
    fn collision_detected(&self /*other object */);
    fn bounds<const N: usize>(vertices: Vertices<N>) -> CollisionBox {
        let (lower, upper) = Vec::<Vec2>::from(vertices.value())
            .iter()
            .fold((Vec2::MAX, Vec2::MIN), |acc, v| (v.min(acc.0), v.max(acc.1)));
        CollisionBox { lower, upper }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct CollisionBox {
    pub lower: Vec2,
    pub upper: Vec2,
}
impl CollisionBox {
    fn collision_with(&self, other: &CollisionBox) -> bool {
        is_overlapping((self.lower.x, self.upper.x), (other.lower.x, other.upper.x))
            && is_overlapping((self.lower.y, self.upper.y), (other.lower.y, other.upper.y))
    }
}

fn find_collisions(boxes: &[CollisionBox]) -> Vec<(CollisionBox, CollisionBox)> {
    boxes
        .iter()
        .combinations(2)
        .filter(|p| p[0].collision_with(p[1]))
        .map(|p| (p[0].to_owned(), p[1].to_owned()))
        .collect()
}

fn is_overlapping(p1: (f32, f32), p2: (f32, f32)) -> bool {
    p1.1 > p2.0 && p2.1 > p1.0
}
