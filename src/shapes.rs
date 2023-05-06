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
}
