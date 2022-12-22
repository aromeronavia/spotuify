mod render;

use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, execute, event::{EnableMouseCapture, DisableMouseCapture, self, Event, KeyCode}};
use tokio::{self, time::Instant};
use std::{io::{self, Stdout}, time::Duration, rc::Rc, cell::RefCell, borrow::Borrow};
use tui::{backend::CrosstermBackend, Terminal, widgets::{Block, Borders, List, ListItem}, style::{Style, Modifier}, layout::{Direction, Constraint, Layout}, Frame};

use crate::{app::App, entities::Playlist};
use render::render;

pub fn start_ui(app: Rc<RefCell<App>>) -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout: Stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend: CrosstermBackend<Stdout> = CrosstermBackend::new(stdout);
    let mut terminal: Terminal<CrosstermBackend<Stdout>> = Terminal::new(backend)?;

    let mut last_tick = Instant::now();
    let app = Rc::clone(&app);

    loop {
        let mut app = app.borrow_mut();
        
        // render
        terminal.draw(|rect| render(rect, &mut app))?;

        let tick_rate = Duration::from_millis(50);
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        break;
                    }
                    KeyCode::Char('c') if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                        break;
                    }
                    KeyCode::Char('j') => app.next(),
                    KeyCode::Char('k') => app.previous(),
                    KeyCode::Enter => {
                        let selected_playlist = app.playlists.get(app.playlists_ui_state.selected().unwrap()).unwrap();
                        app.current_playlist = Some(selected_playlist.clone());
                    }
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}