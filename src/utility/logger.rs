use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::PathBuf,
};

use colored::{Color, Colorize};

use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Level {
    Info,
    Warn,
    Error,
}

impl Level {
    pub const fn color(self) -> Color {
        match self {
            Self::Info => Color::BrightBlue,
            Self::Warn => Color::Yellow,
            Self::Error => Color::BrightRed,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Log<'l> {
    pub time: DateTime<Utc>,
    pub level: Level,
    pub text: &'l str,
}

impl Log<'_> {
    pub fn to_colored(self) -> String {
        let time = self.time.format("[%x %X:%3f]").to_string().bright_black();
        let level = format!("({:?})", self.level).color(self.level.color());

        format!("{time} {level} {}", self.text)
    }
}

impl Display for Log<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let time = self.time.format("%x %X:%3f");

        write!(f, "[{time}] ({:?}) {}", self.level, self.text)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Logger {
    pub file: PathBuf,
    quiet: bool,
    store: bool,
}

impl Logger {
    pub const DIR: &str = "logs/";

    pub fn new(quiet: bool, store: bool) -> Result<Self> {
        let time = Utc::now();
        let file = PathBuf::from(time.format("%y%m%d%H%M%S%3f.txt").to_string());

        if store {
            create_dir_all(Self::DIR)?;
            File::create(PathBuf::from(Self::DIR).join(&file))?;
        }

        Ok(Self { file, quiet, store })
    }

    fn __path(&self) -> PathBuf {
        PathBuf::from(Self::DIR).join(&self.file)
    }

    fn __log(&self, level: Level, content: impl Into<String>) -> Result<()> {
        let time = Utc::now();
        let text = &content.into();
        let log = Log { time, level, text };

        if !self.quiet {
            if level == Level::Info {
                println!("{}", log.to_colored());
            } else {
                eprintln!("{}", log.to_colored());
            }

            if self.store {
                let mut file = File::options().append(true).open(self.__path())?;

                file.write_all(log.to_string().as_bytes())?;
                file.write_all(&[b'\n'])?;
                file.flush()?;
            }
        }

        Ok(())
    }

    pub fn info(&self, content: impl Into<String>) -> Result<()> {
        self.__log(Level::Info, content)
    }
    pub fn warn(&self, content: impl Into<String>) -> Result<()> {
        self.__log(Level::Warn, content)
    }
    pub fn error(&self, content: impl Into<String>) -> Result<()> {
        self.__log(Level::Error, content)
    }
}
