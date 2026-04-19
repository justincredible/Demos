use num_traits::{cast, Float, FloatConst, NumCast, one, Unsigned, zero};

use crate::shapes::{Configuration, Shape, Shaper};
use crate::prefab::pentagonal::{Edge, Pentagonal};

/// All possible Kepler-Poinsot polyhedra.
pub enum KpPolyhedron {
    StellatedDodecahedron,
    GreatDodecahedron,
    GreatStellatedDodecahedron,
    GreatIcosahedron,
}

impl<C, I> Shaper<C, I> for KpPolyhedron
where
    C: Float + FloatConst,
    I: Copy + NumCast + Unsigned,
{
    fn make(&self, _request: Configuration) -> Shape<C, I> {
        match self {
            Self::StellatedDodecahedron => {
                // Dodecahedron
                let f0 = zero();
                let fh = cast::<_, C>(0.5).unwrap();
                let pent = Pentagonal::new(Edge::Unit);
                let agon: Pentagonal<C> = Pentagonal::new(Edge::Phi);
                let half_iz = fh * pent.axis.0;

                // Stellation
                let f2 = cast::<_, C>(2.).unwrap();
                let f26 = cast::<_, C>(26.).unwrap();
                let f58 = cast::<_, C>(58.).unwrap();
                let fq = cast::<_, C>(0.25).unwrap();
                let sr5 = cast::<_, C>(5.).unwrap().sqrt();
                let point_top = fq * (f26 + f58 / sr5).sqrt();
                let phi2_width = fh * (f2 + sr5);

                let vertices = vec![
                    [f0, f0, f2 * agon.center + half_iz],
                    [-agon.width, point_top, pent.radius + half_iz],
                    [agon.width, point_top, pent.radius + half_iz],
                    [-phi2_width, -pent.center, pent.radius + half_iz],
                    [phi2_width, -pent.center, pent.radius + half_iz],
                    [f0, -f2 * agon.center, pent.radius + half_iz],
                    [f0, f2 * agon.center, -pent.radius - half_iz],
                    [-phi2_width, pent.center, -pent.radius - half_iz],
                    [phi2_width, pent.center, -pent.radius - half_iz],
                    [-agon.width, -point_top, -pent.radius - half_iz],
                    [agon.width, -point_top, -pent.radius - half_iz],
                    [f0, f0, -f2 * agon.center - half_iz],
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
                    .chain((2..32).map(|i| cast::<_, I>(i).unwrap()))
                    .collect::<Vec<_>>();

                let indices = vec![
                    i[0], i[12], i[13], i[0], i[14], i[12], i[0], i[13], i[15], i[0], i[16], i[14], i[0], i[15], i[16],
                    i[1], i[13], i[12], i[1], i[12], i[17], i[1], i[18], i[13], i[1], i[17], i[22], i[1], i[22], i[18],
                    i[2], i[12], i[14], i[2], i[17], i[12], i[2], i[14], i[19], i[2], i[23], i[17], i[2], i[19], i[23],
                    i[3], i[15], i[13], i[3], i[13], i[18], i[3], i[20], i[15], i[3], i[18], i[24], i[3], i[24], i[20],
                    i[4], i[14], i[16], i[4], i[19], i[14], i[4], i[16], i[21], i[4], i[25], i[19], i[4], i[21], i[25],
                    i[5], i[16], i[15], i[5], i[15], i[20], i[5], i[21], i[16], i[5], i[20], i[26], i[5], i[26], i[21],
                    i[6], i[22], i[17], i[6], i[17], i[23], i[6], i[27], i[22], i[6], i[23], i[28], i[6], i[28], i[27],
                    i[7], i[18], i[22], i[7], i[24], i[18], i[7], i[22], i[27], i[7], i[29], i[24], i[7], i[27], i[29],
                    i[8], i[23], i[19], i[8], i[19], i[25], i[8], i[28], i[23], i[8], i[25], i[30], i[8], i[30], i[28],
                    i[9], i[20], i[24], i[9], i[26], i[20], i[9], i[24], i[29], i[9], i[31], i[26], i[9], i[29], i[31],
                    i[10], i[25], i[21], i[10], i[21], i[26], i[10], i[30], i[25], i[10], i[26], i[31], i[10], i[31], i[30],
                    i[11], i[27], i[28], i[11], i[29], i[27], i[11], i[28], i[30], i[11], i[31], i[29], i[11], i[30], i[31],
                ];

                Shape::Triangles { vertices, indices }
            },
            Self::GreatDodecahedron => {
                // Icosahedron
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

                let indices = vec![
                    i[1], i[2], i[3], i[3], i[2], i[4], i[3], i[4], i[5],
                    i[0], i[3], i[2], i[2], i[3], i[7], i[2], i[7], i[6],
                    i[0], i[1], i[4], i[4], i[1], i[6], i[4], i[6], i[8],
                    i[0], i[5], i[1], i[1], i[5], i[9], i[1], i[9], i[7],
                    i[0], i[2], i[5], i[5], i[2], i[8], i[5], i[8], i[10],
                    i[0], i[4], i[3], i[3], i[4], i[10], i[3], i[10], i[9],
                    i[1], i[7], i[2], i[2], i[7], i[11], i[2], i[11], i[8],
                    i[1], i[3], i[6], i[6], i[3], i[9], i[6], i[9], i[11],
                    i[2], i[6], i[4], i[4], i[6], i[11], i[4], i[11], i[10],
                    i[3], i[5], i[7], i[7], i[5], i[10], i[7], i[10], i[11],
                    i[4], i[8], i[5], i[5], i[8], i[11], i[5], i[11], i[9],
                    i[6], i[7], i[8], i[8], i[7], i[9], i[8], i[9], i[10],
                ];

                Shape::Triangles { vertices, indices }
            },
            Self::GreatStellatedDodecahedron => {
                // Icosahedron
                let f0 = zero();
                let fh = cast::<_, C>(0.5).unwrap();
                let pent = Pentagonal::new(Edge::Unit);
                let y_diff = pent.axis.0;
                let half_middle = fh * pent.axis.1;

                // GSD
                let agon: Pentagonal<C> = Pentagonal::new(Edge::Phi);
                let f2 = cast::<_, C>(2.).unwrap();
                let f3 = cast::<_, C>(3.).unwrap();
                let fq = cast::<_, C>(0.25).unwrap();
                let sr5 = cast::<_, C>(5.).unwrap().sqrt();
                let phi2h = fq * (f3 + sr5);
                let phi3h = fh * (f2 + sr5);
                let phi2r_phim = agon.radius + pent.radius - agon.middle;
                let phir_phi2m = agon.radius - agon.middle - pent.middle;
                let phi2c_phim = agon.center + pent.center + agon.middle;
                let phi2y = pent.axis.0 + pent.axis.1;

                let vertices = vec![
                    [-pent.width, phi2y + half_middle, -agon.center],
                    [pent.width, phi2y + half_middle, -agon.center],
                    [-phi2h, phi2y + half_middle, agon.middle],
                    [phi2h, phi2y + half_middle, agon.middle],
                    [f0, phi2y + half_middle, agon.radius],
                    [-phi2h, half_middle, -phi2r_phim],
                    [phi2h, half_middle, -phi2r_phim],
                    [-phi3h, half_middle, phir_phi2m],
                    [phi3h, half_middle, phir_phi2m],
                    [f0, half_middle, phi2c_phim],
                    [f0, -half_middle, -phi2c_phim],
                    [-phi3h, -half_middle, -phir_phi2m],
                    [phi3h, -half_middle, -phir_phi2m],
                    [-phi2h, -half_middle, phi2r_phim],
                    [phi2h, -half_middle, phi2r_phim],
                    [f0, -phi2y - half_middle, -agon.radius],
                    [-phi2h, -phi2y - half_middle, -agon.middle],
                    [phi2h, -phi2y - half_middle, -agon.middle],
                    [-pent.width, -phi2y - half_middle, agon.center],
                    [pent.width, -phi2y - half_middle, agon.center],
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
                    .chain((2..32).map(|i| cast::<_, I>(i).unwrap()))
                    .collect::<Vec<_>>();

                let indices = vec![
                    i[0], i[20], i[21], i[0], i[22], i[20], i[0], i[21], i[22],
                    i[1], i[21], i[20], i[1], i[20], i[23], i[1], i[23], i[21],
                    i[2], i[20], i[22], i[2], i[24], i[20], i[2], i[22], i[24],
                    i[3], i[23], i[20], i[3], i[20], i[25], i[3], i[25], i[23],
                    i[4], i[20], i[24], i[4], i[25], i[20], i[4], i[24], i[25],
                    i[5], i[22], i[21], i[5], i[21], i[26], i[5], i[26], i[22],
                    i[6], i[21], i[23], i[6], i[27], i[21], i[6], i[23], i[27],
                    i[7], i[24], i[22], i[7], i[22], i[28], i[7], i[28], i[24],
                    i[8], i[23], i[25], i[8], i[29], i[23], i[8], i[25], i[29],
                    i[9], i[25], i[24], i[9], i[24], i[30], i[9], i[30], i[25],
                    i[10], i[26], i[21], i[10], i[21], i[27], i[10], i[27], i[26],
                    i[11], i[22], i[26], i[11], i[28], i[22], i[11], i[26], i[28],
                    i[12], i[27], i[23], i[12], i[23], i[29], i[12], i[29], i[27],
                    i[13], i[24], i[28], i[13], i[30], i[24], i[13], i[28], i[30],
                    i[14], i[29], i[25], i[14], i[25], i[30], i[14], i[30], i[29],
                    i[15], i[26], i[27], i[15], i[31], i[26], i[15], i[27], i[31],
                    i[16], i[28], i[26], i[16], i[26], i[31], i[16], i[31], i[28],
                    i[17], i[27], i[29], i[17], i[31], i[27], i[17], i[29], i[31],
                    i[18], i[30], i[28], i[18], i[28], i[31], i[18], i[31], i[30],
                    i[19], i[29], i[30], i[19], i[31], i[29], i[19], i[30], i[31],
                ];

                Shape::Triangles { vertices, indices }
            },
            Self::GreatIcosahedron => {
                // Icosahedron
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

                let indices = vec![
                    i[0], i[6], i[9], i[0], i[9], i[6],
                    i[0], i[6], i[10], i[0], i[10], i[6],
                    i[0], i[7], i[8], i[0], i[8], i[7],
                    i[0], i[7], i[10], i[0], i[10], i[7],
                    i[0], i[8], i[9], i[0], i[9], i[8],
                    i[1], i[4], i[9], i[1], i[9], i[4],
                    i[1], i[4], i[11], i[1], i[11], i[4],
                    i[1], i[5], i[8], i[1], i[8], i[5],
                    i[1], i[5], i[11], i[1], i[11], i[5],
                    i[1], i[8], i[9], i[1], i[9], i[8],
                    i[2], i[3], i[10], i[2], i[10], i[3],
                    i[2], i[3], i[11], i[2], i[11], i[3],
                    i[2], i[5], i[7], i[2], i[7], i[5],
                    i[2], i[5], i[11], i[2], i[11], i[5],
                    i[2], i[7], i[10], i[2], i[10], i[7],
                    i[3], i[4], i[6], i[3], i[6], i[4],
                    i[3], i[4], i[11], i[3], i[11], i[4],
                    i[3], i[6], i[10], i[3], i[10], i[6],
                    i[4], i[6], i[9], i[4], i[9], i[6],
                    i[5], i[7], i[8], i[5], i[8], i[7],
                ];

                Shape::Triangles { vertices, indices }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{KpPolyhedron, Shape, Shaper};

    use crate::prefab::unit_test::{equidistant, distance_neighbour};

    type Real = f64;

    const PHI: Real = 1.6180339887498948482;
    const STAR_EDGE: Real = 4.2360679774997896964; // 2*phi + 1

    fn make_shape(solid: KpPolyhedron) -> Shape<Real, u8> {
        solid.make(Default::default())
    }

    #[test]
    fn stellated_dodecahedron_centered() {
        // only test the polyhedral vertices
        equidistant(&make_shape(KpPolyhedron::StellatedDodecahedron).vertices()[0..12]);
    }

    #[test]
    fn stellated_dodecahedron_edges() {
        let shape = make_shape(KpPolyhedron::StellatedDodecahedron);
        let vertices = shape.vertices();

        distance_neighbour(STAR_EDGE, vertices, 0, 6);
        distance_neighbour(STAR_EDGE, vertices, 0, 7);
        distance_neighbour(STAR_EDGE, vertices, 0, 8);
        distance_neighbour(STAR_EDGE, vertices, 0, 9);
        distance_neighbour(STAR_EDGE, vertices, 0, 10);
        distance_neighbour(STAR_EDGE, vertices, 1, 4);
        distance_neighbour(STAR_EDGE, vertices, 1, 5);
        distance_neighbour(STAR_EDGE, vertices, 1, 8);
        distance_neighbour(STAR_EDGE, vertices, 1, 9);
        distance_neighbour(STAR_EDGE, vertices, 1, 11);
        distance_neighbour(STAR_EDGE, vertices, 2, 3);
        distance_neighbour(STAR_EDGE, vertices, 2, 5);
        distance_neighbour(STAR_EDGE, vertices, 2, 7);
        distance_neighbour(STAR_EDGE, vertices, 2, 10);
        distance_neighbour(STAR_EDGE, vertices, 2, 11);
        distance_neighbour(STAR_EDGE, vertices, 3, 4);
        distance_neighbour(STAR_EDGE, vertices, 3, 6);
        distance_neighbour(STAR_EDGE, vertices, 3, 10);
        distance_neighbour(STAR_EDGE, vertices, 3, 11);
        distance_neighbour(STAR_EDGE, vertices, 4, 6);
        distance_neighbour(STAR_EDGE, vertices, 4, 9);
        distance_neighbour(STAR_EDGE, vertices, 4, 11);
        distance_neighbour(STAR_EDGE, vertices, 5, 7);
        distance_neighbour(STAR_EDGE, vertices, 5, 8);
        distance_neighbour(STAR_EDGE, vertices, 5, 11);
        distance_neighbour(STAR_EDGE, vertices, 6, 9);
        distance_neighbour(STAR_EDGE, vertices, 6, 10);
        distance_neighbour(STAR_EDGE, vertices, 7, 8);
        distance_neighbour(STAR_EDGE, vertices, 7, 10);
        distance_neighbour(STAR_EDGE, vertices, 8, 9);
    }

    #[test]
    fn great_dodecahedron_centered() {
        equidistant(make_shape(KpPolyhedron::GreatDodecahedron).vertices());
    }

    #[test]
    fn great_dodecahedron_edges() {
        let shape = make_shape(KpPolyhedron::GreatDodecahedron);
        let vertices = shape.vertices();

        distance_neighbour(1., vertices, 0, 1);
        distance_neighbour(1., vertices, 0, 2);
        distance_neighbour(1., vertices, 0, 3);
        distance_neighbour(1., vertices, 0, 4);
        distance_neighbour(1., vertices, 0, 5);
        distance_neighbour(1., vertices, 1, 2);
        distance_neighbour(1., vertices, 1, 3);
        distance_neighbour(1., vertices, 1, 6);
        distance_neighbour(1., vertices, 1, 7);
        distance_neighbour(1., vertices, 2, 4);
        distance_neighbour(1., vertices, 2, 6);
        distance_neighbour(1., vertices, 2, 8);
        distance_neighbour(1., vertices, 3, 5);
        distance_neighbour(1., vertices, 3, 7);
        distance_neighbour(1., vertices, 3, 9);
        distance_neighbour(1., vertices, 4, 5);
        distance_neighbour(1., vertices, 4, 8);
        distance_neighbour(1., vertices, 4, 10);
        distance_neighbour(1., vertices, 5, 9);
        distance_neighbour(1., vertices, 5, 10);
        distance_neighbour(1., vertices, 6, 7);
        distance_neighbour(1., vertices, 6, 8);
        distance_neighbour(1., vertices, 6, 11);
        distance_neighbour(1., vertices, 7, 9);
        distance_neighbour(1., vertices, 7, 11);
        distance_neighbour(1., vertices, 8, 10);
        distance_neighbour(1., vertices, 8, 11);
        distance_neighbour(1., vertices, 9, 10);
        distance_neighbour(1., vertices, 9, 11);
        distance_neighbour(1., vertices, 10, 11);
    }

    #[test]
    fn great_stellated_dodecahedron_centered() {
        // only test the polyhedral vertices
        equidistant(&make_shape(KpPolyhedron::GreatStellatedDodecahedron).vertices()[0..20]);
    }

    #[test]
    fn great_stellated_dodecahedron_edges() {
        let shape = make_shape(KpPolyhedron::GreatStellatedDodecahedron);
        let vertices = shape.vertices();

        distance_neighbour(STAR_EDGE, vertices, 0, 14);
        distance_neighbour(STAR_EDGE, vertices, 0, 17);
        distance_neighbour(STAR_EDGE, vertices, 0, 18);
        distance_neighbour(STAR_EDGE, vertices, 1, 13);
        distance_neighbour(STAR_EDGE, vertices, 1, 16);
        distance_neighbour(STAR_EDGE, vertices, 1, 19);
        distance_neighbour(STAR_EDGE, vertices, 2, 12);
        distance_neighbour(STAR_EDGE, vertices, 2, 15);
        distance_neighbour(STAR_EDGE, vertices, 2, 19);
        distance_neighbour(STAR_EDGE, vertices, 3, 11);
        distance_neighbour(STAR_EDGE, vertices, 3, 15);
        distance_neighbour(STAR_EDGE, vertices, 3, 18);
        distance_neighbour(STAR_EDGE, vertices, 4, 10);
        distance_neighbour(STAR_EDGE, vertices, 4, 16);
        distance_neighbour(STAR_EDGE, vertices, 4, 17);
        distance_neighbour(STAR_EDGE, vertices, 5, 8);
        distance_neighbour(STAR_EDGE, vertices, 5, 9);
        distance_neighbour(STAR_EDGE, vertices, 5, 19);
        distance_neighbour(STAR_EDGE, vertices, 6, 7);
        distance_neighbour(STAR_EDGE, vertices, 6, 9);
        distance_neighbour(STAR_EDGE, vertices, 6, 18);
        distance_neighbour(STAR_EDGE, vertices, 7, 8);
        distance_neighbour(STAR_EDGE, vertices, 7, 17);
        distance_neighbour(STAR_EDGE, vertices, 8, 16);
        distance_neighbour(STAR_EDGE, vertices, 9, 15);
        distance_neighbour(STAR_EDGE, vertices, 10, 13);
        distance_neighbour(STAR_EDGE, vertices, 10, 14);
        distance_neighbour(STAR_EDGE, vertices, 11, 12);
        distance_neighbour(STAR_EDGE, vertices, 11, 14);
        distance_neighbour(STAR_EDGE, vertices, 12, 13);
    }

    #[test]
    fn great_icosahedron_centered() {
        equidistant(make_shape(KpPolyhedron::GreatIcosahedron).vertices());
    }

    #[test]
    fn great_icosahedron_edges() {
        let shape = make_shape(KpPolyhedron::GreatIcosahedron);
        let vertices = shape.vertices();

        distance_neighbour(PHI, vertices, 0, 6);
        distance_neighbour(PHI, vertices, 0, 7);
        distance_neighbour(PHI, vertices, 0, 8);
        distance_neighbour(PHI, vertices, 0, 9);
        distance_neighbour(PHI, vertices, 0, 10);
        distance_neighbour(PHI, vertices, 1, 4);
        distance_neighbour(PHI, vertices, 1, 5);
        distance_neighbour(PHI, vertices, 1, 8);
        distance_neighbour(PHI, vertices, 1, 9);
        distance_neighbour(PHI, vertices, 1, 11);
        distance_neighbour(PHI, vertices, 2, 3);
        distance_neighbour(PHI, vertices, 2, 5);
        distance_neighbour(PHI, vertices, 2, 7);
        distance_neighbour(PHI, vertices, 2, 10);
        distance_neighbour(PHI, vertices, 2, 11);
        distance_neighbour(PHI, vertices, 3, 4);
        distance_neighbour(PHI, vertices, 3, 6);
        distance_neighbour(PHI, vertices, 3, 10);
        distance_neighbour(PHI, vertices, 3, 11);
        distance_neighbour(PHI, vertices, 4, 6);
        distance_neighbour(PHI, vertices, 4, 9);
        distance_neighbour(PHI, vertices, 4, 11);
        distance_neighbour(PHI, vertices, 5, 7);
        distance_neighbour(PHI, vertices, 5, 8);
        distance_neighbour(PHI, vertices, 5, 11);
        distance_neighbour(PHI, vertices, 6, 9);
        distance_neighbour(PHI, vertices, 6, 10);
        distance_neighbour(PHI, vertices, 7, 8);
        distance_neighbour(PHI, vertices, 7, 10);
        distance_neighbour(PHI, vertices, 8, 9);
    }
}
