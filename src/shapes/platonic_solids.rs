use crate::shapes::shapes::PosVertex;
use glium::backend::Facade;
use glium::index::{IndexBuffer, PrimitiveType};
use glium::vertex::VertexBuffer;

pub struct Tetrahedron {
    pub vertices: VertexBuffer<PosVertex>,
    pub indices: IndexBuffer<u8>,
}

impl Tetrahedron {
    pub fn new(facade: &dyn Facade) -> Self {
        let depth = f32::sqrt(3.0) / 2.0;
        let z = depth / 2.0 - 0.125 / depth;
        let z_diff = z - depth;
        let y = f32::sqrt(1.0 - z_diff * z_diff);
        let above = 0.75 * y; // depth^2 * y
        let below = above - y;

        Tetrahedron {
            vertices: VertexBuffer::new(
                facade,
                &[
                    PosVertex::new([-0.5, below, z]),
                    PosVertex::new([0.5, below, z]),
                    PosVertex::new([0.0, above, 0.0]),
                    PosVertex::new([0.0, below, -depth + z]),
                ],
            ).unwrap(),
            indices: IndexBuffer::new(
                facade,
                PrimitiveType::TriangleStrip,
                &[0u8, 1, 2, 3, 0, 1]
            ).unwrap(),
        }
    }
}

pub struct Hexahedron {
    pub vertices: VertexBuffer<PosVertex>,
    pub indices: IndexBuffer<u8>,
}

impl Hexahedron {
    pub fn new(facade: &dyn Facade) -> Self {
        Hexahedron {
            vertices: VertexBuffer::new(
                facade,
                &[
                    PosVertex::new([-0.5, -0.5, 0.5]),
                    PosVertex::new([0.5, -0.5, 0.5]),
                    PosVertex::new([-0.5, 0.5, 0.5]),
                    PosVertex::new([0.5, 0.5, 0.5]),
                    PosVertex::new([0.5, 0.5, -0.5]),
                    PosVertex::new([0.5, -0.5, -0.5]),
                    PosVertex::new([-0.5, -0.5, -0.5]),
                    PosVertex::new([-0.5, 0.5, -0.5]),
                ],
            ).unwrap(),
            indices: IndexBuffer::new(
                facade,
                PrimitiveType::TriangleStrip,
                &[0u8, 1, 2, 3, 4, 1, 5, 0, 6, 2, 7, 4, 6, 5]
            ).unwrap(),
        }
    }
}

pub struct Octahedron {
    pub vertices: VertexBuffer<PosVertex>,
    pub indices: IndexBuffer<u8>,
}

impl Octahedron {
    pub fn new(facade: &dyn Facade) -> Self {
        let half_height = 1.0 / f32::sqrt(2.0);

        Octahedron {
            vertices: VertexBuffer::new(
                facade,
                &[
                    PosVertex::new([0.0, half_height, 0.0]),
                    PosVertex::new([-0.5, 0.0, -0.5]),
                    PosVertex::new([-0.5, 0.0, 0.5]),
                    PosVertex::new([0.5, 0.0, 0.5]),
                    PosVertex::new([0.5, 0.0, -0.5]),
                    PosVertex::new([0.0, -half_height, 0.0]),
                ],
            ).unwrap(),
            indices: IndexBuffer::new(
                facade,
                PrimitiveType::TriangleStrip,
                //&[0u8, 1, 2, 5, 3, 4, 1, 5, 2, 3, 0, 4, 1]
                //&[1u8, 2, 0, 3, 4, 5, 1, 2, 3, 5, 4, 1, 0]
                &[2u8, 0, 1, 4, 5, 3, 2, 0, 4, 3, 5, 2, 1]
            ).unwrap(),
        }
    }
}

pub struct Dodecahedron {
    pub vertices: VertexBuffer<PosVertex>,
    pub indices: IndexBuffer<u8>,
}

impl Dodecahedron {
    pub fn new(facade: &dyn Facade) -> Self {
        let phi = (1.0 + f32::sqrt(5.0)) / 2.0;

        let degrees_18 = std::f32::consts::PI / 10.0;
        let degrees_54 = 3.0 * std::f32::consts::PI / 10.0;
        let mid = f32::cos(degrees_18);
        let height = f32::cos(degrees_54) + mid;
        let width = f32::sin(degrees_54);
        let cy = height / 2.0 - 0.125 / height;

        let y_diff = (phi - 1.0) * (height - cy);
        let oz = f32::sqrt(1.0 - y_diff * y_diff);
        let x_diff = 0.5 * phi;
        let y_diff = phi * (height - 2.0 * cy);
        let iz = f32::sqrt(1.0 - x_diff * x_diff - y_diff * y_diff);

        Dodecahedron {
            vertices: VertexBuffer::new(
                facade,
                &[
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
                ],
            ).unwrap(),
            indices: IndexBuffer::new(
                facade,
                PrimitiveType::TriangleStrip,
                &[
                    10u8, 6, 5, 1, 0, 2, 5, 7, 11,
                    13, 16, 18, 19, 13, 14, 9, 8, 4,
                    3, 1, 8, 6, 12, 10, 17, 15, 16,
                    10, 11, 5, 1, 4, 2, 9, 7, 13,
                    16, 19, 17, 14, 12, 8,
                ],
            ).unwrap(),
        }
    }
}

pub struct Icosahedron {
    pub vertices: VertexBuffer<PosVertex>,
    pub indices: IndexBuffer<u8>,
}

impl Icosahedron {
    pub fn new(facade: &dyn Facade) -> Self {
        let degrees_18 = std::f32::consts::PI / 10.0;
        let degrees_54 = 3.0 * std::f32::consts::PI / 10.0;
        let mid = f32::cos(degrees_18);
        let depth = f32::cos(degrees_54) + mid;
        let width = f32::sin(degrees_54);
        let cz = depth / 2.0 - 0.125 / depth;
        let r = depth / 2.0 + 0.125 / depth;

        let z_diff = depth - cz;
        let y_diff = f32::sqrt(1.0 - z_diff * z_diff);
        let x_diff = r * f32::cos(degrees_54);
        let z_diff = z_diff - r * f32::sin(degrees_54);
        let half_middle = f32::sqrt(1.0 - x_diff * x_diff - z_diff * z_diff) / 2.0;

        Icosahedron {
            vertices: VertexBuffer::new(
                facade,
                &[
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
                ],
            ).unwrap(),
            indices: IndexBuffer::new(
                facade,
                PrimitiveType::TriangleStrip,
                &[
                    4u8, 10, 5, 9, 3, 7, 1, 6, 2,
                    8, 4, 10, 9, 11, 7, 6, 6, 11,
                    8, 10, 4, 5, 0, 3, 1, 1, 0,
                    2, 4,
                ],
            ).unwrap(),
        }
    }
}
