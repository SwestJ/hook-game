use macroquad::prelude::*;

use crate::model::*;

pub fn build() {}

pub trait State {}

#[derive(Debug)]
pub struct Idling {}
impl State for Idling {}

impl Idling {
    fn idle() -> Self {
        Idling {}
    }
}

pub struct Moving {
    position: Position,
    direction: Direction,
    speed: Magnitude,
}
impl State for Moving {}
impl Moving {
    fn action(position: Position, direction: Direction, speed: Magnitude) -> Self {
        let new_direction = direction.rotate(Angle(Degrees(10.0)));
        let new_position = Physics::calculate_new_position_from_speed(position, speed, new_direction);
        Moving {position: new_position, direction: new_direction, speed}
    }
}

pub struct Hooked {
    position: Position,
}

impl Hooked {
    fn hook(position: Position) -> Self {
        Hooked { position }
    }
}

//Something like:
struct Item {
    type_of_item: String,
    points_worth: String,
    coins_worth: String,
    buff: String,
    debuff: String,
}
