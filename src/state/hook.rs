
use std::fmt::Display;

use crate::draw::graphics::hook_graphics::{HOOK_LINK, HOOK_LINK_VERTEX};
use crate::draw::{Draw, Drawable};
use crate::model::*;
use crate::state::{StateMachine, StateObject};
use crate::state::state_machine::hook::{Chain, Contracting, Extending, build};
use crate::util::name_of_type;

#[derive(Debug)]
pub enum HookState {
    Extending(Extending),
    Contracting(Contracting),
    End,
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

pub fn hook_chain_as_drawables(chain: &Chain) -> Vec<Drawable> {
    let mut drawables: Vec<Drawable> = vec![];
    let mut link_shape = HOOK_LINK;
    let mut it = chain.chain().iter_full();
    let it_clone = it.clone();
    let mut prev = it.next().unwrap();
    for link in it_clone.skip(1) {
        link_shape.length = link.distance(prev);
        drawables.push(Drawable {
            state: StateObject {
                position: link.position(),
                direction: link.direction(prev),
            },
            shape: link_shape.into(),
        });
        prev = link;
    }

    for link in it {
        drawables.push(Drawable {
            state: StateObject {
                position: link.position(),
                direction: Direction::default(),
            },
            shape: HOOK_LINK_VERTEX.into(),
        });
    }
    drawables
}