use glium::glutin::event::{ElementState, KeyboardInput, ModifiersState, VirtualKeyCode};
use std::ops::BitAnd;
use crate::text::MAX_LINE;

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
            } => if self.buffer.index > 0 { self.buffer.index -= 1 },

            KeyboardInput {
                state: ElementState::Released,
                ..
            } => {
                if self.buffer.index == MAX_LINE {
                    self.flush();
                }
                let mut key: Key = input.into();
                key.modifiers = self.modifiers;

                if Self::key_map(&key.virtual_keycode.unwrap(), &key.modifiers) != '\0' {
                    self.buffer.keys[self.buffer.index] = key;
                    self.buffer.index += 1;
                }
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
