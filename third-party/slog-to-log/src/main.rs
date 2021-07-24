use slog::o;
use slog::Drain;

fn main() {
    env_logger::init();
    log::info!("hello log");

    let logger = slog::Logger::root(slog_stdlog::StdLog.fuse(), o!());
    slog::info!(logger, "hello from slog");

    log::info!("hello log2");
}
