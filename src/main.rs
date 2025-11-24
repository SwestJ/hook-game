#![allow(unused)]

use std::thread::sleep;
use std::time::Duration;

use crate::colors::*;
use crate::model::*;
use crate::state::StateEnum;
use crate::state::player::PlayerStateEnum;
use macroquad::file::set_pc_assets_folder;
use macroquad::text::draw_multiline_text;
use macroquad::time::get_frame_time;
use macroquad::window::clear_background;
use macroquad::window::next_frame;
use macroquad::window::request_new_screen_size;

mod colors;
mod draw;
mod graphics;
mod input;
mod model;
mod persistence;
mod state;

const SLEEP_DURATION: u64 = 10;

#[macroquad::main("Hook")]
async fn main() {
    request_new_screen_size(1200.0, 800.0);
    // set_pc_assets_folder("assets");

    let mut states = vec![init_player()];

    loop {
        // let delta_time = get_frame_time();
        // Use like "MOVEMENT_SPEED * delta_time;"
        clear_background(BLACK.into());
        invoke_states(&mut states);
        draw_states(&states);
        draw_debug_text_vec(&states);
        // sleep(Duration::from_millis(SLEEP_DURATION));
        next_frame().await
    }
}

fn init_player() -> StateEnum {
    StateEnum::Player(PlayerStateEnum::new(Position::new(200.0, 200.0), RIGHT))
}

fn draw_debug_text_vec(states: &[StateEnum]) {
    let debug_text = states
        .iter()
        .fold(String::new(), |acc, s| format!("{}\n{}", acc, s));
    draw_multiline_text(
        debug_text.as_str(),
        20.0,
        20.0,
        20.0,
        None,
        macroquad::color::RED,
    );
}

fn invoke_states(states: &mut [StateEnum]) {
    for state in states.iter_mut() {
        let s1 = std::mem::take(state);
        let s2 = s1.invoke();
        *state = s2;
    }
}

fn draw_states(states: &[StateEnum]) {
    states.iter().for_each(draw::draw_state);
}
