#![allow(non_camel_case_types)]

use std::sync::OnceLock;

use alacritty_terminal::event::Notify;
use alacritty_terminal::term;
use sdl2::event::EventPollIterator;
use TermDisplay::Update;

mod Matrix;
mod TestVars;
mod TermDisplay;
mod VTerm;
mod ATerm;
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
    static ref ATERM: Mutex<ATerm::ATerm> = Mutex::new(ATerm::ATerm::new().expect("Failed to initialize ATerm"));
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

    for event in events {
        match event {
            Event::Quit { .. } | Event::KeyDown {keycode: Some(Keycode::Escape),..} => {
                update = Update::Exit;
            }
            Event::KeyDown {keycode: Some(Keycode::KpEnter), ..}  |
            Event::KeyDown {keycode: Some(Keycode::KP_ENTER), ..} |
            Event::KeyDown {keycode: Some(Keycode::Return), ..}
            => {
                let mut aterm = ATERM.lock().unwrap();
                aterm.tx.notify("\n".to_string().into_bytes());
                let mut matrix = Matrix::Matrix::new(24, 80);
                matrix.populate_from_aterm(&aterm);
                update = Update::MatrixContent(matrix);
                break;
            }
            Event::TextInput {text, ..} => {
                let mut aterm = ATERM.lock().unwrap();
                aterm.tx.notify(text.into_bytes());
                let mut matrix = Matrix::Matrix::new(24, 80);
                matrix.populate_from_aterm(&aterm);
                update = Update::MatrixContent(matrix);
                break;
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