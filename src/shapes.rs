pub mod shapes {

    use glium::backend::Facade;
    use glium::index::{IndexBuffer, NoIndices, PrimitiveType};
    use glium::vertex::VertexBuffer;

    #[derive(Debug, Clone, Copy)]
    pub struct PosVertex {
        position: [f32; 3],
    }

    implement_vertex!(PosVertex, position);

    pub struct Triangle {
        pub vertices: VertexBuffer<PosVertex>,
        pub indices: NoIndices,
    }

    impl Triangle {
        pub fn new(facade: &dyn Facade) -> Self {
            let half_sin_60: f32 = f32::sqrt(3.0) / 4.0;

            Triangle {
                vertices: VertexBuffer::new(
                    facade,
                    &[
                        PosVertex { position: [-0.5, -half_sin_60, 0.0] },
                        PosVertex { position: [0.5, -half_sin_60, 0.0] },
                        PosVertex { position: [0.0, half_sin_60, 0.0] },
                    ],
                ).unwrap(),
                indices: NoIndices(PrimitiveType::TriangleStrip),
            }
        }
    }

    pub struct Quadrilateral {
        pub vertices: VertexBuffer<PosVertex>,
        pub indices: NoIndices,
    }

    impl Quadrilateral {
        pub fn new(facade: &dyn Facade) -> Self {
            Quadrilateral {
                vertices: VertexBuffer::new(
                    facade,
                    &[
                        PosVertex { position: [-0.5, -0.5, 0.0] },
                        PosVertex { position: [0.5, -0.5, 0.0] },
                        PosVertex { position: [-0.5, 0.5, 0.0] },
                        PosVertex { position: [0.5, 0.5, 0.0] },
                    ],
                ).unwrap(),
                indices: NoIndices(PrimitiveType::TriangleStrip),
            }
        }
    }

    pub struct Pentagon {
        pub vertices: VertexBuffer<PosVertex>,
        pub indices: NoIndices,
    }

    impl Pentagon {
        pub fn new(facade: &dyn Facade) -> Self {
            let pi = std::f32::consts::PI;

            let half_width = f32::sin (3.0/10.0*pi);
            let width_offset = f32::cos (2.0/5.0*pi);
            let height_offset = f32::cos (3.0/10.0*pi);
            let half_height = (height_offset + f32::sin (2.0/5.0*pi)) / 2.0;

            Pentagon {
                vertices: VertexBuffer::new(
                    facade,
                    &[
                        PosVertex { position: [0.0, half_height, 0.0] },
                        PosVertex { position: [-half_width, half_height - height_offset, 0.0] },
                        PosVertex { position: [half_width, half_height - height_offset, 0.0] },
                        PosVertex { position: [-half_width + width_offset, -half_height, 0.0] },
                        PosVertex { position: [half_width - width_offset, -half_height, 0.0] },
                    ],
                ).unwrap(),
                indices: NoIndices(PrimitiveType::TriangleStrip),
            }
        }
    }

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
                        PosVertex { position: [-0.5, -half_square_sin_60, half_sin_60] },
                        PosVertex { position: [0.5, -half_square_sin_60, half_sin_60] },
                        PosVertex { position: [0.0, half_square_sin_60, 0.0] },
                        PosVertex { position: [0.0, -half_square_sin_60, -half_sin_60] },
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
                        PosVertex { position: [-0.5, -0.5, 0.5] },
                        PosVertex { position: [0.5, -0.5, 0.5] },
                        PosVertex { position: [-0.5, 0.5, 0.5] },
                        PosVertex { position: [0.5, 0.5, 0.5] },
                        PosVertex { position: [0.5, 0.5, -0.5] },
                        PosVertex { position: [0.5, -0.5, -0.5] },
                        PosVertex { position: [-0.5, -0.5, -0.5] },
                        PosVertex { position: [-0.5, 0.5, -0.5] },
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
                        PosVertex { position: [0.0, half_height, 0.0] },
                        PosVertex { position: [-0.5, 0.0, -0.5] },
                        PosVertex { position: [-0.5, 0.0, 0.5] },
                        PosVertex { position: [0.5, 0.0, 0.5] },
                        PosVertex { position: [0.5, 0.0, -0.5] },
                        PosVertex { position: [0.0, -half_height, 0.0] },
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
                        PosVertex { position: [0.0, inner_height - icy, oz + 0.5*iz] },
                        PosVertex { position: [-inner_width, inner_mid - icy, oz + 0.5*iz] },
                        PosVertex { position: [inner_width, inner_mid - icy, oz + 0.5*iz] },
                        PosVertex { position: [-0.5, -icy, oz + 0.5*iz] },
                        PosVertex { position: [0.5, -icy, oz + 0.5*iz] },
                        PosVertex { position: [0.0, outer_height - ocy, 0.5*iz] },
                        PosVertex { position: [-outer_width, outer_mid - ocy, 0.5*iz] },
                        PosVertex { position: [outer_width, outer_mid - ocy, 0.5*iz] },
                        PosVertex { position: [-0.5*phi, -ocy, 0.5*iz] },
                        PosVertex { position: [0.5*phi, -ocy, 0.5*iz] },
                        PosVertex { position: [-or * f32::cos(degrees_54), or * f32::sin(degrees_54), -0.5*iz] },
                        PosVertex { position: [or * f32::cos(degrees_54), or * f32::sin(degrees_54), -0.5*iz] },
                        PosVertex { position: [-or * f32::cos(-degrees_18), or * f32::sin(-degrees_18), -0.5*iz] },
                        PosVertex { position: [or * f32::cos(-degrees_18), or * f32::sin(-degrees_18), -0.5*iz] },
                        PosVertex { position: [0.0, -or, -0.5*iz] },
                        PosVertex { position: [-ir * f32::cos(degrees_54), ir * f32::sin(degrees_54), -oz - 0.5*iz] },
                        PosVertex { position: [ir * f32::cos(degrees_54), ir * f32::sin(degrees_54), -oz - 0.5*iz] },
                        PosVertex { position: [-ir * f32::cos(-degrees_18), ir * f32::sin(-degrees_18), -oz - 0.5*iz] },
                        PosVertex { position: [ir * f32::cos(-degrees_18), ir * f32::sin(-degrees_18), -oz - 0.5*iz] },
                        PosVertex { position: [0.0, -ir, -oz - 0.5*iz] },
                    ],
                ).unwrap(),
                indices: IndexBuffer::new(
                    facade,
                    PrimitiveType::TrianglesList,
                    &[
                        0u8, 1, 2, 2, 1, 3, 2, 3, 4,
                        10, 6, 5, 5, 6, 1, 5, 1, 0,
                        11, 5, 7, 7, 5, 0, 7, 0, 2,
                        12, 8, 6, 6, 8, 3, 6, 3, 1,
                        13, 7, 9, 9, 7, 2, 9, 2, 4,
                        14, 9, 8, 8, 9, 4, 8, 4, 3,
                        5, 11, 10, 10, 11, 16, 10, 16, 15,
                        6, 10, 12, 12, 10, 15, 12, 15, 17,
                        7, 13, 11, 11, 13, 18, 11, 18, 16,
                        8, 12, 14, 14, 12, 17, 14, 17, 19,
                        9, 14, 13, 13, 14, 19, 13, 19, 18,
                        19, 17, 18, 18, 17, 15, 18, 15, 16,
                    ],
                ).unwrap(),
            }
        }
    }
}
