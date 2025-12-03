use super::*;

const ITEM_NUMBER_OF_VERTICES: usize = ITEM_GRAPHICS_ARRAY.len() * ITEM_GRAPHICS_ARRAY[0].0.len();
const ITEM_GRAPHICS_ARRAY: [Vertices<3>; 4] =
    [triangle(0, 0), triangle(0, 1), triangle(1, 0), triangle(1, 1)];

#[derive(Clone, Debug)]
pub struct ItemGraphics {
    pub model: Vertices<ITEM_NUMBER_OF_VERTICES>,
    pub color: Color,
}
pub const ITEM_GRAPHICS: ItemGraphics = ItemGraphics {
    model: create_vertex_graphics(ITEM_GRAPHICS_ARRAY).rotate_const(DOWN.value()).scale_const(10.0),
    color: PINK,
};