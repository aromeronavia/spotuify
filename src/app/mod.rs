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
}