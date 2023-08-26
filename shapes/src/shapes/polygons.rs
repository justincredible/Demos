use crate::shapes::shapes::PosVertex;
use glium::backend::Facade;
use glium::index::{NoIndices, PrimitiveType};
use glium::vertex::VertexBuffer;

pub enum Polygons {
    Trigon,
    Tetragon,
    Pentagon,
}

pub struct Polygon {
    pub vertices: VertexBuffer<PosVertex>,
    pub indices: NoIndices,
}

impl Polygon {
    pub fn new(facade: &dyn Facade, variant: Polygons) -> Self {
        match variant {
            Polygons::Trigon => {
                let half_sin_60 = f32::sqrt(3.0) / 4.0;

                Polygon {
                    vertices: VertexBuffer::new(
                            facade,
                            &[
                                PosVertex::new([-0.5, -half_sin_60, 0.0]),
                                PosVertex::new([0.5, -half_sin_60, 0.0]),
                                PosVertex::new([0.0, half_sin_60, 0.0]),
                            ],
                        )
                        .unwrap(),
                    indices: NoIndices(PrimitiveType::TriangleStrip),
                }
            },
            Polygons::Tetragon => {
                Polygon {
                    vertices: VertexBuffer::new(
                            facade,
                            &[
                                PosVertex::new([-0.5, -0.5, 0.0]),
                                PosVertex::new([0.5, -0.5, 0.0]),
                                PosVertex::new([-0.5, 0.5, 0.0]),
                                PosVertex::new([0.5, 0.5, 0.0]),
                            ],
                        )
                        .unwrap(),
                    indices: NoIndices(PrimitiveType::TriangleStrip),
                }
            },
            Polygons::Pentagon => {
                let pi = std::f32::consts::PI;

                let half_width = f32::sin(3.0 / 10.0 * pi);
                let width_offset = f32::cos(2.0 / 5.0 * pi);
                let height_offset = f32::cos(3.0 / 10.0 * pi);
                let half_height = (height_offset + f32::sin(2.0 / 5.0 * pi)) / 2.0;

                Polygon {
                    vertices: VertexBuffer::new(
                            facade,
                            &[
                                PosVertex::new([0.0, half_height, 0.0]),
                                PosVertex::new([-half_width, half_height - height_offset, 0.0]),
                                PosVertex::new([half_width, half_height - height_offset, 0.0]),
                                PosVertex::new([-half_width + width_offset, -half_height, 0.0]),
                                PosVertex::new([half_width - width_offset, -half_height, 0.0]),
                            ],
                        )
                        .unwrap(),
                    indices: NoIndices(PrimitiveType::TriangleStrip),
                }
            },
        }
    }
}
