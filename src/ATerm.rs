use std::result;
use std::sync::{mpsc, Arc};

use alacritty_terminal::event::{Event, EventListener, WindowSize};
use alacritty_terminal::event_loop::{EventLoop, Notifier};
use alacritty_terminal::sync::FairMutex;
use alacritty_terminal::term::{self, Term};
use alacritty_terminal::tty;

use crate::TerminalSize;

#[derive(Clone)]
pub struct EventProxy(mpsc::Sender<Event>);
impl EventListener for EventProxy {
    fn send_event(&self, event: Event) {
        let _ = self.0.send(event.clone());
    }
}

pub struct ATerm {
    pub term: Arc<FairMutex<Term<EventProxy>>>,

    /// Use tx to write things to terminal instance from outside world
    pub tx: Notifier,

    /// Use rx to read things from terminal instance.
    /// Rx only has data when terminal state has changed,
    /// otherwise, `std::sync::mpsc::recv` will block and sleep
    /// until there is data.
    pub rx: mpsc::Receiver<(u64, Event)>,
}

impl ATerm {
    pub fn new() -> result::Result<ATerm, std::io::Error> {
        let id = 1;
        let pty_config = tty::Options {
            shell: Some(tty::Shell::new("/bin/bash".to_string(), vec![])),
            ..tty::Options::default()
        };
        let config = term::Config::default();
        let window_size = WindowSize {
            num_lines: 80,
            num_cols: 24,
            cell_width: 8,
            cell_height: 16,
        };
        let terminal_size = TerminalSize::TerminalSize::default();
        let pty = tty::new(&pty_config, terminal_size.into(), id)?;
        let (event_sender, event_receiver) = mpsc::channel();
        let event_proxy = EventProxy(event_sender);
        let term = Term::new::<TerminalSize::TerminalSize>(
            config,
            &terminal_size.into(),
            event_proxy.clone(),
        );
        let term = Arc::new(FairMutex::new(term));
        let pty_event_loop = EventLoop::new(term.clone(), event_proxy, pty, false, false)?;
        let notifier = Notifier(pty_event_loop.channel());
        let (pty_proxy_sender, pty_proxy_receiver) = std::sync::mpsc::channel();
        let _pty_event_loop_thread = pty_event_loop.spawn();
        let _pty_event_subscription = std::thread::Builder::new()
            .name(format!("pty_event_subscription_{}", id))
            .spawn(move || loop {
                if let Ok(event) = event_receiver.recv() {
                    pty_proxy_sender
                        .send((id, event.clone()))
                        .unwrap_or_else(|_| {
                            panic!("pty_event_subscription_{}: sending PtyEvent is failed", id)
                        });
                    if let Event::Exit = event {
                        break;
                    }
                }
            })?;
        Ok(ATerm {
            term,
            tx: notifier,
            rx: pty_proxy_receiver,
        })
    }
}
