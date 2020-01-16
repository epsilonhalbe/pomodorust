use failure;
use pomodorust::config::Cfg;
use pomodorust::database::{create_table, HEADER};
use pomodorust::events::{Event, Events};
use pomodorust::state::App;
use std::io;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{
    Constraint::{Length, Min, Percentage},
    Direction::{Horizontal, Vertical},
    Layout,
};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Row, Table, Tabs, Widget};
use tui::Terminal;

fn main() -> Result<(), failure::Error> {
    let cfg = Cfg::from_opts()?;
    create_table(&cfg.conn)?;

    let events = Events::new();
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new(&cfg);
    terminal.clear()?;
    terminal.hide_cursor()?;
    let select_style = Style::default()
        .bg(Color::Yellow)
        .fg(Color::Black)
        .modifier(Modifier::BOLD);

    loop {
        terminal.draw(|mut f| {
            let size = f.size();
            Block::default().render(&mut f, size);

            let chunks = Layout::default()
                .direction(Vertical)
                .constraints([Length(3), Min(0)].as_ref())
                .split(size);

            Tabs::default()
                .block(Block::default().borders(Borders::ALL))
                .titles(&app.tabs())
                .select(app.current_tab)
                .highlight_style(select_style)
                .render(&mut f, chunks[0]);
            match app.current_tab {
                0 => {
                    let chunks_ = Layout::default()
                        .direction(Vertical)
                        .margin(0)
                        .constraints([Percentage(75), Percentage(25)].as_ref())
                        .split(chunks[1]);

                    Block::default()
                        .borders(Borders::ALL)
                        .render(&mut f, chunks_[0]);
                    {
                        let chunks__ = Layout::default()
                            .direction(Horizontal)
                            .margin(1)
                            .constraints([Percentage(50), Percentage(50)].as_ref())
                            .split(chunks_[0]);

                        app.paragraph(&mut f, chunks__[0]);
                        cfg.paragraph(&mut f, chunks__[1]);
                    }
                    {
                        app.gauge(&cfg, &mut f, chunks_[1]);
                    }
                }
                1 => {
                    let rows = app.pomodoros.iter().map(|pom| {
                        Row::Data(
                            vec![
                                format!("{}", pom.id),
                                format!("{}", pom.created_at),
                                format!("{}", pom.duration),
                                format!("{}", pom.ticket_id.to_owned().unwrap_or_default()),
                                format!("{}", pom.note.to_owned().unwrap_or_default()),
                            ]
                            .into_iter(),
                        )
                    });

                    let rects = Layout::default()
                        .constraints([Percentage(100)].as_ref())
                        .split(chunks[1]);
                    Table::new(HEADER.into_iter(), rows)
                        .block(Block::default().borders(Borders::ALL))
                        .widths(&[Length(05), Length(30), Length(30), Min(50), Min(50)])
                        .render(&mut f, rects[0]);
                }
                _ => {}
            }
        })?;

        match events.next()? {
            Event::Input(key) => {
                if app.key_handler(key) {
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
