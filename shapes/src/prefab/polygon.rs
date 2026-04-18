use std::ops::AddAssign;

use num_traits::{cast, Float, FloatConst, NumCast, one, Unsigned, zero};

use crate::shapes::{Configuration, Shape, Shaper};

/// Regular polygons with less than 65536 sides.
pub struct Polygon {
    sides: u16
}

impl Polygon {
    /// Create a regular polygon with vertices in triangle strip order.
    /// # Panics
    ///
    /// May panic if `sides` is less than three.
    pub fn new(sides: u16) -> Self {
        if sides < 3 { panic!("degenerate polygon") }

        Self { sides }
    }
}

impl<C, I> Shaper<C, I> for Polygon
where
    C: Float + FloatConst,
    I: AddAssign + Copy + NumCast + Unsigned,
{
    fn make(&self, request: Configuration) -> Shape<C, I> {
        let zero = zero();
        let one: C = one();
        let angle = <C as FloatConst>::TAU() / cast::<_, C>(self.sides).unwrap();
        let half = cast::<_, C>(0.5).unwrap() * angle;
        let radius = half.cos() / angle.sin();

        let odd = !self.sides.is_multiple_of(2);
        let mut vertices = if odd {
            vec!([zero, radius, zero])
        } else {
            vec!()
        };
        let first = if odd && request.orientation.is_ccw() || !odd && request.orientation.is_cw() {
            -one
        } else {
            one
        };

        for step in 0..self.sides/2 {
            let value = if odd {
                angle * cast::<_, C>(step+1).unwrap()
            } else {
                half + angle * cast::<_, C>(step).unwrap()
            };
            let mut point = [radius * first * value.sin(), radius * value.cos(), zero];
            vertices.push(point);
            point[0] = -point[0];
            vertices.push(point);
        }

        if request.prefer_strips {
            Shape::Strips { vertices, strips: vec!() }
        } else {
            let mut indices = vec!();

            let mut a: I = num_traits::zero();
            let mut b: I = num_traits::one();
            let mut c = cast::<_, I>(2).unwrap();
            let inc = b;

            for i in 0..self.sides-2 {
                indices.push(a);
                indices.push(b);
                indices.push(c);

                if i.is_multiple_of(2) {
                    a = c;
                } else {
                    b = c;
                }
                c += inc;
            }

            if request.generate_normals {
                let normals = if request.orientation.is_left() {
                    vec!([zero, zero, -one])
                } else {
                    vec!([zero, zero, one])
                };

                Shape::NormalTriangles { vertices, normals, indices }
            } else {
                Shape::Triangles { vertices, indices }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Polygon, Shape, Shaper};

    use crate::prefab::unit_test::{epsilon_error, magnitude_diff, unit_neighbour};

    fn make_shape(size: u16) -> Shape<f64, u16> {
        Polygon::new(size).make(Default::default())
    }

    #[test]
    #[should_panic]
    fn zero_sides() {
        make_shape(0);
    }

    #[test]
    #[should_panic]
    fn two_sides() {
        make_shape(2);
    }

    #[test]
    fn three_sides() {
        make_shape(3);
    }

    #[test]
    fn u8_max_sides() {
        let _: Shape<f32, u8> = Polygon::new(255).make(Default::default());
    }

    #[test]
    #[should_panic]
    fn u8_overflow() {
        let _: Shape<f32, u8> = Polygon::new(256).make(Default::default());
    }

    #[test]
    fn u16_min_sides() {
        let _: Shape<f32, u16> = Polygon::new(256).make(Default::default());
    }

    #[test]
    fn max_sides() {
        make_shape(u16::MAX);
    }

    #[test]
    fn side_length_odd() {
        let shape = make_shape(11);
        let vertices = shape.vertices();

        unit_neighbour(vertices, 1, 0);
        for i in 2..vertices.len() {
            unit_neighbour(vertices, i, i-2);
        }
        unit_neighbour(vertices, vertices.len()-1, vertices.len()-2);
    }

    #[test]
    fn error_total_odd() {
        let shape = make_shape(32773);
        let vertices = shape.vertices();

        let mut error = 0.;
        error += 1. - magnitude_diff(vertices[1], vertices[0]);
        for i in 2..vertices.len() {
            error += 1. - magnitude_diff(vertices[i], vertices[i-2]);
        }
        error += 1. - magnitude_diff(vertices[vertices.len()-1], vertices[vertices.len()-2]);

        epsilon_error(error);
    }

    #[test]
    fn side_length_even() {
        let shape = make_shape(10);
        let vertices = shape.vertices();

        unit_neighbour(vertices, 1, 0);
        for i in 2..vertices.len() {
            unit_neighbour(vertices, i, i-2);
        }
        unit_neighbour(vertices, vertices.len()-1, vertices.len()-2);
    }

    #[test]
    fn error_total_even() {
        let shape = make_shape(32768);
        let vertices = shape.vertices();

        let mut error = 0.;
        error += 1. - magnitude_diff(vertices[1], vertices[0]);
        for i in 2..vertices.len() {
            error += 1. - magnitude_diff(vertices[i], vertices[i-2]);
        }
        error += 1. - magnitude_diff(vertices[vertices.len()-1], vertices[vertices.len()-2]);

        epsilon_error(error);
    }
}
