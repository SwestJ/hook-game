use std::{
    f32::consts::PI, ops::{Add, Deref, DerefMut}, process::Output
};

use macroquad::{
    color::Color as ColorExt,
    math::{Rect, Vec2},
};
use serde::{Deserialize, Serialize};
use typenum::P5;

use super::*;
use crate::{draw::graphics::player_graphics::PlayerGraphics, model::*};
use hook_graphics::*;
use item_graphics::*;

pub mod hook_graphics;
pub mod item_graphics;
pub mod player_graphics;

pub const THETA0: Angle<Radians> = Angle(Radians(0.0));
pub const THETA1: Angle<Radians> = Angle(Radians(PI / 3.0));
pub const THETA2: Angle<Radians> = Angle(Radians(2.0 * PI / 3.0));
pub const THETA3: Angle<Radians> = Angle(Radians(PI));
pub const THETA4: Angle<Radians> = Angle(Radians(4.0 * PI / 3.0));
pub const THETA5: Angle<Radians> = Angle(Radians(5.0 * PI / 3.0));

pub const UNIT_CIRCLE_RADIUS: f32 = 1.0;
pub const UNIT_TRIANGLE_HEIGHT: f32 = 1.5;
pub const SIN_PI_OVER_3: f32 = 0.866_025_4;
pub const UNIT_TRIANGLE_SLOPE: f32 = SIN_PI_OVER_3 / UNIT_TRIANGLE_HEIGHT;

pub const COS_THETA0: f32 = 1.0;
pub const SIN_THETA0: f32 = 0.0;
pub const COS_THETA1: f32 = 0.5;
pub const SIN_THETA1: f32 = SIN_PI_OVER_3;
pub const COS_THETA2: f32 = -0.5;
pub const SIN_THETA2: f32 = SIN_PI_OVER_3;
pub const COS_THETA3: f32 = -1.0;
pub const SIN_THETA3: f32 = 0.0;
pub const COS_THETA4: f32 = -0.5;
pub const SIN_THETA4: f32 = -SIN_PI_OVER_3;
pub const COS_THETA5: f32 = 0.5;
pub const SIN_THETA5: f32 = -SIN_PI_OVER_3;

/// Position on a circle: P(x = r*cos(theta), y = r*sin(theta))
pub const THETA0_UNIT: Vec2 = Vec2 { x: COS_THETA0, y: SIN_THETA0 };
pub const THETA1_UNIT: Vec2 = Vec2 { x: COS_THETA1, y: SIN_THETA1 };
pub const THETA2_UNIT: Vec2 = Vec2 { x: COS_THETA2, y: SIN_THETA2 };
pub const THETA3_UNIT: Vec2 = Vec2 { x: COS_THETA3, y: SIN_THETA3 };
pub const THETA4_UNIT: Vec2 = Vec2 { x: COS_THETA4, y: SIN_THETA4 };
pub const THETA5_UNIT: Vec2 = Vec2 { x: COS_THETA5, y: SIN_THETA5 };

const fn vertex_y_from_x_a(x: f32, a: f32) -> f32 {
    UNIT_TRIANGLE_SLOPE * x - a * UNIT_TRIANGLE_SLOPE
}
const fn vertex(x: f32, a: f32) -> Vec2 {
    Vec2 { x, y: vertex_y_from_x_a(x, a) }
}
const fn triangle_inner(ix: f32, ia: f32, factor: f32) -> Vertices<3> {
    let mut h = UNIT_TRIANGLE_HEIGHT;
    let x = (ix) * h;
    let a = (ia + ix) * h;
    h = h * factor;
    Vertices([vertex(x, a), vertex(x - h, a), vertex(x - h, a - 2.0 * h)])
}
const fn triangle(ix: i32, ia: i32) -> Vertices<3> {
    if (ix + ia) % 2 == 0 {
        triangle_inner(ix as f32, ia as f32, 1.0)
    } else {
        triangle_inner((ix - 1) as f32, ia as f32, -1.0)
    }
}
const fn triangles<const N: usize>(points: [(f32, f32); N]) -> [Vertices<3>; N] {
    let i = 0;
    while i < points.len() {

    }
    todo!()
}

const fn vertex_graphics_from_triangle_points<const N: usize, const O: usize>(
    array: [(i32, i32); O],
) -> Vertices<N> {
    VerticesBuilder::<N, 3, 0>::new().fill_triangles(array).build()
}

pub trait IShape<T: IBuilder<Self>>: Sized {
    fn builder() -> T {
        T::default()
    }
}

pub trait IBuilder<T: IShape<Self>>: Sized + Default {}

#[allow(clippy::large_enum_variant)]
#[derive(Default, Clone, Debug)]
pub enum Shape {
    HookObject(HookGraphics),
    ItemObject(ItemGraphics),
    PlayerObject(PlayerGraphics),
    Polygon(Polygon),
    Rectangle(Rectangle),
    Triangle(Triangle),
    Circle(Circle),
    Line(Line),
    #[default]
    Point,
}

impl From<Polygon> for Shape {
    fn from(value: Polygon) -> Self {
        Shape::Polygon(value)
    }
}
impl From<Rectangle> for Shape {
    fn from(value: Rectangle) -> Self {
        Shape::Rectangle(value)
    }
}
impl From<Triangle> for Shape {
    fn from(value: Triangle) -> Self {
        Shape::Triangle(value)
    }
}
impl From<Circle> for Shape {
    fn from(value: Circle) -> Self {
        Shape::Circle(value)
    }
}
impl From<Line> for Shape {
    fn from(value: Line) -> Self {
        Shape::Line(value)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Polygon {
    pub radius: f32,
    pub sides: u8,
    pub color: Color,
}

#[derive(Clone, Copy, Debug)]
pub struct Rectangle {
    pub height: f32,
    pub width: f32,
    pub color: Color,
}

#[derive(Clone, Copy, Debug)]
pub struct Triangle {
    pub height: f32,
    pub base: f32,
    pub color: Color,
}
impl Triangle {
    pub fn vertices(&self) -> TriangleVertices {
        TriangleVertices(
            Vec2::new(-self.height / 2.0, self.base / 2.0),
            Vec2::new(-self.height / 2.0, -self.base / 2.0),
            Vec2::new(self.height / 2.0, 0.0),
        )
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TriangleVertices(pub Vec2, pub Vec2, pub Vec2);
impl TriangleVertices {
    pub fn rotate<T: AsRef<Vec2>>(self, vec: T) -> Self {
        let Self(v1, v2, v3) = self;
        let vec = vec.as_ref();
        Self(vec.rotate(v1), vec.rotate(v2), vec.rotate(v3))
    }
    pub fn translate<T: AsRef<Vec2>>(self, vec: T) -> Self {
        let Self(v1, v2, v3) = self;
        let vec = vec.as_ref().to_owned();
        Self(vec + v1, vec + v2, vec + v3)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Vertices<const N: usize>(pub [Vec2; N]);
impl<const N: usize> Vertices<N> {
    pub fn value(&self) -> [Vec2; N] {
        self.0
    }
}
impl<const N: usize> Vertices<N> {
    pub fn rotate_by_angle(self, angle: f32) -> Self {
        self.rotate_const(Vec2::from_angle(angle))
    }
    pub fn rotate<T: AsRef<Vec2>>(self, vector: T) -> Self {
        self.rotate_const(vector.as_ref().to_owned())
    }
    pub const fn rotate_const(mut self, vector: Vec2) -> Self {
        let mut i = 0;
        while i < N {
            self.0[i] = rotate(vector, self.0[i]);
            i += 1;
        }
        self
    }
    pub fn translate<T: AsRef<Vec2>>(self, vector: T) -> Self {
        self.translate_const(vector.as_ref().to_owned())
    }
    pub const fn translate_const(mut self, vector: Vec2) -> Self {
        let mut i = 0;
        while i < N {
            self.0[i] = translate(vector, self.0[i]);
            i += 1;
        }
        self
    }
    pub const fn scale_const(mut self, factor: f32) -> Self {
        let mut i = 0;
        while i < N {
            self.0[i] = Vec2 { x: self.0[i].x * factor, y: self.0[i].y * factor };
            i += 1;
        }
        self
    }
    pub fn scale(self, factor: f32) -> Self {
        Self(self.map(|v| v * factor))
    }
}
impl<const N: usize> Deref for Vertices<N> {
    type Target = [Vec2; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<const N: usize> DerefMut for Vertices<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct VerticesBuilder<const N: usize, const M: usize, const I: usize> {
    vertices: Vertices<N>,
    index: usize,
}

pub struct VerticesBuilder2<const N: usize, const M: usize> {
    vertices: Vertices<N>,
    index: usize,
}

impl<const N: usize, const M: usize> VerticesBuilder2<N, M> {
    pub const fn fill<const O: usize>(
        mut self,
        from: [Vertices<M>; O],
    ) -> VerticesBuilder<N, M, N> {
        let mut vertices = Vertices([Vec2::NAN; N]);
        let mut o = 0;
        while o < O {
            let mut m = 0;
            while m < M {
                vertices.0[m + o * M] = from[o].0[m];
                m += 1;
            }
            o += 1;
        }
        VerticesBuilder { vertices: self.vertices, index: N }
    }
}

impl<const N: usize, const M: usize> VerticesBuilder<N, M, 0> {
    pub const fn new() -> Self {
        VerticesBuilder { vertices: Vertices([Vec2::NAN; N]), index: 0 }
    }
}

pub trait GenericSize<const I: usize> {
    const INDEX: usize;
    type Item;

    fn insert<const J: usize>(self, from: Self::Item) -> impl GenericSize<J>;
}

pub struct _VerticesBuilder<const N: usize, T: IndexState> {
    vertices: Vertices<N>,
    index: T,
}
pub trait IndexState {}
pub struct IndexZero();
impl IndexState for IndexZero {}
pub struct IndexNonZero();
impl IndexState for IndexNonZero {}
pub struct IndexLast();
impl IndexState for IndexLast {}
// pub trait GenSize {
//     type Index: IndexState;
//     type Next: IndexState;
// }

pub trait Insert {
    type Item;
    type Index: IndexState;
    type Next: IndexState;
    type Output: Insert<Index = Self::Next>;
    fn insert(self, _: Self::Item) -> Output;
}
// impl<const N: usize, T> GenSize for _VerticesBuilder<N, T> {
//     type Index = IndexZero;
//     type Next = IndexNonZero;
// }
impl<const N: usize> Insert for _VerticesBuilder<N, IndexZero> {
    type Item = Vertices<3>;
    type Index = IndexZero;
    type Next = IndexNonZero;
    type Output = _VerticesBuilder<N, IndexNonZero>;

    fn insert(self, _: Self::Item) -> Output {
        todo!()
    }
}
impl<const N: usize> Insert for _VerticesBuilder<N, IndexNonZero> {
    type Item = Vertices<3>;
    type Index = IndexNonZero;
    type Next = IndexLast;
    type Output = _VerticesBuilder<N, IndexLast>;

    fn insert(self, _: Self::Item) -> Output {
        todo!()
    }
}
impl<const N: usize> Insert for _VerticesBuilder<N, IndexLast> {
    type Item = Vertices<3>;
    type Index = IndexLast;
    type Next = IndexLast;
    type Output = _VerticesBuilder<N, IndexLast>;

    fn insert(self, _: Self::Item) -> Output {
        todo!()
    }
}

impl<const N: usize, const M: usize, const I: usize> VerticesBuilder<N, M, I> {
    pub const fn insert<const J: usize>(mut self, from: Vertices<M>) -> VerticesBuilder<N, M, J> {
        assert!(I + M <= N, "Index would be out-of-bounds");
        assert!(I == self.index, "Index does not match expected value");
        assert!(J == M + self.index, "Return index does not match expected value");
        let mut i = 0;
        while i < M {
            self.vertices.0[I + i] = from.0[i];
            i += 1;
        }
        VerticesBuilder { vertices: self.vertices, index: J }
    }

    pub const fn fill<const O: usize>(
        mut self,
        from: [Vertices<M>; O],
    ) -> VerticesBuilder<N, M, N> {
        let mut o = 0;
        while o < O {
            let mut m = 0;
            while m < M {
                self.vertices.0[m + o * M] = from[o].0[m];
                m += 1;
            }
            o += 1;
        }
        VerticesBuilder { vertices: self.vertices, index: N }
    }
}
impl<const N: usize, const I: usize> VerticesBuilder<N, 3, I> {
    pub const fn fill_triangles<const O: usize>(
        mut self,
        from: [(i32, i32); O],
    ) -> VerticesBuilder<N, 3, N> {
        let mut o = 0;
        while o < O {
            let mut m = 0;
            let v = triangle(from[o].0, from[o].1);
            while m < 3 {
                self.vertices.0[m + o * 3] = v.0[m];
                m += 1;
            }
            o += 1;
        }
        VerticesBuilder { vertices: self.vertices, index: N }
    }
}

// impl<const N: usize, const I: usize> VerticesBuilder<N, I> {
//     pub const fn insert<const M: usize, const J: usize>(mut self, from: Vertices<M>) -> VerticesBuilder<N, J> {
//         assert!(I + M <= N);
//         assert!(J == I + M);

//         let mut i = 0;
//         while i < M {
//             self.vertices.0[I + i] = from.0[i];
//             i += 1;
//         }
//         VerticesBuilder { vertices: self.vertices, index: J }
//     }
// }

impl<const N: usize, const M: usize> VerticesBuilder<N, M, N> {
    pub const fn build(self) -> Vertices<N> {
        self.vertices
    }
}

pub struct V2(pub Vec2);
impl AsRef<Vec2> for V2 {
    fn as_ref(&self) -> &Vec2 {
        &self.0
    }
}
const fn rotate(lhs: Vec2, rhs: Vec2) -> Vec2 {
    Vec2 { x: lhs.x * rhs.x - lhs.y * rhs.y, y: lhs.y * rhs.x + lhs.x * rhs.y }
}
const fn translate(lhs: Vec2, rhs: Vec2) -> Vec2 {
    Vec2 { x: lhs.x + rhs.x, y: lhs.y + rhs.y }
}
// impl Vertices<3> {
//     pub fn rotate<T: AsRef<Vec2>>(self, vec: T) -> Self {
//         let Self([v1, v2, v3]) = self;
//         let vec = vec.as_ref();
//         Self ([
//             vec.rotate(v1),
//             vec.rotate(v2),
//             vec.rotate(v3),
//         ])
//     }
//     pub fn translate<T: AsRef<Vec2>>(self, vec: T) -> Self {
//         let Self([v1, v2, v3]) = self;
//         let vec = vec.as_ref().to_owned();
//         Self ([
//             vec + v1,
//             vec + v2,
//             vec + v3,
//         ])
//     }
// }

#[derive(Clone, Copy, Debug)]
pub struct TriangleEquilateral {
    pub side: f32,
    pub color: Color,
}
impl TriangleEquilateral {
    pub fn vertices(&self) {}
}

#[derive(Clone, Copy, Debug)]
pub struct Circle {
    pub radius: Radius,
    pub color: Color,
}
impl Circle {
    pub fn color(&self) -> &Color {
        &self.color
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Line {
    pub length: f32,
    pub thickness: f32,
    pub color: Color,
}

#[derive(Default, Clone, Copy, Debug)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Color {
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color { r, g, b, a }
    }
}

impl From<&Color> for ColorExt {
    fn from(color: &Color) -> Self {
        let Color { r, g, b, a } = *color;
        ColorExt { r, g, b, a }
    }
}

impl From<Color> for ColorExt {
    fn from(color: Color) -> Self {
        let Color { r, g, b, a } = color;
        ColorExt { r, g, b, a }
    }
}

//* Typestate pattern using the "Builder<A, B>" approach
/*
    - In this approach the generic types used for state are more closely coupled with the actual model.
    Radius and Color can be used in the type being build, instead of the primitives types.
    However it might be cleaner to use the primitive types if they are hidden behind an API anyway.
    - Allows the client to chain the builder function calls in a freely chosen order
    - If the number of needed fields in builder/target increases, it seems that there needs to be
    a generic type representing each (i.e. <A, B, C, ..>).
*/

#[derive(Default)]
pub struct CircleBuilder<R, C> {
    radius: R,
    color: C,
}

#[derive(Default, Clone, Copy, Debug)]
pub struct Radius(pub f32);
impl From<f32> for Radius {
    fn from(value: f32) -> Self {
        Radius(value)
    }
}

// Giving a single zero-sized private field makes the types unconstructible
// outside this module. This limits the API surface area and ensures
// that clients do not accidentally create instances of the types.
#[derive(Default)]
pub struct NoRadius(());
#[derive(Default)]
pub struct NoColor(());

impl IShape<CircleBuilder<NoRadius, NoColor>> for Circle {}

impl IBuilder<Circle> for CircleBuilder<NoRadius, NoColor> {}

impl<C> CircleBuilder<NoRadius, C> {
    pub fn radius(self, radius: f32) -> CircleBuilder<Radius, C> {
        let Self { color, .. } = self;
        CircleBuilder { radius: Radius(radius), color }
    }
}

impl<R> CircleBuilder<R, NoColor> {
    pub fn color(self, color: Color) -> CircleBuilder<R, Color> {
        let Self { radius, .. } = self;
        CircleBuilder { radius, color }
    }
}

impl CircleBuilder<Radius, Color> {
    pub fn build(self) -> Circle {
        let Self { radius, color } = self;
        Circle { radius, color }
    }
}

//* Typestate pattern using the "Builder<S>" approach
/*
    - In this approach the genetic type represents the state of the builder.
    - The state types can be used to include data when transitioning between states.
    However it is not needed in this example, so the "state" field could have been a PhantomData marker type.
    - The Builder<A,B> approach seems better suited for builder patterns, but Builder<S> does have the
    advantage that it does not need generic types for each field.
    - The Builder<S> approach could be used in various game states.
        - Pause/Menu/Playing
        - Walking/Falling/Jumping
*/

#[derive(Default)]
pub struct RectangleBuilder<S> {
    height: f32,
    width: f32,
    color: Color,
    state: S,
}

#[derive(Default)]
pub struct Start(());
pub struct Dimensions(());

pub struct Ready(());

impl IShape<RectangleBuilder<Start>> for Rectangle {}
impl IBuilder<Rectangle> for RectangleBuilder<Start> {}

impl RectangleBuilder<Start> {
    pub fn dimensions(self, width: f32, height: f32) -> RectangleBuilder<Dimensions> {
        RectangleBuilder { height, width, color: self.color, state: Dimensions(()) }
    }
}

impl RectangleBuilder<Dimensions> {
    pub fn color(self, color: Color) -> RectangleBuilder<Ready> {
        let Self { height, width, .. } = self;
        RectangleBuilder { height, width, color, state: Ready(()) }
    }
}

impl RectangleBuilder<Ready> {
    pub fn build(self) -> Rectangle {
        let Self { height, width, color, .. } = self;
        Rectangle { height, width, color }
    }
}
