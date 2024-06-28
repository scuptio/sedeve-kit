use std::backtrace::Backtrace;
use std::panic;

/// DO NOT set panic hook when testing should_panic
pub fn set_panic_hook() {
    panic::set_hook(Box::new(|info| {
        let stacktrace = Backtrace::force_capture();
        println!("Got panic. @info:{}\n@stackTrace:{}", info, stacktrace);
        std::process::abort();
    }));
}