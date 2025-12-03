use std::{
    f32::consts::PI,
    fmt::Display,
    ops::{Add, Deref, DerefMut, Mul},
    slice::{self, Windows},
};

use crate::graphics::*;
use anyhow::*;
use macroquad::{
    math::Vec2,
    shapes::{draw_circle, draw_rectangle},
};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

pub const UP: Direction = Direction(Vec2::NEG_Y);
pub const DOWN: Direction = Direction(Vec2::Y);
pub const LEFT: Direction = Direction(Vec2::NEG_X);
pub const RIGHT: Direction = Direction(Vec2::X);

pub trait Object: Sized + core::fmt::Debug + Copy + Display {
    fn position(&self) -> Position;
    fn set_position(&mut self, position: &Position);
    fn update_position(&mut self, v: Magnitude, direction: Direction) {
        self.set_position(&self.physics().calculate_new_position(self.position(), v, direction));
    }
    fn physics(&self) -> &Physics;
    fn direction(&self) -> &Direction;
    fn set_direction(&mut self, direction: &Direction);
}

#[derive(Serialize, Deserialize, Default, TypedBuilder, Clone, Copy, Debug)]
pub struct Physics {
    pub min_speed: f32,
    pub max_speed: f32,
    pub acceleration: f32,
}

impl Physics {
    pub fn accelerate(&self, current_speed: Magnitude, elapsed_time: f32) -> Magnitude {
        let v = current_speed.value();
        self.max_speed.min(v + self.acceleration * elapsed_time).into()
    }

    pub fn calculate_new_position(
        &self,
        position: Position,
        speed: Magnitude,
        direction: Direction,
    ) -> Position {
        Self::calculate_new_position_from_speed(position, speed, direction)
    }

    pub fn calculate_new_position_from_speed(
        position: Position,
        speed: Magnitude,
        direction: Direction,
    ) -> Position {
        let velocity = direction * speed;
        Position::new(position.x() + velocity.x(), position.y() + velocity.y())
    }

    pub fn min_speed(&self) -> Magnitude {
        self.min_speed.into()
    }
}

trait Collision {
    fn collision_box(&self) -> &CollisionBox;
    fn handle_collision(&mut self, cbox: CollisionBox);
}

struct CollisionBox {
    vec1: Vec2,
    vec2: Vec2,
}

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct Position(Vec2);
impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self(Vec2::new(x, y))
    }
    pub fn from_vec(vec: Vec2) -> Self {
        Self(vec)
    }
    pub fn value(&self) -> Vec2 {
        self.0
    }
    pub fn x(&self) -> f32 {
        self.value().x
    }
    pub fn y(&self) -> f32 {
        self.value().y
    }
    pub fn distance(self, position: &Position) -> f32 {
        self.value().distance(position.value())
    }
    pub fn direction_to<T: AsRef<Position>>(self, position: T) -> Direction {
        Direction::a_to_b(self, position)
    }
    pub fn move_towards(self, position: Position, distance: f32) -> Self {
        self.move_in_direction(self.direction_to(position), distance)
    }
    pub fn move_in_direction(self, direction: Direction, distance: f32) -> Self {
        self + (direction * distance)
    }
}
impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Position({:.02}, {:.02})", self.x(), self.y())
    }
}
impl From<(f32, f32)> for Position {
    fn from(value: (f32, f32)) -> Self {
        Position::new(value.0, value.1)
    }
}
impl From<(i32, i32)> for Position {
    fn from(value: (i32, i32)) -> Self {
        Position::from((value.0 as f32, value.1 as f32))
    }
}
impl AsRef<Vec2> for Position {
    fn as_ref(&self) -> &Vec2 {
        &self.0
    }
}

//* Direction
/// A normalized 2D vector
#[derive(Default, Clone, Copy, Debug, derive_more::Add)]
pub struct Direction(Vec2);
impl Direction {
    pub fn new(x: f32, y: f32) -> Self {
        Self(Vec2::new(x, y).normalize_or_zero())
    }
    pub fn new_vec(vec: Vec2) -> Self {
        Self(vec.normalize_or_zero())
    }
    pub const fn value(&self) -> Vec2 {
        self.0
    }
    pub fn is_zero(&self) -> bool {
        self.value().x == 0.0 && self.value().y == 0.0
    }
    pub fn x(&self) -> f32 {
        self.value().x
    }
    pub fn y(&self) -> f32 {
        self.value().y
    }
    pub fn normalize_or_zero(self) -> Self {
        Direction::new_vec(self.value())
    }
    pub fn a_to_b<T: AsRef<Position>, U: AsRef<Position>>(a: U, b: T) -> Self {
        Self::new_vec(b.as_ref().value() - a.as_ref().value())
    }
    pub fn rotate<T>(self, angle: Angle<T>) -> Self
    where
        Angle<T>: Into<Direction>,
    {
        rotate_by_direction(self, angle.into())
    }
}
impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Direction({:.01}, {:.01})", self.x(), self.y())
    }
}

impl From<Direction> for Angle<Degrees> {
    fn from(direction: Direction) -> Self {
        Angle(Degrees(direction.value().to_angle().to_degrees()))
    }
}
impl From<Angle<Degrees>> for Direction {
    fn from(angle: Angle<Degrees>) -> Self {
        Direction::from(angle.to_radians())
    }
}
impl From<Angle<Radians>> for Direction {
    fn from(angle: Angle<Radians>) -> Self {
        Direction(Vec2::from_angle(angle.into()))
    }
}
impl AsRef<Vec2> for Direction {
    fn as_ref(&self) -> &Vec2 {
        &self.0
    }
}
impl AsRef<Vec2> for Velocity {
    fn as_ref(&self) -> &Vec2 {
        &self.0
    }
}

pub fn rotate_by_direction<T: AsRef<Vec2>>(vec: T, direction: Direction) -> Direction {
    Direction(direction.value().rotate(vec.as_ref().to_owned()))
}

#[derive(Clone, Copy, Debug, derive_more::Add)]
pub struct Angle<T>(pub T);
#[derive(Clone, Copy, Debug, derive_more::Add)]
pub struct Degrees(pub f32);
#[derive(Clone, Copy, Debug, derive_more::Add)]
pub struct Radians(pub f32);

//* Angle */
impl<T> Angle<T>
where
    T: Copy,
{
    pub fn value(&self) -> T {
        self.0
    }
}
impl Angle<Degrees> {
    pub fn new(degrees: f32) -> Self {
        Self(Degrees(degrees))
    }
    pub fn to_radians(self) -> Angle<Radians> {
        Angle(self.value().to_radians())
    }
}
impl Angle<Radians> {
    pub fn new(radians: f32) -> Self {
        Self(Radians(radians))
    }
    pub fn to_degrees(self) -> Angle<Degrees> {
        Angle(self.value().to_degrees())
    }
}
impl Degrees {
    pub fn value(&self) -> f32 {
        self.0
    }
    pub fn to_radians(self) -> Radians {
        Radians(self.value().to_radians())
    }
}
impl Radians {
    pub fn value(&self) -> f32 {
        self.0
    }
    pub fn to_degrees(self) -> Degrees {
        Degrees(self.value().to_degrees())
    }
}
impl<T> From<T> for Angle<T> {
    fn from(value: T) -> Self {
        Angle(value)
    }
}

impl From<Degrees> for f32 {
    fn from(degrees: Degrees) -> Self {
        degrees.value()
    }
}
impl From<Radians> for f32 {
    fn from(radians: Radians) -> Self {
        radians.value()
    }
}
impl<T> From<Angle<T>> for f32
where
    T: Copy + Into<f32>,
{
    fn from(value: Angle<T>) -> Self {
        value.value().into()
    }
}
impl From<Degrees> for Radians {
    fn from(degrees: Degrees) -> Self {
        degrees.to_radians()
    }
}
impl From<Radians> for Degrees {
    fn from(radians: Radians) -> Self {
        radians.to_degrees()
    }
}
impl<T> Deref for Angle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for Angle<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

//* Velocity
pub struct Velocity(Vec2);

impl Velocity {
    pub fn value(&self) -> Vec2 {
        self.0
    }
    pub fn new<T: Into<Magnitude>>(direction: Direction, magnitude: T) -> Self {
        direction * magnitude.into()
    }

    fn x(&self) -> f32 {
        self.value().x
    }

    fn y(&self) -> f32 {
        self.value().y
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub struct Magnitude(f32);
impl Magnitude {
    pub const fn new_const(v: f32) -> Magnitude {
        Magnitude(v)
    }
    pub fn value(&self) -> f32 {
        self.0
    }
}
impl Display for Magnitude {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Velocity({:.02})", self.0)
    }
}
impl From<f32> for Magnitude {
    fn from(value: f32) -> Self {
        Magnitude(value)
    }
}
impl From<i32> for Magnitude {
    fn from(value: i32) -> Self {
        Magnitude(value as f32)
    }
}

//* Arithmetic */
impl Add<Magnitude> for Magnitude {
    type Output = Magnitude;

    fn add(self, rhs: Self) -> Self::Output {
        Magnitude(self.0 + rhs.0)
    }
}
impl Add<f32> for Magnitude {
    type Output = Magnitude;

    fn add(self, rhs: f32) -> Self::Output {
        Magnitude(self.0 + rhs)
    }
}
impl Add<Magnitude> for f32 {
    type Output = Magnitude;

    fn add(self, rhs: Magnitude) -> Self::Output {
        Magnitude(self + rhs.0)
    }
}

impl<T> Mul<T> for Direction
where
    T: Into<Magnitude>,
{
    type Output = Velocity;

    fn mul(self, rhs: T) -> Self::Output {
        Velocity(self.value() * rhs.into().value())
    }
}

impl Add<Velocity> for Position {
    type Output = Position;

    fn add(self, rhs: Velocity) -> Self::Output {
        Position(self.value() + rhs.value())
    }
}

use std::collections::VecDeque;

#[derive(Debug)]
pub struct Stack<T, A, B>
where
    T: Clone,
    A: AsRef<T> + AsMut<T>,
    B: AsRef<T>,
{
    stack: Vec<T>,
    head: A,
    tail: B,
    function: fn(&mut T, &T),
}

impl<T, A, B> Stack<T, A, B>
where
    T: Clone,
    A: AsRef<T> + AsMut<T>,
    B: AsRef<T>,
{
    pub fn new(head: A, tail: B, function: fn(&mut T, &T)) -> Self {
        Stack { stack: vec![], head, tail, function }
    }
    pub fn head(&self) -> &A {
        &self.head
    }
    pub fn head_mut(&mut self) -> &mut A {
        &mut self.head
    }
    pub fn set_head(&mut self, head: A) {
        self.head = head;
    }
    pub fn tail(&self) -> &B {
        &self.tail
    }
    pub fn tail_mut(&mut self) -> &mut B {
        &mut self.tail
    }
    pub fn set_tail(&mut self, tail: B) {
        self.tail = tail;
    }
    pub fn last(&self) -> &T {
        self.stack.last().unwrap_or(self.head.as_ref())
    }
    pub fn first(&self) -> &T {
        self.stack.first().unwrap_or(self.tail.as_ref())
    }
    pub fn pop(&mut self) -> T {
        self.stack.pop().unwrap_or(self.head.as_ref().clone())
    }
    pub fn pop_if(&mut self, predicate: impl FnOnce(&mut T) -> bool) -> Option<T> {
        self.stack.pop_if(predicate)
    }
    pub fn push(&mut self, item: T) {
        self.stack.push(item);
    }
    pub fn push_tail(&mut self) {
        self.stack.push(self.tail.as_ref().clone());
    }
    pub fn len(&self) -> usize {
        self.stack.len()
    }
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
    pub fn iter_full(
        &self,
    ) -> std::iter::Chain<
        std::iter::Chain<std::array::IntoIter<&T, 1>, slice::Iter<'_, T>>,
        std::array::IntoIter<&T, 1>,
    > {
        [self.head.as_ref()].into_iter().chain(self.stack.iter()).chain([self.tail.as_ref()])
    }
    pub fn iter(&self) -> slice::Iter<'_, T> {
        self.stack.iter()
    }
    pub fn into_iter(self) -> <std::vec::Vec<T> as std::iter::IntoIterator>::IntoIter {
        self.stack.into_iter()
    }

    pub fn fold_into_self(self) -> Self {
        let Self { mut stack, function, .. } = self;
        stack.iter_mut().fold(Stack { stack: vec![], ..self }, |mut acc, mut current| {
            function(current, acc.last());
            acc.push(current.clone());
            acc
        })
    }
    pub fn rfold_into_self(self, init: &[T]) -> Self {
        let Self { mut stack, head, function, .. } = self;

        let mut stack = stack.iter_mut().rfold(
            Stack { head, stack: init.into(), ..self },
            |mut acc, mut current| {
                function(current, acc.last());
                acc.push(current.clone());
                acc
            },
        );

        let last = stack.last().clone();
        function(stack.head.as_mut(), &last);
        stack.stack.reverse();
        stack
    }
}

// todo Could make a more "case" specific collection
// Chain
//  - could keep elements "spaced". User should supply how much "space" should be between each item and a closure which can evaluate "space" between two items.
//      > When head or tail is updated, the list checks and updates all elements.
//      > "Space" between items could instead be calculated from the space between head and tail.
//      > GetSpaceToPreviousItem(..) -> T, T != SpaceBetweenItems => UpdateSpaceTowardsPreviousItemBy(T)
//  - Could execute a function on each element whenever head or tail is updated. User supplies a closure which takes the current and previous item, and returns an updated current item.
//      > This would be less specific than above. The above could probably be achieved in this version, with the right closure.
//      > Could be used on all kind of types, that have a relationship that require them to be updated when something happens to the "main" type (i.e. the head or tail)
//      > Probably only on types that have some inherent relationship, so they almost never have to be handled individually.
//  - What would be the benefit of using such a collection compared to e.g. Vec<T> and implement the relationship yourself?
//      > It would guarantee that all items are updated when the head/tail is updated.

// impl<T, A, B> TryFrom<&[T]> for Stack<T, A, B>
// where
//     T: Clone,
//     A: AsRef<T>,
//     B: AsRef<T>,
// {
//     type Error = anyhow::Error;

//     fn try_from(slice: &[T]) -> std::result::Result<Self, Self::Error> {
//         match slice {
//             [] => Err(anyhow!("Cannot convert to Stack from empty list")),
//             [single] => Ok(Stack {
//                 stack: vec![],
//                 head: single.clone(),
//                 tail: single.clone(),
//             }),
//             [head, stack @ .., tail] => Ok(Stack {
//                 stack: stack.into(),
//                 head: head.clone(),
//                 tail: tail.clone(),
//             }),
//         }
//     }
// }

fn match_slice_examples(vec: &[i32]) {
    match vec {
        [] => println!("Empty"),
        [head] => println!("1 element: {:?}", head),
        [-1, head, middle @ .., tail] => {
            println!("1: head, middle, tail: {:?}, {:?}, {:?}", head, middle, tail)
        }
        [-2, head, tail @ ..] => println!("2: head, tail: {:?}, {:?}", head, tail),
        [-3, head, middle @ .., _] => println!("3: head, middle: {:?}, {:?}", head, middle),
        [..] => println!("Catch all"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let vec = vec![0];
        match_slice_examples(&vec);
        let vec = vec![-1, 1, 2, 3, 4, 5];
        match_slice_examples(&vec);
        let vec = vec![-2, 1, 2, 3, 4, 5];
        match_slice_examples(&vec);
        let vec = vec![-3, 1, 2, 3, 4, 5];
        match_slice_examples(&vec);
    }
}
