use std::slice;

use crate::{state::state_machine::hook::action::execute_actions, util::Stack};

use super::*;

pub fn build(speed: Magnitude, direction: Direction, origin: Position, amount_of_links: usize) -> Extending {
    Extending::extend(speed, direction, origin, amount_of_links)
}

pub enum HookState {
    Extending(Extending),
    Contracting(Contracting),
}
impl State for HookState {
    type Output = Self;

    fn position(&self) -> Position {
        match self {
            HookState::Extending(extending) => extending.position(),
            HookState::Contracting(contracting) => contracting.position(),
        }
    }

    fn direction(&self) -> Direction {
        match self {
            HookState::Extending(extending) => extending.direction(),
            HookState::Contracting(contracting) => contracting.direction(),
        }
    }

    fn update(self) -> Self::Output {
        match self {
            HookState::Extending(extending) => extending.update(),
            HookState::Contracting(contracting) => contracting.update(),
        }
    }
}

const EXTENDING_ACTIONS: [action::Action; 2] = [action::Action::Extend, action::Action::StartContract];
#[derive(Debug)]
pub struct Extending {
    max_amount_links: usize,
    chain: Chain,
    extend_speed: Magnitude,
}
impl Extending {
    pub fn max_links(&self) -> usize {
        self.max_amount_links
    }
    pub fn extend_speed(&self) -> Magnitude {
        self.extend_speed
    }

    pub fn chain(&self) -> &Chain {
        &self.chain
    }

    fn extend(speed: Magnitude, direction: Direction, origin: Position, max_amount_links: usize) -> Self {
        let hook = Hook::new(origin, direction);
        Extending {
            max_amount_links,
            chain: Chain::new(hook, origin, HOOK_LINK_LENGTH),
            extend_speed: speed,
        }
    }
}
impl State for Extending {
    type Output = HookState;
    fn position(&self) -> Position {
        self.chain.head().position()
    }

    fn direction(&self) -> Direction {
        self.chain.head_direction()
    }

    fn update(self) -> HookState {
        execute_actions(EXTENDING_ACTIONS.into(), self.into())
    }
}
impl action::Extend for Extending {
    fn chain(self) -> Chain {
        self.chain
    }

    fn extend_speed(&self) -> Magnitude {
        self.extend_speed
    }

    fn max_amount_links(&self) -> usize {
        self.max_amount_links
    }

    fn into_state(chain: Chain, max_amount_links: usize, extend_speed: Magnitude) -> Self {
        Extending { max_amount_links, chain, extend_speed }
    }
}
impl action::StartContract for Extending {
    fn max_amount_links(&self) -> usize {
        self.max_amount_links
    }

    fn amount_links(&self) -> usize {
        self.chain.count()
    }

    fn into_state(self) -> Contracting {
        Contracting { chain: self.chain, contract_speed: HOOK_CONTRACTING_SPEED }
    }
}

const CONTRACTING_ACTIONS: [action::Action; 0] = [];
#[derive(Debug)]
pub struct Contracting {
    chain: Chain,
    contract_speed: Magnitude,
}

impl Contracting {
    pub fn contract_speed(&self) -> Magnitude {
        self.contract_speed
    }
    pub fn chain(&self) -> &Chain {
        &self.chain
    }
    pub fn into_chain(self) -> Chain {
        self.chain
    }
    fn contract(mut chain: Chain, speed: Magnitude) -> Self {
        let position = chain.tail().position();
        Contracting { chain, contract_speed: speed }
    }

    pub fn contract_self(self, tail_position: Position) -> Contracting {
        let Self { chain, contract_speed: speed } = self;
        let distance = distance(chain.tail(), &tail_position);
        let chain = chain
            .update_tail_position(tail_position)
            .move_links_toward_tail(Magnitude::from(distance) + speed)
            .maybe_remove_link();
        Contracting { chain, contract_speed: speed }
    }

    pub fn update_tail_position(self, tail_position: Position) -> Contracting {
        let Self { chain, contract_speed: speed } = self;
        let distance = distance(chain.tail(), &tail_position);
        let chain = chain
            .update_tail_position(tail_position)
            .move_links_toward_tail(Magnitude::from(distance))
            .maybe_remove_link();
        Contracting { chain, contract_speed: speed }
    }
}
impl State for Contracting {
    type Output = HookState;
    fn position(&self) -> Position {
        self.chain.head().position()
    }

    fn direction(&self) -> Direction {
        self.chain.head_direction()
    }

    fn update(self) -> Self::Output {
        execute_actions(CONTRACTING_ACTIONS.into(), self.into())
    }
}
impl action::Contract for Contracting {
    fn chain(self) -> Chain {
        self.chain
    }
    fn contract_speed(&self) -> Magnitude {
        self.contract_speed
    }
    fn into_state(chain: Chain, contract_speed: Magnitude) -> Self {
        Contracting { chain, contract_speed }
    }
}

#[derive(Debug)]
pub struct Hook {
    direction: Direction,
    link: Link,
}

impl Hook {
    fn new(position: Position, direction: Direction) -> Self {
        Self {
            direction,
            link: Link::new(position),
        }
    }
    pub fn position(&self) -> Position {
        self.link.position
    }
    fn set_position(&mut self, position: Position) {
        self.link.set_position(position);
    }
    pub fn direction(&self) -> Direction {
        self.direction
    }
}

#[derive(Debug)]
pub struct Chain {
    chain: Stack<Link, Hook, Tail>,
    link_length: f32,
}

impl Chain {
    fn new(hook: Hook, tail_position: Position, link_length: f32) -> Self {
        let stack = Stack::new(hook, Tail::new(tail_position), |left, right| {
            Link::clamp_to_length_mut(left, right, HOOK_LINK_LENGTH)
        });
        Self {
            chain: stack,
            link_length,
        }
    }

    fn length_straight_line(&self) -> f32 {
        self.first().distance(self.last())
    }

    fn length_of_links(&self) -> f32 {
        self.chain
            .iter()
            .skip(1)
            .fold((0.0, self.first()), |acc, link| {
                (acc.0 + link.clone().distance(acc.1), link)
            })
            .0
    }
    pub fn head(&self) -> &Hook {
        self.chain.head()
    }
    pub fn head_direction(&self) -> Direction {
        self.chain.first().direction(self.chain.head())
    }
    pub fn tail(&self) -> &Tail {
        self.chain.tail()
    }

    pub fn first(&self) -> &Link {
        self.chain.first()
    }

    pub fn last(&self) -> &Link {
        self.chain.last()
    }

    pub fn pop(&mut self) -> Link {
        self.chain.pop()
    }

    pub fn push<T: AsRef<Link>>(&mut self, link: T) {
        self.chain.push(link.as_ref().clone());
    }

    pub fn set_head(&mut self, head: Hook) {
        self.chain.set_head(head);
    }

    pub fn set_tail(&mut self, tail: Tail) {
        self.chain.set_tail(tail);
    }

    pub fn iter(&self) -> slice::Iter<'_, Link> {
        self.chain.iter()
    }

    pub fn chain(&self) -> &Stack<Link, Hook, Tail> {
        &self.chain
    }

    pub fn is_empty(&self) -> bool {
        self.chain.is_empty()
    }

    pub fn into_iter(self) -> <std::vec::Vec<Link> as std::iter::IntoIterator>::IntoIter {
        self.chain.into_iter()
    }

    fn maybe_add_link(mut self) -> Self {
        if distance(self.tail(), self.last()) > self.link_length {
            self.chain.push_tail();
        }
        self
    }

    fn maybe_remove_link(mut self) -> Self {
        if distance(self.tail(), self.last()) < HOOK_LINK_DIST_TREAT_AS_ZERO {
            self.chain.pop();
        }
        self
    }

    fn update_head_position(mut self, head_position: Position) -> Chain {
        self.chain.head_mut().set_position(head_position);
        self
    }

    fn update_tail_position(mut self, tail_position: Position) -> Chain {
        self.chain.tail_mut().set_position(tail_position);
        self
    }

    fn move_links_toward_head(mut self) -> Self {
        self.chain = self.chain.fold_into_self();
        self
    }

    fn move_links_toward_tail(mut self, length: Magnitude) -> Chain {
        let Chain { mut chain, link_length } = self;
        let last = chain.pop().move_towards(chain.tail(), length.value());
        let chain = chain.rfold_into_self(&[last]);
        Chain { chain, link_length }
    }

    fn count(&self) -> usize {
        self.chain.len()
    }
}

pub fn link_relationship(left: Link, right: Link, link_length: f32) {
    Link::clamp_to_length(left, &right, link_length);
}

#[derive(Debug, Clone)]
pub struct Link {
    position: Position,
}
impl Link {
    fn new(position: Position) -> Self {
        Self { position }
    }

    pub fn position(&self) -> Position {
        self.position
    }
    pub fn x(&self) -> f32 {
        self.position().x()
    }
    pub fn y(&self) -> f32 {
        self.position().y()
    }
    fn set_position(&mut self, position: Position) {
        self.position = position
    }

    fn move_by_vector(self, vec: Vec2) -> Self {
        Link {
            position: Position::from_vec(self.position().value() + vec),
        }
    }

    /// Moves a link towards another link
    /// The distance travelled is clamped to the distance between them,
    /// so the link cannot travel past the other link
    fn move_towards<T: AsRef<Link>>(self, link: &T, distance: f32) -> Self {
        let max_dist = self.distance(link.as_ref());
        Link {
            position: self
                .position()
                .move_towards(link.as_ref().position(), distance.min(max_dist)),
        }
    }

    fn clamp_to_length(self, link: &Link, link_length: f32) -> Self {
        let current = self.distance(link);
        let diff = current - link_length;
        Link {
            position: self.position().move_towards(link.position(), diff.max(0.0)),
        }
    }

    fn clamp_to_length_mut(&mut self, link: &Link, link_length: f32) {
        let current = self.distance(link);
        let diff = current - link_length;
        self.set_position(self.position().move_towards(link.position(), diff.max(0.0)));
    }

    fn move_link_projection(self, prev: Link, next: &Link, factor: f32) -> Self {
        let position = project_c_onto_ab(prev.position(), next.position(), self.position(), factor);
        Link { position }
    }

    pub fn distance<T: AsRef<Position>>(&self, position: T) -> f32 {
        self.position().distance(position.as_ref())
    }
    pub fn direction<T: AsRef<Position>>(&self, position: T) -> Direction {
        self.position().direction_to(*position.as_ref())
    }
}

fn distance<T: AsRef<Position>, U: AsRef<Position>>(from: &T, to: &U) -> f32 {
    from.as_ref().distance(to.as_ref())
}

fn project_c_onto_ab(a_p: Position, b_p: Position, c_p: Position, factor: f32) -> Position {
    let a = a_p.value();
    let b = b_p.value();
    let c = c_p.value();
    let v_ab = b - a;
    let v_ac = c - a;
    let v_ad = v_ac.project_onto(v_ab);
    let d = v_ad + a;
    Position::from_vec(c + (d - c) * factor)
}

fn _hook_direction_function(position_hook: Position, goal: Position) -> Direction {
    match _hook_path_function(position_hook, goal) {
        Left(f) => {
            let dx = 30.0;
            let dx = if position_hook.x() > goal.x() { -dx } else { dx };
            let x_new = position_hook.x() + dx;
            Direction::a_to_b(position_hook, Position::new(x_new, f(x_new)))
        }
        Right(f) => Direction::a_to_b(position_hook, goal),
    }
}
pub fn _hook_path_function(
    position_hook: Position,
    position_goal: Position,
) -> Either<impl Fn(f32) -> f32, impl Fn(f32) -> f32> {
    let Vec2 { x: x0, y: y0 } = position_hook.value();
    let Vec2 { x, y } = position_goal.value();

    if x == x0 {
        Right(move |dy: f32| y0 - dy)
    } else {
        Left(_hook_path_function_non_zero(x0, y0, x, y))
    }
}

fn _hook_path_function_non_zero(x0: f32, y0: f32, x: f32, y: f32) -> impl Fn(f32) -> f32 {
    let a = (y - y0) / (x - x0).powi(2);
    move |x: f32| a * (x - x0).powi(2) + y0
}

#[derive(Debug)]
pub struct Tail(pub Link);
#[derive(Debug)]
pub struct Head(Link);
impl Tail {
    fn new(position: Position) -> Self {
        Tail(Link { position })
    }
    pub fn position(&self) -> Position {
        self.0.position()
    }
    pub fn set_position(&mut self, position: Position) {
        self.0.set_position(position);
    }
}
impl Head {
    fn new(position: Position) -> Self {
        Head(Link { position })
    }
    pub fn position(&self) -> Position {
        self.0.position()
    }
}
impl AsRef<Link> for Link {
    fn as_ref(&self) -> &Link {
        self
    }
}
impl AsRef<Link> for Tail {
    fn as_ref(&self) -> &Link {
        &self.0
    }
}
impl AsRef<Link> for Head {
    fn as_ref(&self) -> &Link {
        &self.0
    }
}
impl AsMut<Link> for Head {
    fn as_mut(&mut self) -> &mut Link {
        &mut self.0
    }
}
impl AsRef<Position> for Position {
    fn as_ref(&self) -> &Position {
        self
    }
}
impl AsRef<Position> for Link {
    fn as_ref(&self) -> &Position {
        &self.position
    }
}
impl AsRef<Position> for Head {
    fn as_ref(&self) -> &Position {
        AsRef::<Link>::as_ref(self).as_ref()
    }
}
impl AsRef<Position> for Tail {
    fn as_ref(&self) -> &Position {
        AsRef::<Link>::as_ref(self).as_ref()
    }
}
impl AsMut<Link> for Hook {
    fn as_mut(&mut self) -> &mut Link {
        &mut self.link
    }
}

impl AsRef<Link> for Hook {
    fn as_ref(&self) -> &Link {
        &self.link
    }
}
impl AsRef<Position> for Hook {
    fn as_ref(&self) -> &Position {
        AsRef::<Link>::as_ref(self).as_ref()
    }
}
//* Std trait implementations */
impl Display for Extending {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Extending {}, {}, links: {}, length: {}",
            self.chain.head().position(),
            self.extend_speed(),
            self.chain().chain.len(),
            self.chain().length_of_links()
        )
    }
}
impl Display for Contracting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Contracting {}, {}, {}, links: {}, length: {}",
            self.chain.head().position(),
            self.contract_speed(),
            self.chain.head_direction(),
            self.chain().chain.len(),
            self.chain().length_of_links()
        )
    }
}

impl From<Extending> for HookState {
    fn from(value: Extending) -> Self {
        HookState::Extending(value)
    }
}
impl From<Contracting> for HookState {
    fn from(value: Contracting) -> Self {
        HookState::Contracting(value)
    }
}
impl<T, U> From<Result<T, U>> for HookState
where
    T: State + Into<HookState>,
    U: State + Into<HookState>,
{
    fn from(value: Result<T, U>) -> Self {
        value.map_or_else(|s| s.into(), |s| s.into())
    }
}

pub(super) mod action {
    use super::*;

    pub enum Action {
        Extend,
        StartContract,
    }

    pub fn execute_actions(actions: Vec<Action>, executor: HookState) -> HookState {
        let mut state = executor;
        for action in actions {
            state = match action {
                Action::Extend => try_extend(state),
                Action::StartContract => try_start_contract(state),
            };
        }
        state
    }

    pub trait Extend: State {
        fn chain(self) -> Chain;
        fn extend_speed(&self) -> Magnitude;
        fn max_amount_links(&self) -> usize;
        fn into_state(chain: Chain, max_amount_links: usize, extend_speed: Magnitude) -> Self;
    }
    pub fn try_extend(state: HookState) -> HookState {
        match state {
            HookState::Extending(state) => extend(state).into(),
            _ => state,
        }
    }
    pub fn extend<T: Extend>(state: T) -> T {
        let max_amount_links = state.max_amount_links();
        let extend_speed = state.extend_speed();
        let chain = state.chain();
        let position = calculate_new_head_position(&chain, extend_speed);
        let chain = chain
            .update_head_position(position)
            .move_links_toward_head()
            .maybe_add_link();
        Extend::into_state(chain, max_amount_links, extend_speed)
    }

    pub trait Contract: State {
        fn chain(self) -> Chain;
        fn contract_speed(&self) -> Magnitude;
        fn into_state(chain: Chain, contract_speed: Magnitude) -> Self;
    }
    pub fn contract<T: Contract>(state: T) -> Option<T> {
        let speed = state.contract_speed();
        let chain = state.chain();
        let chain = chain
            .move_links_toward_tail(speed)
            .maybe_remove_link();
        if chain.is_empty() {
            None
        } else {
            Some(Contract::into_state(chain, speed))
        }
    }

    pub trait StartContract: State {
        fn max_amount_links(&self) -> usize;
        fn amount_links(&self) -> usize;
        fn into_state(self) -> Contracting;
    }
    pub fn try_start_contract(state: HookState) -> HookState {
        match state {
            HookState::Extending(extending) => start_contract(extending).into(),
            _ => state,
        }
    }
    pub fn start_contract<T: StartContract>(state: T) -> Result<Contracting, T> {
        let max_amount_links = state.max_amount_links();
        let amount_links = state.amount_links();
        if amount_links < max_amount_links {
            Err(state)
        } else {
            Ok(state.into_state())
        }
    }

    fn calculate_new_head_position(chain: &Chain, speed: Magnitude) -> Position {
        Physics::calculate_new_position_from_speed(
            chain.head().position(),
            speed,
            chain.head().direction(),
        )
    }
}
