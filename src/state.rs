use std::fmt::Display;

use crate::state::{item::{ItemState, Moving}, player::PlayerState};

pub mod hook;
pub mod item;
pub mod player;

#[derive(Default)]
pub enum StateEnum {
    Player(PlayerState),
    Item(ItemState),
    #[default]
    Default,
}
impl StateEnum {
    pub fn invoke(self) -> Self {
        match self {
            StateEnum::Player(player_state_enum) => StateEnum::Player(player_state_enum.invoke()),
            StateEnum::Item(item_state_enum) => StateEnum::Item(item_state_enum.update()),
            StateEnum::Default => panic!("Default invariant should not be used"),
        }
    }

}
impl Display for StateEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StateEnum::Player(state_enum) => write!(f, "{}", state_enum),
            StateEnum::Item(item_state_enum) => write!(f, "{}", item_state_enum),
            StateEnum::Default => write!(f, "Default"),
        }
    }
}

