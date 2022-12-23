mod app;
mod client;
mod entities;
mod inputs;
mod ui;

use anyhow::Result;
use app::App;
use client::SpotifyClient;
use dotenv;
use entities::Playlist;
use std::sync::Arc;
use ui::start_ui;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let spotify_client = SpotifyClient::new();
    spotify_client.authenticate().await;

    let playlists: Vec<Playlist> = get_playlists(&spotify_client).await;

    let app = Arc::new(tokio::sync::Mutex::new(App::new(playlists)));
    start_ui(&app).await?;

    Ok(())
}

async fn get_playlists(spotify_client: &SpotifyClient) -> Vec<Playlist> {
    let playlists: Vec<entities::Playlist> = spotify_client.get_my_playlists().await;
    playlists
}
