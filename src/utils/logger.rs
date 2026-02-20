use colored::Colorize;
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

pub fn init(verbose: bool) {
    let level = if verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    Builder::new()
        .filter_level(level)
        .format(|buf, record| {
            let level_str = match record.level() {
                log::Level::Error => "ERROR".red().bold().to_string(),
                log::Level::Warn => "WARN".yellow().bold().to_string(),
                log::Level::Info => "INFO".green().to_string(),
                log::Level::Debug => "DEBUG".blue().to_string(),
                log::Level::Trace => "TRACE".dimmed().to_string(),
            };
            let timestamp = chrono::Local::now().format("%H:%M:%S");
            writeln!(buf, "{} [{}] {}", timestamp.to_string().dimmed(), level_str, record.args())
        })
        .init();
}

pub fn debug(msg: &str) {
    log::debug!("{}", msg);
}

pub fn info(msg: &str) {
    log::info!("{}", msg);
}

pub fn warn(msg: &str) {
    log::warn!("{}", msg);
}

pub fn error(msg: &str) {
    log::error!("{}", msg);
}

// iteration 27
