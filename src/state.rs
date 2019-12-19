// use chrono::Duration;
use crate::config::Cfg;
use std::time::Duration;
use termion::event::Key;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::terminal::Frame;
use tui::widgets::{Block, Borders, Gauge, Paragraph, Text, Widget};

pub struct App {
    pub current_pomodoro: Duration,
    pub current_break: Duration,
    pub past_pomodoros: u32,
    pub state: State,
}

pub enum State {
    Running,
    NextBreak(Duration),
    Paused,
}
impl App {
    // constructor
    pub fn new() -> App {
        App {
            current_pomodoro: Duration::from_secs(0),
            current_break: Duration::from_secs(0),
            past_pomodoros: 0,
            state: State::Running,
        }
    }

    // event handlers

    // returns true when to quit
    pub fn quit_or_pause(&mut self, key: Key, _cfg: &Cfg) -> bool {
        if key == Key::Char('p') {
            self.state = match self.state {
                State::Paused => State::Running,
                _ => State::Paused,
            }
        };
        key == Key::Char('q')
    }

    pub fn tick(&mut self, cfg: &Cfg, duration: Duration) {
        match self.state {
            State::Running => {
                self.current_pomodoro = self.current_pomodoro + duration;
                if cfg.working <= self.current_pomodoro {
                    self.past_pomodoros += 1;
                    self.current_pomodoro = Duration::from_secs(0);
                    if self.past_pomodoros % 4 == 0 {
                        self.state = State::NextBreak(cfg.long_break);
                    } else {
                        self.state = State::NextBreak(cfg.short_break);
                    }
                }
            }
            State::NextBreak(next_break) => {
                self.current_break = self.current_break + duration;
                if next_break <= self.current_break {
                    self.current_break = Duration::from_secs(0);
                    self.state = State::Running
                }
            }
            State::Paused => {}
        }
    }

    // render functions
    pub fn paragraph<B>(&self, f: &mut Frame<B>, area: Rect)
    where
        B: Backend,
    {
        let content = [
            Text::raw(format!("Past pomodoros: {}\n", self.past_pomodoros)),
            Text::raw("Press 'p' to toggle pause.\n"),
            Text::raw("Toggle pause to skip break.\n"),
            Text::raw("Press 'q' to quit."),
        ];
        Paragraph::new(content.iter())
            .block(
                Block::default()
                    .title("Statistics")
                    .title_style(Style::default().fg(Color::Yellow)),
            )
            .render(f, area);
    }

    pub fn gauge<B>(&self, cfg: &Cfg, f: &mut Frame<B>, area: Rect)
    where
        B: Backend,
    {
        Gauge::default()
            .block(Block::default().title("Pomodoro").borders(Borders::ALL))
            .ratio(match self.state {
                State::Running => self.current_pomodoro.as_secs_f64() / cfg.working.as_secs_f64(),
                State::NextBreak(break_duration) => {
                    self.current_break.as_secs_f64() / break_duration.as_secs_f64()
                }
                State::Paused => 1.0,
            })
            .label(
                match self.state {
                    State::Running => {
                        let remaining_time = (cfg.working - self.current_pomodoro).as_secs();
                        format!("{:02}:{:02}", remaining_time / 60, remaining_time % 60)
                    }
                    State::NextBreak(break_duration) => {
                        let remaining_time = (break_duration - self.current_break).as_secs();
                        format!("{:02}:{:02}", remaining_time / 60, remaining_time % 60)
                    }
                    State::Paused => String::from("Paused"),
                }
                .as_ref(),
            )
            .style(Style::default().fg(match self.state {
                State::Running => Color::Blue,
                State::Paused => Color::Red,
                State::NextBreak(_) => Color::Gray,
            }))
            .render(f, area);
    }
}
