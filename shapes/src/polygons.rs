use num_traits::{cast, Float, FloatConst, Unsigned, zero};

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
    fn make(&self, _request: Configuration) -> Shape<C, I> {
        let zero = zero();
        let tau: C = FloatConst::TAU();
        let exterior = tau / cast::<_, C>(self.sides).unwrap();
        let offset = cast::<_, C>(0.5).unwrap() * exterior;
        let radius = offset.cos() / exterior.sin();

        let mut vertices = vec![];
        if self.sides % 2 == 0 {
            for step in 0..self.sides/2 {
                let stepf = cast::<_, C>(step).unwrap();
                let s_val = (offset + stepf * exterior).sin();
                let c_val = (offset + stepf * exterior).cos();
                vertices.push([radius * s_val, radius * c_val, zero]);
                vertices.push([radius * -s_val, radius * c_val, zero]);
            }
        } else {
            vertices.push([zero, radius, zero]);

            for step in 1..=self.sides/2 {
                let stepf = cast::<_, C>(step).unwrap();
                let s_val = (stepf * exterior).sin();
                let c_val = (stepf * exterior).cos();
                vertices.push([radius * -s_val, radius * c_val, zero]);
                vertices.push([radius * s_val, radius * c_val, zero]);
            }
        };

        Shape::Strips { vertices, strips: vec!() }
    }
}
