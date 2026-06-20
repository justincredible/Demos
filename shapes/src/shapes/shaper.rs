use num_traits::{Float, Unsigned};

use super::Configuration;

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

    pub fn normals(&self) -> Option<&Vec<[C; 3]>> {
        match self {
            Shape::NormalTriangles { normals, .. } => Some(normals),
            _ => None,
        }
    }

    pub fn indices(&self, strip: Option<usize>) -> &Vec<I> {
        match self {
            Shape::NormalTriangles { indices, .. } => indices,
            Shape::Triangles { indices, .. } => indices,
            Shape::Strips { strips, .. } => &strips[
                std::cmp::min(strips.len()-1, strip.unwrap_or_default())
            ],
        }
    }
}

/// Construct a well-defined [`Shape`] on demand.
pub trait Shaper<C: Float, I: Unsigned> {
    fn shape(&self, request: Configuration) -> Shape<C, I>;
}

