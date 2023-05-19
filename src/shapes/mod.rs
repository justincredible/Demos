pub mod platonic_solids;
pub mod polygons;

pub mod shapes {
    #[derive(Debug, Clone, Copy)]
    pub struct PosVertex {
        pub position: [f32; 3],
    }

    impl PosVertex {
        pub fn new(position: [f32; 3]) -> Self {
            PosVertex { position }
        }
    }

    implement_vertex!(PosVertex, position);
}
