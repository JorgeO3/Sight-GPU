use ratatui::{backend::CrosstermBackend, Terminal};

use sight_gpu::core::{app, event, tui};
use sight_gpu::prelude::*;

fn main() -> Result<()> {
    // Initialize the application.
    let mut app = app::App::new();

    // Initialize the terminal.
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = event::EventHandler::new(250);
    let mut tui = tui::Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop
    while app.running {
        tui.draw(&mut app)?;
        use event::Event::*;

        match tui.events.next()? {
            Tick => app.tick(),
            Key(_k) => todo!(),
            Mouse(_) => {}
            Resize(_, _) => todo!(),
        }
    }

    // Exit the application.
    tui.exit()?;
    Ok(())
}
