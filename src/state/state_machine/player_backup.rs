pub const PLAYER_SPEED: f32 = 2.5;

use std::fmt::Display;

use either::Either::{Left, Right};

use super::State;
use crate::draw::Draw;
use crate::draw::Drawable;
use crate::draw::graphics::player_graphics::*;
use crate::input::*;
use crate::model::*;
use crate::state::StateObject;
use crate::state::hook::*;
use crate::state::player::state_player::*;
use crate::util::*;
use state_player::*;

pub trait StateResult<E> {
    fn or_try<U: Into<PlayerState>, F, O: FnOnce(E) -> Result<U, F>>(self, op: O ) -> Result<PlayerState, F>;
}

impl<T, E> StateResult<E> for Result<T, E> where T: Into<PlayerState> {
    fn or_try<U: Into<PlayerState>, F, O: FnOnce(E) -> Result<U, F>>(self, op: O ) -> Result<PlayerState, F> {
        match self.map_err(op) {
            Ok(s) => Ok(s.into()),
            Err(Ok(s)) => Ok(s.into()),
            Err(Err(s)) => Err(s),
        }
    }
}

#[derive(Debug)]
pub enum PlayerState {
    Idling(Idling),
    Moving(Moving),
    Shooting(Shooting),
    DualityIdlingShooting(Duality<Idling, Shooting>),
    DualityMovingShooting(Duality<Moving, Shooting>),
}

impl PlayerState {
    pub fn new(position: Position, direction: Direction) -> Self {
        Self::Idling(state_player::build(position, direction))
    }
}
impl State for PlayerState {
    fn state_object(&self) -> super::StateObject {
        let mut position: Position;
        let mut direction: Direction;
        match self {
            PlayerState::Idling(state) => {
                position = state.position();
                direction = state.direction();
            }
            PlayerState::Moving(state) => {
                position = state.position();
                direction = state.direction();
            }
            PlayerState::Shooting(state) => {
                position = state.position();
                direction = state.direction();
            }
            PlayerState::DualityIdlingShooting(state) => {
                position = state.position();
                direction = state.direction();
            }
            PlayerState::DualityMovingShooting(state) => {
                position = state.position();
                direction = state.direction();
            }
        }
        StateObject { position, direction }
    }

    fn update(self) -> Self {
        match self {
            PlayerState::Idling(state) => match state.r#move().or_try(Idling::shoot) {
                Ok(state) => state,
                Err(idling) => PlayerState::Idling(idling),
            },
            PlayerState::Moving(state) => match state.move_or_idle() {
                Ok(moving) => PlayerState::Moving(moving),
                Err(idling) => PlayerState::Idling(idling),
            },
            PlayerState::Shooting(state) => {
                let position = state.position();
                match state.action_update_hook(position).try_idle() {
                    Ok(Left(duality)) => duality.into(),
                    Ok(Right(idling)) => idling.into(),
                    Err(shooting) => shooting.into(),
                }
            }
            PlayerState::DualityIdlingShooting(state) => {
                match state.action_update_hook().map_left(|s| s.move_or_idle()) {
                    Left(Left(duality)) => duality.into(),
                    Left(Right(duality)) => duality.into(),
                    Right(idling) => idling.into(),
                }
            }
            PlayerState::DualityMovingShooting(state) => {
                match state.action_update_hook().map_left(|s| s.move_or_idle()) {
                    Left(Left(dual)) => dual.into(),
                    Left(Right(dual)) => dual.into(),
                    Right(moving) => moving.into(),
                }
            }
        }
    }
}
impl Draw for PlayerState {
    fn drawable(&self) -> Vec<Drawable> {
        let state_object = self.state_object();
        match self {
            PlayerState::Idling(state) => {
                vec![Drawable {
                    state: state_object,
                    shape: PLAYER_IDLING.into(),
                }]
            }
            PlayerState::Moving(state) => {
                vec![Drawable {
                    state: state_object,
                    shape: PLAYER_MOVING.into(),
                }]
            }
            PlayerState::Shooting(state) => {
                state.hook();
                let mut vec = vec![Drawable {
                    state: state_object,
                    shape: PLAYER_SHOOTING.into(),
                }];
                vec.append(&mut state.hook().drawable());
                vec
            }
            PlayerState::DualityIdlingShooting(state) => {
                let mut vec = vec![Drawable {
                    state: state_object,
                    shape: PLAYER_IDLING.into(),
                }];
                vec.append(&mut state.yang().hook().drawable());
                vec
            }
            PlayerState::DualityMovingShooting(state) => {
                let mut vec = vec![Drawable {
                    state: state_object,
                    shape: PLAYER_MOVING.into(),
                }];
                vec.append(&mut state.yang().hook().drawable());
                vec
            }
        }
    }
}
impl Display for PlayerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", name_of_type(self));
        match self {
            PlayerState::Idling(state) => write!(f, "{}", state),
            PlayerState::Moving(state) => write!(f, "{}", state),
            PlayerState::Shooting(state) => {
                write!(f, "{}", state)
            }
            PlayerState::DualityIdlingShooting(state) => {
                write!(f, "{}", state)
            }
            PlayerState::DualityMovingShooting(state) => {
                write!(f, "{}", state)
            }
        }
    }
}
impl From<Idling> for PlayerState {
    fn from(value: Idling) -> Self {
        PlayerState::Idling(value)
    }
}
impl From<Shooting> for PlayerState {
    fn from(value: Shooting) -> Self {
        PlayerState::Shooting(value)
    }
}
impl From<Moving> for PlayerState {
    fn from(value: Moving) -> Self {
        PlayerState::Moving(value)
    }
}
impl From<Duality<Idling, Shooting>> for PlayerState {
    fn from(value: Duality<Idling, Shooting>) -> Self {
        PlayerState::DualityIdlingShooting(value)
    }
}
impl From<Duality<Moving, Shooting>> for PlayerState {
    fn from(value: Duality<Moving, Shooting>) -> Self {
        PlayerState::DualityMovingShooting(value)
    }
}

pub mod state_player {
    use std::{any::type_name_of_val, fmt::Display};

    use either::Either::{self, Left, Right};

    use crate::state::{StateObject, hook::state_hook::Extending};

    use super::*;

    pub fn build(position: Position, direction: Direction) -> Idling {
        Idling::idle(position, direction)
    }

    pub trait LocalState {
        fn position(&self) -> Position;
        fn direction(&self) -> Direction;
    }

    #[derive(Debug)]
    pub enum LocalStateEnum {
        Idling(Idling),
        Moving(Moving),
        Shooting(Shooting),
        IdlingShooting(Idling, Shooting),
        MovingShooting(Moving, Shooting),
    }

    pub enum IdlingAction {
        Run(_Idling),
        Shoot(Shooting), //still its own state since player can't run while shooting
        Dash(_Idling), // Maybe you should be in a Dashing state shortly, because the player can't be controlled while dashing
        Graple(Grapled) //i.e. getting pulled by the chain towards the hook.
    }

    pub fn update<T: LocalState>(state: T) -> LocalStateEnum {
        // let state.update()
        todo!()
    }

    pub struct _Idling {

    }
    impl _Idling {
        /// Updates "state object". This should always be called. But it can be after "trying actions", and then only if no action succeeds.
        /// Update should contain mandatory actions. E.g. updating position from falling or for a thrown object
        /// How can it be enforced?
        /// If the state has some limit (i.e. chain length), update() should itself move to another state - it should not be dependent on the caller to make a "try action" call.
        /// Use "Try actions" for actions which are logically a choice. I.e. "jump" and "walk". "Stop falling because you hot the ground" is not a choice.
        /// The state machine could perform needed updates and actions itself, but otherwise return itself, and let the caller choose and try actions.
        fn update(self) -> IdlingAction {

            todo!()
        }
    }

    pub struct Grapled {

    }

    #[derive(Debug)]
    pub struct Idling {
        position: Position,
        direction: Direction,
    }
    impl LocalState for Idling {
        fn position(&self) -> Position {
            self.position
        }
        fn direction(&self) -> Direction {
            self.direction
        }
    }
    impl Idling {
        fn idle(position: Position, direction: Direction) -> Self {
            let state = Idling { position, direction };
            state.idle_self()
        }

        pub fn idle_self(self) -> Self {
            self
        }

        pub fn r#move(self) -> Result<Moving, Self> {
            let direction = get_player_move();
            if direction.is_zero() {
                Err(self)
            } else {
                Ok(Moving::r#move(self.position(), self.direction(), PLAYER_SPEED.into()))
            }
        }

        pub fn shoot(self) -> Result<Shooting, Self> {
            if is_shooting() {
                Ok(Shooting::shoot(self.position(), self.direction(), HOOK_EXTENDING_SPEED))
            } else {
                Err(self)
            }
        }
    }

    #[derive(Debug)]
    pub struct Moving {
        position: Position,
        velocity: Magnitude,
        direction: Direction,
    }
    impl LocalState for Moving {
        fn position(&self) -> Position {
            self.position
        }
        fn direction(&self) -> Direction {
            self.direction
        }
    }
    impl Moving {
        pub fn velocity(&self) -> Magnitude {
            self.velocity
        }
        fn r#move(position: Position, direction: Direction, velocity: Magnitude) -> Self {
            let state = Moving {
                position,
                velocity,
                direction,
            };
            state.move_self(direction)
        }

        pub fn move_self(self, direction: Direction) -> Moving {
            let position = Physics::calculate_new_position_from_speed(self.position(), self.velocity(), direction);
            Moving {
                position,
                direction,
                ..self
            }
        }

        pub fn move_or_idle(self) -> Result<Self, Idling> {
            let direction = get_player_move();
            if direction.is_zero() {
                Err(Idling::idle(self.position(), self.direction()))
            } else {
                Ok(self.move_self(direction))
            }
        }
    }

    #[derive(Debug)]
    pub struct Shooting {
        position: Position,
        velocity: Magnitude,
        direction: Direction,
        hook: HookState,
    }
    impl LocalState for Shooting {
        fn position(&self) -> Position {
            self.position
        }
        fn direction(&self) -> Direction {
            self.direction
        }
    }
    impl Shooting {
        pub fn velocity(&self) -> Magnitude {
            self.velocity
        }
        pub fn hook(&self) -> &HookState {
            &self.hook
        }
        fn shoot(position: Position, direction: Direction, velocity: Magnitude) -> Self {
            let hook = HookState::new(velocity, direction, position, HOOK_AMOUNT_LINKS);
            Shooting {
                position,
                velocity,
                direction,
                hook,
            }
        }

        pub fn action_update_hook(self, position: Position) -> Self {
            todo!("hook needs player position");
            Shooting {
                hook: self.hook.update(),
                ..self
            }
        }

        pub fn try_idle(self) -> Result<Either<Duality<Idling, Self>, Idling>, Self> {
            match self.hook {
                HookState::Extending(_) => Err(self),
                HookState::Contracting(_) => Ok(Left(Duality {
                    ying: Idling::idle(self.position(), self.direction()),
                    yang: self,
                })),
                HookState::End => Ok(Right(Idling::idle(self.position(), self.direction()))),
            }
        }
    }

    #[derive(Debug)]
    pub struct Duality<A, B>
    where
        A: LocalState,
        B: LocalState,
    {
        ying: A,
        yang: B,
    }
    impl<A, B> LocalState for Duality<A, B>
    where
        A: LocalState,
        B: LocalState,
    {
        fn position(&self) -> Position {
            self.ying.position()
        }
        fn direction(&self) -> Direction {
            self.ying.direction()
        }
    }

    impl<A> Duality<A, Shooting>
    where
        A: LocalState,
    {
        pub fn action_update_hook(self) -> Either<Self, A> {
            let Duality { ying, yang } = self;
            let yang = yang.action_update_hook(ying.position());
            match yang.hook {
                HookState::Extending(_) => Left(Duality { ying, yang }),
                HookState::Contracting(_) => Left(Duality { ying, yang }),
                HookState::End => Right(ying),
            }
        }
    }

    impl Duality<Idling, Shooting> {
        pub fn move_or_idle(self) -> Either<Duality<Moving, Shooting>, Self> {
            let Duality { ying, yang } = self;
            match ying.r#move() {
                Ok(moving) => Left(Duality { ying: moving, yang }),
                Err(idling) => Right(Duality { ying: idling, yang }),
            }
        }
    }

    impl Duality<Moving, Shooting> {
        pub fn move_or_idle(self) -> Either<Self, Duality<Idling, Shooting>> {
            let Duality { ying, yang } = self;
            match ying.move_or_idle() {
                Ok(moving) => Left(Duality { ying: moving, yang }),
                Err(idling) => Right(Duality { ying: idling, yang }),
            }
        }
    }

    pub struct Relation<A, B>
    where
        A: LocalState,
    {
        parent: A,
        child: B,
    }
    impl Relation<Idling, Extending> {
        pub fn update(self) -> Self {
            let Relation { parent, child } = self;
            Relation {
                parent: parent.idle_self(),
                child: child.extend_self(),
            }
        }
        pub fn action_parent(self) {
            todo!()
        }
    }

    // impl From<Idling> for Moving {
    //     fn from(value: Idling) -> Self {
    //
    //     }
    // }

    // trait Move: PlayerState + Sized + Into<Moving> {
    //     fn r#move(self) -> Result<Moving, Self> {
    //         let direction = get_player_move();
    //         if direction.is_zero() {
    //             Err(self)
    //         } else {
    //             Ok(self.into())
    //         }
    //     }
    // }

    // trait Shoot: PlayerState + Sized + Into<Shooting> {
    //     fn shoot(self) -> Result<Shooting, Self> {
    //         if is_shooting() {
    //             Ok(self.into())
    //         } else {
    //             Err(self)
    //         }
    //     }
    // }

    // fn r#move(current: impl Into<Moving>) -> Result<Moving, impl Into<Moving>> {
    //     let direction = get_player_move();
    //     if direction.is_zero() {
    //         Err(current)
    //     } else {
    //         Ok(current.into())
    //     }
    // }

    // fn shoot(current: impl PlayerState) -> Result<Shooting, impl PlayerState> {
    //     if is_shooting() {
    //         Ok(Shooting::shoot(
    //             current.position(),
    //             current.direction(),
    //             HOOK_EXTENDING_SPEED,
    //         ))
    //     } else {
    //         Err(current)
    //     }
    // }

    impl<A, B> Duality<A, B>
    where
        A: LocalState,
        B: LocalState,
    {
        pub fn ying(&self) -> &A {
            &self.ying
        }
        pub fn yang(&self) -> &B {
            &self.yang
        }
    }

    //** Std trait implementations */
    impl Display for Idling {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} {}", name_of_type(self), self.position())
        }
    }
    impl Display for Moving {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{} {} {} {}",
                name_of_type(self),
                self.position(),
                self.direction(),
                self.velocity()
            )
        }
    }
    impl Display for Shooting {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{} {} {} \n{}",
                name_of_type(self),
                self.position(),
                self.direction(),
                self.hook
            )
        }
    }
    impl<A, B> Display for Duality<A, B>
    where
        A: LocalState + Display,
        B: LocalState + Display,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Duality\n\tYing {}\n\tYang {}", self.ying(), self.yang())
        }
    }
}
