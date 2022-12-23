use tui::{backend::Backend, Frame};
use tui::{
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem},
};

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

    let list_items: Vec<ListItem> = app
        .playlists
        .iter()
        .map(|playlist: &Playlist| ListItem::new(playlist.name.as_str()))
        .collect::<Vec<ListItem>>();

    let block = Block::default().title("Playlists").borders(Borders::ALL);
    let playlists_ui: List = List::new(list_items)
        .block(block)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">>");

    rect.render_stateful_widget(playlists_ui, chunks[0], &mut app.playlists_ui_state);
}
