#![allow(unused)]

use std::thread::sleep;
use std::time::Duration;

use crate::colors::*;
use crate::model::*;
use crate::state::StateEnum;
use crate::state::player::PlayerStateEnum;
use macroquad::file::set_pc_assets_folder;
use macroquad::text::draw_multiline_text;
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

/* TODO
- Design and implement input/action model
- Implement game loop in PlayerController
    - Should be in new thread probably
    - Each future controller could then keep their own game loop
    - Main loop could continue to be responsible for draw(), by keeping the list of graphics.
    - It might be necessary to ensure some order of draw, so background get drawn first and then player etc.
- Make player move on input from keyboard
- Restrict player to "area"
- Make player shoot a "bullet" from keyboard

- (DONE) Draw player via custom model and shapes
- (DONE) Looking into using typestate pattern for player states
- (DONE) Add controller and refactor to use it
- (DONE) Addded build/builder traits and refactor to use them
- (DONE) PlayerBuilder: Is it possible to pass on a state which keeps the field values?
    When build() is called, the values are mapped to the buildee's fields.
    Motivation is to avoid having fields in builder which corresponds 1:1 with those in buildee
- (DONE) Refactor so GraphicsObject keeps a copy of Position, so draw() does not need Position and I can keep a list of &GraphObjects.
    Position will be updated whenever position on player/object changes.
    The thought is that this position changes less often than we need to draw.
*/

/* NOTES
- PlayerController could take keyboard-keys as arguments, so the caller defines which keys should control the player
- Game Loop: How will main game-loop interact with objects, draws, keyboard events etc?
    - It could keep lists of related objects and call relevant functions.
        - Drawable objects
        - Interactable objects
        - Maybe list of objects to (re)calculate certain properties on. E.g. a projectile needs to have movement/position recalculated
    - Objects / Controllers could keep their own loops to some degree.
        - All objects could have their own loop, and maybe even seperated into each component/trait
            - Draw trait of Player and Interact trait of Player each have their own loop and will update draw/actions as needed
            - Player or PlayerController could be responsible. So it would make sure to draw and perform actions as needed
        - Some components could still be handled by main. E.g. draw would probably make sense,
            since the order matters and that would be hard to control if each object handles their own drawing.
        - All or some controllers could handle a list of related objects.
            - Could be devided as Draw controller, interaction controller etc.
            - Could be devided as projectiles, characters, foilage etc. Probably doesn't make sense.
- States
    - All logic related to changing state should be implemented in the typestate implementations.
    - States should somehow be used to indicate if an object should be drawn/interactable/recalculated etc.
        - Interact logic could match current state and only call e.g. move() on the state if it is allowed
            (e.g. not allowed to move when in shooting state). move() would consume state and return new state if necessary (e.g. idle -> move)
        - Should Interact or State do the actual movement of the object?
            - If all states implements move(), then Interact could just always call move().
                Then the given state implementation would control what happens (if anything).
        - Idea: State functions are "actions" and handles all logic related to that action in that state.
            - How would e.g. move() update anything? It should make sure that position is updated, but how?
                - It could take a mutuable reference to position. But it would also need to know speed and elapsed time.
                - It could send on the speed to the Moving state. So Moving keeps the information needed to calculate movement.
    - Example flow
        - When player is idle and user presses "move right" key
            - Interact logic registers that key is pressed (it is either called from main/controller or keeps check-loop itself)
            - Interact logic calls the function move() in the Idle state implementation
            - Idle::move ..
            - Idle::move consumes self and returns the new state Moving
            - Interact updates the PlayerController with the new state
- Thoughts on states
    - Get objects in Interact states -> Call them with pressed key
        - During runtime, how can you check if an object implements a trait?

*/

const SLEEP_DURATION: u64 = 10;

#[macroquad::main("Hook")]
async fn main() {
    request_new_screen_size(1200.0, 800.0);
    set_pc_assets_folder("assets");

    let mut states = vec![init_player()];

    loop {
        clear_background(BLACK.into());
        invoke_states(&mut states);
        draw_states(&states);
        draw_debug_text_vec(&states);
        sleep(Duration::from_millis(SLEEP_DURATION));
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
