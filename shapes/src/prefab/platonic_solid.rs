use num_traits::{cast, Float, FloatConst, NumCast, one, Unsigned, zero};

use crate::shapes::{Configuration, Shape, Shaper};
use crate::prefab::pentagonal::{Edge, Pentagonal};

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
                let f0 = zero::<C>();
                let fh = cast::<_, C>(0.5).unwrap();
                let pent = Pentagonal::new(Edge::Unit);
                let agon = Pentagonal::new(Edge::Phi);
                let inner = fh * fh / pent.width * pent.radius;
                let outer = fh * (agon.radius + pent.radius);

                let vertices = vec![
                    [f0, pent.radius, outer],
                    [-pent.width, pent.middle, outer],
                    [pent.width, pent.middle, outer],
                    [-fh, -pent.center, outer],
                    [fh, -pent.center, outer],
                    [f0, agon.radius, inner],
                    [-agon.width, agon.middle, inner],
                    [agon.width, agon.middle, inner],
                    [-pent.width, -agon.center, inner],
                    [pent.width, -agon.center, inner],
                    [-pent.width, agon.center, -inner],
                    [pent.width, agon.center, -inner],
                    [-agon.width, -agon.middle, -inner],
                    [agon.width, -agon.middle, -inner],
                    [f0, -agon.radius, -inner],
                    [-fh, pent.center, -outer],
                    [fh, pent.center, -outer],
                    [-pent.width, -pent.middle, -outer],
                    [pent.width, -pent.middle, -outer],
                    [f0, -pent.radius, -outer],
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
                let pent = Pentagonal::<C>::new(Edge::Unit);
                let apex = (pent.width / fh - fh) * pent.radius;

                let vertices = vec![
                    [f0, apex, f0],
                    [f0, fh * pent.radius, -pent.radius],
                    [-pent.width, fh * pent.radius, -pent.middle],
                    [pent.width, fh * pent.radius, -pent.middle],
                    [-fh, fh * pent.radius, pent.center],
                    [fh, fh * pent.radius, pent.center],
                    [-fh, -fh * pent.radius, -pent.center],
                    [fh, -fh * pent.radius, -pent.center],
                    [-pent.width, -fh * pent.radius, pent.middle],
                    [pent.width, -fh * pent.radius, pent.middle],
                    [f0, -fh * pent.radius, pent.radius],
                    [f0, -apex, f0],
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

#[cfg(test)]
mod tests {
    use super::{PlatonicSolid, Shape, Shaper};

    use crate::prefab::unit_test::{distance_neighbour, equidistant, unit_neighbour};

    type Real = f64;

    const SQRT_2: Real = 1.41421356237309504880;
    const SQRT_3: Real = 1.73205080756887729352;
    const SQRT_5: Real = 2.23606797749978969640;

    fn make_shape(solid: PlatonicSolid) -> Shape<Real, u8> {
        solid.make(Default::default())
    }

    #[test]
    fn tetrahedron_centered() {
        equidistant(make_shape(PlatonicSolid::Tetrahedron).vertices());
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
    fn tetrahedron_diameters() {
        tetrahedron_edges();
    }

    #[test]
    fn hexahedron_centered() {
        equidistant(make_shape(PlatonicSolid::Hexahedron).vertices());
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
    fn hexahedron_diameters() {
        let shape = make_shape(PlatonicSolid::Hexahedron);
        let vertices = shape.vertices();

        distance_neighbour(SQRT_3, vertices, 0, 7);
        distance_neighbour(SQRT_3, vertices, 1, 6);
        distance_neighbour(SQRT_3, vertices, 2, 5);
        distance_neighbour(SQRT_3, vertices, 3, 4);
    }

    #[test]
    fn octahedron_centered() {
        equidistant(make_shape(PlatonicSolid::Octahedron).vertices());
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
    fn octahedron_diameters() {
        let shape = make_shape(PlatonicSolid::Octahedron);
        let vertices = shape.vertices();

        distance_neighbour(SQRT_2, vertices, 0, 5);
        distance_neighbour(SQRT_2, vertices, 1, 3);
        distance_neighbour(SQRT_2, vertices, 2, 4);
    }

    #[test]
    fn dodecahedron_centered() {
        equidistant(make_shape(PlatonicSolid::Dodecahedron).vertices());
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
    fn dodecahedron_diameters() {
        let shape = make_shape(PlatonicSolid::Dodecahedron);
        let vertices = shape.vertices();

        let diameter = Real::sqrt(0.5*(9. + 3.*SQRT_5)); // sqrt(3phi^2)
        distance_neighbour(diameter, vertices, 0, 19);
        distance_neighbour(diameter, vertices, 1, 18);
        distance_neighbour(diameter, vertices, 2, 17);
        distance_neighbour(diameter, vertices, 3, 16);
        distance_neighbour(diameter, vertices, 4, 15);
        distance_neighbour(diameter, vertices, 5, 14);
        distance_neighbour(diameter, vertices, 6, 13);
        distance_neighbour(diameter, vertices, 7, 12);
        distance_neighbour(diameter, vertices, 8, 11);
        distance_neighbour(diameter, vertices, 9, 10);
    }

    #[test]
    fn icosahedron_centered() {
        equidistant(make_shape(PlatonicSolid::Icosahedron).vertices());
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

    #[test]
    fn icosahedron_diameters() {
        let shape = make_shape(PlatonicSolid::Icosahedron);
        let vertices = shape.vertices();

        let diameter = Real::sqrt(0.5*(5. + SQRT_5)); // sqrt(phi + 2)
        distance_neighbour(diameter, vertices, 0, 11);
        distance_neighbour(diameter, vertices, 1, 10);
        distance_neighbour(diameter, vertices, 2, 9);
        distance_neighbour(diameter, vertices, 3, 8);
        distance_neighbour(diameter, vertices, 4, 7);
        distance_neighbour(diameter, vertices, 5, 6);
    }
}
