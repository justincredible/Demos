pub mod text {
    use crate::glutin::event::{ElementState, KeyboardInput, ModifiersState, VirtualKeyCode};
    use std::ops::BitAnd;

    const MAX_LINE: usize = 256;

    struct KeyBuffer {
        index: usize,
        keys: [KeyboardInput; MAX_LINE],
    }

    pub struct Console {
        buffer: KeyBuffer,
    }

    impl Console {
        #![allow(deprecated)]
        pub fn new() -> Self {
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

            match key {
                VirtualKeyCode::Space => String::from(" "),

                // Numpad keys cannot be shifted
                VirtualKeyCode::Key1 | VirtualKeyCode::Numpad1 => if shifted {
                        String::from("!")
                    } else {
                        String::from("1")
                    },
                VirtualKeyCode::Key2 | VirtualKeyCode::Numpad2 => if shifted {
                        String::from("@")
                    } else {
                        String::from("2")
                    },
                VirtualKeyCode::Key3 | VirtualKeyCode::Numpad3 => if shifted {
                        String::from("#")
                    } else {
                        String::from("3")
                    },
                VirtualKeyCode::Key4 | VirtualKeyCode::Numpad4 => if shifted {
                        String::from("$")
                    } else {
                        String::from("4")
                    },
                VirtualKeyCode::Key5 | VirtualKeyCode::Numpad5 => if shifted {
                        String::from("%")
                    } else {
                        String::from("5")
                    },
                VirtualKeyCode::Key6 | VirtualKeyCode::Numpad6 => if shifted {
                        String::from("^")
                    } else {
                        String::from("6")
                    },
                VirtualKeyCode::Key7 | VirtualKeyCode::Numpad7 => if shifted {
                        String::from("&")
                    } else {
                        String::from("7")
                    },
                VirtualKeyCode::Key8 | VirtualKeyCode::Numpad8 => if shifted {
                        String::from("*")
                    } else {
                        String::from("8")
                    },
                VirtualKeyCode::Key9 | VirtualKeyCode::Numpad9 => if shifted {
                        String::from("(")
                    } else {
                        String::from("9")
                    },
                VirtualKeyCode::Key0 | VirtualKeyCode::Numpad0 => if shifted {
                        String::from(")")
                    } else {
                        String::from("0")
                    },

                VirtualKeyCode::A => if shifted {
                        String::from("A")
                    } else {
                        String::from("a")
                    },
                VirtualKeyCode::B => if shifted {
                        String::from("B")
                    } else {
                        String::from("b")
                    },
                VirtualKeyCode::C => if shifted {
                        String::from("C")
                    } else {
                        String::from("c")
                    },
                VirtualKeyCode::D => if shifted {
                        String::from("D")
                    } else {
                        String::from("d")
                    },
                VirtualKeyCode::E => if shifted {
                        String::from("E")
                    } else {
                        String::from("e")
                    },
                VirtualKeyCode::F => if shifted {
                        String::from("F")
                    } else {
                        String::from("f")
                    },
                VirtualKeyCode::G => if shifted {
                        String::from("G")
                    } else {
                        String::from("g")
                    },
                VirtualKeyCode::H => if shifted {
                        String::from("H")
                    } else {
                        String::from("h")
                    },
                VirtualKeyCode::I => if shifted {
                        String::from("I")
                    } else {
                        String::from("i")
                    },
                VirtualKeyCode::J => if shifted {
                        String::from("J")
                    } else {
                        String::from("j")
                    },
                VirtualKeyCode::K => if shifted {
                        String::from("K")
                    } else {
                        String::from("k")
                    },
                VirtualKeyCode::L => if shifted {
                        String::from("L")
                    } else {
                        String::from("l")
                    },
                VirtualKeyCode::M => if shifted {
                        String::from("M")
                    } else {
                        String::from("m")
                    },
                VirtualKeyCode::N => if shifted {
                        String::from("N")
                    } else {
                        String::from("n")
                    },
                VirtualKeyCode::O => if shifted {
                        String::from("O")
                    } else {
                        String::from("o")
                    },
                VirtualKeyCode::P => if shifted {
                        String::from("P")
                    } else {
                        String::from("p")
                    },
                VirtualKeyCode::Q => if shifted {
                        String::from("Q")
                    } else {
                        String::from("q")
                    },
                VirtualKeyCode::R => if shifted {
                        String::from("R")
                    } else {
                        String::from("r")
                    },
                VirtualKeyCode::S => if shifted {
                        String::from("S")
                    } else {
                        String::from("s")
                    },
                VirtualKeyCode::T => if shifted {
                        String::from("T")
                    } else {
                        String::from("t")
                    },
                VirtualKeyCode::U => if shifted {
                        String::from("U")
                    } else {
                        String::from("u")
                    },
                VirtualKeyCode::V => if shifted {
                        String::from("V")
                    } else {
                        String::from("v")
                    },
                VirtualKeyCode::W => if shifted {
                        String::from("W")
                    } else {
                        String::from("w")
                    },
                VirtualKeyCode::X => if shifted {
                        String::from("X")
                    } else {
                        String::from("x")
                    },
                VirtualKeyCode::Y => if shifted {
                        String::from("Y")
                    } else {
                        String::from("y")
                    },
                VirtualKeyCode::Z => if shifted {
                        String::from("Z")
                    } else {
                        String::from("z")
                    },

                VirtualKeyCode::Minus => if shifted {
                        String::from("_")
                    } else {
                        String::from("-")
                    },
                VirtualKeyCode::Equals => if shifted {
                        String::from("+")
                    } else {
                        String::from("=")
                    },
                VirtualKeyCode::LBracket => if shifted {
                        String::from("{")
                    } else {
                        String::from("[")
                    },
                VirtualKeyCode::RBracket => if shifted {
                        String::from("}")
                    } else {
                        String::from("]")
                    },
                VirtualKeyCode::Backslash => if shifted {
                        String::from("|")
                    } else {
                        String::from("\\")
                    },
                VirtualKeyCode::Semicolon => if shifted {
                        String::from(":")
                    } else {
                        String::from(";")
                    },
                VirtualKeyCode::Apostrophe => if shifted {
                        String::from("\"")
                    } else {
                        String::from("'")
                    },
                VirtualKeyCode::Comma => if shifted {
                        String::from("<")
                    } else {
                        String::from(",")
                    },
                VirtualKeyCode::Period => if shifted {
                        String::from(">")
                    } else {
                        String::from(".")
                    },
                VirtualKeyCode::Slash => if shifted {
                        String::from("?")
                    } else {
                        String::from("/")
                    },

                _ => String::from(""),
            }
        }
    }
}
