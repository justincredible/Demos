use std::collections::hash_map::{Entry, HashMap};

use num_traits::{cast, Float, NumCast, Unsigned};

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

impl<C, I> Shape<C, I>
where
    C: Float,
    I: Unsigned + NumCast + Copy,
{
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

    pub fn indexed_normal_vertices(&self) -> Result<NormalShape<C, I>, ShapingError> {
        if self.normals().is_empty() {
            Err(ShapingError::NoData)
        } else {
            let indices_per_normal = self.indices()[0].len() / self.normals().len();
            let mut new_indices = vec![];
            let mut new_vertices = vec![];
            let mut normal_index = 0;
            let mut map = HashMap::new();
            for (index_index, index) in self.indices()[0].iter().enumerate() {
                if index_index > 0 && index_index.is_multiple_of(indices_per_normal) {
                    normal_index += 1;
                }

                let usz_idx = cast::<I, usize>(*index).unwrap();
                let key = (usz_idx, normal_index);
                if let Entry::Vacant(e) = map.entry(key) {
                    e.insert(new_vertices.len());
                    new_vertices.push(NormalVertex {
                        position: self.vertices()[usz_idx],
                        normal: self.normals()[normal_index],
                    });
                }
                new_indices.push(cast::<usize, I>(map[&key]).ok_or(ShapingError::IndexOverflow)?);
            }

            Ok(NormalShape { vertices: new_vertices, indices: new_indices })
        }
    }
}

pub struct NormalVertex<C: Float> {
    pub position: [C; 3],
    pub normal: [C; 3],
}

impl<C: Float> From<NormalVertex<C>> for Vec<[C; 3]> {
    fn from(value: NormalVertex<C>) -> Self {
        vec![value.position, value.normal]
    }
}

pub struct NormalShape<C: Float, I: Unsigned> {
    pub vertices: Vec<NormalVertex<C>>,
    pub indices: Vec<I>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ShapingError {
    NoData,
    IndexOverflow,
}

/// Construct a well-defined [`Shape`] on demand.
pub trait Shaper<C: Float, I: Unsigned> {
    fn shape(&self, request: Configuration) -> Shape<C, I>;
}

