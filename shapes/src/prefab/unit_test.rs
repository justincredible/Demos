use num_traits::Float;

#[inline]
pub fn magnitude_diff<C>(minuend: [C; 3], subtrahend: [C; 3]) -> C
where C: Float
{
    let x = minuend[0] - subtrahend[0];
    let y = minuend[1] - subtrahend[1];
    let z = minuend[2] - subtrahend[2];

    (x * x + y * y + z * z).sqrt()
}

#[inline]
pub fn unit_neighbour<C>(vertices: &Vec<[C; 3]>, i: usize, j: usize)
where C: Float
{
    assert!(C::abs(C::one() - magnitude_diff(vertices[i], vertices[j])) <= C::epsilon());
}

#[inline]
pub fn distance_neighbour<C>(distance: C, vertices: &Vec<[C; 3]>, i: usize, j: usize)
where C: Float
{
    assert!(C::abs(distance - magnitude_diff(vertices[i], vertices[j])) <= C::epsilon());
}
