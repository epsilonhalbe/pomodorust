use failure;
use pomodorust::config::Cfg;
use pomodorust::events::{Event, Events};
use pomodorust::state::{App, State};
use std::io;
use std::time::Duration;
use termion::event::Key;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Gauge, Paragraph, Text, Widget};
use tui::Terminal;

fn main() -> Result<(), failure::Error> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new();
    let cfg = Cfg::from_opts();
    let events = Events::new();
    terminal.clear()?;
    terminal.hide_cursor()?;

    loop {
        terminal.draw(|mut f| {
            let size = f.size();

            Block::default().render(&mut f, size);

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(0)
                .constraints(
                    [
                        Constraint::Percentage(75), //Table
                        Constraint::Percentage(25), //Gauge
                    ]
                    .as_ref(),
                )
                .split(size);
            Block::default()
                .borders(Borders::ALL)
                .render(&mut f, chunks[0]);
            {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(1)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                    .split(chunks[0]);

                let statistics = [
                    Text::raw(format!("Past pomodoros: {}\n", app.past_pomodoros)),
                    Text::raw("Press 'p' to toggle pause.\n"),
                    Text::raw("Toggle pause to skip break.\n"),
                    Text::raw("Press 'q' to quit.")
                ];
                Paragraph::new(statistics.iter())
                    .block(
                        Block::default()
                            .title("Statistics")
                            .title_style(Style::default().fg(Color::Yellow)),
                    )
                    .render(&mut f, chunks[0]);

                let config = [
                    Text::raw(format!(
                        "Working duration: {}:{:02}:{:02}\n",
                        cfg.working.as_secs() / 3600,
                        cfg.working.as_secs() / 60,
                        cfg.working.as_secs() % 60
                    )),
                    Text::raw(format!(
                        "Short break:      {}:{:02}:{:02}\n",
                        cfg.short_break.as_secs() / 3600,
                        cfg.short_break.as_secs() / 60,
                        cfg.short_break.as_secs() % 60
                    )),
                    Text::raw(format!(
                        "Long break:       {}:{:02}:{:02}",
                        cfg.long_break.as_secs() / 3600,
                        cfg.long_break.as_secs() / 60,
                        cfg.long_break.as_secs() % 60
                    )),
                ];
                Paragraph::new(config.iter())
                    .block(
                        Block::default()
                            .title("Configuration")
                            .title_style(Style::default().fg(Color::Yellow)),
                    )
                    .render(&mut f, chunks[1]);
            }

            Gauge::default()
                .block(Block::default().title("Pomodoro").borders(Borders::ALL))
                .ratio(match app.state {
                    State::Running => {
                        app.current_pomodoro.as_secs_f64() / cfg.working.as_secs_f64()
                    }
                    State::NextBreak(break_duration) => {
                        app.current_break.as_secs_f64() / break_duration.as_secs_f64()
                    }
                    State::Paused => 1.0,
                })
                .label(
                    match app.state {
                        State::Running => {
                            let remaining_time =
                                cfg.working.as_secs() - app.current_pomodoro.as_secs();
                            format!("{:02}:{:02}", remaining_time / 60, remaining_time % 60)
                        }
                        State::NextBreak(break_duration) => {
                            let remaining_time =
                                break_duration.as_secs() - app.current_break.as_secs();
                            format!("{:02}:{:02}", remaining_time / 60, remaining_time % 60)
                        }
                        State::Paused => String::from("Paused"),
                    }
                    .as_ref(),
                )
                .style(Style::default().fg(match app.state {
                    State::Running => Color::Blue,
                    State::Paused => Color::Red,
                    State::NextBreak(_) => Color::Gray,
                }))
                .render(&mut f, chunks[1]);
        })?;

        match events.next()? {
            Event::Input(key) => {
                if key == Key::Char('q') {
                    break;
                } else if key == Key::Char('p') {
                    app.state = match app.state {
                        State::Paused => State::Running,
                        _ => State::Paused,
                    }
                }
            }
            Event::Tick(duration) => match app.state {
                State::Running => {
                    app.current_pomodoro = app.current_pomodoro + duration;
                    if cfg.working <= app.current_pomodoro {
                        app.past_pomodoros += 1;
                        app.current_pomodoro = Duration::from_secs(0);
                        if app.past_pomodoros % 4 == 0 {
                            app.state = State::NextBreak(cfg.long_break);
                        } else {
                            app.state = State::NextBreak(cfg.short_break);
                        }
                    }
                }
                State::NextBreak(next_break) => {
                    app.current_break = app.current_break + duration;
                    if next_break <= app.current_break {
                        app.current_break = Duration::from_secs(0);
                        app.state = State::Running
                    }
                }
                State::Paused => {}
            },
        }
    }
    Ok(())
}
