use glium::{backend::Facade, index::PrimitiveType, IndexBuffer, VertexBuffer};
use crate::text::{MAX_LINE, tex_map};

#[derive(Clone, Copy, Debug, Default)]
pub struct CharVertex {
    pub pos: [f32; 2],
    pub tex: [f32; 2],
}

implement_vertex!(CharVertex, pos, tex);

const HEIGHT: f32 = 0.05f32;
const WIDTH: f32 = 0.03f32;
const SPACE: f32 = 0.01f32;

pub struct CharString {
    vertex_count: usize,
    vertices: VertexBuffer<CharVertex>,
    indices: IndexBuffer<u16>,
}

impl CharString {
    pub fn new(facade: &dyn Facade) -> Self {
        let indices: Vec<u16> = (0..MAX_LINE as u16)
            .flat_map(| i | {
                let i = 4 * i;
                [i, i + 1, i + 2, i + 2, i + 1, i + 3]
            })
            .collect();

        CharString {
            vertex_count: 0,
            vertices: VertexBuffer::dynamic(facade, &[Default::default(); 4 * MAX_LINE]).unwrap(),
            indices: IndexBuffer::immutable(
                facade,
                PrimitiveType::TrianglesList,
                &indices
            ).unwrap(),
        }
    }

    pub fn vertices(&self) -> glium::vertex::VertexBufferSlice<CharVertex> {
        self.vertices.slice(0..(4 * self.vertex_count)).unwrap()
    }

    pub fn indices(&self) -> glium::index::IndexBufferSlice<u16> {
        self.indices.slice(0..(6 * self.vertex_count)).unwrap()
    }

    pub fn clear(&mut self) {
        self.vertex_count = 0;
    }

    pub fn append(&mut self, ch: char) {

        let start = (WIDTH + SPACE) * self.vertex_count as f32;
        let index = 4 * self.vertex_count;
        let [left, right, bottom, top] = tex_map(ch);

        let vertices = [
            CharVertex {
                pos: [start, 0.0],
                tex: [left, bottom],
            },
            CharVertex {
                pos: [start + WIDTH, 0.0],
                tex: [right, bottom],
            },
            CharVertex {
                pos: [start, HEIGHT],
                tex: [left, top],
            },
            CharVertex {
                pos: [start + WIDTH, HEIGHT],
                tex: [right, top],
            },
        ];

        self.vertices.slice_mut(index..(index + 4)).unwrap().write(&vertices);

        self.vertex_count += 1;
    }

    /// # Panics
    ///
    /// Panics if called more times than `append`
    pub fn unappend(&mut self) {
        self.vertex_count -= 1;
    }
}
