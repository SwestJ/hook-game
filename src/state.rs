use std::fmt::Display;

use crate::state::player::PlayerStateEnum;

pub mod hook;
pub mod player;

#[derive(Default)]
pub enum StateEnum {
    Player(PlayerStateEnum),
    #[default]
    Default,
}
impl StateEnum {
    pub fn invoke(self) -> Self {
        match self {
            StateEnum::Player(player_state_enum) => StateEnum::Player(player_state_enum.invoke()),
            StateEnum::Default => panic!("Default invariant should not be used"),
        }
    }
}
impl Display for StateEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StateEnum::Player(state_enum) => write!(f, "{}", state_enum),
            StateEnum::Default => write!(f, "Default"),
        }
    }
}
