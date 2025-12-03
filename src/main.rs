#![allow(unused)]

use std::thread::sleep;
use std::time::Duration;

use crate::colors::*;
use crate::draw::*;
use crate::model::*;
use crate::state::StateEnum;
use crate::state::player::PlayerState;
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

const DRAW_SCREEN_WIDTH: f32 = 1200.0;
const DRAW_SCREEN_HEIGHT: f32 = 800.0;

const DEBUG_DRAW_STATE_TEXT: bool = true;
const DEBUG_DRAW_GRID: bool = true;
const DEBUG_DRAW_ORIGIN_FACTOR: Vec2 = Vec2::new(0.5, 0.5);

#[macroquad::main("Hook")]
async fn main() {
    mq::request_new_screen_size(DRAW_SCREEN_WIDTH, DRAW_SCREEN_HEIGHT);
    // set_pc_assets_folder("assets");

    let mut states = vec![init_player(), init_item()];

    loop {
        // let delta_time = get_frame_time();
        // Use like "MOVEMENT_SPEED * delta_time;"

        mq::clear_background(BLACK.into());
        invoke_states(&mut states);
        draw_states(&states);

        if DEBUG_DRAW_STATE_TEXT {
            debug_draw_state_text(&states);
        }
        if DEBUG_DRAW_GRID {
            debug_draw_grid();
        }
        mq::next_frame().await
    }
}

fn init_player() -> StateEnum {
    StateEnum::Player(PlayerState::new(Position::new(200.0, 200.0), RIGHT))
}

fn init_item() -> StateEnum {
    StateEnum::Item(state::item::ItemState::Moving(state::item::build(Position::new(200.0, 200.0), RIGHT, Magnitude::new_const(1.0))))
}

fn invoke_states(states: &mut [StateEnum]) {
    for state in states.iter_mut() {
        let s1 = std::mem::take(state);
        let s2 = s1.invoke();
        *state = s2;
    }
}

pub fn check_collisions(state: &StateEnum) {
    match state {
        StateEnum::Player(player_state_enum) => todo!(),
        StateEnum::Item(_) => todo!(),
        StateEnum::Default => todo!(),
    }
}
