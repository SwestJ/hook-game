use super::*;

const HOOK_NUMBER_OF_VERTICES: usize = HOOK_GRAPHICS_ARRAY.len() * 3;
const HOOK_GRAPHICS_ARRAY: [(i32, i32); 14] = [
    (0, 0),
    (0, 1),
    (0, 2),
    (0, 3),
    (1, 3),
    (1, 4),
    (2, 4),
    (2, 5),
    (2, 6),
    (2, 7),
    (2, 8),
    (1, 8),
    (1, 7),
    (0, 7),
];

#[derive(Clone, Debug)]
pub struct HookGraphics {
    pub model: Vertices<HOOK_NUMBER_OF_VERTICES>,
    pub color: Color,
}

pub const HOOK_GRAPHICS: HookGraphics = HookGraphics {
    model: vertex_graphics_from_triangle_points(LINK_GRAPHICS_ARRAY)
        .rotate_const(THETA1_UNIT)
        .scale_const(5.0),
    color: GRAY,
};

pub const HOOK_LINK: Line = Line {
    length: 5.0,
    thickness: 5.0,
    color: DARKGRAY,
};
pub const HOOK_LINK_VERTEX: Circle = Circle {
    radius: Radius(5.0),
    color: GRAY,
};

const LINK_NUMBER_OF_VERTICES: usize = LINK_GRAPHICS_ARRAY.len() * 3;
const LINK_GRAPHICS_ARRAY: [(i32, i32); 14] = [
    (0, 0),
    (0, 1),
    (0, 2),
    (0, 3),
    (1, 3),
    (1, 4),
    (2, 4),
    (2, 5),
    (2, 6),
    (2, 7),
    (2, 8),
    (1, 8),
    (1, 7),
    (0, 7),
];

#[derive(Clone, Debug)]
pub struct LinkGraphics {
    pub model: Vertices<LINK_NUMBER_OF_VERTICES>,
    pub color: Color,
}

pub const LINK_GRAPHICS: LinkGraphics = LinkGraphics {
    model: vertex_graphics_from_triangle_points(LINK_GRAPHICS_ARRAY)
        .rotate_const(THETA1_UNIT)
        .scale_const(5.0),
    color: GRAY,
};
