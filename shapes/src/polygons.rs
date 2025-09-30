use num_traits::{cast, Float, FloatConst, one, Unsigned, zero};

use crate::shapes::{Configuration, Shape, Shaper};

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

impl<C: Float + FloatConst, I: Unsigned> Shaper<C, I> for Polygon {
    fn make(&self, request: Configuration) -> Shape<C, I> {
        let zero = zero();
        let one: C = one();
        let tau: C = FloatConst::TAU();
        let angle = tau / cast::<_, C>(self.sides).unwrap();
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

        Shape::Strips { vertices, strips: vec!() }
    }
}
