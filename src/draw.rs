use either::Either::*;
use glam::Vec2;
use macroquad::shapes::*;

use crate::{
    colors::*,
    graphics::{Circle, Radius, Rectangle, Shape},
    model::Position,
    state::{
        hook::{state_hook::*, *},
        player::{state_player::*, *},
        *,
    },
};

pub fn draw_state(state: &StateEnum) {
    match state {
        StateEnum::Player(player_state_enum) => draw_player(player_state_enum),
        StateEnum::Default => todo!(),
    }
}

pub fn draw_player(state: &PlayerStateEnum) {
    match state {
        PlayerStateEnum::Idling(state) => {
            let position = state.position();
            draw(&PLAYER_IDLING, position);
        }
        PlayerStateEnum::Moving(state) => {
            let position = state.position();
            draw(&PLAYER_MOVING, position);
        }
        PlayerStateEnum::Shooting(state) => {
            let position = state.position();
            draw(&PLAYER_SHOOTING, position);
            draw_hook(state.state().hook(), position);
        }
        PlayerStateEnum::DualityIdlingShooting(state) => {
            let position = state.position();
            draw(&PLAYER_IDLING, position);
            draw_hook(state.state().yang().hook(), position)
        }
        PlayerStateEnum::DualityMovingShooting(state) => {
            let position = state.position();
            draw(&PLAYER_MOVING, position);
            draw_hook(state.state().yang().hook(), position)
        }
    }
}

pub fn draw_hook(state: &HookStateEnum, goal: Position) {
    match state {
        HookStateEnum::Extending(state) => {
            let position = state.position();
            draw(&HOOK_EXTENDING, position);
            draw_hook_chain_extending(state.state());
        }
        HookStateEnum::Contracting(state) => {
            let position = state.position();
            draw(&HOOK_CONTRACTING, position);
            draw_hook_chain_contracting(state.state());
        }
        HookStateEnum::End => (),
    }
}

fn draw_hook_chain_extending(state: &Extending) {
    draw_hook_chain_ovals(state.hook().chain());
}

fn draw_hook_chain_contracting(state: &Contracting) {
    draw_hook_chain_ovals(state.hook().chain());
}

fn draw_hook_chain_ovals(chain: &Chain) {
    let mut x_prev = chain.front().x();
    let mut y_prev = chain.front().y();
    for link in chain.iter().skip(1) {
        let x = link.x();
        let y = link.y();
        let r = HOOK_DIST_END_CONTRACT / 10.0;
        draw_line(x, y, x_prev, y_prev, 5.0, DARKGRAY.into());
        draw_circle_lines(x, y, r, 2.0, GRAY.into());
        x_prev = link.x();
        y_prev = link.y();
    }
}

fn draw_hook_chain_lines(state: &Contracting) {
    let chain = state.hook().chain();
    let mut x_prev = state.position().x();
    let mut y_prev = state.position().y();
    for link in chain.iter() {
        draw_line(link.x(), link.y(), x_prev, y_prev, 1.0, PINK.into());
        x_prev = link.x();
        y_prev = link.y();
    }
}

fn _draw_hook_chain_exponent_graph(position_hook: Position, position_goal: Position) {
    let Vec2 { x: x0, y: y0 } = position_hook.value();
    let Vec2 { x, y } = position_goal.value();

    let f = match _hook_path_function(position_hook, position_goal) {
        Left(f) => f,
        Right(_) => {
            draw_line(x0, y0, x, y, 1.0, PINK.into());
            return;
        }
    };
    let skip = (x - x0) / 10.0;
    let mut xp = x0;
    let mut yp = y0;
    for i in 1..=10 {
        let xi = x0 + (i as f32) * skip;
        let yi = f(xi);
        draw_line(xp, yp, xi, yi, 1.0, PINK.into());
        xp = xi;
        yp = yi;
    }
}

fn draw(shape: &Shape, position: Position) {
    match shape {
        Shape::Rectangle(s) => {
            draw_rectangle(
                position.x(),
                position.y(),
                s.width,
                s.height,
                s.color().into(),
            );
        }
        Shape::Circle(s) => {
            draw_circle(position.x(), position.y(), s.radius.0, s.color().into());
        }
        Shape::Line(s) => todo!(),
        Shape::Point => todo!(),
    }
}

pub const PLAYER_IDLING: Shape = Shape::Circle(Circle {
    radius: Radius(10.0),
    color: BLUE,
});
pub const PLAYER_MOVING: Shape = Shape::Circle(Circle {
    radius: Radius(9.0),
    color: YELLOW,
});
pub const PLAYER_SHOOTING: Shape = Shape::Circle(Circle {
    radius: Radius(10.0),
    color: BLUE,
});
pub const HOOK_EXTENDING: Shape = Shape::Rectangle(Rectangle {
    height: 7.0,
    width: 7.0,
    color: RED,
});
pub const HOOK_CONTRACTING: Shape = Shape::Rectangle(Rectangle {
    height: 7.0,
    width: 7.0,
    color: GREEN,
});
