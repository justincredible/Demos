use num_traits::{Float, Unsigned};

use super::Configuration;

pub enum Indices<I: Unsigned> {
    Indexes(Vec<I>),
    Strips(Vec<Vec<I>>),
}

pub struct Shape<C: Float, I: Unsigned> {
    vertices: Vec<[C; 3]>,
    normals: Vec<[C; 3]>,
    indices: Indices<I>,
}

impl<C: Float, I: Unsigned> Shape<C, I> {
    pub fn new(vertices: Vec<[C; 3]>, indices: Vec<I>) -> Self {
        Shape { vertices, normals: vec![], indices: Indices::Indexes(indices) }
    }

    pub fn with_normals(vertices: Vec<[C; 3]>, normals: Vec<[C; 3]>, indices: Vec<I>) -> Self {
        Shape { vertices, normals, indices: Indices::Indexes(indices) }
    }

    pub fn as_strips(vertices: Vec<[C; 3]>, strips: Vec<Vec<I>>) -> Self {
        Shape { vertices, normals: vec![], indices: Indices::Strips(strips) }
    }

    pub fn is_strips(&self) -> bool {
        matches!(self.indices, Indices::Strips(_))
    }

    pub fn is_indexes(&self) -> bool {
        matches!(self.indices, Indices::Indexes(_))
    }

    pub fn vertices(&self) -> &Vec<[C; 3]> {
        &self.vertices
    }

    pub fn normals(&self) -> &Vec<[C; 3]> {
        &self.normals
    }

    pub fn indices(&self) -> Vec<&Vec<I>> {
        match &self.indices {
            Indices::Indexes(indices) => vec![indices],
            Indices::Strips(strips) => strips.iter().collect(),
        }
    }
}

/// Construct a well-defined [`Shape`] on demand.
pub trait Shaper<C: Float, I: Unsigned> {
    fn shape(&self, request: Configuration) -> Shape<C, I>;
}

