use super::*;

pub const HOOK_EXTENDING: Triangle = Triangle { height: 40.0, base: 20.0, color: PURPLE };
pub const HOOK_CONTRACTING: Triangle = Triangle { height: 40.0, base: 20.0, color: PURPLE };
pub const HOOK_LINK: Line = Line { length: 5.0, thickness: 5.0, color: DARKGRAY };
pub const HOOK_LINK_VERTEX: Circle = Circle { radius: Radius(5.0), color: GRAY };

const HOOK_NUMBER_OF_VERTICES: usize = HOOK_GRAPHICS_ARRAY.len() * HOOK_GRAPHICS_ARRAY[0].0.len();
const HOOK_GRAPHICS_ARRAY: [Vertices<3>; 14] = [
    triangle(0, 0),
    triangle(0, 1),
    triangle(0, 2),
    triangle(0, 3),
    triangle(1, 3),
    triangle(1, 4),
    triangle(2, 4),
    triangle(2, 5),
    triangle(2, 6),
    triangle(2, 7),
    triangle(2, 8),
    triangle(1, 8),
    triangle(1, 7),
    triangle(0, 7),
];

#[derive(Clone, Debug)]
pub struct HookGraphics {
    pub model: Vertices<HOOK_NUMBER_OF_VERTICES>,
    pub color: Color,
}

pub const HOOK_GRAPHICS: HookGraphics = HookGraphics {
    model: create_vertex_graphics(HOOK_GRAPHICS_ARRAY).rotate_const(THETA1_UNIT).scale_const(5.0),
    color: GRAY,
};