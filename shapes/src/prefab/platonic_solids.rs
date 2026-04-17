use num_traits::{cast, Float, FloatConst, NumCast, one, Unsigned, zero};

use crate::shapes::{Configuration, Shape, Shaper};

/// All possible Platonic solids.
pub enum PlatonicSolid {
    Tetrahedron,
    Hexahedron,
    Octahedron,
    Dodecahedron,
    Icosahedron,
}

impl<C, I> Shaper<C, I> for PlatonicSolid
where
    C: Float + FloatConst,
    I: Copy + NumCast + Unsigned,
{
    fn make(&self, request: Configuration) -> Shape<C, I> {
        match self {
            Self::Tetrahedron => {
                let f0 = zero();
                let fh = cast::<_, C>(0.5).unwrap();
                let sr2 = FloatConst::SQRT_2();
                let sr3 = cast::<_, C>(3.).unwrap().sqrt();
                let base = -fh / sr2 / sr3;

                let vertices = vec![
                    [-fh, base, fh / sr3],
                    [fh, base, fh / sr3],
                    [f0, base + sr2 / sr3, f0],
                    [f0, base, -one::<C>() / sr3],
                ];

                let i = vec![zero(), one()]
                    .into_iter()
                    .chain((2..4).map(|i| cast::<_, I>(i).unwrap()))
                    .collect::<Vec<_>>();

                if request.prefer_strips {
                    let strips = vec!(vec![i[0], i[1], i[2], i[3], i[0], i[1]]);

                    Shape::Strips { vertices, strips }
                } else {
                    let indices = vec![
                        i[0], i[1], i[2], i[0], i[2], i[3],
                        i[0], i[3], i[1], i[1], i[3], i[2],
                    ];

                    Shape::Triangles { vertices, indices }
                }
            },
            Self::Hexahedron => {
                let fh = cast::<_, C>(0.5).unwrap();

                let vertices = vec![
                    [-fh, -fh, fh],
                    [fh, -fh, fh],
                    [-fh, fh, fh],
                    [fh, fh, fh],
                    [-fh, -fh, -fh],
                    [fh, -fh, -fh],
                    [-fh, fh, -fh],
                    [fh, fh, -fh],
                ];

                let i = vec![zero(), one()]
                    .into_iter()
                    .chain((2..8).map(|i| cast::<_, I>(i).unwrap()))
                    .collect::<Vec<_>>();

                if request.prefer_strips {
                    let strips = vec!(vec![
                        i[0], i[1], i[2], i[3], i[7], i[1], i[5],
                        i[0], i[4], i[2], i[6], i[7], i[4], i[5],
                    ]);

                    Shape::Strips { vertices, strips }
                } else {
                    let indices = vec![
                        i[0], i[1], i[2], i[0], i[2], i[4], i[0], i[4], i[5],
                        i[0], i[5], i[1], i[1], i[3], i[2], i[1], i[5], i[7],
                        i[1], i[7], i[3], i[2], i[3], i[7], i[2], i[6], i[4],
                        i[2], i[7], i[6], i[4], i[6], i[7], i[4], i[7], i[5],
                    ];

                    Shape::Triangles { vertices, indices }
                }
            },
            Self::Octahedron => {
                let f0 = zero();
                let fh = cast::<_, C>(0.5).unwrap();
                let half_height = FloatConst::FRAC_1_SQRT_2();

                let vertices = vec![
                    [f0, half_height, f0],
                    [-fh, f0, -fh],
                    [-fh, f0, fh],
                    [fh, f0, fh],
                    [fh, f0, -fh],
                    [f0, -half_height, f0],
                ];

                let i = vec![zero(), one()]
                    .into_iter()
                    .chain((2..6).map(|i| cast::<_, I>(i).unwrap()))
                    .collect::<Vec<_>>();

                if request.prefer_strips {
                    let strips = vec![
                        vec![i[1], i[0], i[4], i[3], i[5], i[2]],
                        vec![i[3], i[0], i[2], i[1], i[5], i[4]],
                    ];

                    Shape::Strips { vertices, strips }
                } else {
                    let indices = vec![
                        i[0], i[1], i[2], i[0], i[2], i[3],
                        i[0], i[3], i[4], i[0], i[4], i[1],
                        i[1], i[4], i[5], i[1], i[5], i[2],
                        i[2], i[5], i[3], i[3], i[5], i[4],
                    ];

                    Shape::Triangles { vertices, indices }
                }
            },
            Self::Dodecahedron => {
                let f0 = zero();
                let fh = cast::<_, C>(0.5).unwrap();
                let pent = Pentagonal::new(Edge::Unit);
                let agon = Pentagonal::new(Edge::Phi);
                let half_iz = fh * pent.axis.0;

                let vertices = vec![
                    [f0, pent.radius, pent.radius + half_iz],
                    [-pent.width, pent.middle, pent.radius + half_iz],
                    [pent.width, pent.middle, pent.radius + half_iz],
                    [-fh, -pent.center, pent.radius + half_iz],
                    [fh, -pent.center, pent.radius + half_iz],
                    [f0, agon.radius, half_iz],
                    [-agon.width, agon.middle, half_iz],
                    [agon.width, agon.middle, half_iz],
                    [-pent.width, -agon.center, half_iz],
                    [pent.width, -agon.center, half_iz],
                    [-pent.width, agon.center, -half_iz],
                    [pent.width, agon.center, -half_iz],
                    [-agon.width, -agon.middle, -half_iz],
                    [agon.width, -agon.middle, -half_iz],
                    [f0, -agon.radius, -half_iz],
                    [-fh, pent.center, -pent.radius - half_iz],
                    [fh, pent.center, -pent.radius - half_iz],
                    [-pent.width, -pent.middle, -pent.radius - half_iz],
                    [pent.width, -pent.middle, -pent.radius - half_iz],
                    [f0, -pent.radius, -pent.radius - half_iz],
                ];

                let i = vec![zero(), one()]
                    .into_iter()
                    .chain((2..20).map(|i| cast::<_, I>(i).unwrap()))
                    .collect::<Vec<_>>();

                if request.prefer_strips {
                    let strips = vec!(vec![
                        i[1], i[3], i[0], i[4], i[2], i[7], i[0], i[11], i[5], i[10],
                        i[0], i[6], i[1], i[12], i[3], i[8], i[4], i[9], i[7], i[13],
                        i[11], i[16], i[10], i[15], i[6], i[17], i[12], i[19], i[8],
                        i[14], i[9], i[19], i[13], i[18], i[16], i[19], i[15], i[17],
                    ]);

                    Shape::Strips { vertices, strips }
                } else {
                    let indices = vec![
                        i[0], i[1], i[3], i[0], i[2], i[7], i[0], i[3], i[4],
                        i[0], i[4], i[2], i[0], i[5], i[10], i[0], i[6], i[1],
                        i[0], i[7], i[11], i[0], i[10], i[6], i[0], i[11], i[5],
                        i[1], i[6], i[12], i[1], i[12], i[3], i[2], i[4], i[7],
                        i[3], i[12], i[8], i[3], i[8], i[4], i[4], i[8], i[9],
                        i[4], i[9], i[7], i[5], i[11], i[10], i[6], i[10], i[15],
                        i[6], i[15], i[17], i[6], i[17], i[12], i[7], i[9], i[13],
                        i[7], i[13], i[11], i[8], i[12], i[19], i[8], i[19], i[14],
                        i[8], i[14], i[9], i[9], i[14], i[19], i[9], i[19], i[13],
                        i[10], i[11], i[16], i[10], i[16], i[15], i[11], i[13], i[16],
                        i[12], i[17], i[19], i[13], i[19], i[18], i[13], i[18], i[16],
                        i[15], i[16], i[19], i[15], i[19], i[17], i[16], i[18], i[19],
                    ];

                    Shape::Triangles { vertices, indices }
                }
            },
            Self::Icosahedron => {
                let f0 = zero();
                let fh = cast::<_, C>(0.5).unwrap();
                let pent = Pentagonal::new(Edge::Unit);
                let y_diff = pent.axis.0;
                let half_middle = fh * pent.axis.1;

                let vertices = vec![
                    [f0, half_middle + y_diff, f0],
                    [f0, half_middle, -pent.radius],
                    [-pent.width, half_middle, -pent.middle],
                    [pent.width, half_middle, -pent.middle],
                    [-fh, half_middle, pent.center],
                    [fh, half_middle, pent.center],
                    [-fh, -half_middle, -pent.center],
                    [fh, -half_middle, -pent.center],
                    [-pent.width, -half_middle, pent.middle],
                    [pent.width, -half_middle, pent.middle],
                    [f0, -half_middle, pent.radius],
                    [f0, -half_middle - y_diff, f0],
                ];

                let i = vec![zero(), one()]
                    .into_iter()
                    .chain((2..12).map(|i| cast::<_, I>(i).unwrap()))
                    .collect::<Vec<_>>();

                if request.prefer_strips {
                    let strips = vec![
                        vec![i[1], i[2], i[0], i[4], i[5], i[10], i[9], i[11], i[7], i[6], i[1], i[2]],
                        vec![i[7], i[1], i[3], i[0], i[5], i[4], i[10], i[8], i[11], i[6]],// i[7], i[1]],
                        vec![i[6], i[8], i[2], i[4], i[0], i[5], i[3], i[9], i[7]],// i[11], i[6], i[8]],
                    ];

                    Shape::Strips { vertices, strips }
                } else {
                    let indices = vec![
                        i[0], i[1], i[2], i[0], i[2], i[4],
                        i[0], i[3], i[1], i[0], i[4], i[5],
                        i[0], i[5], i[3], i[1], i[3], i[7],
                        i[1], i[6], i[2], i[1], i[7], i[6],
                        i[2], i[6], i[8], i[2], i[8], i[4],
                        i[3], i[9], i[7], i[3], i[5], i[9],
                        i[4], i[8], i[10], i[4], i[10], i[5],
                        i[5], i[10], i[9], i[6], i[7], i[11],
                        i[6], i[11], i[8], i[7], i[9], i[11],
                        i[8], i[11], i[10], i[9], i[10], i[11],
                    ];

                    Shape::Triangles { vertices, indices }
                }
            },
        }
    }
}

use num_traits;
enum Edge {
    Unit,
    Phi,
}

struct Pentagonal<C>
where
    C: Float + FloatConst,
{
    width: C,
    center: C,
    radius: C,
    middle: C,
    axis: (C, C)
}

impl<C> Pentagonal<C>
where
    C: Float + FloatConst,
{
    fn new(edge: Edge) -> Self {
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
                let f10 = cast::<_, C>(10.).unwrap();
                let f22 = cast::<_, C>(22.).unwrap();
                let fq = cast::<_, C>(0.25).unwrap();

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

#[cfg(test)]
mod tests {
    use super::{PlatonicSolid, Shape, Shaper};

    use crate::prefab::unit_test::unit_neighbour;

    type Double = f64;

    const TOLERANCE: Double = 2. * Double::EPSILON;

    fn magnitude_squared(vertex: [Double; 3]) -> Double {
        vertex[0] * vertex[0] + vertex[1] * vertex[1] + vertex[2] * vertex[2]
    }

    fn make_shape(solid: PlatonicSolid) -> Shape<Double, u8> {
        solid.make(Default::default())
    }

    #[test]
    fn tetrahedron_centered() {
        let shape = make_shape(PlatonicSolid::Tetrahedron);

        for vertex in shape.vertices() {
            assert!(Double::abs(magnitude_squared(shape.vertices()[0]) - magnitude_squared(*vertex)) <= TOLERANCE);
        }
    }

    #[test]
    fn tetrahedron_edges() {
        let shape = make_shape(PlatonicSolid::Tetrahedron);
        let vertices = shape.vertices();

        unit_neighbour(vertices, 0, 1);
        unit_neighbour(vertices, 0, 2);
        unit_neighbour(vertices, 0, 3);
        unit_neighbour(vertices, 1, 2);
        unit_neighbour(vertices, 1, 3);
        unit_neighbour(vertices, 2, 3);
    }

    #[test]
    fn hexahedron_centered() {
        let shape = make_shape(PlatonicSolid::Hexahedron);

        for vertex in shape.vertices() {
            assert!(Double::abs(magnitude_squared(shape.vertices()[0]) - magnitude_squared(*vertex)) <= TOLERANCE);
        }
    }

    #[test]
    fn hexahedron_edges() {
        let shape = make_shape(PlatonicSolid::Hexahedron);
        let vertices = shape.vertices();

        unit_neighbour(vertices, 0, 1);
        unit_neighbour(vertices, 0, 2);
        unit_neighbour(vertices, 0, 4);
        unit_neighbour(vertices, 1, 3);
        unit_neighbour(vertices, 1, 5);
        unit_neighbour(vertices, 2, 3);
        unit_neighbour(vertices, 2, 6);
        unit_neighbour(vertices, 3, 7);
        unit_neighbour(vertices, 4, 5);
        unit_neighbour(vertices, 4, 6);
        unit_neighbour(vertices, 5, 7);
        unit_neighbour(vertices, 6, 7);
    }

    #[test]
    fn octahedron_centered() {
        let shape = make_shape(PlatonicSolid::Octahedron);

        for vertex in shape.vertices() {
            assert!(Double::abs(magnitude_squared(shape.vertices()[0]) - magnitude_squared(*vertex)) <= TOLERANCE);
        }
    }

    #[test]
    fn octahedron_edges() {
        let shape = make_shape(PlatonicSolid::Octahedron);
        let vertices = shape.vertices();

        unit_neighbour(vertices, 0, 1);
        unit_neighbour(vertices, 0, 2);
        unit_neighbour(vertices, 0, 3);
        unit_neighbour(vertices, 0, 4);
        unit_neighbour(vertices, 1, 2);
        unit_neighbour(vertices, 1, 4);
        unit_neighbour(vertices, 1, 5);
        unit_neighbour(vertices, 2, 3);
        unit_neighbour(vertices, 2, 5);
        unit_neighbour(vertices, 3, 4);
        unit_neighbour(vertices, 3, 5);
        unit_neighbour(vertices, 4, 5);
    }

    #[test]
    fn dodecahedron_centered() {
        let shape = make_shape(PlatonicSolid::Dodecahedron);

        for vertex in shape.vertices() {
            assert!(Double::abs(magnitude_squared(shape.vertices()[0]) - magnitude_squared(*vertex)) <= TOLERANCE);
        }
    }

    #[test]
    fn dodecahedron_edges() {
        let shape = make_shape(PlatonicSolid::Dodecahedron);
        let vertices = shape.vertices();

        unit_neighbour(vertices, 0, 1);
        unit_neighbour(vertices, 0, 2);
        unit_neighbour(vertices, 0, 5);
        unit_neighbour(vertices, 1, 3);
        unit_neighbour(vertices, 1, 6);
        unit_neighbour(vertices, 2, 4);
        unit_neighbour(vertices, 2, 7);
        unit_neighbour(vertices, 3, 4);
        unit_neighbour(vertices, 3, 8);
        unit_neighbour(vertices, 4, 9);
        unit_neighbour(vertices, 5, 10);
        unit_neighbour(vertices, 5, 11);
        unit_neighbour(vertices, 6, 10);
        unit_neighbour(vertices, 6, 12);
        unit_neighbour(vertices, 7, 11);
        unit_neighbour(vertices, 7, 13);
        unit_neighbour(vertices, 8, 12);
        unit_neighbour(vertices, 8, 14);
        unit_neighbour(vertices, 9, 13);
        unit_neighbour(vertices, 9, 14);
        unit_neighbour(vertices, 10, 15);
        unit_neighbour(vertices, 11, 16);
        unit_neighbour(vertices, 12, 17);
        unit_neighbour(vertices, 13, 18);
        unit_neighbour(vertices, 14, 19);
        unit_neighbour(vertices, 15, 16);
        unit_neighbour(vertices, 15, 17);
        unit_neighbour(vertices, 16, 18);
        unit_neighbour(vertices, 17, 19);
        unit_neighbour(vertices, 18, 19);
    }

    #[test]
    fn icosahedron_centered() {
        let shape = make_shape(PlatonicSolid::Icosahedron);

        for vertex in shape.vertices() {
            assert!(Double::abs(magnitude_squared(shape.vertices()[0]) - magnitude_squared(*vertex)) <= TOLERANCE);
        }
    }

    #[test]
    fn icosahedron_edges() {
        let shape = make_shape(PlatonicSolid::Icosahedron);
        let vertices = shape.vertices();

        unit_neighbour(vertices, 0, 1);
        unit_neighbour(vertices, 0, 2);
        unit_neighbour(vertices, 0, 3);
        unit_neighbour(vertices, 0, 4);
        unit_neighbour(vertices, 0, 5);
        unit_neighbour(vertices, 1, 2);
        unit_neighbour(vertices, 1, 3);
        unit_neighbour(vertices, 1, 6);
        unit_neighbour(vertices, 1, 7);
        unit_neighbour(vertices, 2, 4);
        unit_neighbour(vertices, 2, 6);
        unit_neighbour(vertices, 2, 8);
        unit_neighbour(vertices, 3, 5);
        unit_neighbour(vertices, 3, 7);
        unit_neighbour(vertices, 3, 9);
        unit_neighbour(vertices, 4, 5);
        unit_neighbour(vertices, 4, 8);
        unit_neighbour(vertices, 4, 10);
        unit_neighbour(vertices, 5, 9);
        unit_neighbour(vertices, 5, 10);
        unit_neighbour(vertices, 6, 7);
        unit_neighbour(vertices, 6, 8);
        unit_neighbour(vertices, 6, 11);
        unit_neighbour(vertices, 7, 9);
        unit_neighbour(vertices, 7, 11);
        unit_neighbour(vertices, 8, 10);
        unit_neighbour(vertices, 8, 11);
        unit_neighbour(vertices, 9, 10);
        unit_neighbour(vertices, 9, 11);
        unit_neighbour(vertices, 10, 11);
    }
}
