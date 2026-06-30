// imports
use std::{error::Error, io};
use chrono::Local;
use fern::{
    Dispatch,
    colors::{Color, ColoredLevelConfig},
};
use log::LevelFilter;

// functions
pub fn init() -> Result<(), Box<dyn Error>> {
    let level = match std::env::var("RUST_LOG").as_deref() {
        Ok("trace") => LevelFilter::Trace,
        Ok("debug") => LevelFilter::Debug,
        Ok("warn") => LevelFilter::Warn,
        Ok("error") => LevelFilter::Error,

        _ => LevelFilter::Info,
    };

    let colors = ColoredLevelConfig::new()
        .info(Color::Green)
        .warn(Color::Yellow)
        .error(Color::Red)
        .debug(Color::Blue)
        .trace(Color::Magenta);

    Dispatch::new()
        .level(level)

        // note(swell): why does every graphics library always spam the console with debug statements </3
        .level_for("naga", LevelFilter::Warn)

        .level_for("cosmic_text", LevelFilter::Warn)

        .level_for("iced_wgpu", LevelFilter::Warn)
        .level_for("iced_winit", LevelFilter::Warn)

        .level_for("wgpu_core", LevelFilter::Warn)
        .level_for("wgpu_hal", LevelFilter::Warn)

        .chain(
            Dispatch::new()
                .format(move |out, message, record| out.finish(format_args!("{} {} [{}] {}", Local::now().format("%H:%M:%S"), colors.color(record.level()), record.target(), message)))
                .chain(io::stdout()),
        )
        .apply()?;

    log::debug!("logger initialized");

    Ok(())
}