use std::collections::HashSet;
use num_traits::{cast, Float, FloatConst, one, zero};

use crate::prefab::{
    pentagonal::{Edge, Pentagonal},
    platonic_solid::PlatonicSolid,
    polyhedral::{platonic_solid, Polyhedral},
};

/// All possible Kepler-Poinsot polyhedra.
#[derive(Clone, Copy)]
pub enum KpPolyhedron {
    StellatedDodecahedron,
    GreatDodecahedron,
    GreatStellatedDodecahedron,
    GreatIcosahedron,
}

impl Polyhedral for KpPolyhedron
{
    fn vertices<C>(&self) -> Vec<[C; 3]>
    where C: Float + FloatConst
    {
        match self {
            Self::StellatedDodecahedron => {
                let f0 = zero();
                let f1 = one::<C>();
                let f2 = cast::<_, C>(2.).unwrap();
                let f3 = cast::<_, C>(3.).unwrap();
                let fh = cast::<_, C>(0.5).unwrap();
                let sr5 = cast::<_, C>(5.).unwrap().sqrt();
                let npsi = fh * (sr5 - f1);
                let psi2 = fh * (f3 - sr5);
                let npsi3h = fh * (sr5 - f2);
                let pent = Pentagonal::<C>::new(Edge::Unit);

                PlatonicSolid::Icosahedron.vertices()
                    .into_iter()
                    .map(|[x,y,z]: [C; 3]| [x,-z,y])
                    .chain([
                        [-fh * psi2, psi2 * pent.center, fh * pent.radius],
                        [fh * psi2, psi2 * pent.center, fh * pent.radius],
                        [-fh * npsi, -psi2 * pent.middle, fh * pent.radius],
                        [fh * npsi, -psi2 * pent.middle, fh * pent.radius],
                        [f0, psi2 * -pent.radius, fh * pent.radius],
                        [-fh * npsi, npsi * pent.center, npsi3h * pent.radius],
                        [fh * npsi, npsi * pent.center, npsi3h * pent.radius],
                        [-fh, -npsi * pent.middle, npsi3h * pent.radius],
                        [fh, -npsi * pent.middle, npsi3h * pent.radius],
                        [f0, -npsi * pent.radius, npsi3h * pent.radius],
                        [f0, npsi * pent.radius, -npsi3h * pent.radius],
                        [-fh, npsi * pent.middle, -npsi3h * pent.radius],
                        [fh, npsi * pent.middle, -npsi3h * pent.radius],
                        [-fh * npsi, -npsi * pent.center, -npsi3h * pent.radius],
                        [fh * npsi, -npsi * pent.center, -npsi3h * pent.radius],
                        [f0, psi2 * pent.radius, -fh * pent.radius],
                        [-fh * npsi, psi2 * pent.middle, -fh * pent.radius],
                        [fh * npsi, psi2 * pent.middle, -fh * pent.radius],
                        [-fh * psi2, -psi2 * pent.center, -fh * pent.radius],
                        [fh * psi2, -psi2 * pent.center, -fh * pent.radius],
                    ])
                    .collect()
            },
            Self::GreatDodecahedron => {
                PlatonicSolid::Icosahedron.vertices()
                    .into_iter()
                    .map(|[x,y,z]: [C; 3]| [x,-z,y])
                    .collect()
            },
            Self::GreatStellatedDodecahedron => {
                let f0 = zero::<C>();
                let f1 = one::<C>();
                let f3 = cast::<_, C>(3.).unwrap();
                let f5 = cast::<_, C>(5.).unwrap();
                let fh = cast::<_, C>(0.5).unwrap();
                let fq = cast::<_, C>(0.25).unwrap();
                let sr5 = cast::<_, C>(5.).unwrap().sqrt();
                let npsi = fh * (sr5 - f1);
                let psi2 = fh * (f3 - sr5);
                let pent = Pentagonal::new(Edge::Unit);
                let half_height = fq * (f5 - sr5) * pent.radius;
                let circle = fh * npsi * pent.radius;

                PlatonicSolid::Dodecahedron.vertices()
                    .into_iter()
                    .chain([
                        [f0, f0, half_height],
                        [-fh * npsi, -psi2 * pent.middle + npsi * pent.radius, circle],
                        [fh * npsi, -psi2 * pent.middle + npsi * pent.radius, circle],
                        [-fh, -psi2 * pent.radius + npsi * pent.middle, circle],
                        [fh, -psi2 * pent.radius + npsi * pent.middle, circle],
                        [f0, -npsi * pent.radius, circle],
                        [f0, npsi * pent.radius, -circle],
                        [-fh, psi2 * pent.radius - npsi * pent.middle, -circle],
                        [fh, psi2 * pent.radius - npsi * pent.middle, -circle],
                        [-fh * npsi, psi2 * pent.middle - npsi * pent.radius, -circle],
                        [fh * npsi, psi2 * pent.middle - npsi * pent.radius, -circle],
                        [f0, f0, -half_height],
                    ])
                    .collect()
            },
            Self::GreatIcosahedron => {
                PlatonicSolid::Icosahedron.vertices()
                    .into_iter()
                    .map(|[x,y,z]: [C; 3]| [x,z,y])
                    .collect()
            },
        }
    }

    fn edges(&self) -> Vec<Vec<usize>> {
        match self {
            Self::GreatStellatedDodecahedron => [
                [14,17,18], [13,16,19], [12,15,19], [11,15,18], [10,16,17],
                [8,9,19], [7,9,18], [6,8,17], [5,7,16], [5,6,15],
                [4,13,14], [3,12,14], [2,11,13], [1,10,12], [0,10,11],
                [2,3,9], [1,4,8], [0,4,7], [0,3,6], [1,2,5],
            ].iter().map(Into::into).collect(),
            _ => {
                let iterator = [
                    [6,7,8,9,10], [4,5,8,9,11], [3,5,7,10,11], [2,4,6,10,11], [1,3,6,9,11], [1,2,7,8,11],
                    [0,3,4,9,10], [0,2,5,8,10], [0,1,5,7,9], [0,1,4,6,8], [0,2,3,6,7], [1,2,3,4,5],
                ].iter().map(Into::into);
                if matches!(self, Self::GreatDodecahedron) {
                    iterator.rev().collect()
                } else {
                    iterator.collect()
                }
            },
        }
    }

    fn vertices_per_face(&self) -> usize {
        // geometrically incorrect but matches `faces()` definition
        match self {
            Self::GreatDodecahedron => 5,
            _ => 3,
        }
    }

    fn faces(&self) -> HashSet<Vec<usize>> {
        match self {
            Self::GreatIcosahedron => platonic_solid(self),
            Self::GreatDodecahedron => self.edges().into_iter().collect(),
            Self::StellatedDodecahedron => [
                [0, 13, 12], [0, 12, 14], [0, 15, 13], [0, 14, 16], [0, 16, 15],
                [1, 12, 13], [1, 17, 12], [1, 13, 18], [1, 22, 17], [1, 18, 22],
                [2, 14, 12], [2, 12, 17], [2, 19, 14], [2, 17, 23], [2, 23, 19],
                [3, 13, 15], [3, 18, 13], [3, 15, 20], [3, 24, 18], [3, 20, 24],
                [4, 16, 14], [4, 14, 19], [4, 21, 16], [4, 19, 25], [4, 25, 21],
                [5, 15, 16], [5, 20, 15], [5, 16, 21], [5, 26, 20], [5, 21, 26],
                [6, 17, 22], [6, 23, 17], [6, 22, 27], [6, 28, 23], [6, 27, 28],
                [7, 22, 18], [7, 18, 24], [7, 27, 22], [7, 24, 29], [7, 29, 27],
                [8, 19, 23], [8, 25, 19], [8, 23, 28], [8, 30, 25], [8, 28, 30],
                [9, 24, 20], [9, 20, 26], [9, 29, 24], [9, 26, 31], [9, 31, 29],
                [10, 21, 25], [10, 26, 21], [10, 25, 30], [10, 31, 26], [10, 30, 31],
                [11, 28, 27], [11, 27, 29], [11, 30, 28], [11, 29, 31], [11, 31, 30],
            ].iter().map(Into::into).collect(),
            Self::GreatStellatedDodecahedron => [
                [0, 21, 20], [0, 20, 22], [0, 22, 21],
                [1, 20, 21], [1, 23, 20], [1, 21, 23],
                [2, 22, 20], [2, 20, 24], [2, 24, 22],
                [3, 20, 23], [3, 25, 20], [3, 23, 25],
                [4, 24, 20], [4, 20, 25], [4, 25, 24],
                [5, 21, 22], [5, 26, 21], [5, 22, 26],
                [6, 23, 21], [6, 21, 27], [6, 27, 23],
                [7, 22, 24], [7, 28, 22], [7, 24, 28],
                [8, 25, 23], [8, 23, 29], [8, 29, 25],
                [9, 24, 25], [9, 30, 24], [9, 25, 30],
                [10, 21, 26], [10, 27, 21], [10, 26, 27],
                [11, 26, 22], [11, 22, 28], [11, 28, 26],
                [12, 23, 27], [12, 29, 23], [12, 27, 29],
                [13, 28, 24], [13, 24, 30], [13, 30, 28],
                [14, 25, 29], [14, 30, 25], [14, 29, 30],
                [15, 27, 26], [15, 26, 31], [15, 31, 27],
                [16, 26, 28], [16, 31, 26], [16, 28, 31],
                [17, 29, 27], [17, 27, 31], [17, 31, 29],
                [18, 28, 30], [18, 31, 28], [18, 30, 31],
                [19, 30, 29], [19, 29, 31], [19, 31, 30],
            ].iter().map(Into::into).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{KpPolyhedron, Polyhedral};

    use crate::{Shape, Shaper};
    use crate::prefab::unit_test::{distance_neighbour, equidistant};

    type Real = f64;

    const PHI: Real = 1.618_033_988_749_895;

    fn make_shape(solid: KpPolyhedron) -> Shape<Real, u8> {
        solid.make(Default::default())
    }

    #[test]
    fn stellated_dodecahedron_centered() {
        let shape = KpPolyhedron::StellatedDodecahedron;
        // test the polyhedral and render vertices separately
        equidistant(&make_shape(shape).vertices()[0..shape.vertex_count()]);
        equidistant(&make_shape(shape).vertices()[shape.vertex_count()..]);
    }

    #[test]
    fn stellated_dodecahedron_edges() {
        let solid = KpPolyhedron::StellatedDodecahedron;
        let shape = make_shape(solid);
        let vertices = shape.vertices();

        for i in 0..solid.vertex_count() {
            for j in solid.edges()[i].iter().map(Clone::clone).filter(|&x| x > i) {
                distance_neighbour(PHI, vertices, i, j);
            }
        }
    }

    #[test]
    fn great_dodecahedron_centered() {
        equidistant(make_shape(KpPolyhedron::GreatDodecahedron).vertices());
    }

    #[test]
    fn great_dodecahedron_edges() {
        let solid = KpPolyhedron::GreatDodecahedron;
        let shape = make_shape(solid);
        let vertices = shape.vertices();

        for i in 0..solid.vertex_count() {
            for j in solid.edges()[i].iter().map(Clone::clone).filter(|&x| x > i) {
                distance_neighbour(1., vertices, i, j);
            }
        }
    }

    #[test]
    fn great_stellated_dodecahedron_centered() {
        let shape = KpPolyhedron::GreatStellatedDodecahedron;
        // test the polyhedral and render vertices separately
        equidistant(&make_shape(shape).vertices()[0..shape.vertex_count()]);
        equidistant(&make_shape(shape).vertices()[shape.vertex_count()..]);
    }

    #[test]
    fn great_stellated_dodecahedron_edges() {
        let solid = KpPolyhedron::GreatStellatedDodecahedron;
        let shape = make_shape(solid);
        let vertices = shape.vertices();

        for i in 0..solid.vertex_count() {
            for j in solid.edges()[i].iter().map(Clone::clone).filter(|&x| x > i) {
                distance_neighbour(PHI + 1., vertices, i, j);
            }
        }
    }

    #[test]
    fn great_icosahedron_centered() {
        equidistant(make_shape(KpPolyhedron::GreatIcosahedron).vertices());
    }

    #[test]
    fn great_icosahedron_edges() {
        let solid = KpPolyhedron::GreatIcosahedron;
        let shape = make_shape(solid);
        let vertices = shape.vertices();

        for i in 0..solid.vertex_count() {
            for j in solid.edges()[i].iter().map(Clone::clone).filter(|&x| x > i) {
                distance_neighbour(PHI, vertices, i, j);
            }
        }
    }
}
