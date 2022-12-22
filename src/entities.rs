use std::fmt::{Formatter, Debug};

pub struct Song {
  pub artist: String,
  pub title: String,
  pub album: String,
  pub duration: u64,
}

impl Song {
  pub fn new(artist: &str, title: &str, album: &str, duration: u64) -> Song {
    Song {
      artist: artist.to_string(),
      title: title.to_string(),
      album: album.to_string(),
      duration: duration,
    }
  }
}

impl Debug for Song {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Song")
      .field("artist", &self.artist)
      .field("title", &self.title)
      .field("album", &self.album)
      .field("duration", &self.duration)
      .finish()
  }
}

pub struct Artist {
  pub name: String,
  pub songs: Vec<Song>,
}

impl Artist {
  pub fn new(name: &str) -> Artist {
    Artist {
      name: name.to_string(),
      songs: Vec::new(),
    }
  }
}

pub struct Album {
  pub name: String,
  pub songs: Vec<Song>,
}

impl Debug for Album {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Album")
      .field("name", &self.name)
      .field("songs", &self.songs)
      .finish()
  }
}

impl Album {
  pub fn new(name: &str, songs: Vec<Song>) -> Album {
    Album {
      name: name.to_string(),
      songs,
    }
  }
}

pub struct Playlist {
  pub name: String,
  pub songs: Vec<Song>,
}

impl Playlist {
  pub fn new(name: &str) -> Playlist {
    Playlist {
      name: name.to_string(),
      songs: Vec::new(),
    }
  }
}
