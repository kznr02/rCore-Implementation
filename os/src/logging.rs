use log::Log;

struct SimpleLogger;

impl Log for SimpleLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let color = match record.level() {
            log::Level::Debug => 32,
            log::Level::Info => 34,
            log::Level::Warn => 93,
            log::Level::Error => 31,
            log::Level::Trace => 90,
        };

        println!(
            "\u{1B}[{}m[{:>5}] {}\u{1B}[0m",
            color,
            record.level(),
            record.args(),
        );
    }

    fn flush(&self) {}
}
static LOGGER: SimpleLogger = SimpleLogger;
pub fn init() {
    log::set_logger(&LOGGER).unwrap();
    // log::set_max_level(match option_env!("LOG") {
    //     Some("ERROR") => log::LevelFilter::Error,
    //     Some("WARN") => log::LevelFilter::Warn,
    //     Some("INFO") => log::LevelFilter::Info,
    //     Some("DEBUG") => log::LevelFilter::Debug,
    //     Some("TRACE") => log::LevelFilter::Trace,
    //     _ => log::LevelFilter::Off,
    // });
    log::set_max_level(log::LevelFilter::Trace);
}
