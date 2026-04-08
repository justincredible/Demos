use num_traits::{cast, Float, FloatConst, NumCast, one, Unsigned, zero};

use crate::shapes::{Configuration, Shape, Shaper};

/// All possible Platonic solids.
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
                let mid = fq * (f10 + f2 * sr5).sqrt();
                let top = fq * (f10 - f2 * sr5).sqrt();
                let width = fq * (f1 + sr5);
                let height = top + mid;
                let circle_offset = fq * (f2 + sr5) / height;
                let circle_radius = fq * (f3 + sr5) / height;
                let centred_mid = fe * (f1 + sr5) / height;
                let phi_width = fq * (f3 + sr5);
                let phi_offset = fe * (f7 + f3 * sr5) / height;
                let phi_radius = fh * (f2 + sr5) / height;
                let phi_mid = fe * (f3 + sr5) / height;
                let half_iz = fh * (fh - ft * sr5).sqrt();

                // Stellation
                let phi2_radius = fq * (f7 + f3 * sr5) / height;
                let f5 = cast::<_, C>(5.).unwrap();
                let f11 = cast::<_, C>(11.).unwrap();
                let point_top = fe * (f11 + f5 * sr5) / height;
                let phi2_width = fh * (f2 + sr5);
                let point_mid = fq * (f2 + sr5) / height;
                let point_bot = fq * (f7 + f3 * sr5) / height; // circle_offset + height

                let vertices = vec![
                    [f0, f0, phi2_radius + half_iz],
                    [f0, circle_radius, circle_radius + half_iz],
                    [-width, centred_mid, circle_radius + half_iz],
                    [width, centred_mid, circle_radius + half_iz],
                    [-fh, -circle_offset, circle_radius + half_iz],
                    [fh, -circle_offset, circle_radius + half_iz],
                    [-phi_width, point_top, circle_radius + half_iz],
                    [phi_width, point_top, circle_radius + half_iz],
                    [-phi2_width, -point_mid, circle_radius + half_iz],
                    [phi2_width, -point_mid, circle_radius + half_iz],
                    [f0, -point_bot, circle_radius + half_iz],
                    [f0, phi_radius, half_iz],
                    [-phi_width, phi_mid, half_iz],
                    [phi_width, phi_mid, half_iz],
                    [-width, -phi_offset, half_iz],
                    [width, -phi_offset, half_iz],
                    [-width, phi_offset, -half_iz],
                    [width, phi_offset, -half_iz],
                    [-phi_width, -phi_mid, -half_iz],
                    [phi_width, -phi_mid, -half_iz],
                    [f0, -phi_radius, -half_iz],
                    [f0, point_bot, -circle_radius - half_iz],
                    [-phi2_width, point_mid, -circle_radius - half_iz],
                    [phi2_width, point_mid, -circle_radius - half_iz],
                    [-phi_width, -point_top, -circle_radius - half_iz],
                    [phi_width, -point_top, -circle_radius - half_iz],
                    [-fh, circle_offset, -circle_radius - half_iz],
                    [fh, circle_offset, -circle_radius - half_iz],
                    [-width, -centred_mid, -circle_radius - half_iz],
                    [width, -centred_mid, -circle_radius - half_iz],
                    [f0, -circle_radius, -circle_radius - half_iz],
                    [f0, f0, -phi2_radius - half_iz],
                ];

                let i = vec![zero(), one()]
                    .into_iter()
                    .chain((2..32).map(|i| cast::<_, I>(i).unwrap()))
                    .collect::<Vec<_>>();

                let indices = vec![
                    i[0], i[1], i[2], i[0], i[2], i[4], i[0], i[4], i[5], i[0], i[5], i[3], i[0], i[3], i[1],
                    i[6], i[1], i[11], i[6], i[11], i[16], i[6], i[16], i[12], i[6], i[12], i[2], i[6], i[2], i[1],
                    i[7], i[1], i[3], i[7], i[3], i[13], i[7], i[13], i[17], i[7], i[17], i[11], i[7], i[11], i[1],
                    i[8], i[2], i[12], i[8], i[12], i[18], i[8], i[18], i[14], i[8], i[14], i[4], i[8], i[4], i[2],
                    i[9], i[3], i[5], i[9], i[5], i[15], i[9], i[15], i[19], i[9], i[19], i[13], i[9], i[13], i[3],
                    i[10], i[4], i[14], i[10], i[14], i[20], i[10], i[20], i[15], i[10], i[15], i[5], i[10], i[5], i[4],
                    i[21], i[11], i[17], i[21], i[17], i[27], i[21], i[27], i[26], i[21], i[26], i[16], i[21], i[16], i[11],
                    i[22], i[12], i[16], i[22], i[16], i[26], i[22], i[26], i[28], i[22], i[28], i[18], i[22], i[18], i[12],
                    i[23], i[13], i[19], i[23], i[19], i[29], i[23], i[29], i[27], i[23], i[27], i[17], i[23], i[17], i[13],
                    i[24], i[14], i[18], i[24], i[18], i[28], i[24], i[28], i[30], i[24], i[30], i[20], i[24], i[20], i[14],
                    i[25], i[15], i[20], i[25], i[20], i[30], i[25], i[30], i[29], i[25], i[29], i[19], i[25], i[19], i[15],
                    i[31], i[26], i[27], i[31], i[27], i[29], i[31], i[29], i[30], i[31], i[30], i[28], i[31], i[28], i[26],
                ];

                Shape::Triangles { vertices, indices }
            },
            Self::GreatDodecahedron => {
                // Icosahedron
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

                // GSD
                let phi = fh * (f1 + sr5);
                let phi2 = fh * (f3 + sr5);
                let phi2h = fq * (f3 + sr5);
                let phi3h = fh * (f2 + sr5);

                let vertices = vec![
                    [f0, half_middle + y_diff, f0],
                    [f0, half_middle, -radius],
                    [-width, half_middle, -radius + top],
                    [width, half_middle, -radius + top],
                    [-fh, half_middle, center],
                    [fh, half_middle, center],
                    [-width, phi2 * y_diff + half_middle, -phi * center],
                    [width, phi2 * y_diff + half_middle, -phi * center],
                    [-phi2h, phi2 * y_diff + half_middle, phi * (radius - top)],
                    [phi2h, phi2 * y_diff + half_middle, phi * (radius - top)],
                    [f0, phi2 * y_diff + half_middle, phi * radius],
                    [-phi2h, half_middle, -(phi * top + radius)],
                    [phi2h, half_middle, -(phi * top + radius)],
                    [-phi3h, half_middle, phi2 * top - radius],
                    [phi3h, half_middle, phi2 * top - radius],
                    [f0, half_middle, phi2 * center + phi * (radius - top)],
                    [f0, -half_middle, -phi2 * center - phi * (radius - top)],
                    [-phi3h, -half_middle, -phi2 * top + radius],
                    [phi3h, -half_middle, -phi2 * top + radius],
                    [-phi2h, -half_middle, phi * top + radius],
                    [phi2h, -half_middle, phi * top + radius],
                    [f0, -phi2 * y_diff - half_middle, -phi * radius],
                    [-phi2h, -phi2 * y_diff - half_middle, -phi * (radius - top)],
                    [phi2h, -phi2 * y_diff - half_middle, -phi * (radius - top)],
                    [-width, -phi2 * y_diff - half_middle, phi * center],
                    [width, -phi2 * y_diff - half_middle, phi * center],
                    [-fh, -half_middle, -center],
                    [fh, -half_middle, -center],
                    [-width, -half_middle, radius - top],
                    [width, -half_middle, radius - top],
                    [f0, -half_middle, radius],
                    [f0, -half_middle - y_diff, f0],
                ];

                let i = vec![zero(), one()]
                    .into_iter()
                    .chain((2..32).map(|i| cast::<_, I>(i).unwrap()))
                    .collect::<Vec<_>>();

                let indices = vec![
                    i[0], i[1], i[6], i[1], i[2], i[6], i[0], i[6], i[2],
                    i[0], i[7], i[1], i[0], i[3], i[7], i[1], i[7], i[3],
                    i[0], i[2], i[8], i[2], i[4], i[8], i[0], i[8], i[4],
                    i[0], i[9], i[3], i[0], i[5], i[9], i[3], i[9], i[5],
                    i[0], i[4], i[10], i[4], i[5], i[10], i[0], i[10], i[5],
                    i[1], i[11], i[2], i[1], i[26], i[11], i[2], i[11], i[26],
                    i[1], i[3], i[12], i[1], i[12], i[27], i[3], i[27], i[12],
                    i[2], i[13], i[4], i[2], i[28], i[13], i[4], i[13], i[28],
                    i[3], i[5], i[14], i[3], i[14], i[29], i[5], i[29], i[14],
                    i[4], i[15], i[5], i[4], i[30], i[15], i[5], i[15], i[30],
                    i[1], i[16], i[26], i[1], i[27], i[16], i[26], i[16], i[27],
                    i[2], i[26], i[17], i[2], i[17], i[28], i[26], i[28], i[17],
                    i[3], i[18], i[27], i[3], i[29], i[18], i[27], i[18], i[29],
                    i[4], i[28], i[19], i[4], i[19], i[30], i[28], i[30], i[19],
                    i[5], i[20], i[29], i[5], i[30], i[20], i[29], i[20], i[30],
                    i[26], i[27], i[21], i[26], i[21], i[31], i[27], i[31], i[21],
                    i[26], i[22], i[28], i[26], i[31], i[22], i[28], i[22], i[31],
                    i[27], i[29], i[23], i[27], i[23], i[31], i[29], i[31], i[23],
                    i[28], i[24], i[30], i[28], i[31], i[24], i[30], i[24], i[31],
                    i[29], i[30], i[25], i[29], i[25], i[31], i[30], i[31], i[25],
                ];

                Shape::Triangles { vertices, indices }
            },
            Self::GreatIcosahedron => {
                // Icosahedron
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
