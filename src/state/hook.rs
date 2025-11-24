//* ------------------------------------------------------------------------*/
//* ------------------------------------------------------------------------*/
//* ----------------------------------HOOK----------------------------------*/
//* ------------------------------------------------------------------------*/
//* ------------------------------------------------------------------------*/
pub const HOOK_AMOUNT_LINKS: usize = 40;
pub const HOOK_LINK_LENGTH: f32 = 20.0;
pub const HOOK_EXTENDING_SPEED: Magnitude = Magnitude::new_const(6.5);
pub const HOOK_CONTRACTING_SPEED: Magnitude = Magnitude::new_const(3.5);
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
impl<T> HookState for HookStateMachine<T>
where
    T: HookState,
{
    fn position(&self) -> Position {
        self.state().position()
    }

    fn direction(&self) -> Direction {
        self.state().direction()
    }
}

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

    pub trait HookState: Display {
        fn position(&self) -> Position;
        fn direction(&self) -> Direction;
    }

    #[derive(Debug)]
    pub struct Extending {
        amount_of_links: usize,
        hook: Hook,
    }
    impl HookState for Extending {
        fn position(&self) -> Position {
            self.hook.position()
        }

        fn direction(&self) -> Direction {
            self.hook.direction()
        }
    }
    impl Extending {
        pub fn amount_of_links(&self) -> usize {
            self.amount_of_links
        }
        pub fn speed(&self) -> Magnitude {
            self.hook.speed()
        }

        pub fn hook(&self) -> &Hook {
            &self.hook
        }

        fn extend(
            speed: Magnitude,
            direction: Direction,
            origin: Position,
            amount_of_links: usize,
        ) -> Self {
            let hook = Hook::new(origin, speed, direction, HOOK_LINK_LENGTH);
            let state = Extending {
                amount_of_links,
                hook,
            };
            state.extend_self()
        }

        pub fn extend_self(self) -> Self {
            let hook = self.hook.travel();
            Extending { hook, ..self }
        }

        fn contract(self) -> Contracting {
            Contracting::contract(self.hook, HOOK_CONTRACTING_SPEED)
        }

        pub fn try_contract(self, goal: Position) -> Result<Contracting, Self> {
            if self.hook().chain().count() < self.amount_of_links() {
                Err(self)
            } else {
                Ok(self.contract())
            }
        }
    }

    #[derive(Debug)]
    pub struct Contracting {
        hook: Hook,
    }

    impl HookState for Contracting {
        fn position(&self) -> Position {
            self.hook.position()
        }

        fn direction(&self) -> Direction {
            self.hook.direction()
        }
    }
    impl Contracting {
        pub fn speed(&self) -> Magnitude {
            self.hook.speed()
        }
        pub fn hook(&self) -> &Hook {
            &self.hook
        }
        fn contract(mut hook: Hook, speed: Magnitude) -> Self {
            hook.speed = speed;
            let tail = hook.chain().back().clone();
            let state = Contracting { hook };
            state.contract_self(tail.position())
        }

        pub fn contract_self(self, tail: Position) -> Contracting {
            let hook = self.hook.travel_back(tail);
            Contracting { hook }
        }

        pub fn try_end(self) -> Result<(), Self> {
            let distance = self.hook.chain.length_straight_line();
            if distance < HOOK_DIST_END_CONTRACT {
                Ok(())
            } else {
                Err(self)
            }
        }
    }

    #[derive(Debug)]
    pub struct Hook {
        position: Position,
        speed: Magnitude,
        direction: Direction,
        chain: Chain,
    }

    impl Hook {
        fn new(
            position: Position,
            speed: Magnitude,
            direction: Direction,
            link_length: f32,
        ) -> Self {
            let chain = Chain::new(position, link_length);
            Self {
                position,
                speed,
                direction,
                chain: Chain::new(position, link_length),
            }
        }
        fn position(&self) -> Position {
            self.position
        }
        fn direction(&self) -> Direction {
            self.direction
        }
        pub fn speed(&self) -> Magnitude {
            self.speed
        }
        pub fn chain(&self) -> &Chain {
            &self.chain
        }
        fn chain_mut(&mut self) -> &mut Chain {
            &mut self.chain
        }
        fn travel(self) -> Self {
            let Hook {
                position,
                speed,
                direction,
                chain,
            } = self;
            let position = Physics::calculate_new_position_from_speed(position, speed, direction);
            let chain = chain.update_head_position(position);
            // chain.extend_back(Link::new(position));
            Hook {
                position,
                chain,
                ..self
            }
        }

        fn travel_back(self, tail_position: Position) -> Self {
            let Hook {
                position,
                speed,
                chain,
                ..
            } = self;
            // chain.extend_back(Link::new(tail_position));
            let chain = chain
                .update_tail_position(tail_position, HOOK_CHAIN_PROJECTION_FACTOR) //todo move and retract should be done in "one operation". Could maybe just move/retract the tail/second and then update rest of chain.
                .retract(speed);
            Hook {
                position: chain.front().position(),
                chain,
                ..self
            }
        }

        fn direction_towards_next_link(current: Position, next: Position) -> Direction {
            Direction::a_to_b(current, next)
        }
    }

    #[derive(Debug)]
    pub struct Chain {
        chain: VecDeque<Link>,
        link_length: f32,
        head: Link,
        tail: Link,
    }

    impl Chain {
        fn new(hook_position: Position, link_length: f32) -> Self {
            Self {
                chain: vec![Link::new(hook_position), Link::new(hook_position)].into(),
                link_length,
                head: Link::new(hook_position),
                tail: Link::new(hook_position),
            }
        }

        fn length_straight_line(&self) -> f32 {
            self.front().distance(self.back())
        }

        fn length_of_links(&self) -> f32 {
            self.chain
                .iter()
                .skip(1)
                .fold((0.0, self.front()), |acc, link| {
                    (acc.0 + link.clone().distance(acc.1), link)
                })
                .0
        }

        pub fn front(&self) -> &Link {
            self.chain.front().unwrap()
        }

        pub fn back(&self) -> &Link {
            self.chain.back().unwrap()
        }

        pub fn remove_back(&mut self) {
            self.chain.pop_back();
        }

        pub fn pop_back(&mut self) -> Link {
            self.chain.pop_back().unwrap()
        }

        pub fn pop_front(&mut self) -> Link {
            self.chain.pop_front().unwrap()
        }

        pub fn push_back(&mut self, link: Link) {
            self.chain.push_back(link);
        }

        pub fn push_front(&mut self, link: Link) {
            self.chain.push_front(link);
        }

        /// Updates head position and returns the distance travelled
        pub fn move_head(&mut self, position: Position) -> Magnitude {
            let head = self.chain.front_mut().unwrap();
            let distance = head.distance_to_position(position);
            head.set_position(position);
            distance.into()
        }

        /// Updates head position and returns the distance travelled
        pub fn move_tail(&mut self, position: Position) -> Magnitude {
            let tail = self.chain.back_mut().unwrap();
            let distance = tail.distance_to_position(position);
            tail.set_position(position);
            distance.into()
        }

        pub fn iter(&self) -> vec_deque::Iter<'_, Link> {
            self.chain.iter()
        }

        pub fn into_iter(self) -> vec_deque::IntoIter<Link> {
            self.chain.into_iter()
        }

        fn update_head_position(mut self, head_position: Position) -> Chain {
            let tail = self.back().clone();
            let distance = self.move_head(head_position);
            let mut chain = self.move_links_toward_head();
            chain.remove_back();
            chain.extend_back(tail.clone());
            chain.push_back(tail);
            chain
        }

        fn update_tail_position(mut self, tail_position: Position, factor: f32) -> Chain {
            let distance = self.move_tail(tail_position);
            self.move_links_toward_tail()
            // .adjust_chain_by_projection(factor)
        }

        //todo could make a wrapper to VecDeque witch contain a head and tail in separate fields. Maybe a syntax like [tail, ..] could be possible, when moving the wrapper between methods.
        //todo Could make more of the Chain functions follow a builder pattern, so they consume and return Self. Would allow chaining method calls-
        // Retracts chain by a given length - usually the retraction "speed"
        // tail is popped before and pushed after, because the retraction should not affect it
        fn retract(mut self, length: Magnitude) -> Chain {
            let tail = self.pop_back();
            let back = self.pop_back().move_towards(&tail, length.value());
            self.push_back(back);
            let mut chain = self.move_links_toward_tail();
            chain.contract_back(&tail);
            chain.push_back(tail);
            chain
        }

        fn adjust_chain_by_projection(self, factor: f32) -> Chain {
            let links_rev: Vec<Link> = self.chain.into_iter().rev().collect();
            Chain {
                chain: adjust_chain_inner(links_rev, factor),
                ..self
            }
        }

        fn move_links_toward_head(mut self) -> Chain {
            let head = self.chain.pop_front().unwrap();
            // let next = self.chain.pop_front().unwrap().move_towards(&head, distance.value());

            let chain = self
                .iter()
                .fold(VecDeque::from([head.clone()]), |mut acc, link| {
                    acc.push_back(
                        link.clone()
                            .clamp_to_length(acc.back().unwrap(), self.link_length),
                    );
                    acc
                });

            Chain { chain, ..self }
        }

        fn move_links_toward_tail(mut self) -> Chain {
            let tail = self.pop_back();
            // let next = self.pop_back().move_towards(&tail, distance.value());

            let chain = self
                .iter()
                .rfold(VecDeque::from([tail.clone()]), |mut acc, link| {
                    acc.push_front(
                        link.clone()
                            .clamp_to_length(acc.front().unwrap(), self.link_length),
                    );
                    acc
                });

            Chain { chain, ..self }
        }

        fn extend_front(&mut self, link: Link) {
            let previous = self.front();
            if previous.distance(&link) > self.link_length {
                self.push_front(link);
            }
        }

        fn extend_back(&mut self, link: Link) {
            let previous = self.back();
            if previous.distance(&link) > self.link_length {
                self.push_back(link);
            }
        }

        fn contract_back(&mut self, link: &Link) {
            let previous = self.back();
            if previous.distance(link) < HOOK_LINK_DIST_TREAT_AS_ZERO {
                self.pop_back();
            }
        }

        fn count(&self) -> usize {
            self.chain.len()
        }
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
        fn move_towards(self, link: &Link, distance: f32) -> Self {
            let max_dist = self.distance(link);
            Link {
                position: self
                    .position()
                    .move_towards(link.position(), distance.min(max_dist)),
            }
        }

        fn clamp_to_length(self, link: &Link, link_length: f32) -> Self {
            let current = self.distance(link);
            let diff = current - link_length;
            Link {
                position: self.position().move_towards(link.position(), diff.max(0.0)),
            }
        }

        fn move_link_projection(self, prev: Link, next: &Link, factor: f32) -> Self {
            let position =
                project_c_onto_ab(prev.position(), next.position(), self.position(), factor);
            Link { position }
        }

        fn distance(&self, link: &Link) -> f32 {
            self.position().distance(link.position())
        }

        fn distance_to_position(&self, position: Position) -> f32 {
            self.position().distance(position)
        }
    }

    fn adjust_chain_inner(links: Vec<Link>, factor: f32) -> VecDeque<Link> {
        if links.len() < 2 {
            return links.into();
        }

        let mut links_moved = links[..]
            .windows(2)
            .fold(VecDeque::<Link>::new(), |acc, slice| {
                accumulate_project_onto(acc, slice, factor)
            });
        links_moved.push_front(links.last().unwrap().clone());
        links_moved
    }

    fn accumulate_project_onto(
        mut acc: VecDeque<Link>,
        slice: &[Link],
        factor: f32,
    ) -> VecDeque<Link> {
        let link = if acc.is_empty() {
            slice[0].clone()
        } else {
            let prev = acc.front().unwrap().clone();
            let current = slice[0].clone();
            current.move_link_projection(prev, &slice[1], factor)
        };
        acc.push_front(link);
        acc
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

    //* Std trait implementations */
    impl Display for Extending {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "Extending {}, {}, links: {}, length: {}",
                self.position(),
                self.speed(),
                self.hook().chain().chain.len(),
                self.hook().chain().length_of_links()
            )
        }
    }
    impl Display for Contracting {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "Contracting {}, {}, {}, link: {}, length: {}",
                self.position(),
                self.speed(),
                self.direction(),
                self.hook().chain().chain.len(),
                self.hook().chain().length_of_links()
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
