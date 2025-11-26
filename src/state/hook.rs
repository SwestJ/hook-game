//* ------------------------------------------------------------------------*/
//* ------------------------------------------------------------------------*/
//* ----------------------------------HOOK----------------------------------*/
//* ------------------------------------------------------------------------*/
//* ------------------------------------------------------------------------*/
pub const HOOK_AMOUNT_LINKS: usize = 40;
pub const HOOK_LINK_LENGTH: f32 = 20.0;
pub const HOOK_EXTENDING_SPEED: Magnitude = Magnitude::new_const(5.5);
pub const HOOK_CONTRACTING_SPEED: Magnitude = Magnitude::new_const(2.5);
// pub const HOOK_CONTRACTING_SPEED: Magnitude = Magnitude::new_const(0.0);
pub const HOOK_CONTRACTING_HIST_LENGTH: usize = 50;
pub const HOOK_DIST_END_CONTRACT: f32 = 10.0;
pub const HOOK_CHAIN_PROJECTION_FACTOR: f32 = 0.1;
pub const HOOK_LINK_DIST_TREAT_AS_ZERO: f32 = 1.0;

use std::fmt::Display;

use crate::model::*;
use crate::state::hook::state_hook::*;

#[derive(Debug)]
pub enum HookStateEnum {
    Extending(HookStateMachine<Extending>),
    Contracting(HookStateMachine<Contracting>),
    End,
}
impl Display for HookStateEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HookStateEnum::Extending(state) => write!(f, "Hook - {}", state),
            HookStateEnum::Contracting(state) => write!(f, "{}", state),
            HookStateEnum::End => write!(f, "Hook - End"),
        }
    }
}
impl HookStateEnum {
    pub fn new(
        speed: Magnitude,
        direction: Direction,
        origin: Position,
        amount_of_links: usize,
    ) -> Self {
        Self::Extending(HookStateMachine {
            state: state_hook::build_hook(speed, direction, origin, amount_of_links),
        })
    }

    pub fn invoke(self, spawner_position: Position) -> Self {
        match self {
            HookStateEnum::Extending(hook_state_machine) => {
                hook_state_machine.invoke(spawner_position)
            }
            HookStateEnum::Contracting(hook_state_machine) => {
                hook_state_machine.invoke(spawner_position)
            }
            HookStateEnum::End => self,
        }
    }
}

// By having some struct wrap the states, the underlying states can be kept simple,
// focusing on the action logic and storing the state in its fields.
// The wrapper struct will handle choosing an action based on the current state.
// If a state was its own controller, how would e.g. Extending handle that Contract needs the players current position?
//  It would need to always take the position as input. It makes more sense for the wrapper to take this input,
//  and disregard it if in Expanding state
#[derive(Debug)]
pub struct HookStateMachine<T: HookState> {
    state: T,
}
impl<T> HookStateMachine<T>
where
    T: HookState,
{
    pub fn state(&self) -> &T {
        &self.state
    }
}
impl HookStateMachine<Extending> {
    fn invoke(self, spawner_position: Position) -> HookStateEnum {
        let extending = self.state.extend_self();
        match extending.try_contract(spawner_position) {
            Ok(contracting) => HookStateEnum::Contracting(HookStateMachine { state: contracting }),
            Err(extending) => HookStateEnum::Extending(HookStateMachine { state: extending }),
        }
    }
}
impl HookStateMachine<Contracting> {
    fn invoke(self, spawner_position: Position) -> HookStateEnum {
        let contracting = self.state.contract_self(spawner_position);
        match contracting.try_end() {
            Ok(_) => HookStateEnum::End,
            Err(contracting) => HookStateEnum::Contracting(HookStateMachine { state: contracting }),
        }
    }
}
impl<T> HookState for HookStateMachine<T> where T: HookState {}

impl<T> Display for HookStateMachine<T>
where
    T: HookState,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.state())
    }
}

pub mod state_hook {
    use std::{
        collections::{VecDeque, vec_deque},
        fmt::Display,
        slice, vec,
    };

    use either::Either::{self, Left, Right};
    use iter_num_tools::lin_space;
    use macroquad::math::Vec2;

    use crate::{
        model::{Direction, Magnitude, Physics, Position},
        state::hook::*,
    };

    pub fn build_hook(
        speed: Magnitude,
        direction: Direction,
        origin: Position,
        amount_of_links: usize,
    ) -> Extending {
        Extending::extend(speed, direction, origin, amount_of_links)
    }

    pub trait HookState: Display {}

    #[derive(Debug)]
    pub struct Extending {
        amount_of_links: usize,
        chain: Chain,
        speed: Magnitude,
    }
    impl HookState for Extending {}
    impl Extending {
        pub fn amount_of_links(&self) -> usize {
            self.amount_of_links
        }
        pub fn speed(&self) -> Magnitude {
            self.speed
        }

        pub fn chain(&self) -> &Chain {
            &self.chain
        }

        fn extend(
            speed: Magnitude,
            direction: Direction,
            origin: Position,
            amount_of_links: usize,
        ) -> Self {
            let hook = Hook::new(origin, direction);
            let state = Extending {
                amount_of_links,
                chain: Chain::new(hook, origin, HOOK_LINK_LENGTH),
                speed,
            };
            state.extend_self()
        }

        pub fn extend_self(self) -> Self {
            let position = self.calculate_new_head_position();
            let chain = self
                .chain
                .update_head_position(position)
                .move_links_toward_head()
                .maybe_add_link();
            Extending { chain, ..self }
        }

        fn contract(self) -> Contracting {
            Contracting::contract(self.chain, HOOK_CONTRACTING_SPEED)
        }

        pub fn try_contract(self, goal: Position) -> Result<Contracting, Self> {
            if self.chain().count() < self.amount_of_links() {
                Err(self)
            } else {
                Ok(self.contract())
            }
        }

        fn calculate_new_head_position(&self) -> Position {
            Physics::calculate_new_position_from_speed(
                self.chain.head().position(),
                self.speed,
                self.chain.head().direction(),
            )
        }
    }

    #[derive(Debug)]
    pub struct Contracting {
        chain: Chain,
        speed: Magnitude,
    }

    impl HookState for Contracting {}
    impl Contracting {
        pub fn speed(&self) -> Magnitude {
            self.speed
        }
        pub fn chain(&self) -> &Chain {
            &self.chain
        }
        fn contract(mut chain: Chain, speed: Magnitude) -> Self {
            let position = chain.tail().position();
            let state = Contracting { chain, speed };
            state.contract_self(position)
        }

        pub fn contract_self(self, tail_position: Position) -> Contracting {
            let Self { chain, speed } = self;
            let distance = distance(chain.tail(), &tail_position);
            let chain = chain
                .update_tail_position(tail_position)
                .move_links_toward_tail(Magnitude::from(distance) + speed)
                .maybe_remove_link();
            Contracting { chain, speed }
        }

        pub fn try_end(self) -> Result<(), Self> {
            let distance = self.chain.length_straight_line();
            if distance < HOOK_DIST_END_CONTRACT {
                Ok(())
            } else {
                Err(self)
            }
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
        fn direction_towards_next_link(current: Position, next: Position) -> Direction {
            Direction::a_to_b(current, next)
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

        /// Updates head position and returns the distance travelled
        pub fn set_head(&mut self, head: Hook) {
            self.chain.set_head(head);
        }

        /// Updates head position and returns the distance travelled
        pub fn set_tail(&mut self, tail: Tail) {
            self.chain.set_tail(tail);
        }

        pub fn iter(&self) -> slice::Iter<'_, Link> {
            self.chain.iter()
        }

        pub fn chain(&self) -> &Stack<Link, Hook, Tail> {
            &self.chain
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
            let Chain {
                mut chain,
                link_length,
            } = self;
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
                position: Position::from(self.position().value() + vec),
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
            let position =
                project_c_onto_ab(prev.position(), next.position(), self.position(), factor);
            Link { position }
        }

        fn distance<T: AsRef<Position>>(&self, position: T) -> f32 {
            self.position().distance(position.as_ref())
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
        (c + (d - c) * factor).into()
    }

    fn _evenly_spaced_positions(
        position: Position,
        goal: Position,
        amount: usize,
    ) -> VecDeque<Position> {
        let x_space = lin_space(position.x()..goal.x(), amount);
        let y_space = lin_space(position.y()..goal.y(), amount);
        x_space
            .zip(y_space)
            .fold(VecDeque::<Position>::new(), |mut acc, p| {
                acc.push_back(Position::new(p.0, p.1));
                acc
            })
    }

    fn _hook_direction_function(position_hook: Position, goal: Position) -> Direction {
        match _hook_path_function(position_hook, goal) {
            Left(f) => {
                let dx = 30.0;
                let dx = if position_hook.x() > goal.x() {
                    -dx
                } else {
                    dx
                };
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
                self.speed(),
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
                self.speed(),
                self.chain.head().direction(),
                self.chain().chain.len(),
                self.chain().length_of_links()
            )
        }
    }

    // #[cfg(test)]
    // mod tests {
    //     use super::*;

    //     #[test]
    //     fn test_move_chain_empty() {
    //         let vec = vec![];
    //         let vec_moved = move_chain_inner(vec, 1.0);

    //         let expect = vec![];
    //         assert_eq!(vec_moved, VecDeque::from(expect));
    //     }

    //     #[test]
    //     fn test_move_chain_one_item() {
    //         let vec = vec![Position::new(1.0, 1.0)];
    //         let vec_moved = move_chain_inner(vec, 1.0);

    //         let expect = vec![Position::new(1.0, 1.0)];
    //         assert_eq!(vec_moved, VecDeque::from(expect));
    //     }

    //     #[test]
    //     fn test_move_chain_three_items() {
    //         let vec: Vec<Position> = vec![
    //             (1, 0).into(),(0, 0).into(), (0, 1).into()
    //             ];
    //         let vec_moved = move_chain_inner(vec, 1.0);

    //         let mut expect: Vec<Position> = vec![
    //             (1, 0).into(),(0.5, 0.5).into(), (0, 1).into()
    //             ];
    //         expect.reverse();
    //         assert_eq!(vec_moved, VecDeque::from(expect));
    //     }

    //     #[test]
    //     fn test_move_chain_multiple_items() {
    //         let vec: Vec<Position> = vec![
    //             (2, 0).into(),(0, 0).into(), (0, 2).into(),
    //             (0, 4).into(),(0, 6).into(), (0, 8).into()
    //             ];
    //         let vec_moved = move_chain_inner(vec.clone(), 1.0);

    //         let expect_0 = vec.first().unwrap().clone();
    //         let expect_1 = project_c_onto_ab(expect_0, vec[2], vec[1], 1.0);
    //         let expect_2 = project_c_onto_ab(expect_1, vec[3], vec[2], 1.0);
    //         let expect_3 = project_c_onto_ab(expect_2, vec[4], vec[3], 1.0);
    //         let expect_4 = project_c_onto_ab(expect_3, vec[5], vec[4], 1.0);
    //         let expect_5 = vec.last().unwrap().clone();
    //         let mut expect = vec![expect_0, expect_1, expect_2, expect_3, expect_4, expect_5];
    //         expect.reverse();
    //         assert_eq!(vec_moved, VecDeque::from(expect));
    //     }
    // }
}
