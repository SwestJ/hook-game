use std::{fmt::Display, ops::{Add, Mul}};

use glam::Vec2;
use macroquad::shapes::{draw_circle, draw_rectangle};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::graphics::*;

pub trait Object: Sized + core::fmt::Debug + Copy + Display {
    fn position(&self) -> Position;
    fn set_position(&mut self, position: &Position);
    fn update_position(&mut self, v: Magnitude, direction: Direction) {
        self.set_position(
            &self.physics().calculate_new_position(
                self.position(), v, direction));
    }
    fn physics(&self) -> &Physics;
    fn direction(&self) -> &Direction;
    fn set_direction(&mut self, direction: &Direction);
}

#[derive(Deserialize, Default, TypedBuilder, Clone, Copy, Debug)]
pub struct Player {
    health: Health,
    #[serde(skip)]
    position: Position,
    physics: Physics,
    #[serde(skip)]
    direction: Direction,
}

impl Player {
    pub fn health(&self) -> &Health {
        &self.health
    }
}
impl Object for Player {
    fn position(&self) -> Position {
        self.position
    }
    fn set_position(&mut self, position: &Position) {
        self.position = *position;
    }
    fn physics(&self) -> &Physics {
        &self.physics
    }

    fn direction(&self) -> &Direction {
        &self.direction
    }

    fn set_direction(&mut self, direction: &Direction) {
        self.direction = *direction;
    }
}
impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Player: {:?}, {}, {:?}", self.health, self.position, self.direction)
    }
}

#[derive(Deserialize, Default, TypedBuilder, Clone, Copy, Debug)]
pub struct Hook {
    #[serde(skip)]
    position: Position,
    physics: Physics,
    #[serde(skip)]
    direction: Direction,
}

impl Object for Hook {
    fn position(&self) -> Position {
        self.position
    }
    fn set_position(&mut self, position: &Position) {
        self.position = *position;
    }
    fn physics(&self) -> &Physics {
        &self.physics
    }

    fn direction(&self) -> &Direction {
        &self.direction
    }

    fn set_direction(&mut self, direction: &Direction) {
        self.direction = *direction;
    }
}
impl Display for Hook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Hook: {:?}, {:?}", self.position, self.direction)
    }
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

    pub fn calculate_new_position(&self, position: Position, speed: Magnitude, direction: Direction) -> Position {
        Self::calculate_new_position_from_speed(position, speed, direction)
    }

    pub fn calculate_new_position_from_speed(position: Position, speed: Magnitude, direction: Direction) -> Position {
        let velocity = direction * speed;
        Position::new(position.x() + velocity.x(), position.y() + velocity.y())
    }

    pub fn min_speed(&self) -> Magnitude {
        self.min_speed.into()
    }
}

#[derive(Serialize, Deserialize, Default, TypedBuilder, Clone, Copy, Debug)]
pub struct GraphicsObject {
    pub shape: Shape,
    #[serde(skip)]
    pub position: Position,
}

pub trait Drawable {
    fn draw(&self, position: Position);
    fn set_position(&mut self, position: &Position);
}

impl Drawable for GraphicsObject {
    fn draw(&self, position: Position) {
        match self.shape() {
            Shape::Rectangle(s) => {
                draw_rectangle(position.x(), position.y(), s.width, s.height, s.color().into());
            },
            Shape::Circle(s) => {
                draw_circle(position.x(), position.y(), s.radius.0, s.color().into());
            },
            Shape::Line(s) => todo!(),
            Shape::Point => todo!(),
        }
    }

    fn set_position(&mut self, position: &Position) {
        self.position = *position;
    }
}

impl GraphicsObject {
    pub fn shape(&self) -> &Shape {
        &self.shape
    }
}

#[derive(Serialize, Deserialize, Default, Clone, Copy, Debug, derive_more::Add, derive_more::Mul)]
pub struct Health(u8);

impl From<u8> for Health {
    fn from(value: u8) -> Self {
        Health(value)
    }
}

//* Position
#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct Position(Vec2);
impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Position(Vec2::new(x,y))
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
    pub fn distance(self, position: Position) -> f32 {
        self.value().distance(position.value())
    }
    pub fn direction_to(self, position: Position) -> Direction {
        Direction::a_to_b(self, position)
    }
    pub fn move_towards(self, position: Position, distance: f32) -> Self {
        self + (self.direction_to(position) * distance)
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
impl From<Vec2> for Position {
    fn from(value: Vec2) -> Self {
        Position::new(value.x, value.y)
    }
}

//* Direction
/// A normalized 2D vector
#[derive(Default, Clone, Copy, Debug, derive_more::Add)]
pub struct Direction(Vec2);
impl Direction {
    pub fn new(x: f32, y: f32) -> Direction {
        Direction(Vec2::new(x, y).normalize_or_zero())
    }
    pub fn from_vec(vec: Vec2) -> Direction {
        Direction(vec.normalize_or_zero())
    }

    pub fn value(&self) -> &Vec2 {
        &self.0
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
        Direction::from_vec(*self.value())
    }

    pub fn a_to_b(a: Position, b: Position) -> Direction {
        Direction::from_vec(b.value() - a.value())
    }
}
impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Direction({:.01}, {:.01})", self.x(), self.y())
    }
}

pub const UP: Direction = Direction(Vec2::NEG_Y);
pub const DOWN: Direction = Direction(Vec2::Y);
pub const LEFT: Direction = Direction(Vec2::NEG_X);
pub const RIGHT: Direction = Direction(Vec2::X);

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
    pub fn value(&self) -> f32 {self.0}
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

impl<T> Mul<T> for Direction where T: Into<Magnitude> {
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