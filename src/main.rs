#![forbid(missing_debug_implementations)]
pub mod config;
pub mod connection;
pub mod error;
pub mod protocol;
pub mod socket;

use std::{
    fs::{read_to_string, remove_file, File},
    io::{self, ErrorKind, Write},
    net::{SocketAddr, SocketAddrV4, TcpListener},
    path::Path,
    thread,
    time::Instant,
};

use chrono::Local;
use config::Config;
use error::Error;
use fern::{
    colors::{Color, ColoredLevelConfig},
    log_file, Dispatch,
};
use log::{error, info, LevelFilter};
use once_cell::sync::Lazy;
use rsa::RsaPrivateKey;
use toml::{from_str, to_string_pretty};

use crate::connection::Connection;

pub const RSA_KEYPAIR_BITS: usize = 1024;
pub static RSA_KEYPAIR: Lazy<RsaPrivateKey> = Lazy::new(|| {
    RsaPrivateKey::new(&mut rand::thread_rng(), RSA_KEYPAIR_BITS)
        .expect("Failed to generate RSA keypair")
});
fn main() -> Result<(), Error> {
    clear_logs();
    init_logging();
    force_rsa_keypair();
    info!("Server started");
    start(get_config()?)
}
fn force_rsa_keypair() {
    let instant = Instant::now();
    let _ = Lazy::force(&RSA_KEYPAIR);
    info!("Generated RSA keypair in {:#?}", instant.elapsed());
}
fn start(config: Config) -> Result<(), Error> {
    let listener = TcpListener::bind(SocketAddr::V4(SocketAddrV4::new(
        config.address,
        config.port,
    )))?;
    while let Ok((stream, address)) = listener.accept() {
        info!("Recivied new connection from {}", address);
        thread::spawn(move || {
            let connection = Connection::new(stream);
            match connection.start_receiving() {
                Err(Error::Io(io)) if io.kind() == ErrorKind::UnexpectedEof => {
                    info!("{} closed a connection", address)
                }
                Err(err) => error!("{}", err),
                Ok(..) => {}
            }
        });
    }
    Ok(())
}
fn get_config() -> Result<Config, Error> {
    let path = Path::new("config.toml");
    if !path.exists() {
        create_default_config(path)
    } else {
        read_config(path)
    }
}
fn read_config(path: &Path) -> Result<Config, Error> {
    let config = read_to_string(path)?;
    Ok(from_str(&config)?)
}
fn create_default_config(path: &Path) -> Result<Config, Error> {
    let config = Config::default();
    let mut file = File::create(path)?;
    let string = to_string_pretty(&config)?;
    file.write_all(string.as_bytes())?;
    Ok(config)
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
        .level_for("ureq", LevelFilter::Off)
        .level_for("rustls", LevelFilter::Off)
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
