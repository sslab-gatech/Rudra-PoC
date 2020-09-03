use log::{LevelFilter, Metadata, Record};
use logging_allocator::LoggingAllocator;

#[global_allocator]
static ALLOC: LoggingAllocator = LoggingAllocator::new();
static LOGGER: SimpleLogger = SimpleLogger;

// using a custom logger to avoid allocation while logging
struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            eprintln!("[{}] > {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

pub fn init() {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Trace))
        .unwrap();
}

pub fn trace_alloc<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    ALLOC.enable_logging();
    let result = f();
    ALLOC.disable_logging();
    result
}
