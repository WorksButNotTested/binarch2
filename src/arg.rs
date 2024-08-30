use {
    clap::{Parser, ValueEnum},
    log::LevelFilter,
};

#[derive(ValueEnum, Clone, Debug)]
pub enum Level {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl From<Level> for LevelFilter {
    fn from(value: Level) -> LevelFilter {
        match value {
            Level::Off => LevelFilter::Off,
            Level::Error => LevelFilter::Error,
            Level::Warn => LevelFilter::Warn,
            Level::Info => LevelFilter::Info,
            Level::Debug => LevelFilter::Debug,
            Level::Trace => LevelFilter::Trace,
        }
    }
}

#[derive(Parser, Debug)]
#[command(
    version = "1.1",
    about = "binarch2",
    long_about = "Tool for identifying architecture and endianness of binary files"
)]
pub struct Opt {
    #[arg(help = "Input file")]
    pub input: String,

    #[arg(
        help = "Log level",
        required = false,
        default_value = "info",
        short,
        long,
        value_enum
    )]
    pub log_level: Level,
}
