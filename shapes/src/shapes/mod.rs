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

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum Chirality {
    Left,
    #[default]
    Right,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum Winding {
    Clockwise,
    #[default]
    Counterclockwise,
}

#[derive(Default, Clone, Copy)]
pub struct Orientation {
    pub chirality: Chirality,
    pub winding: Winding,
}

impl Orientation {
    pub fn is_left(&self) -> bool {
        self.chirality == Chirality::Left
    }

    pub fn is_right(&self) -> bool {
        self.chirality == Chirality::Right
    }

    pub fn is_cw(&self) -> bool {
        self.winding == Winding::Clockwise
    }

    pub fn is_ccw(&self) -> bool {
        self.winding == Winding::Counterclockwise
    }
}

#[derive(Default, Clone, Copy)]
pub struct Configuration {
    pub orientation: Orientation,
    pub generate_normals: bool,
    pub prefer_strips: bool,
}

pub trait Shaper<C: Float, I: Unsigned> {
    fn make(&self, _request: Configuration) -> Shape<C, I> {
        Shape::Triangles { vertices: vec!(), indices: vec!() }
    }
}

