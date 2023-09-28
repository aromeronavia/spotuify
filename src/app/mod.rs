use ratatui::widgets::ListState;

use crate::{entities::Playlist, inputs::Key};

pub struct App {
    pub playlists: Vec<Playlist>,
    pub playlists_ui_state: ListState,
    pub current_playlist: Option<Playlist>,
}

impl App {
    pub fn new(playlists: Vec<Playlist>) -> Self {
        let mut playlists_ui_state = ListState::default();
        playlists_ui_state.select(Some(0));

        Self {
            playlists,
            playlists_ui_state,
            current_playlist: None,
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

    pub async fn update_on_tick(&mut self) {}
}
