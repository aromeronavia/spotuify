mod entities;
mod client;

use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, execute, event::{EnableMouseCapture, DisableMouseCapture, self, Event, KeyCode}};
use tokio::{self, time::Instant};
use dotenv;
use std::{io::{self, Stdout}, time::Duration};
use tui::{backend::CrosstermBackend, Terminal, widgets::{Block, Borders, List, ListItem, ListState}, style::{Style, Modifier}};

use entities::Playlist;
use client::SpotifyClient;

struct App {
    playlists: Vec<Playlist>,
    playlists_ui_state: ListState,
    current_playlist: Option<Playlist>,
}

impl App {
    fn new(playlists: Vec<Playlist>) -> Self {
        let mut playlists_ui_state = ListState::default();
        playlists_ui_state.select(Some(0));

        Self {
            playlists,
            playlists_ui_state,
            current_playlist: None,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    dotenv::dotenv().ok();
    let playlists: Vec<Playlist> = get_playlists().await;

    enable_raw_mode()?;
    let mut stdout: Stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend: CrosstermBackend<Stdout> = CrosstermBackend::new(stdout);
    let mut terminal: Terminal<CrosstermBackend<Stdout>> = Terminal::new(backend)?;

    let mut last_tick = Instant::now();

    let mut app = App::new(playlists);

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let list_items: Vec<ListItem> = app.playlists.iter().map(
                |playlist: &Playlist| ListItem::new(playlist.name.as_str()
            )).collect::<Vec<ListItem>>();

            let block = Block::default().title("Playlists").borders(Borders::ALL);
            let list: List = List::new(list_items)
                .block(block)
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol(">>");

            f.render_stateful_widget(list, size, &mut app.playlists_ui_state);
        })?;

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

async fn get_playlists() -> Vec<entities::Playlist> {
    let spotify_client = SpotifyClient::new();
    spotify_client.authenticate().await;

    let playlists: Vec<entities::Playlist> = spotify_client.get_my_playlists().await;
    playlists
}
