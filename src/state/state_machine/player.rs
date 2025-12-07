//! Module handling player states

use super::*;
use action::*;

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

const IDLING_ACTIONS: [Action; 2] = [Action::Run, Action::Shoot];

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
impl Run for Idling {
    fn max_speed(&self) -> Magnitude {
        self.max_speed
    }

    fn into_ok_state(self, position: Position, direction: Direction, current_speed: Magnitude) -> Self {
        Self {
            position,
            direction,
            current_speed,
            ..self
        }
    }

    fn into_err_state(self, current_speed: Magnitude) -> Self {
        Self { current_speed, ..self }
    }
}
impl Shoot for Idling {
    fn extend_speed(&self) -> Magnitude {
        HOOK_EXTENDING_SPEED
    }
}

const GRAPLED_ACTIONS: [Action; 0] = [];
pub struct Grapled {}

const IDLING_EXTENDING: [Action; 2] = [Action::Extend, Action::StartContract];
const IDLING_CONTRACTING: [Action; 2] = [Action::Run, Action::Contract];
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
impl action::Run for ParentChild<Idling, Contracting> {
    fn max_speed(&self) -> Magnitude {
        self.parent.max_speed()
    }

    fn into_ok_state(self, position: Position, direction: Direction, current_speed: Magnitude) -> Self {
        Self {
            parent: Idling {
                position,
                direction,
                current_speed,
                ..self.parent
            },
            ..self
        }
    }

    fn into_err_state(self, current_speed: Magnitude) -> Self {
        Self {
            parent: Idling {
                current_speed,
                ..self.parent
            },
            ..self
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

    use crate::state::{self, state_machine::action::Contract};

    use super::*;

    pub(super) enum Action {
        Run,
        Shoot,
        Extend,
        Contract,
        StartContract,
        Dash,
        Graple,
    }

    pub(super) fn execute_actions(actions: Vec<Action>, executor: PlayerState) -> PlayerState {
        let mut state = executor;
        for action in actions {
            state = match action {
                Action::Run => try_run(state),
                Action::Shoot => try_shoot(state),
                Action::Extend => try_extend(state),
                Action::Contract => try_contract(state),
                Action::StartContract => try_start_contract(state),
                Action::Dash => todo!(),
                Action::Graple => todo!(),
            };
        }
        state
    }

    pub(super) trait Run: State {
        fn max_speed(&self) -> Magnitude;
        fn into_ok_state(self, position: Position, direction: Direction, current_speed: Magnitude) -> Self;
        fn into_err_state(self, current_speed: Magnitude) -> Self;
    }
    pub(super) fn try_run(runner: PlayerState) -> PlayerState {
        match runner {
            PlayerState::Idling(state) => run(state).into(),
            PlayerState::ParentChildIdlingContracting(state) => {
                run(state).map(pull_chain).map_or_else(|s| s.into(), |s| s.into())
            }
            _ => runner,
        }
    }
    pub(super) fn run<T: Run>(runner: T) -> Result<T, T> {
        let direction = get_player_move();
        if direction.is_zero() {
            Err(runner.into_err_state(Magnitude::zero()))
        } else {
            let current_speed = runner.max_speed();
            let position = Physics::calculate_new_position_from_speed(runner.position(), current_speed, direction);
            Ok(runner.into_ok_state(position, direction, current_speed))
        }
    }
    fn pull_chain(state: ParentChild<Idling, Contracting>) -> ParentChild<Idling, Contracting> {
        let position = state.position();
        ParentChild {
            parent: state.parent,
            child: state.child.update_tail_position(position),
        }
    }

    pub(super) trait Shoot: State {
        fn extend_speed(&self) -> Magnitude;
    }
    pub(super) fn try_shoot(shooter: PlayerState) -> PlayerState {
        match shooter {
            PlayerState::Idling(state) => shoot(state).into(),
            _ => shooter,
        }
    }
    pub(super) fn shoot<T: Shoot>(shooter: T) -> Result<ParentChild<T, Extending>, T> {
        if is_shooting() {
            Ok(ParentChild {
                child: hook::build(
                    shooter.extend_speed(),
                    shooter.direction(),
                    shooter.position(),
                    HOOK_AMOUNT_LINKS,
                ),
                parent: shooter,
            })
        } else {
            Err(shooter)
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
                    Ok(contracting) => ParentChild { parent, child: contracting }.into(),
                    Err(extending) => ParentChild { parent, child: extending }.into(),
                }
            }
            _ => state,
        }
    }
}
