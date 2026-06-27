use std::collections::hash_map::{Entry, HashMap};

use num_traits::{cast, Float, NumCast, Unsigned};

use super::Configuration;

#[derive(Debug)]
pub enum Indices<I: Unsigned> {
    /// Indices of a triangle list.
    Indexes(Vec<I>),
    /// Indices of triangle strips.
    Strips(Vec<Vec<I>>),
}

#[derive(Debug)]
pub struct Shape<C: Float, I: Unsigned> {
    vertices: Vec<[C; 3]>,
    normals: Vec<[C; 3]>,
    indices: Indices<I>,
}

impl<C, I> Shape<C, I>
where
    C: Float,
    I: Copy + NumCast + Unsigned,
{
    /// Constructs a configured `Shape` from a [`Shaper`].
    ///
    /// # Panics
    ///
    /// May panic if a [`ShapingError`] occurs.
    pub fn new(shape: impl Shaper<C, I>, config: Configuration) -> Self {
        shape.shape(config)
    }

    /// Constructs a triangle list `Shape` without normals.
    ///
    /// # Panics
    ///
    /// May panic if an invalid index is provided.
    pub(in super::super) fn without_normals(vertices: Vec<[C; 3]>, indices: Vec<I>) -> Self {
        Self::validate(vertices.len(), &indices);
        Shape { vertices, normals: vec![], indices: Indices::Indexes(indices) }
    }

    /// Constructs a triangle list `Shape` with normals.
    ///
    /// # Panics
    ///
    /// May panic if an invalid index is provided.
    pub(in super::super) fn with_normals(vertices: Vec<[C; 3]>, normals: Vec<[C; 3]>, indices: Vec<I>) -> Self {
        Self::validate(vertices.len(), &indices);
        Shape { vertices, normals, indices: Indices::Indexes(indices) }
    }

    /// Constructs a `Shape` as triangle strips.
    ///
    /// # Panics
    ///
    /// May panic if an invalid index is provided.
    pub(in super::super) fn as_strips(vertices: Vec<[C; 3]>, strips: Vec<Vec<I>>) -> Self {
        for strip in &strips {
            Self::validate(vertices.len(), strip);
        }
        Shape { vertices, normals: vec![], indices: Indices::Strips(strips) }
    }

    /// Returns `true` if the `Shape`'s `indices` are triangle strips.
    pub fn is_strips(&self) -> bool {
        matches!(self.indices, Indices::Strips(_))
    }

    /// Returns `true` if the `Shape`'s `indices` are a triangle list.
    pub fn is_indexes(&self) -> bool {
        matches!(self.indices, Indices::Indexes(_))
    }

    /// Returns a reference to the vertex positions.
    pub fn vertices(&self) -> &Vec<[C; 3]> {
        &self.vertices
    }

    /// Returns a reference to the normal vectors.
    pub fn normals(&self) -> &Vec<[C; 3]> {
        &self.normals
    }

    /// Returns a list of references to the sets of indices.
    pub fn indices(&self) -> Vec<&Vec<I>> {
        match &self.indices {
            Indices::Indexes(indices) => vec![indices],
            Indices::Strips(strips) => strips.iter().collect(),
        }
    }

    /// Attempts to combine `vertices` and `normals` while generating new `indices`.
    ///
    /// Returns the new data as a [`NormalShape`].
    ///
    /// # Errors
    ///
    /// Returns an error if there is no normal data,
    /// or if the new data cannot be indexed by the index type.
    pub fn normalise(&self) -> Result<NormalShape<C, I>, ShapingError> {
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

    /// Ensures the indices refer to valid indexes.
    fn validate(vertex_count: usize, indices: &[I]) {
        for _ in indices.iter()
            .map(|&i| cast::<I, usize>(i).unwrap())
            .filter(|&i| i >= vertex_count)
            .map(|_| panic!("Invalid index")) {
        }
    }
}

/// A vertex with `position` and `normal` vectors.
#[derive(Debug)]
pub struct NormalVertex<C: Float> {
    pub position: [C; 3],
    pub normal: [C; 3],
}

impl<C: Float> From<NormalVertex<C>> for Vec<[C; 3]> {
    fn from(value: NormalVertex<C>) -> Self {
        vec![value.position, value.normal]
    }
}

#[derive(Debug)]
pub struct NormalShape<C: Float, I: Unsigned> {
    pub vertices: Vec<NormalVertex<C>>,
    pub indices: Vec<I>,
}

/// Errors in making or working with [`Shape`]s.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ShapingError {
    /// Requisite data is missing.
    NoData,
    /// Insufficient range of values in index type.
    IndexOverflow,
}

/// Construct a well-defined [`Shape`] on demand.
pub trait Shaper<C: Float, I: Unsigned> {
    fn shape(&self, request: Configuration) -> Shape<C, I>;
}

