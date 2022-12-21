use rspotify::{model::AlbumId, prelude::*, ClientCredsSpotify, Credentials};
use tokio;
mod entities;
use dotenv;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    get_album().await;
}

async fn get_album() {
    let creds = Credentials::from_env().unwrap();
    let spotify = ClientCredsSpotify::new(creds);

    spotify.request_token().await.unwrap();

    let birdy_uri = AlbumId::from_uri("spotify:album:0sNOF9WDwhWunNAHPD3Baj").unwrap();
    let albums = spotify.album(birdy_uri).await;

    println!("Response: {albums:#?}");
}