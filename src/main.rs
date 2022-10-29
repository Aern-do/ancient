pub mod connection;
pub mod error;
pub mod protocol;

use std::{
    fs::remove_file,
    io,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener},
    path::Path,
    thread,
};

use chrono::Local;
use error::Error;
use fern::{
    colors::{Color, ColoredLevelConfig},
    log_file, Dispatch,
};
use log::{info, LevelFilter};

use crate::connection::Connection;

fn main() -> Result<(), Error> {
    clear_logs();
    init_logging();
    info!("Starting server...");
    start()
}
fn start() -> Result<(), Error> {
    let listener = TcpListener::bind(SocketAddr::V4(SocketAddrV4::new(
        Ipv4Addr::new(127, 0, 0, 1),
        25565,
    )))?;
    while let Ok((stream, address)) = listener.accept() {
        info!("Recivied new connection from {}", address);
        thread::spawn::<_, Result<(), Error>>(move || {
            let connection = Connection::new();
            connection.start_receiving(stream)
        });
    }
    Ok(())
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
        .debug(Color::BrightBlack)
        .trace(Color::BrightBlack);
    let stdout_dispatcher = Dispatch::new()
        .level(LevelFilter::Debug)
        .chain(io::stdout());
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
