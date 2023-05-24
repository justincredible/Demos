use crate::shapes::shapes::PosVertex;
use glium::backend::Facade;
use glium::index::{IndexBuffer, PrimitiveType};
use glium::vertex::VertexBuffer;

const DEGREES_18: f32 = std::f32::consts::PI / 10.0;
const DEGREES_54: f32 = 3.0 * std::f32::consts::PI / 10.0;

pub enum PlatonicSolids {
    Tetrahedron,
    Hexahedron,
    Octahedron,
    Dodecahedron,
    Icosahedron,
}

pub struct PlatonicSolid {
    pub vertices: VertexBuffer<PosVertex>,
    pub indices: IndexBuffer<u8>,
}

impl PlatonicSolid {
    pub fn new(facade: &dyn Facade, polyhedron: PlatonicSolids) -> Self {

        let (vertices, indices) = match polyhedron {
            PlatonicSolids::Tetrahedron => Self::tetrahedron(),
            PlatonicSolids::Hexahedron => Self::hexahedron(),
            PlatonicSolids::Octahedron => Self::octahedron(),
            PlatonicSolids::Dodecahedron => Self::dodecahedron(),
            PlatonicSolids::Icosahedron => Self::icosahedron(),
        };

        PlatonicSolid {
            vertices: VertexBuffer::new(facade, &vertices).unwrap(),
            indices: IndexBuffer::new(facade, PrimitiveType::TriangleStrip, &indices).unwrap(),
        }
    }

    fn tetrahedron() -> (Vec<PosVertex>, Vec<u8>) {
        let depth = f32::sqrt(3.0) / 2.0;
        let z = 0.5 * depth - 0.125 / depth;
        let r = 0.5 * depth + 0.125 / depth;
        let y = f32::sqrt(1.0 - r * r);
        let above = depth * depth * y;
        let below = above - y;

        let vertices = vec![
            PosVertex::new([-0.5, below, z]),
            PosVertex::new([0.5, below, z]),
            PosVertex::new([0.0, above, 0.0]),
            PosVertex::new([0.0, below, -r]),
        ];

        let indices = vec![0u8, 1, 2, 3, 0, 1];

        (vertices, indices)
    }

    fn hexahedron() -> (Vec<PosVertex>, Vec<u8>) {
        let vertices = vec![
            PosVertex::new([-0.5, -0.5, 0.5]),
            PosVertex::new([0.5, -0.5, 0.5]),
            PosVertex::new([-0.5, 0.5, 0.5]),
            PosVertex::new([0.5, 0.5, 0.5]),
            PosVertex::new([-0.5, -0.5, -0.5]),
            PosVertex::new([0.5, -0.5, -0.5]),
            PosVertex::new([-0.5, 0.5, -0.5]),
            PosVertex::new([0.5, 0.5, -0.5]),
        ];

        let indices = vec![0u8, 1, 2, 3, 7, 1, 5, 0, 4, 2, 6, 7, 4, 5];

        (vertices, indices)
    }

    fn octahedron() -> (Vec<PosVertex>, Vec<u8>) {
        let half_height = 1.0 / f32::sqrt(2.0);

        let vertices = vec![
            PosVertex::new([0.0, half_height, 0.0]),
            PosVertex::new([-0.5, 0.0, -0.5]),
            PosVertex::new([-0.5, 0.0, 0.5]),
            PosVertex::new([0.5, 0.0, 0.5]),
            PosVertex::new([0.5, 0.0, -0.5]),
            PosVertex::new([0.0, -half_height, 0.0]),
        ];

        //let indices = vec![0u8, 1, 2, 5, 3, 4, 1, 5, 2, 3, 0, 4, 1]
        //let indices = vec![1u8, 2, 0, 3, 4, 5, 1, 2, 3, 5, 4, 1, 0]
        let indices = vec![2u8, 0, 1, 4, 5, 3, 2, 0, 4, 3, 5, 2, 1];

        (vertices, indices)
    }

    fn dodecahedron() -> (Vec<PosVertex>, Vec<u8>) {
        let phi = 0.5 * (1.0 + f32::sqrt(5.0));

        let mid = f32::cos(DEGREES_18);
        let top = f32::cos(DEGREES_54);
        let width = f32::sin(DEGREES_54);
        let height = top + mid;
        let circle_offset = 0.5 * height - 0.125 / height;
        let circle_radius = 0.5 * height + 0.125 / height;
        let centred_mid = circle_radius - top;

        let oz = f32::sqrt(1.0 - (2.0 - phi) * circle_radius * circle_radius);
        let half_iz = 0.5 * f32::sqrt(1.0 - 4.0 * (phi + 1.0) * centred_mid * centred_mid);

        let vertices = vec![
            PosVertex::new([0.0, circle_radius, oz + half_iz]),
            PosVertex::new([-width, centred_mid, oz + half_iz]),
            PosVertex::new([width, centred_mid, oz + half_iz]),
            PosVertex::new([-0.5, -circle_offset, oz + half_iz]),
            PosVertex::new([0.5, -circle_offset, oz + half_iz]),
            PosVertex::new([0.0, phi * circle_radius, half_iz]),
            PosVertex::new([-phi * width, phi * centred_mid, half_iz]),
            PosVertex::new([phi * width, phi * centred_mid, half_iz]),
            PosVertex::new([-0.5 * phi, -phi * circle_offset, half_iz]),
            PosVertex::new([0.5 * phi, -phi * circle_offset, half_iz]),
            PosVertex::new([-0.5 * phi, phi * circle_offset, -half_iz]),
            PosVertex::new([0.5 * phi, phi * circle_offset, -half_iz]),
            PosVertex::new([-phi * width, -phi * centred_mid, -half_iz]),
            PosVertex::new([phi * width, -phi * centred_mid, -half_iz]),
            PosVertex::new([0.0, -phi * circle_radius, -half_iz]),
            PosVertex::new([-0.5, circle_offset, -oz - half_iz]),
            PosVertex::new([0.5, circle_offset, -oz - half_iz]),
            PosVertex::new([-width, -centred_mid, -oz - half_iz]),
            PosVertex::new([width, -centred_mid, -oz - half_iz]),
            PosVertex::new([0.0, -circle_radius, -oz - half_iz]),
        ];

        let indices = vec![
            10u8, 6, 5, 1, 0, 2, 5, 7, 11,
            13, 16, 18, 19, 13, 14, 9, 8, 4,
            3, 1, 8, 6, 12, 10, 17, 15, 16,
            10, 11, 5, 1, 4, 2, 9, 7, 13,
            16, 19, 17, 14, 12, 8,
        ];

        (vertices, indices)
    }

    fn icosahedron() -> (Vec<PosVertex>, Vec<u8>) {
        let mid = f32::cos(DEGREES_18);
        let top = f32::cos(DEGREES_54);
        let width = f32::sin(DEGREES_54);
        let depth = top + mid;
        let center = 0.5 * depth - 0.125 / depth;
        let radius = 0.5 * depth + 0.125 / depth;

        let y_diff = f32::sqrt(1.0 - radius * radius);
        let width_diff = 1.0 - width;
        let half_middle = 0.5 * f32::sqrt(1.0 - radius * radius * (top * top + width_diff * width_diff));

        let vertices = vec![
            PosVertex::new([0.0, half_middle + y_diff, 0.0]),
            PosVertex::new([0.0, half_middle, -radius]),
            PosVertex::new([-width, half_middle, -radius + top]),
            PosVertex::new([width, half_middle, -radius + top]),
            PosVertex::new([-0.5, half_middle, center]),
            PosVertex::new([0.5, half_middle, center]),
            PosVertex::new([-0.5, -half_middle, -center]),
            PosVertex::new([0.5, -half_middle, -center]),
            PosVertex::new([-width, -half_middle, radius - top]),
            PosVertex::new([width, -half_middle, radius - top]),
            PosVertex::new([0.0, -half_middle, radius]),
            PosVertex::new([0.0, -half_middle - y_diff, 0.0]),
        ];

        let indices = vec![
            4u8, 10, 5, 9, 3, 7, 1, 6, 2,
            8, 4, 10, 9, 11, 7, 6, 6, 11,
            8, 10, 4, 5, 0, 3, 1, 1, 0,
            2, 4,
        ];

        (vertices, indices)
    }
}

#[cfg(test)]
mod tests {
    use crate::PlatonicSolid;

    const TOLERANCE: f32 = 2.5e-7f32;

    fn magnitude_squared(vertex: &[f32; 3]) -> f32 {
        vertex[0]*vertex[0] + vertex[1]*vertex[1] + vertex[2]*vertex[2]
    }

    fn magnitude_squared_diff(a: &[f32; 3], b: &[f32; 3]) -> f32 {
        let x = a[0] - b[0];
        let y = a[1] - b[1];
        let z = a[2] - b[2];

        x*x + y*y + z*z
    }

    macro_rules! uniform_distance {
        ($polyhedron:ident) => {
            let (vertices, _) = PlatonicSolid::$polyhedron();

            let r_squared = magnitude_squared(&vertices[0].position);

            for vertex in vertices {
                assert!(f32::abs(r_squared - magnitude_squared(&vertex.position)) <= TOLERANCE);
            }
        }
    }

    macro_rules! unit_neighbour {
        ($vertices:expr, $a:expr, $b:expr) => {
            let difference_length = magnitude_squared_diff(&$vertices[$a].position, &$vertices[$b].position);

            assert!(f32::abs(1.0 - difference_length) <= TOLERANCE);
        }
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
