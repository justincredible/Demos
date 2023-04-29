pub mod shapes {

    use glium::backend::Facade;
    use glium::index::{NoIndices, PrimitiveType};
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
            Triangle {
                vertices: VertexBuffer::new(
                    facade,
                    &[
                        PosVertex { position: [-0.5, -0.5, 0.0] },
                        PosVertex { position: [0.5, -0.5, 0.0] },
                        PosVertex { position: [0.0, 0.5, 0.0] },
                    ],
                ).unwrap(),
                indices: NoIndices(PrimitiveType::TriangleStrip),
            }
        }
    }
}
