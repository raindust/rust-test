use log::{debug, error, warn};
use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError};

pub struct Logger {
    default_level: LevelFilter,
}

impl Logger {
    pub fn new() -> Logger {
        Logger {
            default_level: LevelFilter::Trace,
        }
    }

    pub fn with_level(mut self, level: LevelFilter) -> Logger {
        self.default_level = level;
        self
    }
}

impl Default for Logger {
    fn default() -> Self {
        Logger::new()
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        println!("enabled");
        true
    }

    fn log(&self, record: &Record) {
        println!("{}:{}", record.level(), record.args())
    }

    fn flush(&self) {
        println!("flush");
    }
}

static LOGGER: Logger = Logger {
    default_level: LevelFilter::Trace,
};

fn main() {
    log::set_logger(&LOGGER);
    log::set_max_level(LOGGER.default_level);
    debug!("Debug message");
    warn!("Warn message");
    error!("Error message");
    println!("done");
}
