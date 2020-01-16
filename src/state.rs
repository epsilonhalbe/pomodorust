use crate::config::{Cfg, PAUSE_KEY, QUIT_KEY};
use crate::database::{todays_no_pomodoros, Pomodoro, Statistic};
use chrono::naive::NaiveDateTime;
use failure;
use std::convert::TryFrom;
use std::time::Duration;
use std::time::SystemTime;
use termion::event::Key;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::terminal::Frame;
use tui::widgets::{Block, Borders, Gauge, Paragraph, Text, Widget};

pub struct App {
    pub current_pomodoro: Duration,
    pub current_break: Duration,
    pub todays_pomodoros: i64,
    pub pomodoros: Vec<Pomodoro>,
    pub state: State,
    pub current_tab: usize,
    tabs: Vec<String>,
}

pub enum State {
    Running,
    NextBreak(Duration),
    Paused,
}

impl App {
    pub fn new(cfg: &Cfg) -> App {
        let today = NaiveDateTime::from_timestamp(
            TryFrom::try_from(
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            )
            .unwrap(),
            0,
        );
        let pomodoros = Pomodoro::pomodoros_of(&cfg.conn, today).unwrap();
        App {
            current_break: Duration::from_secs(0),
            current_pomodoro: Duration::from_secs(0),
            todays_pomodoros: todays_no_pomodoros(&cfg.conn).unwrap_or(0),
            pomodoros: pomodoros,
            state: State::Running,
            tabs: vec![String::from("Pomodoro"), String::from("Statistics")],
            current_tab: 0,
        }
    }
    pub fn tabs(&self) -> &Vec<String> {
        &self.tabs
    }
    // event handlers

    // returns true when to quit
    pub fn key_handler(&mut self, key: Key) -> bool {
        match key {
            Key::Char(PAUSE_KEY) => {
                self.state = match self.state {
                    State::Paused => State::Running,
                    _ => State::Paused,
                }
            }
            // Key::Tab => {
            // self.current_tab = (self.current_tab + 1) % self.tabs().len();
            // self.state = State::Paused;
            // }
            Key::BackTab => {
                self.current_tab = (self.current_tab + self.tabs.len() - 1) % self.tabs.len();
                self.state = State::Paused;
            }
            _ => {}
        };
        key == Key::Char(QUIT_KEY)
    }
    pub fn tick(&mut self, cfg: &Cfg, duration: Duration) -> Result<(), failure::Error> {
        match self.state {
            State::Running => {
                self.current_pomodoro = self.current_pomodoro + duration;
                if cfg.working <= self.current_pomodoro {
                    let working_mins: i64 = TryFrom::try_from(cfg.working.as_secs() / 60)?;
                    Statistic::new(working_mins).insert(&cfg.conn)?;
                    self.todays_pomodoros += 1;
                    self.current_pomodoro = Duration::from_secs(0);
                    if self.todays_pomodoros % 4 == 0 {
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
        Ok(())
    }

    // render functions
    pub fn paragraph<B>(&self, f: &mut Frame<B>, area: Rect)
    where
        B: Backend,
    {
        let content = [
            Text::raw(format!("Past pomodoros: {}\n", self.todays_pomodoros)),
            Text::raw(format!("Press '{}' to toggle pause.\n", PAUSE_KEY)),
            Text::raw("Toggle pause to skip break.\n"),
            Text::raw(format!("Press '{}' to quit.", QUIT_KEY)),
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
