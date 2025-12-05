pub const PLAYER_SPEED: Magnitude = Magnitude::new(2.5);

use std::fmt::Display;

use either::Either::{Left, Right};

use super::StateMachine;
use crate::draw::Draw;
use crate::draw::Drawable;
use crate::draw::graphics::Shape;
use crate::draw::graphics::hook_graphics::HOOK_GRAPHICS;
use crate::draw::graphics::player_graphics::*;
use crate::input::*;
use crate::model::*;
use crate::state::{
    StateObject,
    state_machine::{
        State,
        hook::{Contracting, Extending},
        player::{Idling, PlayerState, ParentChild, build},
    },
};
use crate::util::*;

pub trait StateResult<E> {
    fn or_try<U: Into<PlayerStateMachine>, F, O: FnOnce(E) -> Result<U, F>>(self, op: O) -> Result<PlayerStateMachine, F>;
}

impl<T, E> StateResult<E> for Result<T, E>
where
    T: Into<PlayerStateMachine>,
{
    fn or_try<U: Into<PlayerStateMachine>, F, O: FnOnce(E) -> Result<U, F>>(self, op: O) -> Result<PlayerStateMachine, F> {
        match self.map_err(op) {
            Ok(s) => Ok(s.into()),
            Err(Ok(s)) => Ok(s.into()),
            Err(Err(s)) => Err(s),
        }
    }
}

#[derive(Debug)]
pub enum PlayerStateMachine {
    Idling(Idling),
    ParentChildIdlingExtending(ParentChild<Idling, Extending>),
    ParentChildIdlingContracting(ParentChild<Idling, Contracting>),
}

impl PlayerStateMachine {
    pub fn new(position: Position, direction: Direction, speed: Magnitude) -> Self {
        Self::Idling(build(position, direction, speed))
    }
}
impl StateMachine for PlayerStateMachine {
    fn state_object(&self) -> Vec<super::StateObject> {
        match self {
            PlayerStateMachine::Idling(state) => vec![state.into()],
            PlayerStateMachine::ParentChildIdlingExtending(state) => vec![state.parent().into(), state.child().into()],
            PlayerStateMachine::ParentChildIdlingContracting(state) => vec![state.parent().into(), state.child().into()],
        }
    }

    fn update(self) -> Self {
        match self {
            PlayerStateMachine::Idling(state) => state.update().into(),
            PlayerStateMachine::ParentChildIdlingExtending(state) => state.update().into(),
            PlayerStateMachine::ParentChildIdlingContracting(state) => state.update().into(),
        }
    }
}
impl Draw for PlayerStateMachine {
    fn drawable(&self) -> Vec<Drawable> {
        match self {
            PlayerStateMachine::Idling(state) => {
                vec![Drawable {
                    state: state.into(),
                    shape: PLAYER_IDLING.into(), //todo choose shape based on player speed
                }]
            }
            PlayerStateMachine::ParentChildIdlingExtending(state) => {
                vec![
                    Drawable {
                        state: state.parent().into(),
                        shape: PLAYER_IDLING.into(),
                    },
                    Drawable {
                        state: state.child().into(),
                        shape: Shape::HookObject(HOOK_GRAPHICS),
                    },
                ]
            }
            PlayerStateMachine::ParentChildIdlingContracting(state) => {
                vec![
                    Drawable {
                        state: state.parent().into(),
                        shape: PLAYER_IDLING.into(),
                    },
                    Drawable {
                        state: state.child().into(),
                        shape: Shape::HookObject(HOOK_GRAPHICS),
                    },
                ]
            }
        }
    }
}

impl Display for PlayerStateMachine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", name_of_type(self));
        match self {
            PlayerStateMachine::Idling(state) => write!(f, "{}", state),
            PlayerStateMachine::ParentChildIdlingExtending(state) => write!(f, "{}", state),
            PlayerStateMachine::ParentChildIdlingContracting(state) => write!(f, "{}", state),
        }
    }
}

impl From<PlayerState> for PlayerStateMachine {
    fn from(value: PlayerState) -> Self {
        match value {
            PlayerState::Idling(state) => PlayerStateMachine::Idling(state),
            PlayerState::ParentChildIdlingExtending(state) => PlayerStateMachine::ParentChildIdlingExtending(state),
            PlayerState::ParentChildIdlingContracting(state) => PlayerStateMachine::ParentChildIdlingContracting(state),
        }
    }
}
