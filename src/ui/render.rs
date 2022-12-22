use tui::{Frame, backend::Backend};
use std::{io::{self, Stdout}, time::Duration, rc::Rc, cell::RefCell, borrow::Borrow};
use tui::{backend::CrosstermBackend, Terminal, widgets::{Block, Borders, List, ListItem}, style::{Style, Modifier}, layout::{Direction, Constraint, Layout}};

use crate::{entities::Playlist, app::App};

pub fn render<B>(rect: &mut Frame<B>, app: &mut App)
  where B: Backend {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(2)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
        .split(rect.size());

    let list_items: Vec<ListItem> = app.playlists.iter().map(
        |playlist: &Playlist| ListItem::new(playlist.name.as_str()
    )).collect::<Vec<ListItem>>();

    let block = Block::default().title("Playlists").borders(Borders::ALL);
    let playlists_ui: List = List::new(list_items)
        .block(block)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">>");

    rect.render_stateful_widget(playlists_ui, chunks[0], &mut app.playlists_ui_state);

    // if app.current_playlist.is_some() {
    //     let current_playlist = app.current_playlist.as_ref().unwrap();
    //     let songs = get_songs(&spotify_client, current_playlist.id.clone());

    //     let list_items: Vec<ListItem> = songs.iter().map(
    //         |song: &entities::Song| ListItem::new(song.name.as_str()
    //     )).collect::<Vec<ListItem>>();
    //     let songs_block = Block::default().title("Songs").borders(Borders::ALL);
    //     let songs_ui: List = List::new(list_items)
    //         .block(songs_block)
    //         .highlight_style(Style::default().add_modifier(Modifier::BOLD))
    //         .highlight_symbol(">>");

    //     f.render_widget(songs_ui, chunks[1]);
    // }
}