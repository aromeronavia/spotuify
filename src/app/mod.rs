use ratatui::widgets::ListState;

use crate::{entities::{Playlist, Song}, inputs::Key, client::SpotifyClient};

pub struct App {
    pub playlists: Vec<Playlist>,
    pub playlists_ui_state: ListState,
    pub spotify_client: SpotifyClient,
    pub current_playlist: Option<Playlist>,
    pub playlist_songs: Option<Vec<Song>>,
}

impl App {
    pub fn new(playlists: Vec<Playlist>, spotify_client: SpotifyClient) -> Self {
        let mut playlists_ui_state = ListState::default();
        playlists_ui_state.select(Some(0));

        Self {
            playlists,
            playlists_ui_state,
            spotify_client,
            current_playlist: None,
            playlist_songs: None,
        }
    }

    pub async fn do_action(&mut self, key: Key) {
        match key {
            Key::Exit => {
                std::process::exit(0);
            }
            Key::Down => self.next(),
            Key::Up => self.previous(),
            Key::Enter => {
                let selected_playlist = self
                    .playlists
                    .get(self.playlists_ui_state.selected().unwrap())
                    .unwrap();

                self.current_playlist = Some(selected_playlist.clone());
                self.playlist_songs = Some(
                    self.spotify_client
                        .get_playlist_songs(selected_playlist.id.clone())
                        .await,
                );
            }
            _ => {}
        }
    }

    pub fn next(&mut self) {
        let i = match self.playlists_ui_state.selected() {
            Some(i) => {
                if i >= self.playlists.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.playlists_ui_state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.playlists_ui_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.playlists.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.playlists_ui_state.select(Some(i));
    }

    pub async fn update_on_tick(&mut self) {

    }
}
