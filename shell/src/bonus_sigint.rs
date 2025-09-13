// Handle Ctrl+C (SIGINT) without crashing the shell
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub fn setup_sigint_handler() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    let _ = ctrlc::set_handler(move || {
        println!("\nReceived Ctrl+C. Type 'exit' to quit.");
        r.store(false, Ordering::SeqCst);
    });
}
