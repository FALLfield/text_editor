mod app;
mod ui;


use std::error::Error;
use std::io;
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind,
    },
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::Terminal;

use crate::{
    app::{App, CurrentlyEditing, CurrentScreen},
};

fn main() -> Result<(), Box<dyn Error>> {

    //set up terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    //Create an app
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    if let Ok(do_print) = res {
        if do_print {
            app.write_txt()?;
        }
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())

}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<bool> {
    loop {

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Release {
                continue;
            }

            match app.current_screen {

            }
        }


    }
}