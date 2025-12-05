use std::fmt::Display;

use crate::state::state_machine::{State, item::ItemState};

use super::*;

pub mod hook;
pub mod item;
pub mod player;
pub mod state_machine;

#[derive(Default)]
pub enum StateMachineEnum {
    Player(PlayerStateMachine),
    Item(ItemStateMachine),
    #[default]
    Default,
}
impl StateMachineEnum {
    pub fn update(self) -> Self {
        match self {
            StateMachineEnum::Player(player_state_enum) => StateMachineEnum::Player(player_state_enum.update()),
            StateMachineEnum::Item(item_state_enum) => StateMachineEnum::Item(item_state_enum.update()),
            StateMachineEnum::Default => panic!("Default invariant should not be used"),
        }
    }
}
impl StateMachine for StateMachineEnum {
    fn state_object(&self) -> Vec<StateObject> {
        match self {
            StateMachineEnum::Player(player_state) => player_state.state_object(),
            StateMachineEnum::Item(item_state) => todo!(),
            StateMachineEnum::Default => todo!(),
        }
    }

    fn update(self) -> Self {
        self.update()
    }
}
impl Draw for StateMachineEnum {
    fn drawable(&self) -> Vec<Drawable> {
        match self {
            StateMachineEnum::Player(player_state) => player_state.drawable(),
            StateMachineEnum::Item(item_state) => item_state.drawable(),
            StateMachineEnum::Default => todo!(),
        }
    }
}
impl Display for StateMachineEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StateMachineEnum::Player(state_enum) => write!(f, "{}", state_enum),
            StateMachineEnum::Item(item_state) => write!(f, "{}", item_state),
            StateMachineEnum::Default => write!(f, "Default"),
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
