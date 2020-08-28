/// Output the info message to stderr.
pub fn log_info(component: &str, message: &str) {
    eprintln!("\x1b[1m[{}]\x1b[0m {}", component, message);
}
