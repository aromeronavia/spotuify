pub struct Song {
  pub artist: String,
  pub title: String,
  pub album: String,
  pub duration: u32,
}

impl Song {
  pub fn new(artist: &str, title: &str, album: &str, duration: u32) -> Song {
    Song {
      artist: artist.to_string(),
      title: title.to_string(),
      album: album.to_string(),
      duration: duration,
    }
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

impl Album {
  pub fn new(name: &str) -> Album {
    Album {
      name: name.to_string(),
      songs: Vec::new(),
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
