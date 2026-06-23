use num_traits::Float;

use super::linear_algebra::{magnitude_diff, magnitude_squared};

/// Asserts `error` is less than epsilon.
#[inline]
pub(super) fn epsilon_error<C>(error: C)
where C: Float
{
    assert!(error <= C::epsilon());
}

/// Asserts the indexed `vertices` have a distance within epsilon.
#[inline]
pub(super) fn distance_neighbour<C>(distance: C, vertices: &[[C; 3]], i: usize, j: usize)
where C: Float
{
    assert!(C::abs(distance - magnitude_diff(vertices[i], vertices[j])) <= C::epsilon());
}

/// Asserts the indexed `vertices` have a distance within `epsilons` * epsilon.
#[inline]
pub(super) fn near_distance_neighbour<C>(distance: C, epsilons: impl Into<C>, vertices: &[[C; 3]], i: usize, j: usize)
where C: Float
{
    assert!(C::abs(distance - magnitude_diff(vertices[i], vertices[j])) <= epsilons.into() * C::epsilon());
}

/// Asserts all vertices are equidistant to the origin.
#[inline]
pub(super) fn equidistant<C>(vertices: &[[C; 3]])
where C: Float
{
    for vertex in vertices {
        assert!(C::abs(magnitude_squared(vertices[0]).sqrt() - magnitude_squared(*vertex).sqrt()) <= C::epsilon());
    }
}
