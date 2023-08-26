use crate::shapes::shapes::PosVertex;
use glium::backend::Facade;
use glium::index::{NoIndices, PrimitiveType};
use glium::vertex::VertexBuffer;

pub enum Polygons {
    Trigon,
    Tetragon,
    Pentagon,
    Hexagon,
    Heptagon,
    Octagon,
    Enneagon,
    Decagon,
    Hendecagon,
    Dodecagon,
    NGon(u16),
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
            Polygons::Hexagon => {
                let sin_60 = 0.5 * f32::sqrt(3.0);

                Polygon {
                    vertices: VertexBuffer::new(
                            facade,
                            &[
                                PosVertex::new([-1.0, 0.0, 0.0]),
                                PosVertex::new([-0.5, -sin_60, 0.0]),
                                PosVertex::new([-0.5, sin_60, 0.0]),
                                PosVertex::new([0.5, -sin_60, 0.0]),
                                PosVertex::new([0.5, sin_60, 0.0]),
                                PosVertex::new([1.0, 0.0, 0.0]),
                            ],
                        )
                        .unwrap(),
                    indices: NoIndices(PrimitiveType::TriangleStrip),
                }
            },
            Polygons::Heptagon => {
                let pi = std::f32::consts::PI;

                let inner_span = f32::cos(pi / 7.0);
                let span = inner_span + f32::cos(3.0 / 7.0 * pi);
                let bottom = f32::sin(2.0 / 7.0 * pi);
                let base = bottom + f32::sin(3.0 / 7.0 * pi);
                let height = base + f32::sin(pi / 7.0);
                let offset = 0.5 * height - 0.125 / height;

                Polygon {
                    vertices: VertexBuffer::new(
                            facade,
                            &[
                                PosVertex::new([0.0, height - offset, 0.0]),
                                PosVertex::new([-inner_span, base - offset, 0.0]),
                                PosVertex::new([inner_span, base - offset, 0.0]),
                                PosVertex::new([-span, bottom - offset, 0.0]),
                                PosVertex::new([span, bottom - offset, 0.0]),
                                PosVertex::new([-0.5, -offset, 0.0]),
                                PosVertex::new([0.5, -offset, 0.0]),
                            ],
                        )
                        .unwrap(),
                    indices: NoIndices(PrimitiveType::TriangleStrip),
                }
            },
            Polygons::Octagon => {
                let extend = 1.0 / f32::sqrt(2.0);

                Polygon {
                    vertices: VertexBuffer::new(
                            facade,
                            &[
                                PosVertex::new([0.5, 0.5 + extend, 0.0]),
                                PosVertex::new([-0.5, 0.5 + extend, 0.0]),
                                PosVertex::new([0.5 + extend, 0.5, 0.0]),
                                PosVertex::new([-0.5 - extend, 0.5, 0.0]),
                                PosVertex::new([0.5 + extend, -0.5, 0.0]),
                                PosVertex::new([-0.5 - extend, -0.5, 0.0]),
                                PosVertex::new([0.5, -0.5 - extend, 0.0]),
                                PosVertex::new([-0.5, -0.5 - extend, 0.0]),
                            ],
                        )
                        .unwrap(),
                    indices: NoIndices(PrimitiveType::TriangleStrip),
                }
            },
            Polygons::Enneagon => {
                let pi = std::f32::consts::PI;

                let inner_span = f32::cos(pi / 9.0);
                let top_span = inner_span + f32::cos(1.0 / 3.0 * pi);
                let bot_span = 0.5 + f32::cos(2.0 / 9.0 * pi);
                let bottom = f32::sin(2.0 / 9.0 * pi);
                let lower = bottom + f32::sin(4.0 / 9.0 * pi);
                let upper = lower + f32::sin(1.0 / 3.0 * pi);
                let height = upper + f32::sin(pi / 9.0);
                let offset = 0.5 * height - 0.125 / height;

                Polygon {
                    vertices: VertexBuffer::new(
                            facade,
                            &[
                                PosVertex::new([0.0, height - offset, 0.0]),
                                PosVertex::new([-inner_span, upper - offset, 0.0]),
                                PosVertex::new([inner_span, upper - offset, 0.0]),
                                PosVertex::new([-top_span, lower - offset, 0.0]),
                                PosVertex::new([top_span, lower - offset, 0.0]),
                                PosVertex::new([-bot_span, bottom - offset, 0.0]),
                                PosVertex::new([bot_span, bottom - offset, 0.0]),
                                PosVertex::new([-0.5, -offset, 0.0]),
                                PosVertex::new([0.5, -offset, 0.0]),
                            ],
                        )
                        .unwrap(),
                    indices: NoIndices(PrimitiveType::TriangleStrip),
                }
            },
            Polygons::Decagon => {
                let pi = std::f32::consts::PI;

                let upper_span = 0.5 + f32::cos(pi / 5.0);
                let lower_span = upper_span + f32::cos(2.0 / 5.0 * pi);
                let inner_height = f32::sin(2.0 / 5.0 * pi);
                let outer_height = inner_height + f32::sin(pi / 5.0);

                Polygon {
                    vertices: VertexBuffer::new(
                            facade,
                            &[
                                PosVertex::new([0.5, outer_height, 0.0]),
                                PosVertex::new([-0.5, outer_height, 0.0]),
                                PosVertex::new([upper_span, inner_height, 0.0]),
                                PosVertex::new([-upper_span, inner_height, 0.0]),
                                PosVertex::new([lower_span, 0.0, 0.0]),
                                PosVertex::new([-lower_span, 0.0, 0.0]),
                                PosVertex::new([upper_span, -inner_height, 0.0]),
                                PosVertex::new([-upper_span, -inner_height, 0.0]),
                                PosVertex::new([0.5, -outer_height, 0.0]),
                                PosVertex::new([-0.5, -outer_height, 0.0]),
                            ],
                        )
                        .unwrap(),
                    indices: NoIndices(PrimitiveType::TriangleStrip),
                }
            },
            Polygons::Hendecagon => {
                let pi = std::f32::consts::PI;

                let inner_span = f32::cos(pi / 11.0);
                let middle_span = inner_span + f32::cos(3.0 / 11.0 * pi);
                let outer_span = middle_span + f32::cos(5.0 / 11.0 * pi);
                let bottom_span = 0.5 + f32::cos(2.0 / 11.0 * pi);
                let bottom = f32::sin(2.0 / 11.0 * pi);
                let lower = bottom + f32::sin(4.0 / 11.0 * pi);
                let middle = lower + f32::sin(5.0 / 11.0 * pi);
                let upper = middle + f32::sin(3.0 / 11.0 * pi);
                let height = upper + f32::sin(pi / 11.0);
                let offset = 0.5 * height - 0.125 / height;

                Polygon {
                    vertices: VertexBuffer::new(
                            facade,
                            &[
                                PosVertex::new([0.0, height - offset, 0.0]),
                                PosVertex::new([-inner_span, upper - offset, 0.0]),
                                PosVertex::new([inner_span, upper - offset, 0.0]),
                                PosVertex::new([-middle_span, middle - offset, 0.0]),
                                PosVertex::new([middle_span, middle - offset, 0.0]),
                                PosVertex::new([-outer_span, lower - offset, 0.0]),
                                PosVertex::new([outer_span, lower - offset, 0.0]),
                                PosVertex::new([-bottom_span, bottom - offset, 0.0]),
                                PosVertex::new([bottom_span, bottom - offset, 0.0]),
                                PosVertex::new([-0.5, -offset, 0.0]),
                                PosVertex::new([0.5, -offset, 0.0]),
                            ],
                        )
                        .unwrap(),
                    indices: NoIndices(PrimitiveType::TriangleStrip),
                }
            },
            Polygons::Dodecagon => {
                let pi = std::f32::consts::PI;

                let upper_span = 0.5 + f32::cos(pi / 6.0);
                let lower_span = upper_span + f32::cos(pi / 3.0);
                let inner_height = 0.5 + f32::sin(pi / 3.0);
                let outer_height = inner_height + f32::sin(pi / 6.0);

                Polygon {
                    vertices: VertexBuffer::new(
                            facade,
                            &[
                                PosVertex::new([0.5, outer_height, 0.0]),
                                PosVertex::new([-0.5, outer_height, 0.0]),
                                PosVertex::new([upper_span, inner_height, 0.0]),
                                PosVertex::new([-upper_span, inner_height, 0.0]),
                                PosVertex::new([lower_span, 0.5, 0.0]),
                                PosVertex::new([-lower_span, 0.5, 0.0]),
                                PosVertex::new([lower_span, -0.5, 0.0]),
                                PosVertex::new([-lower_span, -0.5, 0.0]),
                                PosVertex::new([upper_span, -inner_height, 0.0]),
                                PosVertex::new([-upper_span, -inner_height, 0.0]),
                                PosVertex::new([0.5, -outer_height, 0.0]),
                                PosVertex::new([-0.5, -outer_height, 0.0]),
                            ],
                        )
                        .unwrap(),
                    indices: NoIndices(PrimitiveType::TriangleStrip),
                }
            },
            Polygons::NGon(sides) => {
                let pi = std::f32::consts::PI;

                let exterior = 2.0 / sides as f32 * pi;
                let start = 0.5 * pi;
                let offset = 0.5 * exterior;
                let radius = f32::sin(start - offset) / f32::sin(exterior);

                let mut vertices = vec![];
                if sides % 2 == 0 {
                    for step in 0..sides/2 {
                        let left = start + offset + step as f32 * exterior;
                        let right = start - offset - step as f32 * exterior;
                        vertices.push(PosVertex::new([radius * f32::cos(right), radius * f32::sin(right), 0.0]));
                        vertices.push(PosVertex::new([radius * f32::cos(left), radius * f32::sin(left), 0.0]));
                    }
                } else {
                    vertices.push(PosVertex::new([0.0, radius, 0.0]));

                    for step in 1..=sides/2 {
                        let left = start + step as f32 * exterior;
                        let right = start - step as f32 * exterior;
                        vertices.push(PosVertex::new([radius * f32::cos(left), radius * f32::sin(left), 0.0]));
                        vertices.push(PosVertex::new([radius * f32::cos(right), radius * f32::sin(right), 0.0]));
                    }
                };

                Polygon {
                    vertices: VertexBuffer::new(facade, &vertices).unwrap(),
                    indices: NoIndices(PrimitiveType::TriangleStrip),
                }
            },
        }
    }
}
