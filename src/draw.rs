use either::Either::*;
use macroquad::{math::Vec2, shapes::*};

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
            let position = state.state().chain().head().position();
            draw_hook_chain_extending(state.state());
            draw(&HOOK_EXTENDING, position);
        }
        HookStateEnum::Contracting(state) => {
            let position = state.state().chain().head().position();
            draw_hook_chain_contracting(state.state());
            draw(&HOOK_CONTRACTING, position);
        }
        HookStateEnum::End => (),
    }
}

fn draw_hook_chain_extending(state: &Extending) {
    draw_hook_chain_ovals(state.chain());
}

fn draw_hook_chain_contracting(state: &Contracting) {
    draw_hook_chain_ovals(state.chain());
}

fn draw_hook_chain_ovals(chain: &Chain) {
    let mut it = chain.chain().iter_full();

    let it_clone = it.clone();
    let mut prev = it.next().unwrap();
    for link in it_clone.skip(1) {
        draw_line(link.x(), link.y(), prev.x(), prev.y(), 5.0, DARKGRAY.into());
        prev = link;
    }

    for link in it {
        draw_circle(link.x(), link.y(), 5.0, GRAY.into());
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
                position.x() - s.width / 2.0,
                position.y() - s.height / 2.0,
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
    color: BLUE,
});
pub const PLAYER_SHOOTING: Shape = Shape::Circle(Circle {
    radius: Radius(10.0),
    color: BLUE,
});
pub const HOOK_EXTENDING: Shape = Shape::Rectangle(Rectangle {
    height: 10.0,
    width: 10.0,
    color: GRAY,
});
pub const HOOK_CONTRACTING: Shape = Shape::Rectangle(Rectangle {
    height: 10.0,
    width: 10.0,
    color: GRAY,
});
