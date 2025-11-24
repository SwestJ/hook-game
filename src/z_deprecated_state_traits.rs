use std::fmt::{Debug, Display};
use crate::{create_hook, model::*, state::*};

//* Traits */

pub trait State: Sized + Debug + Display {
    type StateObject: StateObject;
    fn state_object(&self) -> Self::StateObject;
}

pub trait StateObject: Default + Sized + Debug + Copy + Display {
    type Object: Object;
    fn object(&self) -> Self::Object;
    fn object_mut(&mut self) -> &mut Self::Object;
}

/// Makes it possible for user to interact with object through keyboard/mouse
/// Could be used to add input/action pairs during setup (or during gameplay)
/// and get the actions when checking how to handle certain situations e.g. moving
/// E.g. if the object has ControlMoving (which has supertrait Interact), check for inputs for "movable" actions.
///     and listen for those inputs. If no inputs, don't move.
/// TODO maybe implement this. For now we just use "UserControl" to directly checking for movement inputs.
pub trait Interact {
    fn add_action(&mut self, action: &str, input: &str);
}
/// The object can be controlled by user input (for now just moving)
pub trait UserControl: StateObject {
    fn direction_from_user(&self) -> Direction {
        get_player_move()
    }
}

pub trait Draw {
    fn graphics(&self) -> &GraphicsObject;
    fn position(&self) -> Position;
    fn draw(&self) {
        self.graphics().draw(self.position());
    }
}

//* Actions (Traits) */
// A given action always leads to same state, indpependent of the starting state
// An action returns the same object type, wrapped in some state

pub trait Idle<T> where Self: State<StateObject = T>, T: StateObject {
    fn idle(self) -> Result<Idling<T>, Self> {
        Ok(Idling(self.state_object()))
    }
}
pub trait Move<T> where Self: State<StateObject = T>, T: StateObject {
    fn velocity(&self) -> Velocity;
    fn direction(&self) -> Direction;
    fn r#move(self) -> Result<Moving<T>, Self> {
        let direction = self.direction();
        if direction.is_zero() {
            return Err(self) // Can't move without a direction
        }
        let v = self.velocity();
        let mut state_object = self.state_object();
        let object = state_object.object_mut();
        let dt = 0.1;
        let v_new = object.physics().accelerate(v, dt);
        object.set_direction(&direction);
        object.update_position(v, direction);
        Ok(Moving(state_object, v_new))
    }
}
pub trait Shoot<T> where Self: State<StateObject = T>, T: StateObject {
    fn triggered(&self) -> bool;
    fn shoot(self) -> Result<Shooting<T>, Self> {
        if !self.triggered() {
            return Err(self)
        }
        let mut state_object = self.state_object();
        let mut hook = HookStateEnum::new(
            3.0.into(),
            state_object.object().direction().clone(),
            state_object.object().position().clone(),
            200.0);
        Ok(Shooting(state_object, Some(hook)))
    }
}


#[derive(Debug)]
pub struct SpawnWrapper<T, S> where T: StateObject, S: StateObject {
    spawner: StateEnum<T>,
    spawn: StateEnum<S>
}
impl<T, S> State for SpawnWrapper<T, S> where T: StateObject, S: StateObject {
    type StateObject = T;

    fn state_object(&self) -> Self::StateObject {
        match &self.spawner {
            StateEnum::Idling(state) => state.0,
            StateEnum::Moving(state) => state.0,
            StateEnum::Shooting(state) => state.0,
        }
    }
}
impl<S,T> Display for SpawnWrapper<S,T> where S: StateObject, T: StateObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.spawner, self.spawner)
    }
}

trait DualStates: State {
    type Ying: State;
    type Yang: State;
}
// impl<T, S> DualStates for SpawnWrapper<T, S> where T: StateObject, S: StateObject {
//     type Ying = T;
//     type Yang = S;
// }

impl DualStates for Shooting<PlayerStateObject> {
    type Ying = Self;
    type Yang = Moving<HookStateObject>;
}