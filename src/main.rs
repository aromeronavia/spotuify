mod entities;
mod client;
mod app;

use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, execute, event::{EnableMouseCapture, DisableMouseCapture, self, Event, KeyCode}};
use tokio::{self, time::Instant};
use dotenv;
use std::{io::{self, Stdout}, time::Duration};
use tui::{backend::CrosstermBackend, Terminal, widgets::{Block, Borders, List, ListItem, ListState}, style::{Style, Modifier}, layout::{Direction, Constraint, Layout}, Frame};

use entities::Playlist;
use client::SpotifyClient;
use app::App;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    dotenv::dotenv().ok();
    let spotify_client = SpotifyClient::new();
    spotify_client.authenticate().await;

    let playlists: Vec<Playlist> = get_playlists(&spotify_client).await;

    enable_raw_mode()?;
    let mut stdout: Stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend: CrosstermBackend<Stdout> = CrosstermBackend::new(stdout);
    let mut terminal: Terminal<CrosstermBackend<Stdout>> = Terminal::new(backend)?;

    let mut last_tick = Instant::now();

    let mut app = App::new(playlists);

    loop {
        terminal.draw(|f: &mut Frame<CrosstermBackend<Stdout>>| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(2)
                .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
                .split(f.size());

            let list_items: Vec<ListItem> = app.playlists.iter().map(
                |playlist: &Playlist| ListItem::new(playlist.name.as_str()
            )).collect::<Vec<ListItem>>();

            let block = Block::default().title("Playlists").borders(Borders::ALL);
            let playlists_ui: List = List::new(list_items)
                .block(block)
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol(">>");

            f.render_stateful_widget(playlists_ui, chunks[0], &mut app.playlists_ui_state);

            // if app.current_playlist.is_some() {
            //     let current_playlist = app.current_playlist.as_ref().unwrap();
            //     let songs = get_songs(&spotify_client, current_playlist.id.clone());

            //     let list_items: Vec<ListItem> = songs.iter().map(
            //         |song: &entities::Song| ListItem::new(song.name.as_str()
            //     )).collect::<Vec<ListItem>>();
            //     let songs_block = Block::default().title("Songs").borders(Borders::ALL);
            //     let songs_ui: List = List::new(list_items)
            //         .block(songs_block)
            //         .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            //         .highlight_symbol(">>");

            //     f.render_widget(songs_ui, chunks[1]);
            // }
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

async fn get_playlists(spotify_client: &SpotifyClient) -> Vec<Playlist> {
    let playlists: Vec<entities::Playlist> = spotify_client.get_my_playlists().await;
    playlists
}

async fn get_songs(spotify_client: &SpotifyClient, playlist_id: String) -> Vec<entities::Song> {
    let songs: Vec<entities::Song> = spotify_client.get_playlist_tracks(playlist_id).await;
    songs
}