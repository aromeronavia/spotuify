use std::sync::Arc;

use rspotify::{model::{AlbumId, FullAlbum}, prelude::*, ClientCredsSpotify, Credentials};
use tokio;
use dotenv;
use anyhow::Result;

mod entities;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    get_album().await;
}

async fn get_album() {
    let album_uri = "spotify:album:0sNOF9WDwhWunNAHPD3Baj";

    let spotify_client = SpotifyClient::new();
    spotify_client.authenticate().await;

    let album = spotify_client.get_album(album_uri).await.unwrap();

    println!("Response: {:?}", album.songs);
}

pub struct SpotifyClient {
    client: ClientCredsSpotify,
}

impl SpotifyClient {
    pub fn new() -> Self {
        let creds = Credentials::from_env().unwrap();
        let client = ClientCredsSpotify::new(creds);

        Self { client }
    }

    pub async fn authenticate(&self) {
        self.client.request_token().await.unwrap();
    }

    pub async fn get_album(&self, album_id: &str) -> Result<entities::Album> {
        let album_id = AlbumId::from_uri(album_id).unwrap();
        let wrapped_album = self.client.album(album_id).await;
        let album = wrapped_album.unwrap().clone();

        Ok(
            entities::Album::new(
                &album.name,
                self.parse_spotify_songs(album.clone())
            )
        )
    }

    fn parse_spotify_songs(&self, album: FullAlbum) -> Vec<entities::Song> {
        let mut songs = Vec::new();

        for track in album.tracks.items {
            songs.push(
                entities::Song::new(
                    &track.artists.iter().map(|artist| artist.name.clone()).collect::<Vec<String>>().join(", "),
                    &track.name,
                    &album.name,
                    track.duration.as_secs(),
                )
            );
        }

        songs
    }
}