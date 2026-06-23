use num_traits::Float;

/// Asserts `error` is less than epsilon.
#[inline]
pub(super) fn epsilon_error<C>(error: C)
where C: Float
{
    assert!(error <= C::epsilon());
}

/// Returns the length of the difference vector.
#[inline]
pub(super) fn magnitude_diff<C>(minuend: [C; 3], subtrahend: [C; 3]) -> C
where C: Float
{
    let x = minuend[0] - subtrahend[0];
    let y = minuend[1] - subtrahend[1];
    let z = minuend[2] - subtrahend[2];

    (x * x + y * y + z * z).sqrt()
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

/// Returns the square of the vector's magnitude.
#[inline]
fn magnitude_squared<C>(vertex: [C; 3]) -> C
where C: Float
{
    vertex[0] * vertex[0] + vertex[1] * vertex[1] + vertex[2] * vertex[2]
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
