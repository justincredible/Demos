use num_traits::{cast, Float, FloatConst, one};

pub(super) enum Edge {
    Unit,
    Phi,
}

pub(super) struct Pentagonal<C>
where
    C: Float + FloatConst,
{
    pub width: C,
    pub center: C,
    pub radius: C,
    pub middle: C,
    pub axis: (C, C)
}

impl<C> Pentagonal<C>
where
    C: Float + FloatConst,
{
    pub fn new(edge: Edge) -> Self {
        let fh = cast::<_, C>(0.5).unwrap();
        let f1 = one::<C>();
        let f2 = cast::<_, C>(2.).unwrap();
        let f10 = cast::<_, C>(10.).unwrap();
        let fq = cast::<_, C>(0.25).unwrap();
        let sr5 = cast::<_, C>(5.).unwrap().sqrt();

        let (width, center, radius, middle) = match edge {
            Edge::Unit => (
                fq * (f1 + sr5), // phi/2
                fh * (f1 + f2 / sr5).sqrt(),
                fh * (f2 + f2 / sr5).sqrt(),
                fq * (f2 - f2 / sr5).sqrt(),
            ),
            Edge::Phi => {
                let f3 = cast::<_, C>(3.).unwrap();
                let f22 = cast::<_, C>(22.).unwrap();

                (
                    fq * (f3 + sr5),
                    fq * (f10 + f22 / sr5).sqrt(),
                    (f1 + f2 / sr5).sqrt(), // 2 * unit.center
                    fq * (f2 + f2 / sr5).sqrt(), // unit.radius / 2
                )
            },
        };
        let axis = ((fh - sr5 / f10).sqrt(), (fh + sr5 / f10).sqrt());

        Pentagonal { width, center, radius, middle, axis }
    }
}

