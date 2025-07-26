use log::{Level, SetLoggerError, error, info, warn};
use syslog::{Facility, Formatter3164};

struct SyslogLogger;

impl log::Log for SyslogLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let formatter = Formatter3164 {
                facility: Facility::LOG_USER,
                hostname: None,
                process: "rust_systemlog_example".to_string(),
                pid: 0,
            };

            let Ok(mut writer) = syslog::unix(formatter) else {
                return;
            };
            match record.level() {
                Level::Error => writer.err(record.args()),
                Level::Warn => writer.warning(record.args()),
                Level::Info => writer.info(record.args()),
                Level::Debug => writer.debug(record.args()),
                Level::Trace => writer.debug(record.args()),
            }
            .expect("Failed to send log to syslog");
        }
    }

    fn flush(&self) {}
}

static SYSLOG_LOGGER: SyslogLogger = SyslogLogger;

fn main() -> Result<(), SetLoggerError> {
    log::set_logger(&SYSLOG_LOGGER)?;
    log::set_max_level(log::LevelFilter::Info);

    info!("Info message");
    warn!("Warning message");
    error!("Error message");

    Ok(())
}
