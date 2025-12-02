use std::f32::consts::PI;
use std::num::NonZero;
use std::ops::Deref;

use either::Either::*;
use macroquad::math::Vec2;
use macroquad::prelude as mq;
use typenum::*;

use crate::DEBUG_DRAW_ORIGIN_FACTOR;
use crate::{
    colors::*,
    graphics::*,
    model::*,
    state::{
        hook::{state_hook::*, *},
        player::{state_player::*, *},
        *,
    },
};

pub const PLAYER_IDLING: Circle = Circle { radius: Radius(10.0), color: BLUE };
pub const PLAYER_MOVING: Circle = Circle { radius: Radius(9.0), color: BLUE };
pub const PLAYER_SHOOTING: Circle = Circle { radius: Radius(10.0), color: BLUE };
pub const HOOK_EXTENDING: Triangle = Triangle { height: 40.0, base: 20.0, color: PURPLE };
pub const HOOK_CONTRACTING: Triangle = Triangle { height: 40.0, base: 20.0, color: PURPLE };
pub const HOOK_LINK: Line = Line { length: 5.0, thickness: 5.0, color: DARKGRAY };
pub const HOOK_LINK_VERTEX: Circle = Circle { radius: Radius(5.0), color: GRAY };

pub const HOOK_GRAPHICS: Vertices<HOOK_NUMBER_OF_VERTICES> =
    create_vertex_graphics(HOOK_GRAPHICS_ARRAY).rotate_const(THETA1_UNIT);

const HOOK_GRAPHICS_ARRAY: [Vertices<3>; 14] = [
    vertices(0, 0),
    vertices(0, 1),
    vertices(0, 2),
    vertices(0, 3),
    vertices(1, 3),
    vertices(1, 4),
    vertices(2, 4),
    vertices(2, 5),
    vertices(2, 6),
    vertices(2, 7),
    vertices(2, 8),
    vertices(1, 8),
    vertices(1, 7),
    vertices(0, 7),
];

const HOOK_NUMBER_OF_VERTICES: usize = HOOK_GRAPHICS_ARRAY.len() * HOOK_GRAPHICS_ARRAY[0].0.len();
const fn create_vertex_graphics<const N: usize, const M: usize, const O: usize>(
    array: [Vertices<M>; O],
) -> Vertices<N> {
    VerticesBuilder::<N, M, 0>::new().fill(array).build()
}

//* Drawing */
pub fn draw_states(states: &[StateEnum]) {
    let origin = Position::from_vec(Vec2::new(mq::screen_width() / 2.0, mq::screen_height() / 2.0));
    // draw_hook_graphics(origin, RIGHT);
    let drawables: Vec<Drawable> = states.iter().flat_map(Vec::<Drawable>::from).collect();
    drawables.into_iter().for_each(draw_drawable);
}

struct Drawable {
    position: Position,
    direction: Direction,
    shape: Shape,
}

impl From<&StateEnum> for Vec<Drawable> {
    fn from(state: &StateEnum) -> Self {
        match state {
            StateEnum::Player(player_state_enum) => player_state_enum.into(),
            StateEnum::Default => todo!(),
        }
    }
}

impl From<&PlayerStateEnum> for Vec<Drawable> {
    fn from(value: &PlayerStateEnum) -> Self {
        match value {
            PlayerStateEnum::Idling(state) => {
                vec![Drawable {
                    position: state.position(),
                    direction: state.direction(),
                    shape: PLAYER_IDLING.into(),
                }]
            }
            PlayerStateEnum::Moving(state) => {
                vec![Drawable {
                    position: state.position(),
                    direction: state.direction(),
                    shape: PLAYER_MOVING.into(),
                }]
            }
            PlayerStateEnum::Shooting(state) => {
                state.state().hook();
                let mut vec = vec![Drawable {
                    position: state.position(),
                    direction: state.direction(),
                    shape: PLAYER_SHOOTING.into(),
                }];
                vec.append(&mut Vec::<Drawable>::from(state.state().hook()));
                vec
            }
            PlayerStateEnum::DualityIdlingShooting(state) => {
                let mut vec = vec![Drawable {
                    position: state.position(),
                    direction: state.direction(),
                    shape: PLAYER_IDLING.into(),
                }];
                vec.append(&mut Vec::<Drawable>::from(state.state().yang().hook()));
                vec
            }
            PlayerStateEnum::DualityMovingShooting(state) => {
                let mut vec = vec![Drawable {
                    position: state.position(),
                    direction: state.direction(),
                    shape: PLAYER_MOVING.into(),
                }];
                vec.append(&mut Vec::<Drawable>::from(state.state().yang().hook()));
                vec
            }
        }
    }
}

impl From<&HookStateEnum> for Vec<Drawable> {
    fn from(state: &HookStateEnum) -> Self {
        match state {
            HookStateEnum::Extending(hook_state_machine) => {
                let mut vec = vec![Drawable {
                    position: hook_state_machine.state().chain().head().position(),
                    direction: hook_state_machine.state().chain().head_direction(),
                    shape: HOOK_EXTENDING.into(),
                }];
                vec.append(&mut Vec::<Drawable>::from(hook_state_machine.state().chain()));
                vec
            }
            HookStateEnum::Contracting(hook_state_machine) => {
                let mut vec = vec![Drawable {
                    position: hook_state_machine.state().chain().head().position(),
                    direction: hook_state_machine.state().chain().head_direction(),
                    shape: HOOK_CONTRACTING.into(),
                }];
                vec.append(&mut Vec::<Drawable>::from(hook_state_machine.state().chain()));
                vec
            }
            HookStateEnum::End => {
                vec![]
            }
        }
    }
}

impl From<&Chain> for Vec<Drawable> {
    fn from(chain: &Chain) -> Self {
        hook_chain_as_drawables(chain)
    }
}

fn hook_chain_as_drawables(chain: &Chain) -> Vec<Drawable> {
    let mut drawables: Vec<Drawable> = vec![];
    let mut link_shape = HOOK_LINK;
    let mut it = chain.chain().iter_full();
    let it_clone = it.clone();
    let mut prev = it.next().unwrap();
    for link in it_clone.skip(1) {
        link_shape.length = link.distance(prev);
        drawables.push(Drawable {
            position: link.position(),
            direction: link.direction(prev),
            shape: link_shape.into(),
        });
        prev = link;
    }

    for link in it {
        drawables.push(Drawable {
            position: link.position(),
            direction: Direction::default(),
            shape: HOOK_LINK_VERTEX.into(),
        });
    }
    drawables
}

fn draw_drawable(Drawable { position, direction, shape }: Drawable) {
    match shape {
        Shape::Rectangle(rectangle) => draw_rectangle(rectangle, position, direction),
        Shape::Circle(s) => draw_circle(s, position),
        Shape::Line(line) => draw_line(line, position, direction),
        Shape::Polygon(polygon) => draw_polygon(polygon, position, direction),
        Shape::Triangle(triangle) => draw_triangle(triangle, position, direction),
        Shape::Point => todo!(),
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
    let Line { length, thickness, color } = line;
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
    // let vertices = triangle.vertices().rotate(direction).translate(position);
    // mq::draw_triangle(vertices.0, vertices.1, vertices.2, triangle.color.into());
    draw_hook_graphics(position, direction);
}

fn draw_hook_graphics(position: Position, direction: Direction) {
    let vs = HOOK_GRAPHICS.scale(5.0).rotate(direction).translate(position);
    let mut it = vs.chunks_exact(3);
    while let Some(&[v1, v2, v3]) = it.next() {
        mq::draw_triangle(v1, v2, v3, GRAY.into());
    }
}

const _DEV_HOOK_NUMBER_OF_VERTICES: usize = 9;
const _DEV_HOOK: Vertices<9> = _dev_v2_create_hook_graphics();
const _DEV_HOOK_INDEXES: [usize; _DEV_HOOK_NUMBER_OF_VERTICES / 3 + 1] = [0, 3, 6, 9];
const _DEV_VERTEX_SIZE: usize = 3;
const fn _dev_v2_create_hook_graphics<const N: usize>() -> Vertices<N> {
    let mut index: usize = 0;
    VerticesBuilder::<N, 3, 0>::new()
        .insert::<3>(UNIT_TRIANGLE)
        .insert::<6>(UNIT_TRIANGLE)
        // .insert::<9>(UNIT_TRIANGLE, &mut index)
        .insert(UNIT_TRIANGLE)
        .build()
}

/// Position on a circle: P(x = r*cos(theta), y = r*sin(theta))
fn unit_triangle() -> Vertices<3> {
    let mut theta = 0.0;
    let theta_incr = 2.0 * PI / 3.0;
    let v1 = Vec2::new(f32::cos(theta), f32::sin(theta));
    theta += theta_incr;
    let v2 = Vec2::new(f32::cos(theta), f32::sin(theta));
    theta += theta_incr;
    let v3 = Vec2::new(f32::cos(theta), f32::sin(theta));
    Vertices([v1, v2, v3])
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

pub fn debug_draw_state_text(states: &[StateEnum]) {
    mq::draw_fps();
    let debug_text = states.iter().fold(String::new(), |acc, s| format!("{}\n{}", acc, s));
    mq::draw_multiline_text(debug_text.as_str(), 20.0, 20.0, 20.0, None, macroquad::color::RED);
}

mod sandbox_vertices {
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

mod sandbox_vertices2 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_triangle_vertices_count() {
        let triangle = unit_triangle();
        assert_eq!(triangle.0.len(), 3);
    }

    #[test]
    fn test_unit_triangle_on_unit_circle() {
        let triangle = unit_triangle();
        for vertex in triangle.0.iter() {
            let magnitude = (vertex.x * vertex.x + vertex.y * vertex.y).sqrt();
            assert!((magnitude - 1.0).abs() < 0.0001);
        }
    }

    #[test]
    fn test_hook_graphics() {
        let hook_graphics: Vertices<HOOK_NUMBER_OF_VERTICES> =
            create_vertex_graphics(HOOK_GRAPHICS_ARRAY);
        println!("{:?}", hook_graphics)
    }
}
