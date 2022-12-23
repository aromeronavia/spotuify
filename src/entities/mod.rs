use std::fmt::{Debug, Formatter};

pub struct Song {
    pub artist: String,
    pub title: String,
    pub duration: u64,
}

impl Song {
    pub fn new(artist: &str, title: &str, duration: u64) -> Song {
        Song {
            artist: artist.to_string(),
            title: title.to_string(),
            duration: duration,
        }
    }
}

impl Debug for Song {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Song")
            .field("artist", &self.artist)
            .field("title", &self.title)
            .field("duration", &self.duration)
            .finish()
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
#[derive(Clone, PartialEq, Eq, Default)]
pub struct Playlist {
    pub id: String,
    pub name: String,
}

impl Playlist {
    pub fn new(id: &str, name: &str) -> Playlist {
        Playlist {
            id: id.to_string(),
            name: name.to_string(),
        }
    }
}

impl Debug for Playlist {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Playlist")
            .field("id", &self.id)
            .field("name", &self.name)
            .finish()
    }
}
