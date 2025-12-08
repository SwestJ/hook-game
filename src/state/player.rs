pub const PLAYER_SPEED: Magnitude = Magnitude::new(2.5);

use std::fmt::Display;
use std::vec;

use super::StateMachine;
use crate::collision;
use crate::collision::CollisionBox;
use crate::draw::Draw;
use crate::draw::Drawable;
use crate::draw::graphics::Shape;
use crate::draw::graphics::hook_graphics::HOOK_GRAPHICS;
use crate::draw::graphics::player_graphics::*;
use crate::model::*;
use crate::state::hook::hook_chain_as_drawables;
use crate::state::{
    StateObject,
    state_machine::{
        State,
        hook::{Contracting, Extending},
        player::{Idling, ParentChild, PlayerState, build},
    },
};
use crate::util::*;

pub trait StateResult<E> {
    fn or_try<U: Into<PlayerStateMachine>, F, O: FnOnce(E) -> Result<U, F>>(
        self,
        op: O,
    ) -> Result<PlayerStateMachine, F>;
}

impl<T, E> StateResult<E> for Result<T, E>
where
    T: Into<PlayerStateMachine>,
{
    fn or_try<U: Into<PlayerStateMachine>, F, O: FnOnce(E) -> Result<U, F>>(
        self,
        op: O,
    ) -> Result<PlayerStateMachine, F> {
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
            PlayerStateMachine::ParentChildIdlingContracting(state) => {
                vec![state.parent().into(), state.child().into()]
            }
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
                    shape: Shape::PlayerObject(PLAYER_ANIMATION.current_frame()), //todo choose shape based on player speed
                }]
            }
            PlayerStateMachine::ParentChildIdlingExtending(state) => {
                let mut vec = vec![
                    Drawable {
                        state: state.parent().into(),
                        shape: Shape::PlayerObject(PLAYER_GRAPHICS),
                    },
                    Drawable {
                        state: state.child().into(),
                        shape: Shape::HookObject(HOOK_GRAPHICS),
                    },
                ];
                vec.append(&mut hook_chain_as_drawables(state.child().chain()));
                vec
            }
            PlayerStateMachine::ParentChildIdlingContracting(state) => {
                let mut vec = vec![
                    Drawable {
                        state: state.parent().into(),
                        shape: Shape::PlayerObject(PLAYER_GRAPHICS),
                    },
                    Drawable {
                        state: state.child().into(),
                        shape: Shape::HookObject(HOOK_GRAPHICS),
                    },
                ];
                vec.append(&mut hook_chain_as_drawables(state.child().chain()));
                vec
            }
        }
    }
}
impl collision::Collision for PlayerStateMachine {
    fn collision_box(&self) -> Vec<collision::CollisionBox> {
        match self {
            PlayerStateMachine::Idling(state) => {
                let position = state.position();
                let direction = state.direction();
                let object = PLAYER_GRAPHICS;
                vec![Self::bounds(object.model.rotate(direction).translate(position))]
            }
            PlayerStateMachine::ParentChildIdlingExtending(state) => {
                vec![
                    Self::bounds(
                        PLAYER_GRAPHICS
                            .model
                            .rotate(state.direction())
                            .translate(state.position()),
                    ),
                    Self::bounds(
                        HOOK_GRAPHICS
                            .model
                            .rotate(state.child().direction())
                            .translate(state.child().position()),
                    ),
                ]
            }
            PlayerStateMachine::ParentChildIdlingContracting(state) => {
                vec![
                    Self::bounds(
                        PLAYER_GRAPHICS
                            .model
                            .rotate(state.direction())
                            .translate(state.position()),
                    ),
                    Self::bounds(
                        HOOK_GRAPHICS
                            .model
                            .rotate(state.child().direction())
                            .translate(state.child().position()),
                    ),
                ]
            }
        }
    }

    fn collision_detected(&self /*other object */) {
        todo!()
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
