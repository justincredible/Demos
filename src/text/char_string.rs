use glium::{backend::Facade, index::PrimitiveType, IndexBuffer, VertexBuffer};
use crate::text::MAX_LINE;

#[derive(Clone, Copy, Debug, Default)]
pub struct CharVertex {
    pub pos: [f32; 2],
    pub tex: [f32; 2],
}

implement_vertex!(CharVertex, pos, tex);

pub struct CharString {
    vertex_count: usize,
    vertices: VertexBuffer<CharVertex>,
    indices: IndexBuffer<u16>,
}

impl CharString {
    pub fn new(facade: &dyn Facade) -> Self {
        let indices: Vec<u16> = (0..MAX_LINE as u16)
            .map(| i | {
                let i = 4 * i;
                [i, i + 1, i + 2, i + 2, i + 1, i + 3]
            })
            .flatten()
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

    pub fn update(&mut self, line: String) {
        const HEIGHT: f32 = 0.05f32;
        const WIDTH: f32 = 0.03f32;
        const SPACE: f32 = 0.01f32;

        let mut start = 0.0f32;
        self.vertex_count = 0;
        for (i, ch) in line.chars().enumerate() {
            let [left, right, bottom, top] = Self::tex_map(ch);
            let index = 4 * i;

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
            start += WIDTH + SPACE;
        }
    }

    fn tex_map(ch: char) -> [f32; 4] {
        let thirteenth = 0.076923076923;
        let eighth = 0.125;

        match ch {
            'a' => [0.0 * thirteenth, 1.0 * thirteenth, 7.0 * eighth, 8.0 * eighth],
            'b' => [1.0 * thirteenth, 2.0 * thirteenth, 7.0 * eighth, 8.0 * eighth],
            'c' => [2.0 * thirteenth, 3.0 * thirteenth, 7.0 * eighth, 8.0 * eighth],
            'd' => [3.0 * thirteenth, 4.0 * thirteenth, 7.0 * eighth, 8.0 * eighth],
            'e' => [4.0 * thirteenth, 5.0 * thirteenth, 7.0 * eighth, 8.0 * eighth],
            'f' => [5.0 * thirteenth, 6.0 * thirteenth, 7.0 * eighth, 8.0 * eighth],
            'g' => [6.0 * thirteenth, 7.0 * thirteenth, 7.0 * eighth, 8.0 * eighth],
            'h' => [7.0 * thirteenth, 8.0 * thirteenth, 7.0 * eighth, 8.0 * eighth],
            'i' => [8.0 * thirteenth, 9.0 * thirteenth, 7.0 * eighth, 8.0 * eighth],
            'j' => [9.0 * thirteenth, 10.0 * thirteenth, 7.0 * eighth, 8.0 * eighth],
            'k' => [10.0 * thirteenth, 11.0 * thirteenth, 7.0 * eighth, 8.0 * eighth],
            'l' => [11.0 * thirteenth, 12.0 * thirteenth, 7.0 * eighth, 8.0 * eighth],
            'm' => [12.0 * thirteenth, 13.0 * thirteenth, 7.0 * eighth, 8.0 * eighth],
            'n' => [0.0 * thirteenth, 1.0 * thirteenth, 6.0 * eighth, 7.0 * eighth],
            'o' => [1.0 * thirteenth, 2.0 * thirteenth, 6.0 * eighth, 7.0 * eighth],
            'p' => [2.0 * thirteenth, 3.0 * thirteenth, 6.0 * eighth, 7.0 * eighth],
            'q' => [3.0 * thirteenth, 4.0 * thirteenth, 6.0 * eighth, 7.0 * eighth],
            'r' => [4.0 * thirteenth, 5.0 * thirteenth, 6.0 * eighth, 7.0 * eighth],
            's' => [5.0 * thirteenth, 6.0 * thirteenth, 6.0 * eighth, 7.0 * eighth],
            't' => [6.0 * thirteenth, 7.0 * thirteenth, 6.0 * eighth, 7.0 * eighth],
            'u' => [7.0 * thirteenth, 8.0 * thirteenth, 6.0 * eighth, 7.0 * eighth],
            'v' => [8.0 * thirteenth, 9.0 * thirteenth, 6.0 * eighth, 7.0 * eighth],
            'w' => [9.0 * thirteenth, 10.0 * thirteenth, 6.0 * eighth, 7.0 * eighth],
            'x' => [10.0 * thirteenth, 11.0 * thirteenth, 6.0 * eighth, 7.0 * eighth],
            'y' => [11.0 * thirteenth, 12.0 * thirteenth, 6.0 * eighth, 7.0 * eighth],
            'z' => [12.0 * thirteenth, 13.0 * thirteenth, 6.0 * eighth, 7.0 * eighth],
            'A' => [0.0 * thirteenth, 1.0 * thirteenth, 5.0 * eighth, 6.0 * eighth],
            'B' => [1.0 * thirteenth, 2.0 * thirteenth, 5.0 * eighth, 6.0 * eighth],
            'C' => [2.0 * thirteenth, 3.0 * thirteenth, 5.0 * eighth, 6.0 * eighth],
            'D' => [3.0 * thirteenth, 4.0 * thirteenth, 5.0 * eighth, 6.0 * eighth],
            'E' => [4.0 * thirteenth, 5.0 * thirteenth, 5.0 * eighth, 6.0 * eighth],
            'F' => [5.0 * thirteenth, 6.0 * thirteenth, 5.0 * eighth, 6.0 * eighth],
            'G' => [6.0 * thirteenth, 7.0 * thirteenth, 5.0 * eighth, 6.0 * eighth],
            'H' => [7.0 * thirteenth, 8.0 * thirteenth, 5.0 * eighth, 6.0 * eighth],
            'I' => [8.0 * thirteenth, 9.0 * thirteenth, 5.0 * eighth, 6.0 * eighth],
            'J' => [9.0 * thirteenth, 10.0 * thirteenth, 5.0 * eighth, 6.0 * eighth],
            'K' => [10.0 * thirteenth, 11.0 * thirteenth, 5.0 * eighth, 6.0 * eighth],
            'L' => [11.0 * thirteenth, 12.0 * thirteenth, 5.0 * eighth, 6.0 * eighth],
            'M' => [12.0 * thirteenth, 13.0 * thirteenth, 5.0 * eighth, 6.0 * eighth],
            'N' => [0.0 * thirteenth, 1.0 * thirteenth, 4.0 * eighth, 5.0 * eighth],
            'O' => [1.0 * thirteenth, 2.0 * thirteenth, 4.0 * eighth, 5.0 * eighth],
            'P' => [2.0 * thirteenth, 3.0 * thirteenth, 4.0 * eighth, 5.0 * eighth],
            'Q' => [3.0 * thirteenth, 4.0 * thirteenth, 4.0 * eighth, 5.0 * eighth],
            'R' => [4.0 * thirteenth, 5.0 * thirteenth, 4.0 * eighth, 5.0 * eighth],
            'S' => [5.0 * thirteenth, 6.0 * thirteenth, 4.0 * eighth, 5.0 * eighth],
            'T' => [6.0 * thirteenth, 7.0 * thirteenth, 4.0 * eighth, 5.0 * eighth],
            'U' => [7.0 * thirteenth, 8.0 * thirteenth, 4.0 * eighth, 5.0 * eighth],
            'V' => [8.0 * thirteenth, 9.0 * thirteenth, 4.0 * eighth, 5.0 * eighth],
            'W' => [9.0 * thirteenth, 10.0 * thirteenth, 4.0 * eighth, 5.0 * eighth],
            'X' => [10.0 * thirteenth, 11.0 * thirteenth, 4.0 * eighth, 5.0 * eighth],
            'Y' => [11.0 * thirteenth, 12.0 * thirteenth, 4.0 * eighth, 5.0 * eighth],
            'Z' => [12.0 * thirteenth, 13.0 * thirteenth, 4.0 * eighth, 5.0 * eighth],
            '0' => [0.0 * thirteenth, 1.0 * thirteenth, 3.0 * eighth, 4.0 * eighth],
            '1' => [1.0 * thirteenth, 2.0 * thirteenth, 3.0 * eighth, 4.0 * eighth],
            '2' => [2.0 * thirteenth, 3.0 * thirteenth, 3.0 * eighth, 4.0 * eighth],
            '3' => [3.0 * thirteenth, 4.0 * thirteenth, 3.0 * eighth, 4.0 * eighth],
            '4' => [4.0 * thirteenth, 5.0 * thirteenth, 3.0 * eighth, 4.0 * eighth],
            '5' => [5.0 * thirteenth, 6.0 * thirteenth, 3.0 * eighth, 4.0 * eighth],
            '6' => [6.0 * thirteenth, 7.0 * thirteenth, 3.0 * eighth, 4.0 * eighth],
            '7' => [7.0 * thirteenth, 8.0 * thirteenth, 3.0 * eighth, 4.0 * eighth],
            '8' => [8.0 * thirteenth, 9.0 * thirteenth, 3.0 * eighth, 4.0 * eighth],
            '9' => [9.0 * thirteenth, 10.0 * thirteenth, 3.0 * eighth, 4.0 * eighth],
            '!' => [0.0 * thirteenth, 1.0 * thirteenth, 2.0 * eighth, 3.0 * eighth],
            '@' => [1.0 * thirteenth, 2.0 * thirteenth, 2.0 * eighth, 3.0 * eighth],
            '#' => [2.0 * thirteenth, 3.0 * thirteenth, 2.0 * eighth, 3.0 * eighth],
            '$' => [3.0 * thirteenth, 4.0 * thirteenth, 2.0 * eighth, 3.0 * eighth],
            '%' => [4.0 * thirteenth, 5.0 * thirteenth, 2.0 * eighth, 3.0 * eighth],
            '^' => [5.0 * thirteenth, 6.0 * thirteenth, 2.0 * eighth, 3.0 * eighth],
            '&' => [6.0 * thirteenth, 7.0 * thirteenth, 2.0 * eighth, 3.0 * eighth],
            '*' => [7.0 * thirteenth, 8.0 * thirteenth, 2.0 * eighth, 3.0 * eighth],
            '(' => [8.0 * thirteenth, 9.0 * thirteenth, 2.0 * eighth, 3.0 * eighth],
            ')' => [9.0 * thirteenth, 10.0 * thirteenth, 2.0 * eighth, 3.0 * eighth],
            '`' => [0.0 * thirteenth, 1.0 * thirteenth, 1.0 * eighth, 2.0 * eighth],
            '~' => [1.0 * thirteenth, 2.0 * thirteenth, 1.0 * eighth, 2.0 * eighth],
            '-' => [2.0 * thirteenth, 3.0 * thirteenth, 1.0 * eighth, 2.0 * eighth],
            '_' => [3.0 * thirteenth, 4.0 * thirteenth, 1.0 * eighth, 2.0 * eighth],
            '=' => [4.0 * thirteenth, 5.0 * thirteenth, 1.0 * eighth, 2.0 * eighth],
            '+' => [5.0 * thirteenth, 6.0 * thirteenth, 1.0 * eighth, 2.0 * eighth],
            '[' => [6.0 * thirteenth, 7.0 * thirteenth, 1.0 * eighth, 2.0 * eighth],
            '{' => [7.0 * thirteenth, 8.0 * thirteenth, 1.0 * eighth, 2.0 * eighth],
            ']' => [8.0 * thirteenth, 9.0 * thirteenth, 1.0 * eighth, 2.0 * eighth],
            '}' => [9.0 * thirteenth, 10.0 * thirteenth, 1.0 * eighth, 2.0 * eighth],
            '\\' => [10.0 * thirteenth, 11.0 * thirteenth, 1.0 * eighth, 2.0 * eighth],
            '|' => [11.0 * thirteenth, 12.0 * thirteenth, 1.0 * eighth, 2.0 * eighth],
            ';' => [0.0 * thirteenth, 1.0 * thirteenth, 0.0 * eighth, 1.0 * eighth],
            ':' => [1.0 * thirteenth, 2.0 * thirteenth, 0.0 * eighth, 1.0 * eighth],
            '\'' => [2.0 * thirteenth, 3.0 * thirteenth, 0.0 * eighth, 1.0 * eighth],
            '"' => [3.0 * thirteenth, 4.0 * thirteenth, 0.0 * eighth, 1.0 * eighth],
            ',' => [4.0 * thirteenth, 5.0 * thirteenth, 0.0 * eighth, 1.0 * eighth],
            '<' => [5.0 * thirteenth, 6.0 * thirteenth, 0.0 * eighth, 1.0 * eighth],
            '.' => [6.0 * thirteenth, 7.0 * thirteenth, 0.0 * eighth, 1.0 * eighth],
            '>' => [7.0 * thirteenth, 8.0 * thirteenth, 0.0 * eighth, 1.0 * eighth],
            '/' => [8.0 * thirteenth, 9.0 * thirteenth, 0.0 * eighth, 1.0 * eighth],
            '?' => [9.0 * thirteenth, 10.0 * thirteenth, 0.0 * eighth, 1.0 * eighth],
            _ => [0.0; 4]
        }
    }
}
