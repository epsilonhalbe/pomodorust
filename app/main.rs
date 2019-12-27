use failure;
use pomodorust::config::Cfg;
use pomodorust::database::{create_table};
use pomodorust::events::{Event, Events};
use pomodorust::state::App;
use std::io;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{
    Constraint::Percentage,
    Direction::{Horizontal, Vertical},
    Layout,
};
use tui::widgets::{Block, Borders, Widget};
use tui::Terminal;

fn main() -> Result<(), failure::Error> {
    let cfg = Cfg::from_opts()?;
    create_table(&cfg.conn)?;

    let events = Events::new();
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new_with_cfg(&cfg);
    // Statistic::empty().insert(&conn)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    loop {
        terminal.draw(|mut f| {
            let size = f.size();

            Block::default().render(&mut f, size);

            let chunks = Layout::default()
                .direction(Vertical)
                .margin(0)
                .constraints([Percentage(75), Percentage(25)].as_ref())
                .split(size);

            Block::default()
                .borders(Borders::ALL)
                .render(&mut f, chunks[0]);
            {
                let chunks = Layout::default()
                    .direction(Horizontal)
                    .margin(1)
                    .constraints([Percentage(50), Percentage(50)].as_ref())
                    .split(chunks[0]);

                app.paragraph(&cfg, &mut f, chunks[0]);
                cfg.paragraph(&mut f, chunks[1]);
            }
            {
                app.gauge(&cfg, &mut f, chunks[1]);
            }
        })?;

        match events.next()? {
            Event::Input(key) => {
                if app.quit_or_pause(key, &cfg) {
                    break;
                }
            }
            Event::Tick(duration) => app.tick(&cfg, duration)?,
        }
    }
    terminal.clear()?;
    terminal.show_cursor()?;
    Ok(())
}
