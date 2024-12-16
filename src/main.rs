#![allow(non_camel_case_types)]

use std::sync::OnceLock;

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

// Use thread_local for single-threaded mutable access
thread_local! {
    static TOGGLE: RefCell<bool> = RefCell::new(false);
}

fn update_loop(events: EventPollIterator) -> Update {
    let mut update = Update::Nothing;

    for event in events {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                update = Update::Exit;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Space),
                ..
            } => {
                update = TOGGLE.with(|toggle| {
                    let mut toggle_ref = toggle.borrow_mut();
                    *toggle_ref = !*toggle_ref;

                    let mut matrix = Matrix::Matrix::new(24, 80);
                    if *toggle_ref {
                        matrix.set_to_content2();
                    } else {
                        matrix.set_to_content1();
                    }

                    Update::MatrixContent(matrix)
                });
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
    term_display.update_loop(update_loop)?;
    Ok(())
}