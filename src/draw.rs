use std::ops::Deref;

use either::Either::*;
use macroquad::math::Vec2;
use macroquad::prelude as mq;

use crate::DEBUG_DRAW_ORIGIN_FACTOR;
use crate::collision;
use crate::model::*;
use crate::state::StateMachine;
use crate::state::StateMachineEnum;
use crate::state::StateObject;
use colors::*;
use graphics::*;

pub mod colors;
pub mod graphics;

//* Drawing */
pub fn draw_states(states: &[StateMachineEnum]) {
    let origin = Position::from_vec(Vec2::new(mq::screen_width() / 2.0, mq::screen_height() / 2.0));
    let drawables: Vec<Drawable> = states.iter().flat_map(StateMachineEnum::drawable).collect();
    drawables.into_iter().for_each(draw_drawable);
}

pub struct Drawable {
    pub state: StateObject,
    pub shape: Shape,
}

pub trait Draw: StateMachine {
    fn drawable(&self) -> Vec<Drawable>;
}

fn draw_drawable(Drawable { state, shape }: Drawable) {
    let StateObject { position, direction } = state;
    match shape {
        Shape::Rectangle(rectangle) => draw_rectangle(rectangle, position, direction),
        Shape::Circle(s) => draw_circle(s, position),
        Shape::Line(line) => draw_line(line, position, direction),
        Shape::Polygon(polygon) => draw_polygon(polygon, position, direction),
        Shape::Triangle(triangle) => draw_triangle(triangle, position, direction),
        Shape::HookObject(hook) => draw_vertex_graphics(hook.model.rotate(direction).translate(position), hook.color),
        Shape::ItemObject(item) => draw_vertex_graphics(item.model.rotate(direction).translate(position), item.color),
        Shape::PlayerObject(player) => draw_vertex_graphics(player.model.rotate(direction).translate(position), player.color),
        Shape::Point => (),
    }
}

fn draw_circle(s: Circle, position: Position) {
    mq::draw_circle(position.x(), position.y(), s.radius.0, s.color().into());
}

fn draw_polygon(polygon: Polygon, position: Position, direction: Direction) {
    let Polygon { radius, sides, color } = polygon;
    let rotation = Angle::<Degrees>::from(direction);
    mq::draw_poly(position.x(), position.y(), sides, radius, rotation.into(), color.into());
}

fn draw_line(line: Line, position: Position, direction: Direction) {
    let Line {
        length,
        thickness,
        color,
    } = line;
    let position2 = position.move_in_direction(direction, length);
    mq::draw_line(
        position.x(),
        position.y(),
        position2.x(),
        position2.y(),
        thickness,
        color.into(),
    );
}

fn draw_rectangle(rectangle: Rectangle, position: Position, direction: Direction) {
    let Rectangle { height, width, color } = rectangle;
    mq::draw_rectangle(
        position.x() - width / 2.0,
        position.y() - height / 2.0,
        width,
        height,
        color.into(),
    );
}

fn draw_triangle(triangle: Triangle, position: Position, direction: Direction) {
    let vertices = triangle.vertices().rotate(direction).translate(position);
    mq::draw_triangle(vertices.0, vertices.1, vertices.2, triangle.color.into());
}

fn draw_vertex_graphics<const N: usize>(vertices: Vertices<N>, color: Color) {
    let mut it = vertices.chunks_exact(3);
    while let Some(&[v1, v2, v3]) = it.next() {
        mq::draw_triangle(v1, v2, v3, color.into());
    }
}

pub fn debug_draw_grid() {
    let origin = Vec2::new(mq::screen_width(), mq::screen_height()) * DEBUG_DRAW_ORIGIN_FACTOR;
    let length = 10.0;
    let di = 50.0;
    let mut i = 0.0;
    while i < mq::screen_width() {
        mq::draw_line(i, origin.y + length, i, origin.y - length, 0.5, YELLOW.into());
        mq::draw_line(origin.x + length, i, origin.x - length, i, 0.5, YELLOW.into());
        i += di;
    }

    mq::draw_line(0.0, origin.y, mq::screen_width(), origin.y, 0.8, RED.into());
    mq::draw_line(origin.x, 0.0, origin.x, mq::screen_height(), 0.8, RED.into());
    mq::draw_text(
        format!("Origin ({}, {})", origin.x, origin.y).as_str(),
        origin.x + 40.0,
        origin.y + 40.0,
        20.0,
        RED.into(),
    );
}

pub fn debug_draw_state_text(states: &[StateMachineEnum]) {
    mq::draw_fps();
    let debug_text = states.iter().fold(String::new(), |acc, s| format!("{}\n{}", acc, s));
    mq::draw_multiline_text(debug_text.as_str(), 20.0, 20.0, 20.0, None, macroquad::color::RED);
}

pub fn debug_draw_collision_boxes(boxes: &[collision::CollisionBox], color: Color) {
    boxes.iter().for_each(|b| {
        draw_collision_box(b, color);
    });
}
pub fn debug_draw_collided_boxes(boxes: &[(collision::CollisionBox, collision::CollisionBox)], color: Color) {
    boxes.iter().for_each(|p| {
        draw_collision_box(&p.0, color);
        draw_collision_box(&p.1, color);
    });
}

pub fn draw_collision_box(collision::CollisionBox {lower, upper}: &collision::CollisionBox, color: Color) {
    mq::draw_rectangle_lines(
        lower.x,
        lower.y,
        upper.x - lower.x,
        upper.y - lower.y,
        5.0,
        color.into(),
    )
}

mod _dev_vertices {
    use std::ops::Deref;

    fn example() -> _Triangle {
        Vertices([V2::new(1.0, 2.0), V2::new(1.0, 2.0), V2::new(1.0, 2.0)])
    }

    type V2 = Vertex<2>;
    type V3 = Vertex<3>;
    type _Triangle = Vertices<3, 2>;

    #[derive(Debug, Copy, Clone)]
    struct Vertex<const N: usize>([f32; N]);
    impl<const N: usize> Vertex<N> {
        pub fn vertex(&self) -> [f32; N] {
            self.0
        }
        fn v(&self) -> [f32; N] {
            self.0
        }
        pub fn x(&self) -> f32 {
            self[0]
        }
    }
    impl Vertex<2> {
        pub fn new(x: f32, y: f32) -> Self {
            Vertex([x, y])
        }
        pub fn y(&self) -> f32 {
            self[1]
        }
    }
    impl Vertex<3> {
        pub fn new(x: f32, y: f32, z: f32) -> Self {
            Vertex([x, y, z])
        }
        pub fn z(&self) -> f32 {
            self[2]
        }
    }

    impl<const N: usize> Deref for Vertex<N> {
        type Target = [f32; N];

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    struct Vertices<const N: usize, const M: usize>([Vertex<M>; N]);
    impl<const N: usize, const M: usize> Vertices<N, M> {
        fn value(&self) -> [Vertex<M>; N] {
            self.0
        }
    }
}

mod _dev_vertices2 {
    use typenum::P2;

    trait VertexN {
        type N;
    }

    struct Vertex2(f32, f32);
    impl VertexN for Vertex2 {
        type N = usize;
    }

    #[derive(Debug, Copy, Clone)]
    struct Vertices<const N: usize, T: VertexN>([T; N]);
    impl<const N: usize, T: VertexN> Vertices<N, T> {
        fn value(&self) -> &[T; N] {
            &self.0
        }
    }
}
