use macroquad::input::*;
use crate::model::*;

//* Utility functions */

pub fn get_player_move() -> Direction {
    let mut direction = Direction::new(0.0, 0.0);
    if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
        direction = direction + UP;
    }
    if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
        direction = direction + LEFT;
    }
    if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
        direction = direction + DOWN;
    }
    if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
        direction = direction + RIGHT;
    }
    direction.normalize_or_zero()
}
pub fn is_shooting() -> bool {
    is_key_pressed(KeyCode::Space)
}