use ratatui::widgets::Paragraph;
use ratatui::{backend::Backend, Frame};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem},
};

use crate::entities::Song;
use crate::{app::App, entities::Playlist};

pub fn render<B>(rect: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(2)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
        .split(rect.size());

    let block = Block::default().title("Playlists").borders(Borders::ALL);
    let playlists_ui: List = List::new(
        app.playlists
            .iter()
            .map(|playlist: &Playlist| ListItem::new(playlist.name.as_str()))
            .collect::<Vec<ListItem>>(),
    )
    .block(block)
    .highlight_style(Style::default().add_modifier(Modifier::BOLD))
    .highlight_symbol(">>");

    rect.render_stateful_widget(playlists_ui, chunks[0], &mut app.playlists_ui_state);

    if app.current_playlist.is_some() {
        let current_playlist_name = app.current_playlist.as_ref().unwrap().name.as_str();
        let playlist_block = Block::default()
            .title(format!("Current Playlist: {current_playlist_name}"))
            .borders(Borders::ALL);

        let songs: &Vec<Song> = app.playlist_songs.as_ref().unwrap();
        let songs = songs.iter()
            .map(|song| ListItem::new(song.title.as_str()))
            .collect::<Vec<ListItem>>();
        let songs_ui: List = List::new(songs.clone())
            .block(playlist_block);

        rect.render_widget(songs_ui, chunks[1]);
    }
}
