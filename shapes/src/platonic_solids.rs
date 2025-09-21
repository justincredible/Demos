use num_traits::{cast, Float, FloatConst, NumCast, one, Unsigned, zero};

use crate::shapes::{Configuration, Shape, Shaper};

/// All possible Platonic solids.
pub enum PlatonicSolids {
    Tetrahedron,
    Hexahedron,
    Octahedron,
    Dodecahedron,
    Icosahedron,
}

impl<C: Float + FloatConst, I: Copy + NumCast + Unsigned> Shaper<C, I> for PlatonicSolids {
    fn make(&self, _request: Configuration) -> Shape<C, I> {
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
                    .chain((2..4)
                        .into_iter()
                        .map(|i| cast::<_, I>(i).unwrap()))
                    .collect::<Vec<_>>();

                let indices = vec![i[0], i[1], i[2], i[3], i[0], i[1]];

                Shape::Strips { vertices, strips: vec!(indices) }
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
                    .chain((2..8)
                        .into_iter()
                        .map(|i| cast::<_, I>(i).unwrap()))
                    .collect::<Vec<_>>();

                let indices = vec![
                    i[0], i[1], i[2], i[3], i[7], i[1], i[5],
                    i[0], i[4], i[2], i[6], i[7], i[4], i[5],
                ];

                Shape::Strips { vertices, strips: vec!(indices) }
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
                    .chain((2..6)
                        .into_iter()
                        .map(|i| cast::<_, I>(i).unwrap()))
                    .collect::<Vec<_>>();

                let indices = vec![
                    i[0], i[1], i[2], i[5], i[3], i[4], i[1],
                    i[5], i[2], i[3], i[0], i[4], i[1],
                ];

                Shape::Strips { vertices, strips: vec!(indices) }
            },
            Self::Dodecahedron => {
                let f0 = zero();
                let f1 = one::<C>();
                let f2 = cast::<_, C>(2.).unwrap();
                let f3 = cast::<_, C>(3.).unwrap();
                let f7 = cast::<_, C>(7.).unwrap();
                let f10 = cast::<_, C>(10.).unwrap();
                let fh = cast::<_, C>(0.5).unwrap();
                let fq = cast::<_, C>(0.25).unwrap();
                let fe = cast::<_, C>(0.125).unwrap();
                let ft = cast::<_, C>(0.1).unwrap();
                let sr5 = cast::<_, C>(5.).unwrap().sqrt();
                let phi = fh * (f1 + sr5);

                let mid = fq * (f10 + f2 * sr5).sqrt();
                let top = fq * (f10 - f2 * sr5).sqrt();
                let width = fq * (f1 + sr5); // phi/2
                let height = top + mid;
                let circle_offset = fq * (f2 + sr5) / height;
                let circle_radius = fq * (f3 + sr5) / height;
                let centred_mid = fe * (f1 + sr5) / height;
                let phi_width = fq * (f3 + sr5);
                // we use the less accurate (phi * circle_offset)
                // to offset some accumulated error in the tests
                let _phi_offset = fe * (f7 + f3 * sr5) / height;
                let phi_radius = fh * (f2 + sr5) / height; // 2 * circle_offset
                let phi_mid = fe * (f3 + sr5) / height; // circle_radius / 2

                let half_iz = fh * (fh - ft * sr5).sqrt();

                let vertices = vec![
                    [f0, circle_radius, circle_radius + half_iz],
                    [-width, centred_mid, circle_radius + half_iz],
                    [width, centred_mid, circle_radius + half_iz],
                    [-fh, -circle_offset, circle_radius + half_iz],
                    [fh, -circle_offset, circle_radius + half_iz],
                    [f0, phi_radius, half_iz],
                    [-phi_width, phi_mid, half_iz],
                    [phi_width, phi_mid, half_iz],
                    [-width, -phi * circle_offset, half_iz],
                    [width, -phi * circle_offset, half_iz],
                    [-width, phi * circle_offset, -half_iz],
                    [width, phi * circle_offset, -half_iz],
                    [-phi_width, -phi_mid, -half_iz],
                    [phi_width, -phi_mid, -half_iz],
                    [f0, -phi_radius, -half_iz],
                    [-fh, circle_offset, -circle_radius - half_iz],
                    [fh, circle_offset, -circle_radius - half_iz],
                    [-width, -centred_mid, -circle_radius - half_iz],
                    [width, -centred_mid, -circle_radius - half_iz],
                    [f0, -circle_radius, -circle_radius - half_iz],
                ];

                let i = vec![zero(), one()]
                    .into_iter()
                    .chain((2..20)
                        .into_iter()
                        .map(|i| cast::<_, I>(i).unwrap()))
                    .collect::<Vec<_>>();

                let indices = vec![
                    i[1], i[3], i[0], i[4], i[2], i[7], i[0], i[11], i[5], i[10],
                    i[0], i[6], i[1], i[12], i[3], i[8], i[4], i[9], i[7], i[13],
                    i[11], i[16], i[10], i[15], i[6], i[17], i[12], i[19], i[8],
                    i[14], i[9], i[19], i[13], i[18], i[16], i[19], i[15], i[17],
                ];

                Shape::Strips { vertices, strips: vec!(indices) }
            },
            Self::Icosahedron => {
                let f0 = zero();
                let f1 = one::<C>();
                let f2 = cast::<_, C>(2.).unwrap();
                let f3 = cast::<_, C>(3.).unwrap();
                let f10 = cast::<_, C>(10.).unwrap();
                let fh = cast::<_, C>(0.5).unwrap();
                let fq = cast::<_, C>(0.25).unwrap();
                let ft = cast::<_, C>(0.1).unwrap();
                let sr5 = cast::<_, C>(5.).unwrap().sqrt();

                let mid = fq * (f10 + f2 * sr5).sqrt();
                let top = fq * (f10 - f2 * sr5).sqrt();
                let width = fq * (f1 + sr5); // phi/2
                let depth = top + mid;
                let center = fq * (f2 + sr5) / depth;
                let radius = fq * (f3 + sr5) / depth;

                let y_diff = (fh - ft * sr5).sqrt();
                let half_middle = fh * (fh + ft * sr5).sqrt();

                let vertices = vec![
                    [f0, half_middle + y_diff, f0],
                    [f0, half_middle, -radius],
                    [-width, half_middle, -radius + top],
                    [width, half_middle, -radius + top],
                    [-fh, half_middle, center],
                    [fh, half_middle, center],
                    [-fh, -half_middle, -center],
                    [fh, -half_middle, -center],
                    [-width, -half_middle, radius - top],
                    [width, -half_middle, radius - top],
                    [f0, -half_middle, radius],
                    [f0, -half_middle - y_diff, f0],
                ];

                let i = vec![zero(), one()]
                    .into_iter()
                    .chain((2..12)
                        .into_iter()
                        .map(|i| cast::<_, I>(i).unwrap()))
                    .collect::<Vec<_>>();

                let indices = vec![
                    i[4], i[10], i[5], i[9], i[3], i[7], i[1], i[6], i[2], i[8],
                    i[4], i[10], i[9], i[11], i[7], i[6], i[6], i[11], i[8],
                    i[10], i[4], i[5], i[0], i[3], i[1], i[1], i[0], i[2], i[4],
                ];

                Shape::Strips { vertices, strips: vec!(indices) }
            },
        }
    }
}

pub struct PlatonicSolid {
    vertices: Vec<[f32; 3]>,
    indices: Vec<u8>,
}

impl PlatonicSolid {
    /// Create a Platonic solid of the supplied variant.
    /// The indices are in triangle strip order,
    /// but some of the polyhedrons contain degenerate or internal triangles.
    pub fn new(polyhedron: PlatonicSolids) -> Self {
        let (vertices, indices) = match polyhedron {
            PlatonicSolids::Tetrahedron => Self::tetrahedron(),
            PlatonicSolids::Hexahedron => Self::hexahedron(),
            PlatonicSolids::Octahedron => Self::octahedron(),
            PlatonicSolids::Dodecahedron => Self::dodecahedron(),
            PlatonicSolids::Icosahedron => Self::icosahedron(),
        };

        Self {
            vertices,
            indices,
        }
    }

    fn tetrahedron() -> (Vec<[f32; 3]>, Vec<u8>) {
        let sr2 = f32::sqrt(2.0);
        let sr3 = f32::sqrt(3.0);
        let base = -0.5 / sr2 / sr3;

        let vertices = vec![
            [-0.5, base, 0.5 / sr3],
            [0.5, base, 0.5 / sr3],
            [0.0, base + sr2 / sr3, 0.0],
            [0.0, base, -1.0 / sr3],
        ];

        let indices = vec![0u8, 1, 2, 3, 0, 1];

        (vertices, indices)
    }

    fn hexahedron() -> (Vec<[f32; 3]>, Vec<u8>) {
        let vertices = vec![
            [-0.5, -0.5, 0.5],
            [0.5, -0.5, 0.5],
            [-0.5, 0.5, 0.5],
            [0.5, 0.5, 0.5],
            [-0.5, -0.5, -0.5],
            [0.5, -0.5, -0.5],
            [-0.5, 0.5, -0.5],
            [0.5, 0.5, -0.5],
        ];

        let indices = vec![0u8, 1, 2, 3, 7, 1, 5, 0, 4, 2, 6, 7, 4, 5];

        (vertices, indices)
    }

    fn octahedron() -> (Vec<[f32; 3]>, Vec<u8>) {
        let half_height = 1.0 / f32::sqrt(2.0);

        let vertices = vec![
            [0.0, half_height, 0.0],
            [-0.5, 0.0, -0.5],
            [-0.5, 0.0, 0.5],
            [0.5, 0.0, 0.5],
            [0.5, 0.0, -0.5],
            [0.0, -half_height, 0.0],
        ];

        //let indices = vec![0u8, 1, 2, 5, 3, 4, 1, 5, 2, 3, 0, 4, 1];
        //let indices = vec![1u8, 2, 0, 3, 4, 5, 1, 2, 3, 5, 4, 1, 0];
        let indices = vec![2u8, 0, 1, 4, 5, 3, 2, 0, 4, 3, 5, 2, 1];

        (vertices, indices)
    }

    fn dodecahedron() -> (Vec<[f32; 3]>, Vec<u8>) {
        let sr5 = f32::sqrt(5.0);
        let phi = 0.5 * (1.0 + sr5);

        let mid = 0.25 * f32::sqrt(10.0 + 2.0 * sr5);
        let top = 0.25 * f32::sqrt(10.0 - 2.0 * sr5);
        let width = 0.25 * (1.0 + sr5); // phi/2
        let height = top + mid;
        let circle_offset = 0.25 * (2.0 + sr5) / height;
        let circle_radius = 0.25 * (3.0 + sr5) / height;
        let centred_mid = 0.125 * (1.0 + sr5) / height;
        let phi_width = 0.25 * (3.0 + sr5);
        // we use the less accurate (phi * circle_offset)
        // to offset some accumulated error in the tests
        let _phi_offset = 0.125 * (7.0 + 3.0 * sr5) / height;
        let phi_radius = 0.5 * (2.0 + sr5) / height; // 2 * circle_offset
        let phi_mid = 0.125 * (3.0 + sr5) / height; // circle_radius / 2

        let half_iz = 0.5 * f32::sqrt(0.5 - 0.1 * sr5);

        let vertices = vec![
            [0.0, circle_radius, circle_radius + half_iz],
            [-width, centred_mid, circle_radius + half_iz],
            [width, centred_mid, circle_radius + half_iz],
            [-0.5, -circle_offset, circle_radius + half_iz],
            [0.5, -circle_offset, circle_radius + half_iz],
            [0.0, phi_radius, half_iz],
            [-phi_width, phi_mid, half_iz],
            [phi_width, phi_mid, half_iz],
            [-width, -phi * circle_offset, half_iz],
            [width, -phi * circle_offset, half_iz],
            [-width, phi * circle_offset, -half_iz],
            [width, phi * circle_offset, -half_iz],
            [-phi_width, -phi_mid, -half_iz],
            [phi_width, -phi_mid, -half_iz],
            [0.0, -phi_radius, -half_iz],
            [-0.5, circle_offset, -circle_radius - half_iz],
            [0.5, circle_offset, -circle_radius - half_iz],
            [-width, -centred_mid, -circle_radius - half_iz],
            [width, -centred_mid, -circle_radius - half_iz],
            [0.0, -circle_radius, -circle_radius - half_iz],
        ];

        let indices = vec![
            1u8, 3, 0, 4, 2, 7, 0, 11, 5, 10, 0, 6, 1, 12, 3, 8, 4, 9, 7, 13, 11, 16, 10, 15,
            6, 17, 12, 19, 8, 14, 9, 19, 13, 18, 16, 19, 15, 17
        ];

        (vertices, indices)
    }

    fn icosahedron() -> (Vec<[f32; 3]>, Vec<u8>) {
        let sr5 = f32::sqrt(5.0);

        let mid = 0.25 * f32::sqrt(10.0 + 2.0 * sr5);
        let top = 0.25 * f32::sqrt(10.0 - 2.0 * sr5);
        let width = 0.25 * (1.0 + sr5); // phi/2
        let depth = top + mid;
        let center = 0.25 * (2.0 + sr5) / depth;
        let radius = 0.25 * (3.0 + sr5) / depth;

        let y_diff = f32::sqrt(0.5 - 0.1 * sr5);
        let half_middle = 0.5 * f32::sqrt(0.5 + 0.1 * sr5);

        let vertices = vec![
            [0.0, half_middle + y_diff, 0.0],
            [0.0, half_middle, -radius],
            [-width, half_middle, -radius + top],
            [width, half_middle, -radius + top],
            [-0.5, half_middle, center],
            [0.5, half_middle, center],
            [-0.5, -half_middle, -center],
            [0.5, -half_middle, -center],
            [-width, -half_middle, radius - top],
            [width, -half_middle, radius - top],
            [0.0, -half_middle, radius],
            [0.0, -half_middle - y_diff, 0.0],
        ];

        let indices = vec![
            4u8, 10, 5, 9, 3, 7, 1, 6, 2, 8, 4, 10, 9, 11, 7, 6, 6, 11, 8, 10, 4, 5, 0, 3, 1, 1, 0,
            2, 4,
        ];

        (vertices, indices)
    }

    pub fn vertices(&self) -> &Vec<[f32; 3]> {
        &self.vertices
    }

    pub fn indices(&self) -> &Vec<u8> {
        &self.indices
    }
}

#[cfg(test)]
mod tests {
    use super::PlatonicSolid;

    const TOLERANCE: f32 = 1.2e-7f32;

    fn magnitude_squared(vertex: &[f32; 3]) -> f32 {
        vertex[0] * vertex[0] + vertex[1] * vertex[1] + vertex[2] * vertex[2]
    }

    fn magnitude_squared_diff(a: &[f32; 3], b: &[f32; 3]) -> f32 {
        let x = a[0] - b[0];
        let y = a[1] - b[1];
        let z = a[2] - b[2];

        x * x + y * y + z * z
    }

    macro_rules! uniform_distance {
        ($polyhedron:ident) => {
            let (vertices, _) = PlatonicSolid::$polyhedron();

            let r_squared = magnitude_squared(&vertices[0]);

            for vertex in vertices {
                assert!(f32::abs(r_squared - magnitude_squared(&vertex)) <= TOLERANCE);
            }
        };
    }

    macro_rules! unit_neighbour {
        ($vertices:expr, $a:expr, $b:expr) => {
            let difference_length =
                magnitude_squared_diff(&$vertices[$a], &$vertices[$b]);

            assert!(f32::abs(1.0 - difference_length) <= TOLERANCE);
        };
    }

    #[test]
    fn tetrahedron_centered() {
        uniform_distance!(tetrahedron);
    }

    #[test]
    fn tetrahedron_edges() {
        let (vertices, _) = PlatonicSolid::tetrahedron();

        unit_neighbour!(vertices, 0, 1);
        unit_neighbour!(vertices, 0, 2);
        unit_neighbour!(vertices, 0, 3);
        unit_neighbour!(vertices, 1, 2);
        unit_neighbour!(vertices, 1, 3);
        unit_neighbour!(vertices, 2, 3);
    }

    #[test]
    fn hexahedron_centered() {
        uniform_distance!(hexahedron);
    }

    #[test]
    fn hexahedron_edges() {
        let (vertices, _) = PlatonicSolid::hexahedron();

        unit_neighbour!(vertices, 0, 1);
        unit_neighbour!(vertices, 0, 2);
        unit_neighbour!(vertices, 0, 4);
        unit_neighbour!(vertices, 1, 3);
        unit_neighbour!(vertices, 1, 5);
        unit_neighbour!(vertices, 2, 3);
        unit_neighbour!(vertices, 2, 6);
        unit_neighbour!(vertices, 3, 7);
        unit_neighbour!(vertices, 4, 5);
        unit_neighbour!(vertices, 4, 6);
        unit_neighbour!(vertices, 5, 7);
        unit_neighbour!(vertices, 6, 7);
    }

    #[test]
    fn octahedron_centered() {
        uniform_distance!(octahedron);
    }

    #[test]
    fn octahedron_edges() {
        let (vertices, _) = PlatonicSolid::octahedron();

        unit_neighbour!(vertices, 0, 1);
        unit_neighbour!(vertices, 0, 2);
        unit_neighbour!(vertices, 0, 3);
        unit_neighbour!(vertices, 0, 4);
        unit_neighbour!(vertices, 1, 2);
        unit_neighbour!(vertices, 1, 4);
        unit_neighbour!(vertices, 1, 5);
        unit_neighbour!(vertices, 2, 3);
        unit_neighbour!(vertices, 2, 5);
        unit_neighbour!(vertices, 3, 4);
        unit_neighbour!(vertices, 3, 5);
        unit_neighbour!(vertices, 4, 5);
    }

    #[test]
    fn dodecahedron_centered() {
        uniform_distance!(dodecahedron);
    }

    #[test]
    fn dodecahedron_edges() {
        let (vertices, _) = PlatonicSolid::dodecahedron();

        unit_neighbour!(vertices, 0, 1);
        unit_neighbour!(vertices, 0, 2);
        unit_neighbour!(vertices, 0, 5);
        unit_neighbour!(vertices, 1, 3);
        unit_neighbour!(vertices, 1, 6);
        unit_neighbour!(vertices, 2, 4);
        unit_neighbour!(vertices, 2, 7);
        unit_neighbour!(vertices, 3, 4);
        unit_neighbour!(vertices, 3, 8);
        unit_neighbour!(vertices, 4, 9);
        unit_neighbour!(vertices, 5, 10);
        unit_neighbour!(vertices, 5, 11);
        unit_neighbour!(vertices, 6, 10);
        unit_neighbour!(vertices, 6, 12);
        unit_neighbour!(vertices, 7, 11);
        unit_neighbour!(vertices, 7, 13);
        unit_neighbour!(vertices, 8, 12);
        unit_neighbour!(vertices, 8, 14);
        unit_neighbour!(vertices, 9, 13);
        unit_neighbour!(vertices, 9, 14);
        unit_neighbour!(vertices, 10, 15);
        unit_neighbour!(vertices, 11, 16);
        unit_neighbour!(vertices, 12, 17);
        unit_neighbour!(vertices, 13, 18);
        unit_neighbour!(vertices, 14, 19);
        unit_neighbour!(vertices, 15, 16);
        unit_neighbour!(vertices, 15, 17);
        unit_neighbour!(vertices, 16, 18);
        unit_neighbour!(vertices, 17, 19);
        unit_neighbour!(vertices, 18, 19);
    }

    #[test]
    fn icosahedron_centered() {
        uniform_distance!(icosahedron);
    }

    #[test]
    fn icosahedron_edges() {
        let (vertices, _) = PlatonicSolid::icosahedron();

        unit_neighbour!(vertices, 0, 1);
        unit_neighbour!(vertices, 0, 2);
        unit_neighbour!(vertices, 0, 3);
        unit_neighbour!(vertices, 0, 4);
        unit_neighbour!(vertices, 0, 5);
        unit_neighbour!(vertices, 1, 2);
        unit_neighbour!(vertices, 1, 3);
        unit_neighbour!(vertices, 1, 6);
        unit_neighbour!(vertices, 1, 7);
        unit_neighbour!(vertices, 2, 4);
        unit_neighbour!(vertices, 2, 6);
        unit_neighbour!(vertices, 2, 8);
        unit_neighbour!(vertices, 3, 5);
        unit_neighbour!(vertices, 3, 7);
        unit_neighbour!(vertices, 3, 9);
        unit_neighbour!(vertices, 4, 5);
        unit_neighbour!(vertices, 4, 8);
        unit_neighbour!(vertices, 4, 10);
        unit_neighbour!(vertices, 5, 9);
        unit_neighbour!(vertices, 5, 10);
        unit_neighbour!(vertices, 6, 7);
        unit_neighbour!(vertices, 6, 8);
        unit_neighbour!(vertices, 6, 11);
        unit_neighbour!(vertices, 7, 9);
        unit_neighbour!(vertices, 7, 11);
        unit_neighbour!(vertices, 8, 10);
        unit_neighbour!(vertices, 8, 11);
        unit_neighbour!(vertices, 9, 10);
        unit_neighbour!(vertices, 9, 11);
        unit_neighbour!(vertices, 10, 11);
    }
}
