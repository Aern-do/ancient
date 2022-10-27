use std::{fs::remove_file, io, path::Path};

use chrono::Local;
use fern::{
    colors::{Color, ColoredLevelConfig},
    log_file, Dispatch,
};
use log::LevelFilter;

fn main() {
    clear_logs();
    init_logging();
}
fn clear_logs() {
    let path = Path::new("server.log");
    if path.exists() {
        remove_file(path).unwrap();
    }
}
fn init_logging() {
    let colors = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::White)
        .debug(Color::White)
        .trace(Color::BrightBlack);
    let stdout_dispatcher = Dispatch::new().level(LevelFilter::Info).chain(io::stdout());
    let stderr_dispatcher = Dispatch::new()
        .level(LevelFilter::Error)
        .chain(io::stderr());
    let console_dispatcher = Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{color_line}[{date}][{target}/{level}{color_line}] {message}\x1B[0m",
                color_line =
                    format_args!("\x1B[{}m", colors.get_color(&record.level()).to_fg_str()),
                date = Local::now().format("%H:%M:%S"),
                level = colors.color(record.level()),
                target = record.target(),
                message = message
            ))
        })
        .chain(stdout_dispatcher)
        .chain(stderr_dispatcher);
    let file_dispatcher = Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{date}][{target}/{level}] {message}",
                date = Local::now().format("%H:%M:%S"),
                level = record.level(),
                target = record.target(),
                message = message
            ))
        })
        .chain(log_file("server.log").unwrap());
    Dispatch::new()
        .chain(file_dispatcher)
        .chain(console_dispatcher)
        .apply()
        .unwrap();
}
