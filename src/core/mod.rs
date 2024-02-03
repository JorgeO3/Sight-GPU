pub mod app;
pub mod event;
pub mod handler;
pub mod tui;
pub mod ui;

use std::io;

use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use crate::core::app::App;
use crate::core::event::EventHandler;
use crate::prelude::*;

pub fn run() -> Result<()> {
    // Initialize the application.
    let mut app = App::new();

    // Initialize the terminal.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = tui::Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop
    while app.running {
        tui.draw(&mut app)?;
        use event::Event::*;

        match tui.events.next()? {
            Tick => app.tick(),
            Key(_) => todo!(),
            Mouse(_) => todo!(),
            Resize(_, _) => todo!(),
        }
    }

    // Exit the application.
    tui.exit()?;
    Ok(())
}
