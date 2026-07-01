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

// favicon stuff
static ICON: &[u8] = include_bytes!("../../gh-assets/logo.png");

const ICON_HEIGHT: u32 = 512;
const ICON_WIDTH: u32 = 512;

// functions
fn main() -> Result<(), Box<dyn Error>> {
    logger::init()?;

    log::debug!("starting rocky...");

    let image = image::load_from_memory(ICON);

    if image.is_err() {
        log::error!("image not found at 'gh-assets/logo.png': {image:?}");

        return Ok(());
    }

    // note(swell): should be safe, right?
    let image = image.unwrap();
    let icon = iced::window::icon::from_rgba(image.as_bytes().to_vec(), ICON_HEIGHT, ICON_WIDTH);

    if icon.is_err() {
        log::error!("icon failed to be parsed from rgba file: {icon:?}");

        return Ok(());
    }

    // note(swell): should be safe, right?
    let icon = icon.unwrap();

    let window_settings = iced::window::Settings {
        icon: Some(icon),

        ..Default::default()
    };

    let result = iced::application(
        App::default,
        App::update,
        App::view,
    )
        .title("rocky")
        .window(window_settings)
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
