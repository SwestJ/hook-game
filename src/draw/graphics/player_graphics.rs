use std::time::{Instant, SystemTime};

use super::*;

const PLAYER_SCALE: f32 = 10.0;
const PLAYER_COLOR: Color = BLUE;
const PLAYER_GRAPHICS_ARRAY: [(i32, i32); 4] = [(0, 0), (0, 1), (1, 2), (0, 3)];
const PLAYER_GRAPHICS_ARRAY_2: [(i32, i32); 4] = [(0, 0), (0, 1), (1, 2), (1, 3)];

const PLAYER_NUMBER_OF_VERTICES: usize = PLAYER_GRAPHICS_ARRAY.len() * 3;
#[derive(Copy, Clone, Debug)]
pub struct PlayerGraphics {
    pub model: Vertices<PLAYER_NUMBER_OF_VERTICES>,
    pub color: Color,
}

pub const PLAYER_GRAPHICS: PlayerGraphics = PlayerGraphics {
    model: vertex_graphics_from_triangle_points(PLAYER_GRAPHICS_ARRAY)
        .rotate_const(THETA1_UNIT)
        .scale_const(PLAYER_SCALE),
    color: PLAYER_COLOR,
};
pub const PLAYER_GRAPHICS_2: PlayerGraphics = PlayerGraphics {
    model: vertex_graphics_from_triangle_points(PLAYER_GRAPHICS_ARRAY_2)
        .rotate_const(THETA1_UNIT)
        .scale_const(PLAYER_SCALE),
    color: PLAYER_COLOR,
};

pub struct Animation {
    pub index: usize,
    pub frames: [PlayerGraphics; 2],
}
impl Animation {
    pub fn current_frame(&self) -> PlayerGraphics {
        let k = 400;
        let r = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis()
            % k;
        if r < k / 2 { self.frames[0] } else { self.frames[1] }
    }
}

pub const PLAYER_ANIMATION: Animation = Animation {
    index: 2,
    frames: [PLAYER_GRAPHICS, PLAYER_GRAPHICS_2],
};
