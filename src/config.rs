use clap::{App, Arg};
use config::{Config, File, FileFormat};
use diesel::prelude::Connection;
use diesel::sqlite::SqliteConnection;
use failure;
use serde::Deserialize;
use std::env::var_os;
use std::path::PathBuf;
use std::time::Duration;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::terminal::Frame;
use tui::widgets::{Block, Paragraph, Text, Widget};

#[derive(Deserialize, Debug)]
pub struct CfgDTO {
    pub working: u64,
    pub short_break: u64,
    pub long_break: u64,
    pub db_path: PathBuf,
}

pub struct Cfg {
    pub working: Duration,
    pub short_break: Duration,
    pub long_break: Duration,
    pub conn: SqliteConnection,
    pub pause_key: char,
    pub quit_key: char,
}

impl Default for CfgDTO {
    fn default() -> Self {
        CfgDTO {
            working: 25,
            short_break: 5,
            long_break: 10,
            db_path: PathBuf::from("pomodorust.db"),
        }
    }
}

impl CfgDTO {
    fn from(&self) -> Result<Cfg, failure::Error> {
        let conn = SqliteConnection::establish(self.db_path.to_str().unwrap())?;
        Ok(Cfg {
            working: Duration::from_secs(self.working * 60),
            short_break: Duration::from_secs(self.short_break * 60),
            long_break: Duration::from_secs(self.long_break * 60),
            conn: conn,
            pause_key: 'p',
            quit_key: 'q',
        })
    }
}

impl Cfg {
    pub fn from_opts() -> Result<Cfg, failure::Error> {
        let def_path = var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .or(var_os("HOME").map(PathBuf::from).map(|x| x.join(".config")))
            .map(|s| s.join("pomodorust/config.yaml"));

        let options: App = App::new("PomodoRust")
            .version("1.0.0")
            .author("Martin Heuschober <epsilonhalbe@gmail.com>")
            .about("commandline pomodoro thingy")
            .arg(
                Arg::with_name("config")
                    .short("c")
                    .long("config")
                    .value_name("FILE")
                    .help("Sets a custom config file")
                    .takes_value(true),
            );

        let mut cfg = Config::default();
        let dto : CfgDTO =
            match options.get_matches().value_of_os("config").map(PathBuf::from) {
                Some(path) => {
                    if path.is_file() {
                    cfg.merge(File::from(path).format(FileFormat::Yaml))?;
                    cfg.try_into::<CfgDTO>().unwrap()
                    } else {
                        panic!("Configuration-file '{}' does not exist.", path.to_str().unwrap())
                    }
                }
                None => {
                    let path = def_path.unwrap();
                    if path.is_file() {
                        cfg.merge(File::from(path).format(FileFormat::Yaml))?;
                        cfg.try_into::<CfgDTO>()?
                    } else {
                        CfgDTO::default()
                    }
                }
            };
        dto.from()
    }

    pub fn paragraph<B>(&self, f: &mut Frame<B>, area: Rect)
    where
        B: Backend,
    {
        let content = [
            time_with_label(self.working, String::from("Working duration")),
            time_with_label(self.short_break, String::from("Short break")),
            time_with_label(self.long_break, String::from("Long break")),
        ];
        Paragraph::new(content.iter())
            .block(
                Block::default()
                    .title("Configuration")
                    .title_style(Style::default().fg(Color::Yellow)),
            )
            .render(f, area);
    }
}

fn time_with_label<'a>(time: Duration, label: String) -> Text<'a> {
    Text::raw(format!(
        "{}: {}:{:02}:{:02}\n",
        label,
        time.as_secs() / 3600,
        time.as_secs() / 60,
        time.as_secs() % 60
    ))
}
