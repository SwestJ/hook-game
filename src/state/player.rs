//* ------------------------------------------------------------------------*/
//* ------------------------------------------------------------------------*/
//* ---------------------------------PLAYER---------------------------------*/
//* ------------------------------------------------------------------------*/
//* ------------------------------------------------------------------------*/
pub const PLAYER_SPEED: f32 = 2.5;

use std::fmt::Display;

use either::Either::{Left, Right};

use crate::model::*;
use crate::state::player::state_player::*;

#[derive(Debug)]
pub enum PlayerStateEnum {
    Idling(PlayerStateMachine<state_player::Idling>),
    Moving(PlayerStateMachine<state_player::Moving>),
    Shooting(PlayerStateMachine<state_player::Shooting>),
    DualityIdlingShooting(PlayerStateMachine<state_player::Duality<Idling, Shooting>>),
    DualityMovingShooting(PlayerStateMachine<state_player::Duality<Moving, Shooting>>),
}

impl PlayerStateEnum {
    pub fn new(position: Position, direction: Direction) -> Self {
        Self::Idling(PlayerStateMachine {
            state: state_player::build(position, direction),
        })
    }

    pub fn invoke(self) -> Self {
        match self {
            PlayerStateEnum::Idling(player_state_machine) => player_state_machine.invoke(),
            PlayerStateEnum::Moving(player_state_machine) => player_state_machine.invoke(),
            PlayerStateEnum::Shooting(player_state_machine) => player_state_machine.invoke(),
            PlayerStateEnum::DualityIdlingShooting(player_state_machine) => {
                player_state_machine.invoke()
            }
            PlayerStateEnum::DualityMovingShooting(player_state_machine) => {
                player_state_machine.invoke()
            }
        }
    }
}
impl Display for PlayerStateEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayerStateEnum::Idling(player_state_machine) => write!(f, "{}", player_state_machine),
            PlayerStateEnum::Moving(player_state_machine) => write!(f, "{}", player_state_machine),
            PlayerStateEnum::Shooting(player_state_machine) => {
                write!(f, "{}", player_state_machine)
            }
            PlayerStateEnum::DualityIdlingShooting(player_state_machine) => {
                write!(f, "{}", player_state_machine)
            }
            PlayerStateEnum::DualityMovingShooting(player_state_machine) => {
                write!(f, "{}", player_state_machine)
            }
        }
    }
}
impl From<Idling> for PlayerStateEnum {
    fn from(value: Idling) -> Self {
        PlayerStateEnum::Idling(PlayerStateMachine { state: value })
    }
}
impl From<Shooting> for PlayerStateEnum {
    fn from(value: Shooting) -> Self {
        PlayerStateEnum::Shooting(PlayerStateMachine { state: value })
    }
}
impl From<Moving> for PlayerStateEnum {
    fn from(value: Moving) -> Self {
        PlayerStateEnum::Moving(PlayerStateMachine { state: value })
    }
}
impl From<Duality<Idling, Shooting>> for PlayerStateEnum {
    fn from(value: Duality<Idling, Shooting>) -> Self {
        PlayerStateEnum::DualityIdlingShooting(PlayerStateMachine { state: value })
    }
}
impl From<Duality<Moving, Shooting>> for PlayerStateEnum {
    fn from(value: Duality<Moving, Shooting>) -> Self {
        PlayerStateEnum::DualityMovingShooting(PlayerStateMachine { state: value })
    }
}

pub trait StateMachine<T: PlayerState> {
    fn invoke(self) -> PlayerStateEnum;
}

#[derive(Debug)]
pub struct PlayerStateMachine<T: PlayerState> {
    state: T,
}
impl<T> PlayerStateMachine<T>
where
    T: PlayerState,
{
    pub fn state(&self) -> &T {
        &self.state
    }
}
impl PlayerStateMachine<Idling> {
    fn invoke(self) -> PlayerStateEnum {
        match self.state.try_move().map_err(|state| state.try_shoot()) {
            Ok(moving) => PlayerStateEnum::Moving(PlayerStateMachine { state: moving }),
            Err(Ok(shooting)) => PlayerStateEnum::Shooting(PlayerStateMachine { state: shooting }),
            Err(Err(idling)) => PlayerStateEnum::Idling(PlayerStateMachine { state: idling }),
        }
    }
}
impl PlayerStateMachine<Moving> {
    fn invoke(self) -> PlayerStateEnum {
        match self.state.move_or_idle() {
            Ok(moving) => PlayerStateEnum::Moving(PlayerStateMachine { state: moving }),
            Err(idling) => PlayerStateEnum::Idling(PlayerStateMachine { state: idling }),
        }
    }
}
impl PlayerStateMachine<Shooting> {
    fn invoke(self) -> PlayerStateEnum {
        let position = self.position();
        match self.state.action_update_hook(position).try_idle() {
            Ok(Left(duality)) => duality.into(),
            Ok(Right(idling)) => idling.into(),
            Err(shooting) => shooting.into(),
        }
    }
}
impl PlayerStateMachine<Duality<Idling, Shooting>> {
    fn invoke(self) -> PlayerStateEnum {
        match self
            .state
            .action_update_hook()
            .map_left(|s| s.move_or_idle())
        {
            Left(Left(duality)) => duality.into(),
            Left(Right(duality)) => duality.into(),
            Right(idling) => idling.into(),
        }
    }
}
impl PlayerStateMachine<Duality<Moving, Shooting>> {
    fn invoke(self) -> PlayerStateEnum {
        match self
            .state
            .action_update_hook()
            .map_left(|s| s.move_or_idle())
        {
            Left(Left(dual)) => dual.into(),
            Left(Right(dual)) => dual.into(),
            Right(moving) => moving.into(),
        }
    }
}

impl<T> PlayerState for PlayerStateMachine<T>
where
    T: PlayerState,
{
    fn position(&self) -> Position {
        self.state().position()
    }

    fn direction(&self) -> Direction {
        self.state().direction()
    }

    fn to_enum(self) -> State {
        todo!()
    }
}

impl<T> Display for PlayerStateMachine<T>
where
    T: PlayerState,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.state())
    }
}

pub mod state_player {
    use std::{any::type_name_of_val, fmt::Display};

    use either::Either::{self, Left, Right};

    use crate::{
        input::*,
        model::*,
        state::{hook::{state_hook::{Extending, HookState}, *}, player::PLAYER_SPEED},
    };

    pub fn build(position: Position, direction: Direction) -> Idling {
        Idling::idle(position, direction)
    }

    #[derive(Debug)]
    pub enum State {
        Idling(Idling),
        Moving(Moving),
        Shooting(Shooting),
        IdlingShooting(Idling, Shooting),
        MovingShooting(Moving, Shooting),
    }

    pub trait PlayerState: Display {
        fn position(&self) -> Position;
        fn direction(&self) -> Direction;
        fn to_enum(self) -> State;
    }

    #[derive(Debug)]
    pub struct Idling {
        position: Position,
        direction: Direction,
    }
    impl PlayerState for Idling {
        fn position(&self) -> Position {
            self.position
        }
        fn direction(&self) -> Direction {
            self.direction
        }
        fn to_enum(self) -> State {
            State::Idling(self)
        }
    }
    impl Idling {
        fn idle(position: Position, direction: Direction) -> Self {
            let state = Idling {
                position,
                direction,
            };
            state.idle_self()
        }

        pub fn idle_self(self) -> Self {
            self
        }

        pub fn try_move(self) -> Result<Moving, Self> {
            let direction = get_player_move();
            if direction.is_zero() {
                Err(self)
            } else {
                Ok(Moving::r#move(
                    self.position(),
                    self.direction(),
                    PLAYER_SPEED.into(),
                ))
            }
        }

        pub fn try_shoot(self) -> Result<Shooting, Self> {
            if is_shooting() {
                Ok(Shooting::shoot(
                    self.position(),
                    self.direction(),
                    HOOK_EXTENDING_SPEED,
                ))
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

    impl PlayerState for Moving {
        fn position(&self) -> Position {
            self.position
        }

        fn direction(&self) -> Direction {
            self.direction
        }

        fn to_enum(self) -> State {
            State::Moving(self)
        }
    }
    impl Moving {
        fn velocity(&self) -> Magnitude {
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
            let position = Physics::calculate_new_position_from_speed(
                self.position(),
                self.velocity(),
                direction,
            );
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
        hook: HookStateEnum,
    }

    impl PlayerState for Shooting {
        fn position(&self) -> Position {
            self.position
        }

        fn direction(&self) -> Direction {
            self.direction
        }

        fn to_enum(self) -> State {
            State::Shooting(self)
        }
    }
    impl Shooting {
        fn velocity(&self) -> Magnitude {
            self.velocity
        }
        pub fn hook(&self) -> &HookStateEnum {
            &self.hook
        }
        fn shoot(position: Position, direction: Direction, velocity: Magnitude) -> Self {
            let hook = HookStateEnum::new(velocity, direction, position, HOOK_AMOUNT_LINKS);
            Shooting {
                position,
                velocity,
                direction,
                hook,
            }
        }

        pub fn action_update_hook(self, position: Position) -> Self {
            Shooting {
                hook: self.hook.invoke(position),
                ..self
            }
        }

        pub fn try_idle(self) -> Result<Either<Duality<Idling, Self>, Idling>, Self> {
            match self.hook {
                HookStateEnum::Extending(_) => Err(self),
                HookStateEnum::Contracting(_) => Ok(Left(Duality {
                    ying: Idling::idle(self.position(), self.direction()),
                    yang: self,
                })),
                HookStateEnum::End => Ok(Right(Idling::idle(self.position(), self.direction()))),
            }
        }
    }

    #[derive(Debug)]
    pub struct Duality<A, B>
    where
        A: PlayerState,
        B: PlayerState,
    {
        ying: A,
        yang: B,
    }

    impl<A> Duality<A, Shooting>
    where
        A: PlayerState,
    {
        pub fn action_update_hook(self) -> Either<Self, A> {
            let Duality { ying, yang } = self;
            let yang = yang.action_update_hook(ying.position());
            match yang.hook {
                HookStateEnum::Extending(_) => Left(Duality { ying, yang }),
                HookStateEnum::Contracting(_) => Left(Duality { ying, yang }),
                HookStateEnum::End => Right(ying),
            }
        }
    }

    impl Duality<Idling, Shooting> {
        pub fn move_or_idle(self) -> Either<Duality<Moving, Shooting>, Self> {
            let Duality { ying, yang } = self;
            match ying.try_move() {
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

    // impl From<Idling> for Moving {
    //     fn from(value: Idling) -> Self {
    //         todo!()
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
        A: PlayerState,
        B: PlayerState,
    {
        pub fn ying(&self) -> &A {
            &self.ying
        }
        pub fn yang(&self) -> &B {
            &self.yang
        }
    }

    impl<A, B> PlayerState for Duality<A, B>
    where
        A: PlayerState,
        B: PlayerState,
    {
        fn position(&self) -> Position {
            self.ying.position()
        }

        fn direction(&self) -> Direction {
            self.ying.direction()
        }

        fn to_enum(self) -> State {
            todo!()
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
                "{} {} {} \nHook - {}",
                name_of_type(self),
                self.position(),
                self.direction(),
                self.hook
            )
        }
    }
    impl<A, B> Display for Duality<A, B>
    where
        A: PlayerState,
        B: PlayerState,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "Duality\n\tYing - {}\n\tYang - {}",
                self.ying(),
                self.yang()
            )
        }
    }

    fn name_of_type<T>(val: &T) -> &'static str {
        type_name_of_val(val).split("::").last().unwrap()
    }
}
