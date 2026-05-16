use num_traits::{cast, Float, FloatConst, NumCast, one, Unsigned, zero};

use crate::shapes::{Configuration, Orientation, Shape, Shaper};
use crate::prefab::{
    pentagonal::{Edge, Pentagonal},
    polyhedral::Polyhedral,
};

/// All possible Platonic solids.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum PlatonicSolid {
    Tetrahedron,
    Hexahedron,
    Octahedron,
    Dodecahedron,
    Icosahedron,
}

impl Polyhedral for PlatonicSolid {
    fn edges(&self) -> Vec<Vec<usize>> {
        match self {
            Self::Tetrahedron => [[1,2,3], [0,2,3], [0,1,3], [0,1,2]].iter().map(Into::into).collect(),
            Self::Hexahedron => [
                [1,2,4], [0,3,5], [0,3,6], [1,2,7],
                [0,5,6], [1,4,7], [2,4,7], [3,5,6],
            ].iter().map(Into::into).collect(),
            Self::Octahedron => [
                [1,2,3,4], [0,2,3,5], [0,1,4,5], [0,1,4,5], [0,2,3,5], [1,2,3,4],
            ].iter().map(Into::into).collect(),
            Self::Dodecahedron => [
                [1,2,5], [0,3,6], [0,4,7], [1,4,8], [2,3,9],
                [0,10,11], [1,10,12], [2,11,13], [3,12,14], [4,13,14],
                [5,6,15], [5,7,16], [6,8,17], [7,9,18], [8,9,19],
                [10,16,17], [11,15,18], [12,15,19], [13,16,19], [14,17,18],
            ].iter().map(Into::into).collect(),
            Self::Icosahedron => [
                [1,2,3,4,5], [0,2,3,6,7], [0,1,4,6,8], [0,1,5,7,9], [0,2,5,8,10], [0,3,5,9,10],
                [1,2,7,8,11], [1,3,6,9,11], [2,4,6,10,11], [3,5,7,10,11], [4,5,8,9,11], [6,7,8,9,10],
            ].iter().map(Into::into).collect(),
        }
    }

    fn vertices_per_face(&self) -> usize {
        match self {
            Self::Tetrahedron | Self::Octahedron | Self::Icosahedron => 3,
            Self::Hexahedron => 4,
            Self::Dodecahedron => 5,
        }
    }
}

const TETRA_STRIP: [usize; 6] = [0, 1, 2, 3, 0, 1];
const HEXA_STRIP: [usize; 14] = [0, 1, 2, 3, 7, 1, 5, 0, 4, 2, 6, 7, 4, 5];
const OCTA_STRIPS: [[usize; 6]; 2] = [[1, 0, 3, 4, 5, 2], [4, 0, 2, 1, 5, 3]];
const DODECA_STRIP: [usize; 38] = [
    0, 1, 2, 3, 4, 9, 2, 13, 7, 11, 2, 5, 0, 10, 1, 6, 3, 8, 9, 14,
    13, 18, 11, 16, 5, 15, 10, 17, 6, 12, 8, 17, 14, 19, 18, 17, 16, 15,
];
const ICOSA_STRIPS: [[usize; 12]; 3] = [
    [0, 1, 2, 6, 8, 11, 10, 9, 5, 3, 0, 1],
    [0, 2, 4, 8, 10, 11, 9, 7, 3, 1, 0, 2],
    [0, 4, 5, 10, 9, 11, 7, 6, 1, 2, 0, 4],
];

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
                    .chain((2..self.vertex_count()).map(|i| cast::<_, I>(i).unwrap()))
                    .collect::<Vec<_>>();

                if request.prefer_strips {
                    let lookup = |idx| if request.orientation.is_ccw() {
                        i[idx]
                    } else {
                        i[self.vertex_count() - 1 - idx]
                    };
                    let strips = vec![TETRA_STRIP.into_iter().map(lookup).collect()];

                    Shape::Strips { vertices, strips }
                } else {
                    let mut indices = vec![];
                    for face in self.faces() {
                        let triangle = oriented_plane(&vertices, &face, request.orientation);
                        for index in triangle {
                            indices.push(i[index]);
                        }
                    }

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
                    .chain((2..self.vertex_count()).map(|i| cast::<_, I>(i).unwrap()))
                    .collect::<Vec<_>>();

                if request.prefer_strips {
                    let lookup = |idx| if request.orientation.is_ccw() {
                        i[idx]
                    } else {
                        i[self.vertex_count() - 1 - idx]
                    };
                    let strips = vec![HEXA_STRIP.into_iter().map(lookup).collect()];

                    Shape::Strips { vertices, strips }
                } else {
                    let mut indices = vec![];
                    for face in self.faces() {
                        let triangle1 = oriented_plane(&vertices, &face[0..3], request.orientation);
                        let triangle2 = oriented_plane(&vertices, &face[1..4], request.orientation);
                        for index in triangle1.into_iter().chain(triangle2) {
                            indices.push(i[index]);
                        }
                    }

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
                    .chain((2..self.vertex_count()).map(|i| cast::<_, I>(i).unwrap()))
                    .collect::<Vec<_>>();

                if request.prefer_strips {
                    let lookup = |idx| if request.orientation.is_ccw() {
                        i[idx]
                    } else {
                        i[self.vertex_count() - 1 - idx]
                    };
                    let mut strips = vec![];
                    for strip in OCTA_STRIPS {
                        strips.push(strip.into_iter().map(lookup).collect());
                    }

                    Shape::Strips { vertices, strips }
                } else {
                    let mut indices = vec![];
                    for face in self.faces() {
                        let triangle = oriented_plane(&vertices, &face, request.orientation);
                        for index in triangle {
                            indices.push(i[index]);
                        }
                    }

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
                    .chain((2..self.vertex_count()).map(|i| cast::<_, I>(i).unwrap()))
                    .collect::<Vec<_>>();

                if request.prefer_strips {
                    let lookup = |idx| if request.orientation.is_ccw() {
                        i[idx]
                    } else {
                        i[self.vertex_count() - 1 - idx]
                    };
                    let strips = vec![DODECA_STRIP.into_iter().map(lookup).collect()];

                    Shape::Strips { vertices, strips }
                } else {
                    let mut indices = vec![];
                    for face in self.faces() {
                        let triangle1 = oriented_plane(&vertices, &face[0..3], request.orientation);
                        let triangle2 = oriented_plane(&vertices, &face[1..4], request.orientation);
                        let triangle3 = oriented_plane(&vertices, &face[2..5], request.orientation);
                        for index in triangle1.into_iter().chain(triangle2).chain(triangle3) {
                            indices.push(i[index]);
                        }
                    }

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
                    .chain((2..self.vertex_count()).map(|i| cast::<_, I>(i).unwrap()))
                    .collect::<Vec<_>>();

                if request.prefer_strips {
                    let lookup = |idx| if request.orientation.is_ccw() {
                        i[idx]
                    } else {
                        i[self.vertex_count() - 1 - idx]
                    };
                    let mut strips = vec![];
                    for strip in ICOSA_STRIPS {
                        strips.push(strip.into_iter().map(lookup).collect());
                    }

                    Shape::Strips { vertices, strips }
                } else {
                    let mut indices = vec![];
                    for face in self.faces() {
                        let triangle = oriented_plane(&vertices, &face, request.orientation);
                        for index in triangle {
                            indices.push(i[index]);
                        }
                    }

                    Shape::Triangles { vertices, indices }
                }
            },
        }
    }
}

fn oriented_plane<C>(vertices: &Vec<[C; 3]>, unoriented: &[usize], orientation: Orientation) -> [usize; 3]
where
    C: Float
{
    let a1 = vertices[unoriented[1]][0] - vertices[unoriented[0]][0];
    let b1 = vertices[unoriented[1]][1] - vertices[unoriented[0]][1];
    let c1 = vertices[unoriented[1]][2] - vertices[unoriented[0]][2];
    let a2 = vertices[unoriented[2]][0] - vertices[unoriented[1]][0];
    let b2 = vertices[unoriented[2]][1] - vertices[unoriented[1]][1];
    let c2 = vertices[unoriented[2]][2] - vertices[unoriented[1]][2];
    let a = b1*c2 - c1*b2;
    let b = c1*a2 - a1*c2;
    let c = a1*b2 - b1*a2;
    let d = a*vertices[unoriented[0]][0] + b*vertices[unoriented[0]][1] + c*vertices[unoriented[0]][2];

    if d > zero() && orientation.is_ccw() || d < zero() && orientation.is_cw() {
        [unoriented[0], unoriented[1], unoriented[2]]
    } else {
        [unoriented[0], unoriented[2], unoriented[1]]
    }
}

#[cfg(test)]
mod tests {
    use super::{PlatonicSolid, Polyhedral, Shape, Shaper};

    use crate::prefab::unit_test::{distance_neighbour, equidistant, near_distance_neighbour};

    type Real = f64;

    const PHI: Real = 1.618_033_988_749_895;
    const DODECA_NEXTS: [[usize; 6]; 20] = [
        [3,4,6,7,10,11], [2,4,5,8,10,12], [1,3,5,9,11,13], [0,2,6,9,12,14], [0,1,7,8,13,14],
        [1,2,6,7,15,16], [0,3,5,8,15,17], [0,4,5,9,16,18], [1,4,6,9,17,19], [2,3,7,8,18,19],
        [0,1,11,12,16,17], [0,2,10,13,15,18], [1,3,10,14,15,19], [2,4,11,14,16,19], [3,4,12,13,17,18],
        [5,6,11,12,18,19], [5,7,10,13,17,19], [6,8,10,14,16,18], [7,9,11,14,15,17], [8,9,12,13,15,16],
    ];


    fn make_shape(solid: PlatonicSolid) -> Shape<Real, u8> {
        solid.make(Default::default())
    }

    #[test]
    fn tetrahedron_vertices() {
        let solid = PlatonicSolid::Tetrahedron;
        assert!(solid.vertex_count() == make_shape(solid).vertices().len());
    }

    #[test]
    fn tetrahedron_centered() {
        equidistant(make_shape(PlatonicSolid::Tetrahedron).vertices());
    }

    #[test]
    fn tetrahedron_edges() {
        let solid = PlatonicSolid::Tetrahedron;
        let shape = make_shape(solid);
        let vertices = shape.vertices();

        for i in 0..solid.vertex_count() {
            for j in solid.edges()[i].iter().map(Clone::clone).filter(|&x| x > i) {
                distance_neighbour(1., vertices, i, j);
            }
        }
    }

    #[test]
    fn tetrahedron_diameters() {
        tetrahedron_edges();
    }

    #[test]
    fn hexahedron_vertices() {
        let solid = PlatonicSolid::Hexahedron;
        assert!(solid.vertex_count() == make_shape(solid).vertices().len());
    }

    #[test]
    fn hexahedron_centered() {
        equidistant(make_shape(PlatonicSolid::Hexahedron).vertices());
    }

    #[test]
    fn hexahedron_edges() {
        let solid = PlatonicSolid::Hexahedron;
        let shape = make_shape(solid);
        let vertices = shape.vertices();

        for i in 0..solid.vertex_count() {
            for j in solid.edges()[i].iter().map(Clone::clone).filter(|&x| x > i) {
                distance_neighbour(1., vertices, i, j);
            }
        }
    }

    #[test]
    fn hexahedron_diameters() {
        let solid = PlatonicSolid::Hexahedron;
        let shape = make_shape(solid);
        let vertices = shape.vertices();
        let diameter = Real::sqrt(3.);

        for i in 0..solid.vertex_count() / 2 {
            distance_neighbour(diameter, vertices, i, solid.vertex_count() - 1 - i);
        }
    }

    #[test]
    fn hexahedron_nonadjacents() {
        let solid = PlatonicSolid::Hexahedron;
        let shape = make_shape(solid);
        let vertices = shape.vertices();
        let distance = num_traits::FloatConst::SQRT_2();

        for i in 0..solid.vertex_count() {
            for j in solid.edges()[solid.vertex_count() - 1 - i].iter().map(Clone::clone).filter(|&x| x > i) {
                distance_neighbour(distance, vertices, i, j);
            }
        }
    }

    #[test]
    fn octahedron_vertices() {
        let solid = PlatonicSolid::Octahedron;
        assert!(solid.vertex_count() == make_shape(solid).vertices().len());
    }

    #[test]
    fn octahedron_centered() {
        equidistant(make_shape(PlatonicSolid::Octahedron).vertices());
    }

    #[test]
    fn octahedron_edges() {
        let solid = PlatonicSolid::Octahedron;
        let shape = make_shape(solid);
        let vertices = shape.vertices();

        for i in 0..solid.vertex_count() {
            for j in solid.edges()[i].iter().map(Clone::clone).filter(|&x| x > i) {
                distance_neighbour(1., vertices, i, j);
            }
        }
    }

    #[test]
    fn octahedron_diameters() {
        let solid = PlatonicSolid::Octahedron;
        let shape = make_shape(solid);
        let vertices = shape.vertices();
        let diameter = num_traits::FloatConst::SQRT_2();

        for i in 0..solid.vertex_count() / 2 {
            distance_neighbour(diameter, vertices, i, solid.vertex_count() - 1 - i);
        }
    }

    #[test]
    fn dodecahedron_vertices() {
        let solid = PlatonicSolid::Dodecahedron;
        assert!(solid.vertex_count() == make_shape(solid).vertices().len());
    }

    #[test]
    fn dodecahedron_centered() {
        equidistant(make_shape(PlatonicSolid::Dodecahedron).vertices());
    }

    #[test]
    fn dodecahedron_edges() {
        let solid = PlatonicSolid::Dodecahedron;
        let shape = make_shape(solid);
        let vertices = shape.vertices();

        for i in 0..solid.vertex_count() {
            for j in solid.edges()[i].iter().map(Clone::clone).filter(|&x| x > i) {
                distance_neighbour(1., vertices, i, j);
            }
        }
    }

    #[test]
    fn dodecahedron_diameters() {
        let solid = PlatonicSolid::Dodecahedron;
        let shape = make_shape(solid);
        let vertices = shape.vertices();
        let diameter = Real::sqrt(3. * PHI * PHI);

        for i in 0..solid.vertex_count() / 2 {
            near_distance_neighbour(diameter, 2, vertices, i, solid.vertex_count() - 1 - i);
        }
    }

    #[test]
    fn dodecahedron_nonadjacents_first() {
        let solid = PlatonicSolid::Dodecahedron;
        let shape = make_shape(solid);
        let vertices = shape.vertices();

        for i in 0..solid.vertex_count() {
            for j in DODECA_NEXTS[i].into_iter().filter(|&x| x > i) {
                distance_neighbour(PHI, vertices, i, j);
            }
        }
    }

    #[test]
    fn dodecahedron_nonadjacents_second() {
        let solid = PlatonicSolid::Dodecahedron;
        let shape = make_shape(solid);
        let vertices = shape.vertices();
        let distance = Real::sqrt(2. * PHI * PHI);

        for i in 0..solid.vertex_count() {
            for j in DODECA_NEXTS[solid.vertex_count() - 1 - i].into_iter().filter(|&x| x > i) {
                near_distance_neighbour(distance, 2, vertices, i, j);
            }
        }
    }

    #[test]
    fn dodecahedron_nonadjacents_third() {
        let solid = PlatonicSolid::Dodecahedron;
        let shape = make_shape(solid);
        let vertices = shape.vertices();

        for i in 0..solid.vertex_count() {
            for j in solid.edges()[solid.vertex_count() - 1 - i].iter().map(Clone::clone).filter(|&x| x > i) {
                distance_neighbour(1. + PHI, vertices, i, j);
            }
        }
    }

    #[test]
    fn icosahedron_vertices() {
        let solid = PlatonicSolid::Icosahedron;
        assert!(solid.vertex_count() == make_shape(solid).vertices().len());
    }

    #[test]
    fn icosahedron_centered() {
        equidistant(make_shape(PlatonicSolid::Icosahedron).vertices());
    }

    #[test]
    fn icosahedron_edges() {
        let solid = PlatonicSolid::Icosahedron;
        let shape = make_shape(solid);
        let vertices = shape.vertices();

        for i in 0..solid.vertex_count() {
            for j in solid.edges()[i].iter().map(Clone::clone).filter(|&x| x > i) {
                distance_neighbour(1., vertices, i, j);
            }
        }
    }

    #[test]
    fn icosahedron_diameters() {
        let solid = PlatonicSolid::Icosahedron;
        let shape = make_shape(solid);
        let vertices = shape.vertices();
        let diameter = Real::sqrt(2. + PHI);

        for i in 0..solid.vertex_count() / 2 {
            distance_neighbour(diameter, vertices, i, solid.vertex_count() - 1 - i);
        }
    }

    #[test]
    fn icosahedron_nonadjacents() {
        let solid = PlatonicSolid::Icosahedron;
        let shape = make_shape(solid);
        let vertices = shape.vertices();

        for i in 0..solid.vertex_count() {
            for j in solid.edges()[solid.vertex_count() - 1 - i].iter().map(Clone::clone).filter(|&x| x > i) {
                distance_neighbour(PHI, vertices, i, j);
            }
        }
    }
}
