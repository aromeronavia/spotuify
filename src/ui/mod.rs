mod render;

use anyhow::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io::{self, Stdout},
    sync::Arc,
    time::Duration,
};
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::{
    app::App,
    inputs::{Event, Events, Key},
};
use render::render;

pub async fn start_ui(app: &Arc<tokio::sync::Mutex<App>>) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let tick_rate = Duration::from_millis(200);
    let events = Events::new(tick_rate);

    loop {
        let mut app = app.lock().await;

        terminal.draw(|rect| render(rect, &mut app))?;

        match events.next()? {
            Event::Input(key) => match key {
                Key::Exit => {
                    exit_terminal(&mut terminal)?;
                    std::process::exit(0);
                }
                _ => app.do_action(key).await,
            },
            Event::Tick => {
                app.update_on_tick().await;
            }
        }
    }
}

fn exit_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
