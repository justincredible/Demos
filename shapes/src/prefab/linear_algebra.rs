use num_traits::{Float, zero};
use crate::shapes::configuration::Orientation;

pub(super) fn oriented_plane<C>(
    vertices: &[[C; 3]],
    unoriented: &[usize],
    orientation: Orientation,
) -> ([C; 3], [usize; 3]) where C: Float {
    let a1 = vertices[unoriented[1]][0] - vertices[unoriented[0]][0];
    let b1 = vertices[unoriented[1]][1] - vertices[unoriented[0]][1];
    let c1 = vertices[unoriented[1]][2] - vertices[unoriented[0]][2];
    let a2 = vertices[unoriented[2]][0] - vertices[unoriented[1]][0];
    let b2 = vertices[unoriented[2]][1] - vertices[unoriented[1]][1];
    let c2 = vertices[unoriented[2]][2] - vertices[unoriented[1]][2];
    let a = b1*c2 - c1*b2;
    let b = c1*a2 - a1*c2;
    let c = a1*b2 - b1*a2;
    let mag = (a*a + b*b + c*c).sqrt();
    let a = a / mag;
    let b = b / mag;
    let c = c / mag;
    let d = a*vertices[unoriented[0]][0] + b*vertices[unoriented[0]][1] + c*vertices[unoriented[0]][2];

    (
        if d > zero() {
            [a, b, c]
        } else {
            [-a, -b, -c]
        },
        if (d > zero()) == (orientation.is_ccw() == orientation.is_right()) {
            [unoriented[0], unoriented[1], unoriented[2]]
        } else {
            [unoriented[0], unoriented[2], unoriented[1]]
        },
    )
}
