use crate::state::item::ItemStateMachine;

use super::*;

pub fn build(position: Position, direction: Direction, speed: Magnitude) -> Moving {
    Moving::action(position, direction, speed)
}

#[derive(Debug)]
pub enum ItemState {
    Moving(Moving),
    Hooked(Hooked),
}
impl State for ItemState {
    type Output = Self;
    fn position(&self) -> Position {
        match self {
            ItemState::Moving(moving) => moving.position(),
            ItemState::Hooked(hooked) => hooked.position(),
        }
    }
    fn direction(&self) -> Direction {
        match self {
            ItemState::Moving(moving) => moving.direction(),
            ItemState::Hooked(hooked) => hooked.direction(),
        }
    }
    fn update(self) -> Self {
        match self {
            ItemState::Moving(moving) => moving.update(),
            ItemState::Hooked(hooked) => hooked.update(),
        }
    }
}

#[derive(Debug)]
pub struct Moving {
    position: Position,
    direction: Direction,
    speed: Magnitude,
}
impl Display for Moving {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            name_of_type(self),
            self.position,
            self.direction,
            self.speed
        )
    }
}
impl Moving {
    pub fn speed(&self) -> Magnitude {
        self.speed
    }
    fn action(position: Position, direction: Direction, speed: Magnitude) -> Self {
        let new_direction = direction.rotate(Angle(Degrees(0.5)));
        let new_position = Physics::calculate_new_position_from_speed(position, speed, new_direction);
        Moving {
            position: new_position,
            direction: new_direction,
            speed,
        }
    }
}
impl State for Moving {
    type Output = ItemState;

    fn position(&self) -> Position {
        self.position
    }

    fn direction(&self) -> Direction {
        self.direction
    }

    fn update(self) -> Self::Output {
        let Self {
            position,
            direction,
            speed,
        } = self;
        ItemState::Moving(Moving::action(position, direction, speed))
    }
}

#[derive(Debug)]
pub struct Hooked {
    position: Position,
    direction: Direction,
}
impl Hooked {
    fn hook(position: Position, direction: Direction) -> Self {
        Hooked { position, direction }
    }
}
impl State for Hooked {
    type Output = ItemState;
    fn position(&self) -> Position {
        self.position
    }

    fn direction(&self) -> Direction {
        self.direction
    }

    fn update(self) -> Self::Output {
        todo!()
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

impl Display for Hooked {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", name_of_type(self), self.position)
    }
}
