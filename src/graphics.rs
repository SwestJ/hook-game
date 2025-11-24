use macroquad::color::Color as ColorExt;
use serde::{Deserialize, Serialize};

pub trait IShape<T: IBuilder<Self>>: Sized {
    fn builder() -> T {
        T::default()
    }
}

pub trait IBuilder<T: IShape<Self>>: Sized + Default {}

#[derive(Serialize, Deserialize, Default, Clone, Copy, Debug)]
pub enum Shape {
    Rectangle(Rectangle),
    Circle(Circle),
    Line(Line),
    #[default]
    Point,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Rectangle {
    pub height: f32,
    pub width: f32,
    pub color: Color,
}
impl Rectangle {
    pub fn color(&self) -> &Color {
        &self.color
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Circle {
    pub radius: Radius,
    pub color: Color,
}
impl Circle {
    pub fn color(&self) -> &Color {
        &self.color
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Line {
    length: f32,
    color: Color,
}

#[derive(Default)]
pub struct LineBuilder {
    length: f32,
    color: Color,
}

#[derive(Default, Serialize, Deserialize, Clone, Copy, Debug)]
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

//* Builder without typestate pattern

impl IShape<LineBuilder> for Line {}
impl IBuilder<Line> for LineBuilder {}

impl LineBuilder {
    pub fn length(self, length: f32) -> Self {
        LineBuilder {
            length,
            color: self.color,
        }
    }

    pub fn color(self, color: Color) -> Self {
        LineBuilder {
            length: self.length,
            color,
        }
    }

    pub fn build(self) -> Line {
        let Self { length, color } = self;
        Line { length, color }
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

#[derive(Default, Serialize, Deserialize, Clone, Copy, Debug)]
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
        CircleBuilder {
            radius: Radius(radius),
            color,
        }
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
        RectangleBuilder {
            height,
            width,
            color: self.color,
            state: Dimensions(()),
        }
    }
}

impl RectangleBuilder<Dimensions> {
    pub fn color(self, color: Color) -> RectangleBuilder<Ready> {
        let Self { height, width, .. } = self;
        RectangleBuilder {
            height,
            width,
            color,
            state: Ready(()),
        }
    }
}

impl RectangleBuilder<Ready> {
    pub fn build(self) -> Rectangle {
        let Self {
            height,
            width,
            color,
            ..
        } = self;
        Rectangle {
            height,
            width,
            color,
        }
    }
}
