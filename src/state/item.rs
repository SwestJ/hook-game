use std::{fmt::Display, vec};

use crate::{
    collision,
    draw::{
        Draw, Drawable,
        graphics::{Shape, item_graphics::ITEM_GRAPHICS},
    },
    state::{
        StateMachine,
        state_machine::{
            State,
            item::{Hooked, ItemState, Moving},
        },
    },
    util::name_of_type,
};

#[derive(Debug)]
pub enum ItemStateMachine {
    Moving(Moving),
    Hooked(Hooked),
}

impl From<ItemState> for ItemStateMachine {
    fn from(value: ItemState) -> Self {
        match value {
            ItemState::Moving(moving) => ItemStateMachine::Moving(moving),
            ItemState::Hooked(hooked) => ItemStateMachine::Hooked(hooked),
        }
    }
}
impl StateMachine for ItemStateMachine {
    fn state_object(&self) -> Vec<super::StateObject> {
        match self {
            ItemStateMachine::Moving(moving) => vec![moving.into()],
            ItemStateMachine::Hooked(hooked) => vec![hooked.into()],
        }
    }

    fn update(self) -> Self {
        match self {
            ItemStateMachine::Moving(moving) => moving.update().into(),
            ItemStateMachine::Hooked(hooked) => hooked.update().into(),
        }
    }
}
impl Draw for ItemStateMachine {
    fn drawable(&self) -> Vec<Drawable> {
        match self {
            ItemStateMachine::Moving(moving) => vec![Drawable {
                state: moving.into(),
                shape: Shape::ItemObject(ITEM_GRAPHICS),
            }],

            ItemStateMachine::Hooked(hooked) => vec![Drawable {
                state: hooked.into(),
                shape: Shape::ItemObject(ITEM_GRAPHICS),
            }],
        }
    }
}
impl collision::Collision for ItemStateMachine {
    fn collision_box(&self) -> Vec<collision::CollisionBox> {
        match self {
            ItemStateMachine::Moving(state) => vec![Self::bounds(
                ITEM_GRAPHICS
                    .model
                    .rotate(state.direction())
                    .translate(state.position()),
            )],
            ItemStateMachine::Hooked(state) => vec![Self::bounds(
                ITEM_GRAPHICS
                    .model
                    .rotate(state.direction())
                    .translate(state.position()),
            )],
        }
    }

    fn collision_detected(&self /*other object */) {
        todo!()
    }
}
impl Display for ItemStateMachine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", name_of_type(self));
        match self {
            ItemStateMachine::Moving(moving) => write!(f, "{}", moving),
            ItemStateMachine::Hooked(hooked) => write!(f, "{}", hooked),
        }
    }
}
