use glium::glutin::surface::WindowSurface;
use winit::event::{ElementState, KeyEvent};
use winit::keyboard::{KeyCode, ModifiersState, PhysicalKey};
use crate::text::{char_string::CharString, key_map, MAX_LINE};

#[derive(Clone, Copy, Debug)]
struct Key {
    keycode: KeyCode,
    modifiers: ModifiersState,
}

impl From<KeyEvent> for Key {
    fn from(event: KeyEvent) -> Self {
        match event {
            KeyEvent {
                physical_key: PhysicalKey::Code(keycode),
                ..
            } => {
                Key {
                    keycode,
                    modifiers: ModifiersState::empty(),
                }
            },
            _ => panic!("Unexpected event"),
        }
    }
}

pub struct Console {
    history: Vec<Key>,
    modifiers: ModifiersState,
    echo_line: CharString,
}

impl Console {
    pub fn new(display: &glium::Display<WindowSurface>) -> Self {
        let echo_line = CharString::new(display);

        let modifiers = ModifiersState::empty();

        Console {
            history: Vec::with_capacity(MAX_LINE),
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

    pub fn write(&mut self, event: KeyEvent) {
        match event {
            KeyEvent {
                state: ElementState::Released,
                physical_key: PhysicalKey::Code(KeyCode::Enter),
                ..
            } => self.flush(),

            KeyEvent {
                state: ElementState::Released,
                physical_key: PhysicalKey::Code(KeyCode::Escape),
                ..
            } => {
                self.history.clear();
                self.echo_line.clear();
            },

            KeyEvent {
                state: ElementState::Released,
                physical_key: PhysicalKey::Code(KeyCode::Backspace),
                ..
            } => if self.history.len() > 0 {
                self.history.pop();
                self.echo_line.unappend();
            },

            KeyEvent {
                state: ElementState::Released,
                ..
            } => {
                if self.history.len() == MAX_LINE {
                    self.flush();
                }
                let mut key: Key = event.into();
                key.modifiers = self.modifiers;

                let key_char = key_map(&key.keycode, &key.modifiers);
                if key_char != '\0' {
                    self.history.push(key);
                    self.echo_line.append(key_char);
                }
            },

            _ => (),
        }
    }

    fn flush(&mut self) {
        for key in &self.history {
            print!("{}", key_map(&key.keycode, &key.modifiers));
        }

        self.history.clear();
        self.echo_line.clear();
        println!("");
    }
}

