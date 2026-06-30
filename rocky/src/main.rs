// by default, when in release mode, rust opens a terminal window alongside the actual program.
// this disables that!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// modules
pub(crate) mod app;
pub(crate) mod ui;

mod logger;

// imports
use std::error::Error;

use app::App;

// functions
fn main() -> Result<(), Box<dyn Error>> {
    logger::init()?;

    log::debug!("starting rocky...");

    let result = iced::application(
        App::default,
        App::update,
        App::view,
    )
        .title("rocky")
        .run();

    match result {
        Ok(..) => {
            log::debug!("closing rocky...");
        },

        Err(e) => {
            log::error!("error caught: {e}");
        }
    }

    Ok(())
}
