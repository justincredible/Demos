use num_traits::{cast, Float, FloatConst, one};

/// Edge lengths of interest for a regular pentagon.
#[derive(Debug)]
pub(super) enum Edge {
    Unit,
    Phi,
}

/// Distances of utility for a regular pentagon.
#[derive(Debug)]
pub(super) struct Pentagonal<C>
where
    C: Float + FloatConst,
{
    /// Half the distance between non-adjacent vertices.
    pub width: C,
    /// The distance from the center to an edge's bisector.
    pub center: C,
    /// The distance from the center to any vertex.
    pub radius: C,
    /// The distance such that `(radius - middle)` and `width` are legs of a right triangle
    /// with an edge as hypotenuse.
    pub middle: C,
    //pub axis: C = radius / phi,
    //pub base: C = center + middle,
    //pub top: C = radius - middle,
    //pub height: C = center + radius,
}

impl<C> Pentagonal<C>
where
    C: Float + FloatConst,
{
    pub fn new(edge: Edge) -> Self {
        let f1 = one::<C>();
        let f2 = cast::<_, C>(2.).unwrap();
        let fh = cast::<_, C>(0.5).unwrap();
        let fq = cast::<_, C>(0.25).unwrap();
        let sr5 = cast::<_, C>(5.).unwrap().sqrt();

        let (width, center, radius, middle) = match edge {
            Edge::Unit => (
                fq * (f1 + sr5), // phi/2
                fh * (f1 + f2 / sr5).sqrt(),
                (fh + fh / sr5).sqrt(),
                fq * (f2 - f2 / sr5).sqrt(),
            ),
            Edge::Phi => {
                let f3 = cast::<_, C>(3.).unwrap();
                let f10 = cast::<_, C>(10.).unwrap();
                let f22 = cast::<_, C>(22.).unwrap();

                (
                    fq * (f3 + sr5),
                    fq * (f10 + f22 / sr5).sqrt(),
                    (f1 + f2 / sr5).sqrt(), // 2 * unit.center
                    fq * (f2 + f2 / sr5).sqrt(), // unit.radius / 2
                )
            },
        };

        Pentagonal { width, center, radius, middle }
    }
}
