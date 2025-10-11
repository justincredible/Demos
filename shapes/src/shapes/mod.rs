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

impl<C: Float, I: Unsigned> Shape<C, I> {
    pub fn vertices(&self) -> &Vec<[C; 3]> {
        match self {
            Shape::NormalTriangles { vertices, .. } => vertices,
            Shape::Triangles { vertices, .. } => vertices,
            Shape::Strips { vertices, .. } => vertices,
        }
    }
}

/// The handedness of the coordinate system, with:
/// <br>&emsp;+X rightward
/// <br>&emsp;+Y upward
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum Chirality {
    /// +Z forward
    Left,
    /// +Z backward
    #[default]
    Right,
}

/// Which direction defines the front face.
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum Winding {
    Clockwise,
    #[default]
    Counterclockwise,
}

/// Defines the [`Shape`]'s local/object space.
///
/// Generally not sufficient for direct use in the Normalized Device Coordinates (NDC) space.
#[derive(Default, Clone, Copy)]
pub struct Orientation {
    /// Whether the coordinate space is left- or right-handed.
    pub chirality: Chirality,
    /// How the vertex order defines the front and back faces.
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

/// Passed to [`Shaper::make()`] to configure the returned [`Shape`].
#[derive(Default, Clone, Copy)]
pub struct Configuration {
    /// Controls the coordinate space [`Chirality`] and primitive [`Winding`] order.
    pub orientation: Orientation,
    /// Whether the [`Shape`] should include normal data.
    pub generate_normals: bool,
    /// Provides index data as triangle strips, if possible.
    pub prefer_strips: bool,
}

/// Construct a well-defined [`Shape`] on demand.
pub trait Shaper<C: Float, I: Unsigned> {
    fn make(&self, _request: Configuration) -> Shape<C, I> {
        Shape::Triangles { vertices: vec!(), indices: vec!() }
    }
}

