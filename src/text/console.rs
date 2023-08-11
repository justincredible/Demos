use glium::glutin::event::{ElementState, KeyboardInput, ModifiersState, VirtualKeyCode};
use crate::text::{char_string::CharString, key_map, MAX_LINE};

#[derive(Clone, Copy, Debug)]
struct Key {
    virtual_keycode: Option<VirtualKeyCode>,
    modifiers: ModifiersState,
}

impl From<KeyboardInput> for Key {
    fn from(input: KeyboardInput) -> Self {
        Key {
            virtual_keycode: input.virtual_keycode.or_else(|| {
                match input.scancode {
                    2 => VirtualKeyCode::Key1.into(),
                    4 => VirtualKeyCode::Key3.into(),
                    5 => VirtualKeyCode::Key4.into(),
                    6 => VirtualKeyCode::Key5.into(),
                    7 => VirtualKeyCode::Key6.into(),
                    8 => VirtualKeyCode::Key7.into(),
                    10 => VirtualKeyCode::Key9.into(),
                    11 => VirtualKeyCode::Key0.into(),
                    12 => VirtualKeyCode::Minus.into(),
                    26 => VirtualKeyCode::LBracket.into(),
                    27 => VirtualKeyCode::RBracket.into(),
                    40 => VirtualKeyCode::Apostrophe.into(),
                    41 => VirtualKeyCode::Grave.into(),
                    43 => VirtualKeyCode::Backslash.into(),
                    51 => VirtualKeyCode::Comma.into(),
                    52 => VirtualKeyCode::Period.into(),
                    53 => VirtualKeyCode::Slash.into(),
                    _ => None,
                }
            }),
            modifiers: ModifiersState::empty(),
        }
    }
}

pub struct Console {
    buffer: Vec<Key>,
    modifiers: ModifiersState,
    echo_line: CharString,
}

impl Console {
    pub fn new(display: &glium::Display) -> Self {
        let echo_line = CharString::new(display);

        let modifiers = ModifiersState::empty();

        Console {
            buffer: Vec::with_capacity(MAX_LINE),
            modifiers,
            echo_line,
        }
    }

    pub fn echo_line(&self) -> &CharString {
        &self.echo_line
    }

    pub fn set_modifiers(&mut self, modifiers: ModifiersState) {
        self.modifiers = modifiers;
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
            } => {
                self.buffer.clear();
                self.echo_line.clear();
            },

            KeyboardInput {
                state: ElementState::Released,
                virtual_keycode: Some(VirtualKeyCode::Back), // Backspace),
                ..
            } => if self.buffer.len() > 0 {
                self.buffer.pop();
                self.echo_line.unappend();
            },

            KeyboardInput {
                state: ElementState::Released,
                ..
            } => {
                if self.buffer.len() == MAX_LINE {
                    self.flush();
                }
                let mut key: Key = input.into();
                key.modifiers = self.modifiers;

                let key_char = key_map(&key.virtual_keycode.unwrap(), &key.modifiers);
                if key_char != '\0' {
                    self.buffer.push(key);
                    self.echo_line.append(key_char);
                }
            },

            _ => (),
        }
    }

    fn flush(&mut self) {
        for key in &self.buffer {
            match key {
                Key {
                    virtual_keycode: Some(virtual_keycode),
                    modifiers,
                    ..
                } => print!("{}", key_map(virtual_keycode, modifiers)),

                _ => (),
            }
        }

        self.buffer.clear();
        self.echo_line.clear();
        println!("");
    }
}

