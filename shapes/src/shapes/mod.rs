use num_traits::{Float, Unsigned};

pub enum Shape<C: Float, I: Unsigned> {
    NormalTriangles {
        vertices: Vec<[C; 3]>,
        normals: Vec<[C; 3]>,
        indices: Vec<I>,
    },
    Triangles {
        vertices: Vec<[C; 3]>,
        indices: Vec<I>,
    },
    Strips {
        vertices: Vec<[C; 3]>,
        strips: Vec<Vec<I>>,
    },
}

#[derive(Default, Clone, Copy)]
pub enum Chirality {
    Left,
    #[default]
    Right,
}

#[derive(Default, Clone, Copy)]
pub enum Winding {
    Clockwise,
    #[default]
    CounterClockwise,
}

#[derive(Default, Clone, Copy)]
pub struct Orientation {
    chirality: Chirality,
    winding: Winding,
}

#[derive(Default, Clone, Copy)]
pub struct Configuration {
    orientation: Orientation,
    generate_normals: bool,
    prefer_strips: bool,
}

pub trait Shaper<C: Float, I: Unsigned> {
    fn make(&self, _request: Configuration) -> Shape<C, I> {
        Shape::Triangles { vertices: vec!(), indices: vec!() }
    }
}

