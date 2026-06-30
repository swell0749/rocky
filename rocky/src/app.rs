// imports
use iced::{
    Element, Task,
};
use crate::ui;

// enums
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum Message {
    Button1,
    Button2,

    ToggleSidebar,
}

// structs
pub struct App {
    sidebar_open: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            sidebar_open: true,
        }
    }
}

impl App {
    /// the update logic of the application
    /// it processes messages and updates the state of the app
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ToggleSidebar => self.sidebar_open = !self.sidebar_open,
            
            _ => {},
        };

        Task::none()
    }

    /// the view logic of the application
    /// it renders the ui
    pub fn view(&self) -> Element<'_, Message> {
        ui::render(self)
    }

    //////////////////////////////////////////////
    // private field exposers
    // note(swell): what are these called??
    //////////////////////////////////////////////
    pub fn sidebar_open(&self) -> &bool {
        &self.sidebar_open
    }
}
