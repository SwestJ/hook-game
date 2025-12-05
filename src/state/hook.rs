
use std::fmt::Display;

use crate::draw::Draw;
use crate::model::*;
use crate::state::StateMachine;
use crate::state::state_machine::hook::{Contracting, Extending, build};
use crate::util::name_of_type;

#[derive(Debug)]
pub enum HookState {
    Extending(Extending),
    Contracting(Contracting),
    End,
}
impl StateMachine for HookState {
    fn state_object(&self) -> Vec<super::StateObject> {
        todo!()
    }

    fn update(self) -> Self {
        match self {
            HookState::Extending(extending) => match extending.extend_self().try_contract(todo!()) {
                Ok(contracting) => HookState::Contracting(contracting),
                Err(extending) => HookState::Extending(extending),
            },
            HookState::Contracting(contracting) => {
                let contracting = contracting.contract_self(todo!());
                match contracting.try_end() {
                    Ok(_) => HookState::End,
                    Err(contracting) => HookState::Contracting(contracting),
                }
            }
            HookState::End => self,
        }
    }
}
impl Draw for HookState {
    fn drawable(&self) -> Vec<crate::draw::Drawable> {
        todo!()
    }
}
impl Display for HookState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", name_of_type(self));
        match self {
            HookState::Extending(state) => write!(f, "{}", state),
            HookState::Contracting(state) => write!(f, "{}", state),
            HookState::End => write!(f, "End"),
        }
    }
}
impl HookState {
    pub fn new(speed: Magnitude, direction: Direction, origin: Position, amount_of_links: usize) -> Self {
        Self::Extending(build(speed, direction, origin, amount_of_links))
    }
    pub fn update_with_position() {

    }
}