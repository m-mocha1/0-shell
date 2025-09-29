// Handle Ctrl+C (SIGINT) without crashing the shell
// Example Rust code for handling SIGINT gracefully

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::process;

pub fn setup_sigint_handler() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        println!("\nReceived Ctrl+C. Type 'exit' to quit.");
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");
}
