// by default, when in release mode, rust opens a terminal window alongside the actual program.
// this disables that!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;

use std::error::Error;
use app::App;

fn main() -> Result<(), Box<dyn Error>> {
    iced::application(
        App::default,
        App::update,
        App::view,
    )
        .title("rocky")
        .run()?;

    Ok(())
}
