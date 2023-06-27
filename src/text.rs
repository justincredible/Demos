pub mod text {
    use crate::glutin::event::{ElementState, KeyboardInput, ModifiersState, VirtualKeyCode};
    use glium::{backend::Facade, index::PrimitiveType, IndexBuffer, VertexBuffer};
    use std::ops::BitAnd;

    const MAX_LINE: usize = 256;

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
            const HEIGHT: f32 = 0.04f32;
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
            match ch {
                _ => [0.0, 1.0, 0.0, 1.0]
            }
        }
    }

    #[derive(Clone, Copy, Debug)]
    struct Key {
        virtual_keycode: Option<VirtualKeyCode>,
        modifiers: ModifiersState,
    }

    impl From<KeyboardInput> for Key {
        fn from(input: KeyboardInput) -> Self {
            Key {
                virtual_keycode: input.virtual_keycode,
                modifiers: ModifiersState::empty(),
            }
        }
    }

    struct KeyBuffer {
        index: usize,
        keys: [Key; MAX_LINE],
    }

    pub struct Console {
        buffer: KeyBuffer,
        modifiers: ModifiersState,
    }

    impl Console {
        pub fn new() -> Self {
            let modifiers = ModifiersState::empty();

            Console {
                buffer: KeyBuffer {
                    index: 0,
                    keys: [ Key {
                            virtual_keycode: None,
                            modifiers,
                        }; MAX_LINE
                    ]
                },
                modifiers,
            }
        }

        pub fn set_modifiers(&mut self, modifiers: ModifiersState) {
            self.modifiers = modifiers;
        }

        pub fn read(&self) -> String {
            self.buffer.keys
                .iter()
                .take(self.buffer.index)
                .map(| k | Self::key_map(&k.virtual_keycode.unwrap(), &k.modifiers))
                .collect()
        }

        pub fn write(&mut self, input: KeyboardInput) {
            match input {
                KeyboardInput {
                    state: ElementState::Released,
                    virtual_keycode: Some(VirtualKeyCode::Return),
                    ..
                } => self.flush(),

                KeyboardInput {
                    state: ElementState::Released,
                    virtual_keycode: Some(VirtualKeyCode::Escape),
                    ..
                } => self.buffer.index = 0,

                KeyboardInput {
                    state: ElementState::Released,
                    virtual_keycode: Some(VirtualKeyCode::Back), // Backspace),
                    ..
                } => self.buffer.index -= 1,

                KeyboardInput {
                    state: ElementState::Released,
                    ..
                } => {
                    if self.buffer.index == MAX_LINE {
                        self.flush();
                    }
                    let mut key: Key = input.into();
                    key.modifiers = self.modifiers;
                    self.buffer.keys[self.buffer.index] = key;
                    self.buffer.index += 1
                },

                _ => (),
            }
        }

        fn flush(&mut self) {
            for key in &self.buffer.keys[0..self.buffer.index] {
                match key {
                    Key {
                        virtual_keycode: Some(virtual_keycode),
                        modifiers,
                        ..
                    } => print!("{}", Self::key_map(virtual_keycode, modifiers)),

                    _ => (),
                }
            }

            self.buffer.index = 0;
            println!("");
        }

        fn key_map(key: &VirtualKeyCode, modifiers: &ModifiersState) -> char {
            let shifted = modifiers
                .bitand(ModifiersState::SHIFT)
                .eq(&ModifiersState::SHIFT);

            match key {
                VirtualKeyCode::Space | VirtualKeyCode::Tab => if shifted { '\t' } else { ' ' },

                // Numpad keys cannot be shifted
                VirtualKeyCode::Key1 | VirtualKeyCode::Numpad1 => if shifted { '!' } else { '1' },
                VirtualKeyCode::Key2 | VirtualKeyCode::Numpad2 => if shifted { '@' } else { '2' },
                VirtualKeyCode::Key3 | VirtualKeyCode::Numpad3 => if shifted { '#' } else { '3' },
                VirtualKeyCode::Key4 | VirtualKeyCode::Numpad4 => if shifted { '$' } else { '4' },
                VirtualKeyCode::Key5 | VirtualKeyCode::Numpad5 => if shifted { '%' } else { '5' },
                VirtualKeyCode::Key6 | VirtualKeyCode::Numpad6 => if shifted { '^' } else { '6' },
                VirtualKeyCode::Key7 | VirtualKeyCode::Numpad7 => if shifted { '&' } else { '7' },
                VirtualKeyCode::Key8 | VirtualKeyCode::Numpad8 => if shifted { '*' } else { '8' },
                VirtualKeyCode::Key9 | VirtualKeyCode::Numpad9 => if shifted { '(' } else { '9' },
                VirtualKeyCode::Key0 | VirtualKeyCode::Numpad0 => if shifted { ')' } else { '0' },

                VirtualKeyCode::A => if shifted { 'A' } else { 'a' },
                VirtualKeyCode::B => if shifted { 'B' } else { 'b' },
                VirtualKeyCode::C => if shifted { 'C' } else { 'c' },
                VirtualKeyCode::D => if shifted { 'D' } else { 'd' },
                VirtualKeyCode::E => if shifted { 'E' } else { 'e' },
                VirtualKeyCode::F => if shifted { 'F' } else { 'f' },
                VirtualKeyCode::G => if shifted { 'G' } else { 'g' },
                VirtualKeyCode::H => if shifted { 'H' } else { 'h' },
                VirtualKeyCode::I => if shifted { 'I' } else { 'i' },
                VirtualKeyCode::J => if shifted { 'J' } else { 'j' },
                VirtualKeyCode::K => if shifted { 'K' } else { 'k' },
                VirtualKeyCode::L => if shifted { 'L' } else { 'l' },
                VirtualKeyCode::M => if shifted { 'M' } else { 'm' },
                VirtualKeyCode::N => if shifted { 'N' } else { 'n' },
                VirtualKeyCode::O => if shifted { 'O' } else { 'o' },
                VirtualKeyCode::P => if shifted { 'P' } else { 'p' },
                VirtualKeyCode::Q => if shifted { 'Q' } else { 'q' },
                VirtualKeyCode::R => if shifted { 'R' } else { 'r' },
                VirtualKeyCode::S => if shifted { 'S' } else { 's' },
                VirtualKeyCode::T => if shifted { 'T' } else { 't' },
                VirtualKeyCode::U => if shifted { 'U' } else { 'u' },
                VirtualKeyCode::V => if shifted { 'V' } else { 'v' },
                VirtualKeyCode::W => if shifted { 'W' } else { 'w' },
                VirtualKeyCode::X => if shifted { 'X' } else { 'x' },
                VirtualKeyCode::Y => if shifted { 'Y' } else { 'y' },
                VirtualKeyCode::Z => if shifted { 'Z' } else { 'z' },

                VirtualKeyCode::Grave => if shifted { '~' } else { '`' },
                VirtualKeyCode::Minus => if shifted { '_' } else { '-' },
                VirtualKeyCode::Equals => if shifted { '+' } else { '=' },
                VirtualKeyCode::LBracket => if shifted { '{' } else { '[' },
                VirtualKeyCode::RBracket => if shifted { '}' } else { ']' },
                VirtualKeyCode::Backslash => if shifted { '|' } else { '\\' },
                VirtualKeyCode::Semicolon => if shifted { ':' } else { ';' },
                VirtualKeyCode::Apostrophe => if shifted { '"' } else { '\'' },
                VirtualKeyCode::Comma => if shifted { '<' } else { ',' },
                VirtualKeyCode::Period => if shifted { '>' } else { '.' },
                VirtualKeyCode::Slash => if shifted { '?' } else { '/' },

                _ => '\0',
            }
        }
    }
}
