use rspotify::{model::{AlbumId, FullAlbum, PlaylistId, FullTrack, PlaylistItem, PlayableItem, Page}, prelude::*, Credentials, AuthCodeSpotify, OAuth, scopes};
use tokio;
use dotenv;
use anyhow::Result;

mod entities;

static PLAYLIST_TRACKS_QUERY: &str = "items(
    href,
    track(
        external_urls, 
        name,
        artists(
            external_urls
        ),
        album(
            name, 
            external_urls
        )
    ), 
)";

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    get_album().await;
}

async fn get_album() {
    let spotify_client = SpotifyClient::new();
    spotify_client.authenticate().await;

    let playlists = spotify_client.get_my_playlists().await;
    let tracks = spotify_client.get_playlist_tracks(playlists[10].id.clone()).await;
    println!("Tracks: {:?}", tracks);
}

pub struct SpotifyClient {
    client: AuthCodeSpotify,
}

impl SpotifyClient {
    pub fn new() -> Self {
        let creds = Credentials::from_env().unwrap();
        let oauth = OAuth::from_env(scopes!("user-follow-read user-follow-modify")).unwrap();
        let client = AuthCodeSpotify::new(creds.clone(), oauth.clone());

        Self { client }
    }

    pub async fn authenticate(&self) {
        let url = self.client.get_authorize_url(false).unwrap();
        self.client
            .prompt_for_token(url.as_str())
            .await
            .expect("Couldn't authenticate user")
    }

    pub async fn use_token(&self) {
        self.client.token.lock().await.unwrap();
    }

    pub async fn get_playlist_tracks<'a>(&self, playlist_id: String) -> Vec<entities::Song> {
        let playlist_id = PlaylistId::from_id_or_uri(&playlist_id).unwrap();
        let tracks = self.fetch_playlist_tracks(playlist_id).await;

        self.parse_items_to_songs(tracks)
    }

    async fn fetch_playlist_tracks<'a>(&self, playlist_id: PlaylistId<'a>) -> Page<PlaylistItem> {
        let limit: u32 = 50;
        let offset: u32 = 0;

        let page: Page<PlaylistItem> = self.client.playlist_items_manual(
            playlist_id, 
            Some(PLAYLIST_TRACKS_QUERY), None, Some(limit), Some(offset)).await.unwrap();

        page
    }

    pub async fn get_my_playlists(&self) -> Vec<entities::Playlist> {
        let limit: u32 = 50;
        let offset: u32 = 0;
        let playlists = self.client.current_user_playlists_manual(Some(limit), Some(offset)).await.unwrap();

        let mut parsed_playlists = Vec::new();
        for playlist in playlists.clone().items {
            parsed_playlists.push(
                entities::Playlist::new(
                    &playlist.id.to_string(),
                    &playlist.name,
                )
            );
        }

        parsed_playlists
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

    fn parse_items_to_songs(&self, page: Page<PlaylistItem>) -> Vec<entities::Song> {
        let mut songs: Vec<entities::Song> = Vec::new();

        for track in page.items {
            match track {
                PlaylistItem { added_at: _, added_by: _, is_local: _, track } => {
                    match track {
                        Some(PlayableItem::Track(full_track)) => {
                            let song = entities::Song::new(
                                &full_track.artists.iter().map(|artist| artist.name.clone()).collect::<Vec<String>>().join(", "),
                                &full_track.name,
                                full_track.duration.as_secs(),
                            );
                            songs.push(song);
                        },
                        _ => {}
                    }
                }
            }
        };

        songs
    }

    fn parse_spotify_songs(&self, album: FullAlbum) -> Vec<entities::Song> {
        let mut songs = Vec::new();

        for track in album.tracks.items {
            songs.push(
                entities::Song::new(
                    &track.artists.iter().map(|artist| artist.name.clone()).collect::<Vec<String>>().join(", "),
                    &track.name,
                    track.duration.as_secs(),
                )
            );
        }

        songs
    }
}