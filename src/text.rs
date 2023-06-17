pub mod text {
    use crate::glutin::event::{ElementState, KeyboardInput, ModifiersState, VirtualKeyCode};
    use std::ops::BitAnd;

    const MAX_LINE: usize = 256;

    #[derive(Clone, Copy)]
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

    macro_rules! key_mapper {
        ($key:ident, $shifted:ident, $($pattern:pat, $shift_value:expr, $base_value:expr),*) => {
            match $key {
                $( $pattern => if $shifted {
                        String::from($shift_value)
                    } else {
                        String::from($base_value)
                    },
                )*

                _ => String::from(""),
            }
        }
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

        fn key_map(key: &VirtualKeyCode, modifiers: &ModifiersState) -> String {
            let shifted = modifiers
                .bitand(ModifiersState::SHIFT)
                .eq(&ModifiersState::SHIFT);

            key_mapper!(key, shifted,
                VirtualKeyCode::Space | VirtualKeyCode::Tab, "\t", " ",

                // Numpad keys cannot be shifted
                VirtualKeyCode::Key1 | VirtualKeyCode::Numpad1, "!", "1",
                VirtualKeyCode::Key2 | VirtualKeyCode::Numpad2, "@", "2",
                VirtualKeyCode::Key3 | VirtualKeyCode::Numpad3, "#", "3",
                VirtualKeyCode::Key4 | VirtualKeyCode::Numpad4, "$", "4",
                VirtualKeyCode::Key5 | VirtualKeyCode::Numpad5, "%", "5",
                VirtualKeyCode::Key6 | VirtualKeyCode::Numpad6, "^", "6",
                VirtualKeyCode::Key7 | VirtualKeyCode::Numpad7, "&", "7",
                VirtualKeyCode::Key8 | VirtualKeyCode::Numpad8, "*", "8",
                VirtualKeyCode::Key9 | VirtualKeyCode::Numpad9, "(", "9",
                VirtualKeyCode::Key0 | VirtualKeyCode::Numpad0, ")", "0",

                VirtualKeyCode::A, "A", "a",
                VirtualKeyCode::B, "B", "b",
                VirtualKeyCode::C, "C", "c",
                VirtualKeyCode::D, "D", "d",
                VirtualKeyCode::E, "E", "e",
                VirtualKeyCode::F, "F", "f",
                VirtualKeyCode::G, "G", "g",
                VirtualKeyCode::H, "H", "h",
                VirtualKeyCode::I, "I", "i",
                VirtualKeyCode::J, "J", "j",
                VirtualKeyCode::K, "K", "k",
                VirtualKeyCode::L, "L", "l",
                VirtualKeyCode::M, "M", "m",
                VirtualKeyCode::N, "N", "n",
                VirtualKeyCode::O, "O", "o",
                VirtualKeyCode::P, "P", "p",
                VirtualKeyCode::Q, "Q", "q",
                VirtualKeyCode::R, "R", "r",
                VirtualKeyCode::S, "S", "s",
                VirtualKeyCode::T, "T", "t",
                VirtualKeyCode::U, "U", "u",
                VirtualKeyCode::V, "V", "v",
                VirtualKeyCode::W, "W", "w",
                VirtualKeyCode::X, "X", "x",
                VirtualKeyCode::Y, "Y", "y",
                VirtualKeyCode::Z, "Z", "z",

                VirtualKeyCode::Minus, "_", "-",
                VirtualKeyCode::Equals, "+", "=",
                VirtualKeyCode::LBracket, "{", "[",
                VirtualKeyCode::RBracket, "}", "]",
                VirtualKeyCode::Backslash, "|", "\\",
                VirtualKeyCode::Semicolon, ":", ";",
                VirtualKeyCode::Apostrophe, "\"", "'",
                VirtualKeyCode::Comma, "<", ",",
                VirtualKeyCode::Period, ">", ".",
                VirtualKeyCode::Slash, "?", "/"
            )
        }
    }
}
