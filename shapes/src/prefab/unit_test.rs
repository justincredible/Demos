use num_traits::Float;

#[inline]
pub(super) fn epsilon_error<C>(error: C)
where C: Float
{
    assert!(error <= C::epsilon());
}

#[inline]
pub(super) fn magnitude_diff<C>(minuend: [C; 3], subtrahend: [C; 3]) -> C
where C: Float
{
    let x = minuend[0] - subtrahend[0];
    let y = minuend[1] - subtrahend[1];
    let z = minuend[2] - subtrahend[2];

    (x * x + y * y + z * z).sqrt()
}

#[inline]
pub(super) fn unit_neighbour<C>(vertices: &[[C; 3]], i: usize, j: usize)
where C: Float
{
    assert!(C::abs(C::one() - magnitude_diff(vertices[i], vertices[j])) <= C::epsilon());
}

#[inline]
pub(super) fn distance_neighbour<C>(distance: C, vertices: &[[C; 3]], i: usize, j: usize)
where C: Float
{
    assert!(C::abs(distance - magnitude_diff(vertices[i], vertices[j])) <= C::epsilon());
}

#[inline]
fn magnitude_squared<C>(vertex: [C; 3]) -> C
where C: Float
{
    vertex[0] * vertex[0] + vertex[1] * vertex[1] + vertex[2] * vertex[2]
}

#[inline]
pub(super) fn equidistant<C>(vertices: &[[C; 3]])
where C: Float
{
    for vertex in vertices {
        assert!(C::abs(magnitude_squared(vertices[0]).sqrt() - magnitude_squared(*vertex).sqrt()) <= C::epsilon());
    }
}
