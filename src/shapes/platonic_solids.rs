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
        let z = depth / 2.0 - 0.125 / depth;
        let z_diff = z - depth;
        let y = f32::sqrt(1.0 - z_diff * z_diff);
        let above = depth * depth * y;
        let below = above - y;

        let vertices = vec![
            PosVertex::new([-0.5, below, z]),
            PosVertex::new([0.5, below, z]),
            PosVertex::new([0.0, above, 0.0]),
            PosVertex::new([0.0, below, -depth + z]),
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
        let phi = (1.0 + f32::sqrt(5.0)) / 2.0;

        let mid = f32::cos(DEGREES_18);
        let height = f32::cos(DEGREES_54) + mid;
        let width = f32::sin(DEGREES_54);
        let cy = height / 2.0 - 0.125 / height;

        let y_diff = (phi - 1.0) * (height - cy);
        let oz = f32::sqrt(1.0 - y_diff * y_diff);
        let x_diff = 0.5 * phi;
        let y_diff = phi * (height - 2.0 * cy);
        let iz = f32::sqrt(1.0 - x_diff * x_diff - y_diff * y_diff);

        let vertices = vec![
            PosVertex::new([0.0, height - cy, oz + 0.5*iz]),
            PosVertex::new([-width, mid - cy, oz + 0.5*iz]),
            PosVertex::new([width, mid - cy, oz + 0.5*iz]),
            PosVertex::new([-0.5, -cy, oz + 0.5*iz]),
            PosVertex::new([0.5, -cy, oz + 0.5*iz]),
            PosVertex::new([0.0, phi * (height - cy), 0.5*iz]),
            PosVertex::new([-phi * width, phi * (mid - cy), 0.5*iz]),
            PosVertex::new([phi * width, phi * (mid - cy), 0.5*iz]),
            PosVertex::new([-0.5 * phi, -phi * cy, 0.5*iz]),
            PosVertex::new([0.5 * phi, -phi * cy, 0.5*iz]),
            PosVertex::new([-0.5 * phi, phi * cy, -0.5*iz]),
            PosVertex::new([0.5 * phi, phi * cy, -0.5*iz]),
            PosVertex::new([-phi * width, -phi * (mid - cy), -0.5*iz]),
            PosVertex::new([phi * width, -phi * (mid - cy), -0.5*iz]),
            PosVertex::new([0.0, -phi * (height - cy), -0.5*iz]),
            PosVertex::new([-0.5, cy, -oz - 0.5*iz]),
            PosVertex::new([0.5, cy, -oz - 0.5*iz]),
            PosVertex::new([-width, -mid + cy, -oz - 0.5*iz]),
            PosVertex::new([width, -mid + cy, -oz - 0.5*iz]),
            PosVertex::new([0.0, -height + cy, -oz - 0.5*iz]),
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
        let depth = f32::cos(DEGREES_54) + mid;
        let width = f32::sin(DEGREES_54);
        let cz = depth / 2.0 - 0.125 / depth;
        let r = depth / 2.0 + 0.125 / depth;

        let z_diff = depth - cz;
        let y_diff = f32::sqrt(1.0 - z_diff * z_diff);
        let x_diff = r * f32::cos(DEGREES_54);
        let z_diff = z_diff - r * f32::sin(DEGREES_54);
        let half_middle = f32::sqrt(1.0 - x_diff * x_diff - z_diff * z_diff) / 2.0;

        let vertices = vec![
            PosVertex::new([0.0, half_middle + y_diff, 0.0]),
            PosVertex::new([0.0, half_middle, -depth + cz]),
            PosVertex::new([-width, half_middle, -mid + cz]),
            PosVertex::new([width, half_middle, -mid + cz]),
            PosVertex::new([-0.5, half_middle, cz]),
            PosVertex::new([0.5, half_middle, cz]),
            PosVertex::new([-0.5, -half_middle, -cz]),
            PosVertex::new([0.5, -half_middle, -cz]),
            PosVertex::new([-width, -half_middle, mid - cz]),
            PosVertex::new([width, -half_middle, mid - cz]),
            PosVertex::new([0.0, -half_middle, depth - cz]),
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

    fn magnitude_squared(vertex: &[f32; 3]) -> f32 {
        vertex[0]*vertex[0] + vertex[1]*vertex[1] + vertex[2]*vertex[2]
    }

    fn magnitude_squared_diff(a: &[f32; 3], b: &[f32; 3]) -> f32 {
        let x = a[0] - b[0];
        let y = a[1] - b[1];
        let z = a[2] - b[2];

        x*x + y*y + z*z
    }

    #[test]
    #[ignore]
    fn tetrahedron_centered() {
        let (vertices, _) = PlatonicSolid::tetrahedron();

        let r_squared = magnitude_squared(&vertices[0].position);

        for vertex in vertices {
            println!("{:?}", vertex);
            assert_eq!(r_squared, magnitude_squared(&vertex.position));
        }
    }

    #[test]
    fn tetrahedron_edges() {
        let (vertices, _) = PlatonicSolid::tetrahedron();

        assert_eq!(1.0, magnitude_squared_diff(&vertices[0].position, &vertices[1].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[0].position, &vertices[2].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[0].position, &vertices[3].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[1].position, &vertices[2].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[1].position, &vertices[3].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[2].position, &vertices[3].position));
    }

    #[test]
    fn hexahedron_centered() {
        let (vertices, _) = PlatonicSolid::hexahedron();

        let r_squared = magnitude_squared(&vertices[0].position);

        for vertex in vertices {
            assert_eq!(r_squared, magnitude_squared(&vertex.position));
        }
    }

    #[test]
    fn hexahedron_edges() {
        let (vertices, _) = PlatonicSolid::hexahedron();

        assert_eq!(1.0, magnitude_squared_diff(&vertices[0].position, &vertices[1].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[0].position, &vertices[2].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[0].position, &vertices[4].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[1].position, &vertices[3].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[1].position, &vertices[5].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[2].position, &vertices[3].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[2].position, &vertices[6].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[3].position, &vertices[7].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[4].position, &vertices[5].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[4].position, &vertices[6].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[5].position, &vertices[7].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[6].position, &vertices[7].position));
    }

    #[test]
    #[ignore]
    fn octahedron_centered() {
        let (vertices, _) = PlatonicSolid::octahedron();

        let r_squared = magnitude_squared(&vertices[0].position);

        for vertex in vertices {
            println!("{:?}", vertex);
            assert_eq!(r_squared, magnitude_squared(&vertex.position));
        }
    }

    #[test]
    fn octahedron_edges() {
        let (vertices, _) = PlatonicSolid::octahedron();

        assert_eq!(1.0, magnitude_squared_diff(&vertices[0].position, &vertices[1].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[0].position, &vertices[2].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[0].position, &vertices[3].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[0].position, &vertices[4].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[1].position, &vertices[2].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[1].position, &vertices[4].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[1].position, &vertices[5].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[2].position, &vertices[3].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[2].position, &vertices[5].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[3].position, &vertices[4].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[3].position, &vertices[5].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[4].position, &vertices[5].position));
    }

    #[test]
    #[ignore]
    fn dodecahedron_centered() {
        let (vertices, _) = PlatonicSolid::dodecahedron();

        let r_squared = magnitude_squared(&vertices[0].position);

        for vertex in vertices {
            println!("{:?}", vertex);
            assert_eq!(r_squared, magnitude_squared(&vertex.position));
        }
    }

    #[test]
    #[ignore]
    fn dodecahedron_edges() {
        let (vertices, _) = PlatonicSolid::dodecahedron();

        assert_eq!(1.0, magnitude_squared_diff(&vertices[0].position, &vertices[1].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[0].position, &vertices[2].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[0].position, &vertices[5].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[1].position, &vertices[3].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[1].position, &vertices[6].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[2].position, &vertices[4].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[2].position, &vertices[7].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[3].position, &vertices[4].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[3].position, &vertices[8].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[4].position, &vertices[9].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[5].position, &vertices[10].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[5].position, &vertices[11].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[6].position, &vertices[10].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[6].position, &vertices[12].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[7].position, &vertices[11].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[7].position, &vertices[13].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[8].position, &vertices[12].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[8].position, &vertices[14].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[9].position, &vertices[13].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[9].position, &vertices[14].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[10].position, &vertices[15].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[11].position, &vertices[16].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[12].position, &vertices[17].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[13].position, &vertices[18].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[14].position, &vertices[19].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[15].position, &vertices[16].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[15].position, &vertices[17].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[16].position, &vertices[18].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[17].position, &vertices[19].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[18].position, &vertices[19].position));
    }

    #[test]
    #[ignore]
    fn icosahedron_centered() {
        let (vertices, _) = PlatonicSolid::icosahedron();

        let r_squared = magnitude_squared(&vertices[0].position);

        for vertex in vertices {
            println!("{:?}", vertex);
            assert_eq!(r_squared, magnitude_squared(&vertex.position));
        }
    }

    #[test]
    #[ignore]
    fn icosahedron_edges() {
        let (vertices, _) = PlatonicSolid::icosahedron();

        assert_eq!(1.0, magnitude_squared_diff(&vertices[0].position, &vertices[1].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[0].position, &vertices[2].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[0].position, &vertices[3].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[0].position, &vertices[4].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[0].position, &vertices[5].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[1].position, &vertices[2].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[1].position, &vertices[3].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[1].position, &vertices[6].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[1].position, &vertices[7].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[2].position, &vertices[4].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[2].position, &vertices[6].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[2].position, &vertices[8].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[3].position, &vertices[5].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[3].position, &vertices[7].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[3].position, &vertices[9].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[4].position, &vertices[5].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[4].position, &vertices[8].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[4].position, &vertices[10].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[5].position, &vertices[9].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[5].position, &vertices[10].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[6].position, &vertices[7].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[6].position, &vertices[8].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[6].position, &vertices[11].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[7].position, &vertices[9].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[7].position, &vertices[11].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[8].position, &vertices[10].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[8].position, &vertices[11].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[9].position, &vertices[10].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[9].position, &vertices[11].position));
        assert_eq!(1.0, magnitude_squared_diff(&vertices[10].position, &vertices[11].position));
    }
}
