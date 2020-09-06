use log::{LevelFilter, Metadata, Record};
use logging_allocator::LoggingAllocator;
use std::panic::{catch_unwind, UnwindSafe};

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

#[inline(never)]
pub fn assert_aligned<T>(ptr: *const T) {
    let align = std::mem::align_of::<T>();
    if (ptr as usize) % align != 0 {
        panic!(
            "Reference is not aligned - addr: {:p}, align: 0x{:x}",
            ptr, align
        );
    }
}

pub fn test_case<F>(s: impl ToString, f: F)
where
    F: FnOnce() + UnwindSafe,
{
    let s = s.to_string();
    let outline = "-".repeat(s.len() + 4);

    println!("{}", outline);
    println!("| {} |", s);
    println!("{}", outline);

    if let Ok(_) = catch_unwind(f) {
        println!("Test case did not panic");
    }
    println!("");
}
