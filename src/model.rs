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
    pub const fn new(v: f32) -> Magnitude {
        Magnitude(v)
    }
    pub const fn zero() -> Magnitude {
        Magnitude(0.0)
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
