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
                let f1 = one::<C>();
                let fh = cast::<_, C>(0.5).unwrap();
                let sr2 = FloatConst::SQRT_2();
                let sr3 = cast::<_, C>(3.).unwrap().sqrt();
                let base = -fh / sr2 / sr3;
                let apex = fh * sr3 / sr2;

                let vertices = vec![
                    [-fh, base, fh / sr3],
                    [fh, base, fh / sr3],
                    [f0, apex, f0],
                    [f0, base, -f1 / sr3],
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
                let f0 = zero::<C>();
                let half_h = FloatConst::FRAC_1_SQRT_2();

                let vertices = vec![
                    [f0, half_h, f0],
                    [f0, f0, -half_h],
                    [-half_h, f0, f0],
                    [half_h, f0, f0],
                    [f0, f0, half_h],
                    [f0, -half_h, f0],
                ];

                let i = vec![zero(), one()]
                    .into_iter()
                    .chain((2..6).map(|i| cast::<_, I>(i).unwrap()))
                    .collect::<Vec<_>>();

                if request.prefer_strips {
                    let strips = vec![
                        vec![i[1], i[0], i[3], i[4], i[5], i[2]],
                        vec![i[4], i[0], i[2], i[1], i[5], i[3]],
                    ];

                    Shape::Strips { vertices, strips }
                } else {
                    let indices = vec![
                        i[0], i[1], i[2], i[0], i[2], i[4],
                        i[0], i[4], i[3], i[0], i[3], i[1],
                        i[1], i[3], i[5], i[1], i[5], i[2],
                        i[2], i[5], i[4], i[4], i[5], i[3],
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

const TETRA_VERTEX: usize = 4;
const HEXA_VERTEX: usize = 8;
const OCTA_VERTEX: usize = 6;
const DODECA_VERTEX: usize = 20;
const ICOSA_VERTEX: usize = 12;
const TETRA_EDGES: [[usize; 3]; TETRA_VERTEX] = [[1,2,3], [0,2,3], [0,1,3], [0,1,2]];
const HEXA_EDGES: [[usize; 3]; HEXA_VERTEX] = [
    [1,2,4], [0,3,5], [0,3,6], [1,2,7],
    [0,5,6], [1,4,7], [2,4,7], [3,5,6],
];
const OCTA_EDGES: [[usize; 4]; OCTA_VERTEX] = [[1,2,3,4], [0,2,3,5], [0,1,4,5], [0,1,4,5], [0,2,3,5], [1,2,3,4]];
const DODECA_EDGES: [[usize; 3]; DODECA_VERTEX] = [
    [1,2,5], [0,3,6], [0,4,7], [1,4,8], [2,3,9],
    [0,10,11], [1,10,12], [2,11,13], [3,12,14], [4,13,14],
    [5,6,15], [5,7,16], [6,8,17], [7,9,18], [8,9,19],
    [10,16,17], [11,15,18], [12,15,19], [13,16,19], [14,17,18],
];
const DODECA_NEXTS: [[usize; 6]; DODECA_VERTEX] = [
    [3,4,6,7,10,11], [2,4,5,8,10,12], [1,3,5,9,11,13], [0,2,6,9,12,14], [0,1,7,8,13,14],
    [1,2,6,7,15,16], [0,3,5,8,15,17], [0,4,5,9,16,18], [1,4,6,9,17,19], [2,3,7,8,18,19],
    [0,1,11,12,16,17], [0,2,10,13,15,18], [1,3,10,14,15,19], [2,4,11,14,16,19], [3,4,12,13,17,18],
    [5,6,11,12,18,19], [5,7,10,13,17,19], [6,8,10,14,16,18], [7,9,11,14,15,17], [8,9,12,13,15,16],
];
const ICOSA_EDGES: [[usize; 5]; ICOSA_VERTEX] = [
    [1,2,3,4,5], [0,2,3,6,7], [0,1,4,6,8], [0,1,5,7,9], [0,2,5,8,10], [0,3,5,9,10],
    [1,2,7,8,11], [1,3,6,9,11], [2,4,6,10,11], [3,5,7,10,11], [4,5,8,9,11], [6,7,8,9,10],
];

#[cfg(test)]
mod tests {
    use super::{PlatonicSolid, Shape, Shaper};

    use crate::prefab::unit_test::{distance_neighbour, equidistant, near_distance_neighbour};

    type Real = f64;

    const PHI: Real = 1.618_033_988_749_895;

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

        for i in 0..super::TETRA_VERTEX {
            for j in super::TETRA_EDGES[i].into_iter().filter(|&x| x > i) {
                distance_neighbour(1., vertices, i, j);
            }
        }
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

        for i in 0..super::HEXA_VERTEX {
            for j in super::HEXA_EDGES[i].into_iter().filter(|&x| x > i) {
                distance_neighbour(1., vertices, i, j);
            }
        }
    }

    #[test]
    fn hexahedron_diameters() {
        let shape = make_shape(PlatonicSolid::Hexahedron);
        let vertices = shape.vertices();
        let diameter = Real::sqrt(3.);

        for i in 0..super::HEXA_VERTEX / 2 {
            distance_neighbour(diameter, vertices, i, super::HEXA_VERTEX - 1 - i);
        }
    }

    #[test]
    fn hexahedron_nonadjacents() {
        let shape = make_shape(PlatonicSolid::Hexahedron);
        let vertices = shape.vertices();
        let distance = num_traits::FloatConst::SQRT_2();

        for i in 0..super::HEXA_VERTEX {
            for j in super::HEXA_EDGES[super::HEXA_VERTEX - 1 - i].into_iter().filter(|&x| x > i) {
                distance_neighbour(distance, vertices, i, j);
            }
        }
    }

    #[test]
    fn octahedron_centered() {
        equidistant(make_shape(PlatonicSolid::Octahedron).vertices());
    }

    #[test]
    fn octahedron_edges() {
        let shape = make_shape(PlatonicSolid::Octahedron);
        let vertices = shape.vertices();

        for i in 0..super::OCTA_VERTEX {
            for j in super::OCTA_EDGES[i].into_iter().filter(|&x| x > i) {
                distance_neighbour(1., vertices, i, j);
            }
        }
    }

    #[test]
    fn octahedron_diameters() {
        let shape = make_shape(PlatonicSolid::Octahedron);
        let vertices = shape.vertices();
        let diameter = num_traits::FloatConst::SQRT_2();

        for i in 0..super::OCTA_VERTEX / 2 {
            distance_neighbour(diameter, vertices, i, super::OCTA_VERTEX - 1 - i);
        }
    }

    #[test]
    fn dodecahedron_centered() {
        equidistant(make_shape(PlatonicSolid::Dodecahedron).vertices());
    }

    #[test]
    fn dodecahedron_edges() {
        let shape = make_shape(PlatonicSolid::Dodecahedron);
        let vertices = shape.vertices();

        for i in 0..super::DODECA_VERTEX {
            for j in super::DODECA_EDGES[i].into_iter().filter(|&x| x > i) {
                distance_neighbour(1., vertices, i, j);
            }
        }
    }

    #[test]
    fn dodecahedron_diameters() {
        let shape = make_shape(PlatonicSolid::Dodecahedron);
        let vertices = shape.vertices();
        let diameter = Real::sqrt(3. * PHI * PHI);

        for i in 0..super::DODECA_VERTEX / 2 {
            near_distance_neighbour(diameter, 2, vertices, i, super::DODECA_VERTEX - 1 - i);
        }
    }

    #[test]
    fn dodecahedron_nonadjacents_first() {
        let shape = make_shape(PlatonicSolid::Dodecahedron);
        let vertices = shape.vertices();

        for i in 0..super::DODECA_VERTEX {
            for j in super::DODECA_NEXTS[i].into_iter().filter(|&x| x > i) {
                distance_neighbour(PHI, vertices, i, j);
            }
        }
    }

    #[test]
    fn dodecahedron_nonadjacents_second() {
        let shape = make_shape(PlatonicSolid::Dodecahedron);
        let vertices = shape.vertices();
        let distance = Real::sqrt(2. * PHI * PHI);

        for i in 0..super::DODECA_VERTEX {
            for j in super::DODECA_NEXTS[super::DODECA_VERTEX - 1 - i].into_iter().filter(|&x| x > i) {
                near_distance_neighbour(distance, 2, vertices, i, j);
            }
        }
    }

    #[test]
    fn dodecahedron_nonadjacents_third() {
        let shape = make_shape(PlatonicSolid::Dodecahedron);
        let vertices = shape.vertices();

        for i in 0..super::DODECA_VERTEX {
            for j in super::DODECA_EDGES[super::DODECA_VERTEX - 1 - i].into_iter().filter(|&x| x > i) {
                distance_neighbour(1. + PHI, vertices, i, j);
            }
        }
    }

    #[test]
    fn icosahedron_centered() {
        equidistant(make_shape(PlatonicSolid::Icosahedron).vertices());
    }

    #[test]
    fn icosahedron_edges() {
        let shape = make_shape(PlatonicSolid::Icosahedron);
        let vertices = shape.vertices();

        for i in 0..super::ICOSA_VERTEX {
            for j in super::ICOSA_EDGES[i].into_iter().filter(|&x| x > i) {
                distance_neighbour(1., vertices, i, j);
            }
        }
    }

    #[test]
    fn icosahedron_diameters() {
        let shape = make_shape(PlatonicSolid::Icosahedron);
        let vertices = shape.vertices();
        let diameter = Real::sqrt(2. + PHI);

        for i in 0..super::ICOSA_VERTEX / 2 {
            distance_neighbour(diameter, vertices, i, super::ICOSA_VERTEX - 1 - i);
        }
    }

    #[test]
    fn icosahedron_nonadjacents() {
        let shape = make_shape(PlatonicSolid::Icosahedron);
        let vertices = shape.vertices();

        for i in 0..super::ICOSA_VERTEX {
            for j in super::ICOSA_EDGES[super::ICOSA_VERTEX - 1 - i].into_iter().filter(|&x| x > i) {
                distance_neighbour(PHI, vertices, i, j);
            }
        }
    }
}
