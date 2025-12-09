#![allow(unused)]

use std::thread::sleep;
use std::time::Duration;

use crate::collision::collisions;
use crate::colors::*;
use crate::draw::*;
use crate::model::*;
use crate::state::StateMachineEnum;
use crate::state::player::PLAYER_SPEED;
use crate::state::player::PlayerStateMachine;
use crate::state::item::*;
use crate::state::state_machine::item::ItemState;
use crate::state::state_machine::item::build;
use macroquad::math::Vec2;
use macroquad::prelude as mq;
use macroquad::window::screen_height;
use macroquad::window::screen_width;

mod draw;
mod input;
mod model;
mod persistence;
mod state;
mod util;
mod collision;

const DRAW_SCREEN_WIDTH: f32 = 1200.0;
const DRAW_SCREEN_HEIGHT: f32 = 800.0;

const DEBUG_DRAW_STATE_TEXT: bool = true;
const DEBUG_DRAW_GRID: bool = true;
const DEBUG_DRAW_ORIGIN_FACTOR: Vec2 = Vec2::new(0.5, 0.5);
const DEBUG_DRAW_COLLISION_BOXES: bool = true;

#[macroquad::main("Hook")]
async fn main() {
    mq::request_new_screen_size(DRAW_SCREEN_WIDTH, DRAW_SCREEN_HEIGHT);
    // set_pc_assets_folder("assets");

    let mut states = vec![init_player(), init_item()];

    loop {
        // let delta_time = get_frame_time();
        // Use like "MOVEMENT_SPEED * delta_time;"

        mq::clear_background(BLACK.into());
        update_states(&mut states);
        draw_states(&states);
        collisions(&states);

        if DEBUG_DRAW_STATE_TEXT {
            debug_draw_state_text(&states);
        }
        if DEBUG_DRAW_GRID {
            debug_draw_grid();
        }
        mq::next_frame().await
    }
}

fn init_player() -> StateMachineEnum {
    StateMachineEnum::Player(PlayerStateMachine::new(Position::new(200.0, 200.0), RIGHT, PLAYER_SPEED))
}

fn init_item() -> StateMachineEnum {
    StateMachineEnum::Item(ItemStateMachine::Moving(build(Position::new(200.0, 200.0), RIGHT, Magnitude::new(1.0))))
}

fn update_states(states: &mut [StateMachineEnum]) {
    for state in states.iter_mut() {
        let s1 = std::mem::take(state);
        let s2 = s1.update();
        *state = s2;
    }
}
