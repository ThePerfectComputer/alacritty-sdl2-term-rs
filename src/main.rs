#![allow(non_camel_case_types)]

use alacritty_terminal::event::Notify;
use sdl2::event::EventPollIterator;
use TermDisplay::Update;

mod ATerm;
mod Matrix;
mod TermDisplay;
mod TerminalSize;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::cell::RefCell;

// use once_cell::sync::Lazy;
use lazy_static::lazy_static;
use std::sync::Mutex;

// Use thread_local for single-threaded mutable access
thread_local! {
    static TOGGLE: RefCell<bool> = RefCell::new(false);
}

// Use thread_local for single-threaded mutable access
lazy_static! {
    static ref ATERM: Mutex<ATerm::ATerm> =
        Mutex::new(ATerm::ATerm::new().expect("Failed to initialize ATerm"));
}

fn convert_keycode(keycode: Keycode) -> Option<char> {
    match keycode {
        // Alphabet Keys
        Keycode::A => Some('A'),
        Keycode::B => Some('B'),
        Keycode::C => Some('C'),
        Keycode::D => Some('D'),
        Keycode::E => Some('E'),
        Keycode::F => Some('F'),
        Keycode::G => Some('G'),
        Keycode::H => Some('H'),
        Keycode::I => Some('I'),
        Keycode::J => Some('J'),
        Keycode::K => Some('K'),
        Keycode::L => Some('L'),
        Keycode::M => Some('M'),
        Keycode::N => Some('N'),
        Keycode::O => Some('O'),
        Keycode::P => Some('P'),
        Keycode::Q => Some('Q'),
        Keycode::R => Some('R'),
        Keycode::S => Some('S'),
        Keycode::T => Some('T'),
        Keycode::U => Some('U'),
        Keycode::V => Some('V'),
        Keycode::W => Some('W'),
        Keycode::X => Some('X'),
        Keycode::Y => Some('Y'),
        Keycode::Z => Some('Z'),

        // Keypad Digits
        Keycode::KP_0 => Some('0'),
        Keycode::KP_1 => Some('1'),
        Keycode::KP_2 => Some('2'),
        Keycode::KP_3 => Some('3'),
        Keycode::KP_4 => Some('4'),
        Keycode::KP_5 => Some('5'),
        Keycode::KP_6 => Some('6'),
        Keycode::KP_7 => Some('7'),
        Keycode::KP_8 => Some('8'),
        Keycode::KP_9 => Some('9'),

        // Fallback for unmatched keycodes
        _ => None,
    }
}

fn check_for_term_update() -> Update {
    let aterm = ATERM.lock().unwrap();
    let has_updates = aterm.rx.try_recv().is_ok();

    if has_updates {
        let mut matrix = Matrix::Matrix::new(24, 80);
        matrix.populate_from_aterm(&aterm);
        Update::MatrixContent(matrix)
    } else {
        Update::Nothing
    }
}

fn update_loop(events: EventPollIterator) -> Update {
    let mut update = Update::Nothing;

    let collected_events: Vec<Event> = events.collect();

    // for event in &collected_events {
    //     dbg!(event);
    // }

    for event in collected_events {
        match event {
            Event::Quit { .. } => {
                update = Update::Exit;
            }
            Event::KeyDown {
                keycode: Some(Keycode::KpEnter),
                ..
            }
            | Event::KeyDown {
                keycode: Some(Keycode::KP_ENTER),
                ..
            }
            | Event::KeyDown {
                keycode: Some(Keycode::Return),
                ..
            } => {
                let mut aterm = ATERM.lock().unwrap();
                aterm.tx.notify("\n".to_string().into_bytes());
                let mut matrix = Matrix::Matrix::new(24, 80);
                matrix.populate_from_aterm(&aterm);
                update = Update::MatrixContent(matrix);
                break;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Backspace),
                ..
            } => {
                let backspace = "\x08".to_string();
                let mut aterm = ATERM.lock().unwrap();
                aterm.tx.notify(backspace.into_bytes());
                let mut matrix = Matrix::Matrix::new(24, 80);
                matrix.populate_from_aterm(&aterm);
                update = Update::MatrixContent(matrix);
                break;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                let backspace = "\x1B".to_string();
                let mut aterm = ATERM.lock().unwrap();
                aterm.tx.notify(backspace.into_bytes());
                let mut matrix = Matrix::Matrix::new(24, 80);
                matrix.populate_from_aterm(&aterm);
                update = Update::MatrixContent(matrix);
                break;
            }
            Event::TextInput { text, .. } => {
                let mut aterm = ATERM.lock().unwrap();
                aterm.tx.notify(text.into_bytes());
                let mut matrix = Matrix::Matrix::new(24, 80);
                matrix.populate_from_aterm(&aterm);
                update = Update::MatrixContent(matrix);
                break;
            }
            Event::KeyDown {
                keycode: Some(key),
                keymod,
                ..
            } => {
                if keymod.contains(sdl2::keyboard::Mod::LCTRLMOD) {
                    if let Some(key_char) = convert_keycode(key) {
                        if key_char >= 'A' && key_char <= 'Z' {
                            let ctrl_char = ((key_char as u8) - ('A') as u8) + 1;
                            let ctrl_char = (ctrl_char as char).to_string();
                            let mut aterm = ATERM.lock().unwrap();
                            aterm.tx.notify(ctrl_char.into_bytes());
                            let mut matrix = Matrix::Matrix::new(24, 80);
                            matrix.populate_from_aterm(&aterm);
                            update = Update::MatrixContent(matrix);
                            break;
                        }
                    }
                }
            }
            _ => {
                update = Update::Nothing;
            }
        }
    }

    update
}

fn main() -> Result<(), String> {
    let mut term_display = TermDisplay::TermDisplay::new()?;
    // let mut term = ATerm::ATerm::new();
    term_display.update_loop(update_loop, check_for_term_update)?;
    Ok(())
}
