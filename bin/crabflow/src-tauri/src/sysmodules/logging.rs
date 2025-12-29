use crate::sysmodules::config::load_logging_config;

fn init_logging() {
    match load_logging_config() {
        Ok(cfg) => {
            println!("Log level: {}", cfg.level);
            println!("Log file: {}", cfg.file);
        }
        Err(e) => eprintln!("Logging config error: {}", e),
    }
}
