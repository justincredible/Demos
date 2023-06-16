pub mod text {
    use crate::glutin::event::{ElementState, KeyboardInput, ModifiersState, VirtualKeyCode};

    const MAX_LINE: usize = 256;

    struct KeyBuffer {
        index: usize,
        keys: [KeyboardInput; MAX_LINE],
    }

    pub struct Console {
        buffer: KeyBuffer,
    }

    impl Console {
        pub fn new() -> Self {
            #![allow(deprecated)]
            Console {
                buffer: KeyBuffer {
                    index: 0,
                    keys: [ KeyboardInput {
                            scancode: 0,
                            state: ElementState::Pressed,
                            virtual_keycode: None,
                            modifiers: ModifiersState::empty(),
                        }; MAX_LINE
                    ]
                }
            }
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
                    ..
                } => {
                    if self.buffer.index == MAX_LINE {
                        self.flush();
                    }
                    self.buffer.keys[self.buffer.index] = input;
                    self.buffer.index += 1
                },

                _ => (),
            }
        }

        fn flush(&mut self) {
            for key in &self.buffer.keys[0..self.buffer.index] {
                match key {
                    KeyboardInput {
                        virtual_keycode: Some(virtual_keycode),
                        modifiers,
                        ..
                    //} => print!("\n{:?}\n{:?}\n", virtual_keycode, modifiers),
                    } => print!("{}", Self::key_map(virtual_keycode, modifiers)),

                    _ => (),
                }
            }

            self.buffer.index = 0;
            println!("");
        }

        fn key_map(key: &VirtualKeyCode, modifiers: &ModifiersState) -> String {
            if key.lt(&VirtualKeyCode::Key1) || key.gt(&VirtualKeyCode::Z) {
                "".to_string()
            } else {
                match key {
                    VirtualKeyCode::Key1 => String::from("1"),
                    VirtualKeyCode::Key2 => String::from("2"),
                    VirtualKeyCode::Key3 => String::from("3"),
                    VirtualKeyCode::Key4 => String::from("4"),
                    VirtualKeyCode::Key5 => String::from("5"),
                    VirtualKeyCode::Key6 => String::from("6"),
                    VirtualKeyCode::Key7 => String::from("7"),
                    VirtualKeyCode::Key8 => String::from("8"),
                    VirtualKeyCode::Key9 => String::from("9"),
                    VirtualKeyCode::Key0 => String::from("0"),
                    _ => format!("{:?}", key),
                }
            }
        }
    }
}
