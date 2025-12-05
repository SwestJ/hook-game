#![doc = r#"
Rough detection:
    Collect bounds from objects. Bounds could be a sorrounding box or something more granular
    Cross-check bounds between each object.

Granular detection:
    Collect all vertices/triangles for objects. Or maybe just the "outer" ones / contour vertices
    Cross-check overlap between each triangle of each object.
    Could perform rough detection first (if it is significantly faster) and only perform granular detection on detected collisions

Rough detection: Collisisions that affect the object as a whole. E.g. collisions with walls, which should shift the position of the whole object back

On detected collision:
    Call "collision detected" trait method.
    Each object implements its own resolution, depending on collision details and the other object. Alternatively, the collision detector could handle resolution logic, and just update objects with e.g. new positions."#]

use super::*;

pub fn collisions(state: &StateMachineEnum) {
    match state {
        StateMachineEnum::Player(player_state_enum) => todo!(),
        StateMachineEnum::Item(_) => todo!(),
        StateMachineEnum::Default => todo!(),
    }
}

pub trait Collision {
    fn bounds(); // Returns a Bounds struct
    fn collision_detected(/*other object */);
}

struct CollisionBox {
    vec1: Vec2,
    vec2: Vec2,
}