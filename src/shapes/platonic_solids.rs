pub mod platonic_solids {
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
            let half_sin_60: f32 = f32::sqrt(3.0) / 4.0;
            let half_square_sin_60 = 3.0 / 8.0;

            Tetrahedron {
                vertices: VertexBuffer::new(
                    facade,
                    &[
                        PosVertex::new([-0.5, -half_square_sin_60, half_sin_60]),
                        PosVertex::new([0.5, -half_square_sin_60, half_sin_60]),
                        PosVertex::new([0.0, half_square_sin_60, 0.0]),
                        PosVertex::new([0.0, -half_square_sin_60, -half_sin_60]),
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
            let half_height: f32 = 1.0 / f32::sqrt(2.0);

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

            // circle derivation
            let degrees_18 = std::f32::consts::PI/10.0;
            let degrees_54 = 3.0*std::f32::consts::PI/10.0;
            let inner_mid = f32::cos(degrees_18);
            let inner_height = f32::cos(degrees_54) + inner_mid;
            let inner_width = f32::sin(degrees_54);
            let outer_mid = phi * inner_mid;
            let outer_height = phi * inner_height;
            let outer_width = phi * inner_width;
            // .*cx = 0
            let icy = (inner_height * inner_height - 1.0 / 4.0) / 2.0 / inner_height;
            let ir = (inner_height * inner_height + 1.0 / 4.0) / 2.0 / inner_height;
            let ocy = (outer_height * outer_height - phi * phi / 4.0) / 2.0 / outer_height;
            let or = (outer_height * outer_height + phi * phi / 4.0) / 2.0 / outer_height;

            let y_diff = (outer_height - ocy) - (inner_height - icy);
            let oz = f32::sqrt(1.0 - y_diff*y_diff);
            let x_diff = or * f32::cos(degrees_54);
            let y_diff = or * f32::sin(degrees_54) - outer_height + ocy;
            let iz = f32::sqrt(1.0 - x_diff*x_diff - y_diff*y_diff);

            Dodecahedron {
                vertices: VertexBuffer::new(
                    facade,
                    &[
                        PosVertex::new([0.0, inner_height - icy, oz + 0.5*iz]),
                        PosVertex::new([-inner_width, inner_mid - icy, oz + 0.5*iz]),
                        PosVertex::new([inner_width, inner_mid - icy, oz + 0.5*iz]),
                        PosVertex::new([-0.5, -icy, oz + 0.5*iz]),
                        PosVertex::new([0.5, -icy, oz + 0.5*iz]),
                        PosVertex::new([0.0, outer_height - ocy, 0.5*iz]),
                        PosVertex::new([-outer_width, outer_mid - ocy, 0.5*iz]),
                        PosVertex::new([outer_width, outer_mid - ocy, 0.5*iz]),
                        PosVertex::new([-0.5*phi, -ocy, 0.5*iz]),
                        PosVertex::new([0.5*phi, -ocy, 0.5*iz]),
                        PosVertex::new([-or * f32::cos(degrees_54), or * f32::sin(degrees_54), -0.5*iz]),
                        PosVertex::new([or * f32::cos(degrees_54), or * f32::sin(degrees_54), -0.5*iz]),
                        PosVertex::new([-or * f32::cos(-degrees_18), or * f32::sin(-degrees_18), -0.5*iz]),
                        PosVertex::new([or * f32::cos(-degrees_18), or * f32::sin(-degrees_18), -0.5*iz]),
                        PosVertex::new([0.0, -or, -0.5*iz]),
                        PosVertex::new([-ir * f32::cos(degrees_54), ir * f32::sin(degrees_54), -oz - 0.5*iz]),
                        PosVertex::new([ir * f32::cos(degrees_54), ir * f32::sin(degrees_54), -oz - 0.5*iz]),
                        PosVertex::new([-ir * f32::cos(-degrees_18), ir * f32::sin(-degrees_18), -oz - 0.5*iz]),
                        PosVertex::new([ir * f32::cos(-degrees_18), ir * f32::sin(-degrees_18), -oz - 0.5*iz]),
                        PosVertex::new([0.0, -ir, -oz - 0.5*iz]),
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
}
