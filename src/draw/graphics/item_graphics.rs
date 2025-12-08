use super::*;

const ITEM_NUMBER_OF_VERTICES: usize = ITEM_GRAPHICS_ARRAY.len() * 3;
const ITEM_GRAPHICS_ARRAY: [(i32, i32); 4] =
    [(0, 0), (0, 1), (1, 0), (1, 1)];

#[derive(Clone, Debug)]
pub struct ItemGraphics {
    pub model: Vertices<ITEM_NUMBER_OF_VERTICES>,
    pub color: Color,
}
pub const ITEM_GRAPHICS: ItemGraphics = ItemGraphics {
    model: vertex_graphics_from_triangle_points(ITEM_GRAPHICS_ARRAY).rotate_const(DOWN.value()).scale_const(10.0),
    color: PINK,
};