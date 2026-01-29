use std::fmt::Display;

use crate::state::state_machine::{State, item::ItemState};

use super::*;

pub mod hook;
pub mod item;
pub mod player;
pub mod state_machine;

#[derive(Default)]
pub enum StateMachineType {
    Player(PlayerStateMachine),
    Item(ItemStateMachine),
    #[default]
    Default,
}
impl StateMachineType {
    pub fn update(self) -> Self {
        match self {
            StateMachineType::Player(player_state_enum) => StateMachineType::Player(player_state_enum.update()),
            StateMachineType::Item(item_state_enum) => StateMachineType::Item(item_state_enum.update()),
            StateMachineType::Default => panic!("Default variant should not be used"),
        }
    }
}
impl StateMachine for StateMachineType {
    fn state_object(&self) -> Vec<StateObject> {
        match self {
            StateMachineType::Player(player_state) => player_state.state_object(),
            StateMachineType::Item(item_state) => todo!(),
            StateMachineType::Default => panic!("Default variant should not be used"),
        }
    }

    fn update(self) -> Self {
        self.update()
    }
}
impl Draw for StateMachineType {
    fn drawable(&self) -> Vec<Drawable> {
        match self {
            StateMachineType::Player(player_state) => player_state.drawable(),
            StateMachineType::Item(item_state) => item_state.drawable(),
            StateMachineType::Default => panic!("Default variant should not be used"),
        }
    }
}
impl collision::Collision for StateMachineType {
    fn collision_box(&self) -> Vec<collision::CollisionBox> {
        match self {
            StateMachineType::Player(state) => state.collision_box(),
            StateMachineType::Item(state) => state.collision_box(),
            StateMachineType::Default => panic!("Default variant should not be used"),
        }
    }

    fn collision_detected(&self, /*other object */) {
        todo!()
    }
}
impl Display for StateMachineType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StateMachineType::Player(state_enum) => write!(f, "{}", state_enum),
            StateMachineType::Item(item_state) => write!(f, "{}", item_state),
            StateMachineType::Default => write!(f, "Default"),
        }
    }
}

pub trait StateMachine: Display {
    fn state_object(&self) -> Vec<StateObject>;
    fn update(self) -> Self;
}

pub struct StateObject {
    pub position: Position,
    pub direction: Direction,
}
impl<T: State> From<&T> for StateObject {
    fn from(state: &T) -> Self {
        StateObject {
            position: state.position(),
            direction: state.direction(),
        }
    }
}
