mod entities;
mod client;

use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, execute, event::{EnableMouseCapture, DisableMouseCapture}};
use tokio;
use dotenv;
use std::{io, thread, time::Duration};
use tui::{backend::CrosstermBackend, Terminal, widgets::{Block, Borders, List, ListItem}};

use entities::Playlist;
use client::SpotifyClient;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    dotenv::dotenv().ok();
    let playlists: Vec<Playlist> = get_playlists().await;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        let size = f.size();
        let list_items = playlists.iter().map(|playlist| 
            ListItem::new(playlist.name.as_str())
        ).collect::<Vec<ListItem>>();
        let list = List::new(list_items);

        f.render_widget(list, size);
    })?;

    thread::sleep(Duration::from_millis(5000));

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
