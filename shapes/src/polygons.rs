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
        let pi = FloatConst::PI();

        let zero = zero();
        let half = cast::<_, C>(0.5).unwrap();
        let exterior = cast::<_, C>(2.0 / self.sides as f64).unwrap() * pi;
        let start = half * pi;
        let offset = half * exterior;
        let radius = (start - offset).sin() / exterior.sin();

        let mut vertices = vec![];
        if self.sides % 2 == 0 {
            for step in 0..self.sides/2 {
                let stepf = cast::<_, C>(step).unwrap();
                let left = start + offset + stepf * exterior;
                let right = start - offset - stepf * exterior;
                vertices.push([radius * right.cos(), radius * right.sin(), zero]);
                vertices.push([radius * left.cos(), radius * left.sin(), zero]);
            }
        } else {
            vertices.push([zero, radius, zero]);

            for step in 1..=self.sides/2 {
                let stepf = cast::<_, C>(step).unwrap();
                let left = start + stepf * exterior;
                let right = start - stepf * exterior;
                vertices.push([radius * left.cos(), radius * left.sin(), zero]);
                vertices.push([radius * right.cos(), radius * right.sin(), zero]);
            }
        };

        Shape::Strips { vertices, strips: vec!() }
    }
}
