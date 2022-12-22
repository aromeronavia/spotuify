mod entities;
mod client;
mod app;
mod ui;

use std::{io, rc::Rc, cell::RefCell};

use dotenv;

use entities::Playlist;
use client::SpotifyClient;
use app::App;
use ui::start_ui;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    dotenv::dotenv().ok();
    let spotify_client = SpotifyClient::new();
    spotify_client.authenticate().await;

    let playlists: Vec<Playlist> = get_playlists(&spotify_client).await;

    let mut app = Rc::new(RefCell::new(App::new(playlists)));
    start_ui(app)
}

async fn get_playlists(spotify_client: &SpotifyClient) -> Vec<Playlist> {
    let playlists: Vec<entities::Playlist> = spotify_client.get_my_playlists().await;
    playlists
}

async fn get_songs(spotify_client: &SpotifyClient, playlist_id: String) -> Vec<entities::Song> {
    let songs: Vec<entities::Song> = spotify_client.get_playlist_tracks(playlist_id).await;
    songs
}