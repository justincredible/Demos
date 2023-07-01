use glium::glutin::event::{ElementState, KeyboardInput, ModifiersState, VirtualKeyCode};
use crate::text::{key_map, MAX_LINE};

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
            .map(| k | key_map(&k.virtual_keycode.unwrap(), &k.modifiers))
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

                if key_map(&key.virtual_keycode.unwrap(), &key.modifiers) != '\0' {
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
                } => print!("{}", key_map(virtual_keycode, modifiers)),

                _ => (),
            }
        }

        self.buffer.index = 0;
        println!("");
    }
}
