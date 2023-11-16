use winit::keyboard::{KeyCode, ModifiersState};
use std::ops::BitAnd;
mod char_string;
mod console;
pub use char_string::CharString;
pub use console::Console;

const MAX_LINE: usize = 256;

fn key_map(key: &KeyCode, modifiers: &ModifiersState) -> char {
    let shifted = modifiers
        .bitand(ModifiersState::SHIFT)
        .eq(&ModifiersState::SHIFT);

    match key {
        KeyCode::Space | KeyCode::Tab => if shifted { '\t' } else { ' ' },

        // Numpad keys cannot be shifted
        KeyCode::Digit1 | KeyCode::Numpad1 => if shifted { '!' } else { '1' },
        KeyCode::Digit2 | KeyCode::Numpad2 => if shifted { '@' } else { '2' },
        KeyCode::Digit3 | KeyCode::Numpad3 => if shifted { '#' } else { '3' },
        KeyCode::Digit4 | KeyCode::Numpad4 => if shifted { '$' } else { '4' },
        KeyCode::Digit5 | KeyCode::Numpad5 => if shifted { '%' } else { '5' },
        KeyCode::Digit6 | KeyCode::Numpad6 => if shifted { '^' } else { '6' },
        KeyCode::Digit7 | KeyCode::Numpad7 => if shifted { '&' } else { '7' },
        KeyCode::Digit8 | KeyCode::Numpad8 => if shifted { '*' } else { '8' },
        KeyCode::Digit9 | KeyCode::Numpad9 => if shifted { '(' } else { '9' },
        KeyCode::Digit0 | KeyCode::Numpad0 => if shifted { ')' } else { '0' },

        KeyCode::KeyA => if shifted { 'A' } else { 'a' },
        KeyCode::KeyB => if shifted { 'B' } else { 'b' },
        KeyCode::KeyC => if shifted { 'C' } else { 'c' },
        KeyCode::KeyD => if shifted { 'D' } else { 'd' },
        KeyCode::KeyE => if shifted { 'E' } else { 'e' },
        KeyCode::KeyF => if shifted { 'F' } else { 'f' },
        KeyCode::KeyG => if shifted { 'G' } else { 'g' },
        KeyCode::KeyH => if shifted { 'H' } else { 'h' },
        KeyCode::KeyI => if shifted { 'I' } else { 'i' },
        KeyCode::KeyJ => if shifted { 'J' } else { 'j' },
        KeyCode::KeyK => if shifted { 'K' } else { 'k' },
        KeyCode::KeyL => if shifted { 'L' } else { 'l' },
        KeyCode::KeyM => if shifted { 'M' } else { 'm' },
        KeyCode::KeyN => if shifted { 'N' } else { 'n' },
        KeyCode::KeyO => if shifted { 'O' } else { 'o' },
        KeyCode::KeyP => if shifted { 'P' } else { 'p' },
        KeyCode::KeyQ => if shifted { 'Q' } else { 'q' },
        KeyCode::KeyR => if shifted { 'R' } else { 'r' },
        KeyCode::KeyS => if shifted { 'S' } else { 's' },
        KeyCode::KeyT => if shifted { 'T' } else { 't' },
        KeyCode::KeyU => if shifted { 'U' } else { 'u' },
        KeyCode::KeyV => if shifted { 'V' } else { 'v' },
        KeyCode::KeyW => if shifted { 'W' } else { 'w' },
        KeyCode::KeyX => if shifted { 'X' } else { 'x' },
        KeyCode::KeyY => if shifted { 'Y' } else { 'y' },
        KeyCode::KeyZ => if shifted { 'Z' } else { 'z' },

        KeyCode::Backquote => if shifted { '~' } else { '`' },
        KeyCode::Minus => if shifted { '_' } else { '-' },
        KeyCode::Equal => if shifted { '+' } else { '=' },
        KeyCode::BracketLeft => if shifted { '{' } else { '[' },
        KeyCode::BracketRight => if shifted { '}' } else { ']' },
        KeyCode::Backslash => if shifted { '|' } else { '\\' },
        KeyCode::Semicolon => if shifted { ':' } else { ';' },
        KeyCode::Quote => if shifted { '"' } else { '\'' },
        KeyCode::Comma => if shifted { '<' } else { ',' },
        KeyCode::Period => if shifted { '>' } else { '.' },
        KeyCode::Slash => if shifted { '?' } else { '/' },

        _ => '\0',
    }
}

fn tex_map(ch: char) -> [f32; 4] {
    let thirteenth = 0.07692308;
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

