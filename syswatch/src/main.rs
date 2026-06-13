mod app;
mod system;
mod ui;

use crossterm::event::{self, Event, KeyCode};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io::stdout;
use std::time::Duration;
use sysinfo::System;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    crossterm::terminal::enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    let mut app = app::App::new();
    let mut sys = System::new_all();
    system::update_stats(&mut app, &mut sys);
    loop {
        terminal.draw(|frame| ui::draw(frame, &app))?;
        if event::poll(Duration::from_millis(500))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    app.should_quit = true
                }
            }
        }
        system::update_stats(&mut app, &mut sys);
        if app.should_quit {
            break;
        }
    }

    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}
