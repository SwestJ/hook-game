use std::fmt::Display;

use macroquad::prelude::*;

use crate::{model::*, util::name_of_type};

#[derive(Debug)]
pub enum ItemState {
    Moving(Moving),
    Hooked(Hooked)
}

impl ItemState {
    pub fn update(self) -> Self {
        match self {
            ItemState::Moving(moving) => moving.update(),
            ItemState::Hooked(hooked) => todo!(),
        }
    }
}
impl Display for ItemState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", name_of_type(self));
        match self {
            ItemState::Moving(moving) => write!(f, "{}", moving),
            ItemState::Hooked(hooked) => write!(f, "{}", hooked),
        }

    }
}

pub fn build(position: Position, direction: Direction, speed: Magnitude) -> Moving {
    Moving::action(position, direction, speed)
}

pub trait State {}

#[derive(Debug)]
pub struct Moving {
    position: Position,
    direction: Direction,
    speed: Magnitude,
}
impl Display for Moving {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {} {}", name_of_type(self), self.position, self.direction, self.speed)
    }
}
impl State for Moving {}
impl Moving {
    pub fn position(&self) -> Position {
        self.position
    }
    pub fn direction(&self) -> Direction {
        self.direction
    }
    pub fn speed(&self) -> Magnitude {
        self.speed
    }
    pub fn update(self) -> ItemState {
        let Self { position, direction, speed } = self;
        ItemState::Moving(Moving::action(position, direction, speed))
    }
    fn action(position: Position, direction: Direction, speed: Magnitude) -> Self {
        let new_direction = direction.rotate(Angle(Degrees(0.5)));
        let new_position = Physics::calculate_new_position_from_speed(position, speed, new_direction);
        Moving {position: new_position, direction: new_direction, speed}
    }
}

#[derive(Debug)]
pub struct Hooked {
    position: Position,
}
impl Display for Hooked {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", name_of_type(self), self.position)
    }
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
