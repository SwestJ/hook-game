use std::fmt::Display;
use either::*;
use macroquad::prelude::Vec2;

use hook::*;
use player::*;
use item::*;

use crate::input::*;
use crate::model::*;
use crate::util::name_of_type;

pub mod player;
pub mod hook;
pub mod item;
pub mod action;

pub const HOOK_AMOUNT_LINKS: usize = 40;
pub const HOOK_LINK_LENGTH: f32 = 20.0;
pub const HOOK_EXTENDING_SPEED: Magnitude = Magnitude::new(5.5);
pub const HOOK_CONTRACTING_SPEED: Magnitude = Magnitude::new(2.5);
// pub const HOOK_CONTRACTING_SPEED: Magnitude = Magnitude::new(0.0);
pub const HOOK_CONTRACTING_HIST_LENGTH: usize = 50;
pub const HOOK_DIST_END_CONTRACT: f32 = 10.0;
pub const HOOK_CHAIN_PROJECTION_FACTOR: f32 = 0.1;
pub const HOOK_LINK_DIST_TREAT_AS_ZERO: f32 = 1.0;

pub trait State {
    type Output: State;
    fn position(&self) -> Position;
    fn direction(&self) -> Direction;
    fn update(self) -> Self::Output;
}


