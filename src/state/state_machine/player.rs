//! Module handling player states

use crate::state::state_machine::action::{Action, Execute};

use super::*;
use action::*;

// Workaround for Tracking Issue "More Qualified Paths": https://github.com/rust-lang/rust/issues/86935#issuecomment-1146670057
type Type<T> = T;

pub fn build(position: Position, direction: Direction, speed: Magnitude) -> Idling {
    Idling::idle(position, direction, speed)
}

#[derive(Debug)]
pub enum PlayerState {
    Idling(Idling),
    ParentChildIdlingExtending(ParentChild<Idling, Extending>),
    ParentChildIdlingContracting(ParentChild<Idling, Contracting>),
}
impl State for PlayerState {
    type Output = Self;

    fn position(&self) -> Position {
        match self {
            PlayerState::Idling(idling) => idling.position(),
            PlayerState::ParentChildIdlingExtending(parent_child) => parent_child.position(),
            PlayerState::ParentChildIdlingContracting(parent_child) => parent_child.position(),
        }
    }

    fn direction(&self) -> Direction {
        todo!()
    }

    fn update(self) -> Self::Output {
        todo!()
    }
}

#[derive(Debug)]
pub enum Relation {
    ParentChild,
    Siblings,
}

const IDLING_ACTIONS: [ActionType; 2] = [ActionType::Run, ActionType::Shoot];

#[derive(Debug, Default)]
pub struct Idling {
    position: Position,
    direction: Direction,
    max_speed: Magnitude,
    current_speed: Magnitude,
    // actions: Vec<IdlingAction>
}
impl State for Idling {
    type Output = PlayerState;
    fn position(&self) -> Position {
        self.position
    }
    fn direction(&self) -> Direction {
        self.direction
    }
    fn update(self) -> PlayerState {
        execute_actions(IDLING_ACTIONS.into(), self.into())
    }
}
impl Idling {
    fn idle(position: Position, direction: Direction, speed: Magnitude) -> Self {
        Idling {
            position,
            direction,
            max_speed: speed,
            ..Default::default()
        }
    }
    // fn _update(self) -> PlayerState {
    //     run(self)
    //         .and_then(shoot)
    //         .or_else(shoot)
    //         .map_or_else(PlayerState::from, PlayerState::from)
    // }
}

impl Execute<Run> for Idling {
    fn prepare(&self) -> Run {
        Run {
            position: self.position,
            max_speed: self.max_speed,
        }
    }
    type OkState = Idling;
    fn move_to_ok_state(self, output: <Run as Action>::OkOutput) -> Self::OkState {
        let Type::<<Run as Action>::OkOutput> {
            position,
            direction,
            current_speed,
        } = output;
        Self::OkState {
            position,
            direction,
            max_speed: self.max_speed,
            current_speed,
        }
    }
    type ErrState = Idling;
    fn move_to_err_state(self, output: <Run as Action>::ErrOutput) -> Self::ErrState {
        let Self {
            position,
            direction,
            max_speed,
            ..
        } = self;
        let Type::<<Run as Action>::ErrOutput> { current_speed } = output;
        Self::OkState {
            position,
            direction,
            max_speed,
            current_speed,
        }
    }
}
impl Execute<Shoot> for Idling {
    fn prepare(&self) -> Shoot {
        Shoot
    }
    type OkState = ParentChild<Idling, Extending>;
    fn move_to_ok_state(self, _: <Shoot as Action>::OkOutput) -> Self::OkState {
        ParentChild {
            child: hook::build(
                HOOK_EXTENDING_SPEED,
                self.direction(),
                self.position(),
                HOOK_AMOUNT_LINKS,
            ),
            parent: self,
        }
    }
    type ErrState = Self;
    fn move_to_err_state(self, _: <Shoot as Action>::ErrOutput) -> Self::ErrState {
        self
    }
}

const GRAPLED_ACTIONS: [ActionType; 0] = [];
pub struct Grapled {}

const IDLING_EXTENDING: [ActionType; 2] = [ActionType::Extend, ActionType::StartContract];
const IDLING_CONTRACTING: [ActionType; 2] = [ActionType::Run, ActionType::Contract];
#[derive(Debug)]
pub struct ParentChild<A, B>
where
    A: State,
{
    parent: A,
    child: B,
}
impl<A: State, B> ParentChild<A, B> {
    pub fn parent(&self) -> &A {
        &self.parent
    }
    pub fn child(&self) -> &B {
        &self.child
    }
}
impl State for ParentChild<Idling, Extending> {
    type Output = PlayerState;
    fn position(&self) -> Position {
        self.parent.position()
    }

    fn direction(&self) -> Direction {
        self.parent.direction()
    }

    fn update(self) -> PlayerState {
        execute_actions(IDLING_EXTENDING.into(), self.into())
    }
}

impl State for ParentChild<Idling, Contracting> {
    type Output = PlayerState;
    fn position(&self) -> Position {
        self.parent.position()
    }
    fn direction(&self) -> Direction {
        self.parent.direction()
    }
    fn update(self) -> PlayerState {
        execute_actions(IDLING_CONTRACTING.into(), self.into())
    }
}
impl Execute<Run> for ParentChild<Idling, Contracting> {
    fn prepare(&self) -> Run {
        Run {
            position: self.parent.position,
            max_speed: self.parent.max_speed,
        }
    }
    type OkState = Self;
    fn move_to_ok_state(self, output: <Run as Action>::OkOutput) -> Self::OkState {
        let Self { parent, child } = self;
        let Type::<<Run as Action>::OkOutput> {
            position,
            direction,
            current_speed,
        } = output;
        Self::OkState {
            parent: Idling {
                position,
                direction,
                max_speed: parent.max_speed,
                current_speed,
            },
            child: child.update_tail_position(position),
        }
    }
    type ErrState = Self;
    fn move_to_err_state(self, output: <Run as Action>::ErrOutput) -> Self::ErrState {
        let Self { parent: Idling { position, direction, max_speed, ..}, child } = self;
        let Type::<<Run as Action>::ErrOutput> { current_speed } = output;
        Self::OkState {
            parent: Idling {
                position,
                direction,
                max_speed,
                current_speed,
            },
            child,
        }
    }
}

#[derive(Debug)]
pub struct Duality<A, B>
where
    A: State,
    B: State,
{
    ying: A,
    yang: B,
}
impl<A, B> Duality<A, B>
where
    A: State,
    B: State,
{
    pub fn ying(&self) -> &A {
        &self.ying
    }
    pub fn yang(&self) -> &B {
        &self.yang
    }
}

//** Std trait implementations */
impl From<Idling> for PlayerState {
    fn from(value: Idling) -> Self {
        PlayerState::Idling(value)
    }
}
impl From<ParentChild<Idling, Extending>> for PlayerState {
    fn from(value: ParentChild<Idling, Extending>) -> Self {
        PlayerState::ParentChildIdlingExtending(value)
    }
}
impl From<ParentChild<Idling, Contracting>> for PlayerState {
    fn from(value: ParentChild<Idling, Contracting>) -> Self {
        PlayerState::ParentChildIdlingContracting(value)
    }
}
impl<T, U> From<Result<T, U>> for PlayerState
where
    T: State + Into<PlayerState>,
    U: State + Into<PlayerState>,
{
    fn from(value: Result<T, U>) -> Self {
        value.map_or_else(|s| s.into(), |s| s.into())
    }
}
impl From<Result<Idling, Idling>> for Idling {
    fn from(value: Result<Idling, Idling>) -> Self {
        value.unwrap_or_else(|s| s)
    }
}
impl Display for Idling {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", name_of_type(self), self.position())
    }
}
impl<A, B> Display for Duality<A, B>
where
    A: State + Display,
    B: State + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Duality\n\tYing {}\n\tYang {}", self.ying(), self.yang())
    }
}
impl<A, B> Display for ParentChild<A, B>
where
    A: State + Display,
    B: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParentChild\n\tParent {}\n\tChild {}", self.parent(), self.child())
    }
}

mod action {
    //! Action traits are used as requirements to the state executing the action, to make sure that needed data can be retrieved. The traits will usually have `State` as supertrait because position and direction are often (if not always) needed.
    //! Action traits does not themselves include a method for executing the action. This should be handled by a blanket implementation in a (free) function (e.g. [run] is a blanket implementation over the [Run] trait)

    use crate::state::state_machine::action::{Action, Execute};

    use super::*;

    pub(super) enum ActionType {
        Run,
        Shoot,
        Extend,
        Contract,
        StartContract,
        Dash,
        Graple,
    }

    pub(super) fn execute_actions(actions: Vec<ActionType>, executor: PlayerState) -> PlayerState {
        let mut state = executor;
        for action in actions {
            state = match action {
                ActionType::Run => try_run(state),
                ActionType::Shoot => try_shoot(state),
                ActionType::Extend => try_extend(state),
                ActionType::Contract => try_contract(state),
                ActionType::StartContract => try_start_contract(state),
                ActionType::Dash => todo!(),
                ActionType::Graple => todo!(),
            };
        }
        state
    }
    pub(super) fn try_shoot(state: PlayerState) -> PlayerState {
        match state {
            PlayerState::Idling(state) => Execute::<Shoot>::execute(state).into(),
            _ => state,
        }
    }
    pub(super) fn try_run(state: PlayerState) -> PlayerState {
        match state {
            PlayerState::Idling(state) => Execute::<Run>::execute(state).into(),
            PlayerState::ParentChildIdlingContracting(state) => Execute::<Run>::execute(state).into(),
            _ => state,
        }
    }

    pub(super) fn try_extend(state: PlayerState) -> PlayerState {
        match state {
            PlayerState::ParentChildIdlingExtending(state) => {
                let ParentChild { parent, child } = state;
                ParentChild {
                    parent,
                    child: hook::action::extend(child),
                }
                .into()
            }
            _ => state,
        }
    }

    pub(super) fn try_contract(state: PlayerState) -> PlayerState {
        match state {
            PlayerState::ParentChildIdlingContracting(state) => {
                let ParentChild { parent, child } = state;
                if let Some(child) = hook::action::contract(child) {
                    ParentChild { parent, child }.into()
                } else {
                    parent.into()
                }
            }
            _ => state,
        }
    }

    pub(super) fn try_start_contract(state: PlayerState) -> PlayerState {
        match state {
            PlayerState::ParentChildIdlingExtending(state) => {
                let ParentChild { parent, child } = state;
                match hook::action::start_contract(child) {
                    Ok(contracting) => ParentChild {
                        parent,
                        child: contracting,
                    }
                    .into(),
                    Err(extending) => ParentChild {
                        parent,
                        child: extending,
                    }
                    .into(),
                }
            }
            _ => state,
        }
    }

    pub struct Run {
        pub position: Position,
        pub max_speed: Magnitude,
    }
    pub struct OkRun {
        pub position: Position,
        pub direction: Direction,
        pub current_speed: Magnitude,
    }
    pub struct ErrRun {
        pub current_speed: Magnitude,
    }
    impl Action for Run {
        type OkOutput = OkRun;
        type ErrOutput = ErrRun;
        fn execute<T: Execute<Self>>(self, state: T) -> Result<T::OkState, T::ErrState> {
            let direction = get_player_move();
            if direction.is_zero() {
                Err(state.move_to_err_state(ErrRun {
                    current_speed: Magnitude::zero(),
                }))
            } else {
                let current_speed = self.max_speed;
                let position = Physics::calculate_new_position_from_speed(self.position, current_speed, direction);
                Ok(state.move_to_ok_state(OkRun {
                    position,
                    direction,
                    current_speed,
                }))
            }
        }
    }

    pub struct Shoot;
    impl Action for Shoot {
        type OkOutput = ();
        type ErrOutput = ();
        fn execute<T: Execute<Self>>(self, state: T) -> Result<T::OkState, T::ErrState> {
            if is_shooting() {
                Ok(state.move_to_ok_state(()))
            } else {
                Err(state.move_to_err_state(()))
            }
        }
    }
}
