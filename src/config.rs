use clap::{App, Arg};
use config::{Config, File, FileFormat};
use serde::{Deserialize, Serialize};
use std::env::var_os;
use std::path::PathBuf;
use std::time::Duration;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::terminal::Frame;
use tui::widgets::{Block, Paragraph, Text, Widget};

#[derive(Deserialize, Serialize, Debug)]
pub struct Cfg {
    pub working: Duration,
    pub short_break: Duration,
    pub long_break: Duration,
}

impl Default for Cfg {
    fn default() -> Self {
        Cfg {
            working: Duration::from_secs(25 * 60),
            short_break: Duration::from_secs(5 * 60),
            long_break: Duration::from_secs(10 * 60),
            // pause_key: Key,
            // exit_key: Key,
        }
    }
}

impl Cfg {
    pub fn from_opts() -> Cfg {
        let def_config: Option<PathBuf> = var_os("XDG_CONFIG_HOME")
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

        let config = options
            .get_matches()
            .value_of_os("config")
            .map(PathBuf::from)
            .or(def_config)
            .map(File::from)
            .map(|f| f.format(FileFormat::Yaml));

        let mut cfg = Config::default();

        config.map(|c| cfg.merge(c));

        cfg.try_into::<Cfg>().unwrap_or_default()
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
