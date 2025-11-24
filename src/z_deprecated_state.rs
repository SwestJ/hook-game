use std::fmt::{Debug, Display};

use crate::{model::*, state::{state_hook::{Contracting, Extending, HookState}, state_player::PlayerState}, state_traits::*};
use macroquad::input::{KeyCode, is_key_down, is_key_pressed};
use toml::value::Time;
use typed_builder::TypedBuilder;

//* Enums */

#[derive(Debug)]
pub enum StateObjectEnum {
    Player(StateEnum<PlayerStateObject>),
    Hook(StateEnum<HookStateObject>),
}
impl StateObjectEnum {
    pub fn invoke(self) -> Self {
        match self {
            StateObjectEnum::Player(state) => Self::Player(state.invoke()),
            StateObjectEnum::Hook(state) => Self::Hook(state.invoke()),
        }
    }
    pub fn draw(&self) {
        match self {
            StateObjectEnum::Player(state_enum) => state_enum.draw(),
            StateObjectEnum::Hook(state_enum) => state_enum.draw(),
        }
    }
}
impl Default for StateObjectEnum {
    fn default() -> Self {
        StateObjectEnum::Player(StateEnum::default())
    }
}
impl From<PlayerStateObject> for StateObjectEnum {
    fn from(value: PlayerStateObject) -> Self {
        Self::Player(value.into())
    }
}
impl From<HookStateObject> for StateObjectEnum {
    fn from(value: HookStateObject) -> Self {
        Self::Hook(Moving(value, value.object().physics().min_velocity()).into())
    }
}
impl Display for StateObjectEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StateObjectEnum::Player(state_enum) => write!(f, "{}", state_enum),
            StateObjectEnum::Hook(state_enum) => write!(f, "{}", state_enum),
        }
    }
}

#[derive(Debug)]
pub enum StateEnum<T> where T: StateObject {
    Idling(Idling<T>),
    Moving(Moving<T>),
    Shooting(Shooting<T>),
}
impl<T> StateEnum<T> where T: StateObject {
    fn draw(&self) where T: StateObject + Draw {
        match self {
            StateEnum::Idling(state) => state.0.draw(),
            StateEnum::Moving(state) => state.0.draw(),
            StateEnum::Shooting(state) => state.0.draw(),
        }
    }
}
impl StateEnum<PlayerStateObject> {
    pub fn invoke(self) -> Self {
        match self {
            StateEnum::Idling(state) => state.invoke(),
            StateEnum::Moving(state) => state.invoke(),
            StateEnum::Shooting(state) => state.invoke(),
        }
    }
}
impl StateEnum<HookStateObject> {
    pub fn invoke(self) -> Self {
        match self {
            StateEnum::Moving(state) => state.invoke(),
            _ => panic!("Hook should not be in this state"),
        }
    }
}

impl<T> Default for StateEnum<T> where T: StateObject {
    fn default() -> Self {
        StateEnum::Idling(Idling::default())
    }
}
impl<T: StateObject> From<Idling<T>> for StateEnum<T> {
    fn from(value: Idling<T>) -> Self {
        StateEnum::Idling(value)
    }
}
impl<T: StateObject> From<Moving<T>> for StateEnum<T> {
    fn from(value: Moving<T>) -> Self {
        StateEnum::Moving(value)
    }
}
impl<T: StateObject> From<Shooting<T>> for StateEnum<T> {
    fn from(value: Shooting<T>) -> Self {
        StateEnum::Shooting(value)
    }
}
impl<T: StateObject> From<T> for StateEnum<T> {
    fn from(value: T) -> Self {
        Self::Idling(value.into())
    }
}
impl<T: StateObject> Display for StateEnum<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StateEnum::Idling(state) => write!(f, "{}", state),
            StateEnum::Moving(state) => write!(f, "{}", state),
            StateEnum::Shooting(state) => write!(f, "{}", state),
        }
    }
}

//* States */

#[derive(Debug, Default)]
pub struct Idling<T: StateObject>(pub T);
impl<T> From<T> for Idling<T> where T: StateObject {
    fn from(value: T) -> Self {
        Self(value)
    }
}

#[derive(Debug)]
pub struct Moving<T: StateObject>(pub T, pub Velocity);
#[derive(Debug)]
pub struct Shooting<T: StateObject>(pub T, pub Option<HookStateEnum>);

impl<T> Display for Idling<T> where T: StateObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Idling {}", self.0)
    }
}
impl<T> Display for Moving<T> where T: StateObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Moving {}, {}", self.0, self.1)
    }
}
impl<T> Display for Shooting<T> where T: StateObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Shooting {}", self.0)
    }
}

//* Player */

#[derive(Default, TypedBuilder, Debug, Copy, Clone)]
pub struct PlayerStateObject {
    object: Player,
    graphics: GraphicsObject,
}

impl StateObject for PlayerStateObject {
    type Object = Player;
    fn object(&self) -> Self::Object {
        self.object
    }

    fn object_mut(&mut self) -> &mut Self::Object {
        &mut self.object
    }
}
impl UserControl for PlayerStateObject {}

impl Idling<PlayerStateObject> {
    fn invoke(self) -> StateEnum<PlayerStateObject> {
        match self.r#move() {
            Ok(state) => state.into(),
            Err(state) => {
                match state.shoot() {
                    Ok(state) => state.into(),
                    Err(state) => state.into(),
                }
            },
        }
    }
}
impl Moving<PlayerStateObject> {
    fn invoke(self) -> StateEnum<PlayerStateObject> {
        match Move::r#move(self) {
            Ok(state) => state.into(),
            Err(state) => state.idle().map_or_else(StateEnum::from, StateEnum::from),
        }
    }
}
impl Shooting<PlayerStateObject> {
    fn invoke(self) -> StateEnum<PlayerStateObject> {
        match self.idle() {
            Ok(state) => state.into(),
            Err(state) => state.into(),
        }
    }
    pub fn hook_remove(&mut self) {
        self.1 = None;
    }
    pub fn hook(&self) -> &Option<HookStateEnum> {
        &self.1
    }
}

impl State for Idling<PlayerStateObject> {
    type StateObject = PlayerStateObject;
    fn state_object(&self) -> Self::StateObject {
        self.0
    }
}
impl State for Moving<PlayerStateObject> {
    type StateObject = PlayerStateObject;
    fn state_object(&self) -> Self::StateObject {
        self.0
    }
}
impl State for Shooting<PlayerStateObject> {
    type StateObject = PlayerStateObject;
    fn state_object(&self) -> Self::StateObject {
        self.0
    }
}

impl Idle<PlayerStateObject> for Idling<PlayerStateObject> {}
impl Idle<PlayerStateObject> for Moving<PlayerStateObject> {}
impl Idle<PlayerStateObject> for Shooting<PlayerStateObject> {}
impl Move<PlayerStateObject> for Idling<PlayerStateObject> {
    fn velocity(&self) -> Velocity {
        0.0.into()
    }
    fn direction(&self) -> Direction {
        self.state_object().direction_from_user()
    }
}
impl Move<PlayerStateObject> for Moving<PlayerStateObject> {
    fn velocity(&self) -> Velocity {
        self.1
    }
    fn direction(&self) -> Direction {
        self.state_object().direction_from_user()
    }
}
impl Shoot<PlayerStateObject> for Idling<PlayerStateObject> {
    fn triggered(&self) -> bool {
        is_shooting()
    }
}

impl Draw for PlayerStateObject {
    fn graphics(&self) -> &GraphicsObject {
        &self.graphics
    }
    fn position(&self) -> Position {
        self.object.position()
    }
}
impl Display for PlayerStateObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.object())
    }
}

//* Hook */

#[derive(Default, TypedBuilder, Debug, Copy, Clone)]
pub struct HookStateObject {
    object: Hook,
    graphics: GraphicsObject,
}
impl StateObject for HookStateObject {
    type Object = Hook;
    fn object(&self) -> Hook {
        self.object
    }
    fn object_mut(&mut self) -> &mut Self::Object {
        &mut self.object
    }
}

impl Moving<HookStateObject> {
    fn invoke(self) -> StateEnum<HookStateObject> {
        match Move::r#move(self) {
            Ok(state) => state.into(),
            Err(state) => state.into(),
        }
    }
}

impl Move<HookStateObject> for Moving<HookStateObject> {
    fn velocity(&self) -> Velocity {
        self.1
    }
    fn direction(&self) -> Direction {
        *self.state_object().object().direction()
    }

    fn r#move(self) -> Result<Moving<HookStateObject>, Self> {
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
impl State for Idling<HookStateObject> {
    type StateObject = HookStateObject;
    fn state_object(&self) -> Self::StateObject {
        self.0
    }
}
impl State for Moving<HookStateObject> {
    type StateObject = HookStateObject;
    fn state_object(&self) -> Self::StateObject {
        self.0
    }
}
impl Draw for HookStateObject {
    fn graphics(&self) -> &GraphicsObject {
        &self.graphics
    }
    fn position(&self) -> Position {
        self.object.position()
    }
}

impl Display for HookStateObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.object())
    }
}


