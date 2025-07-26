use log::SetLoggerError;

static CONSOLE_LOGGER: ConsoleLogger = ConsoleLogger;
struct ConsoleLogger;

impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            println!("Rust says: {} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

fn main() -> Result<(), SetLoggerError> {
    log::set_logger(&CONSOLE_LOGGER)?;
    log::set_max_level(log::LevelFilter::Info);
    log::info!("hello log");
    log::warn!("warning");
    log::error!("opps");

    Ok(())
}
