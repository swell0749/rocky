// modules
mod sidebar;
mod central;

// imports
use iced::{
    Element, Task,
    widget::row,
};

// enums
#[derive(Debug, Clone, Copy)]
pub enum Message {
    Button1,
    Button2,

    ToggleSidebar,
}

// structs
pub struct App {
    pub sidebar_open: bool,
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
    /// essentially, it processes messages and updates the state of the app
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ToggleSidebar => self.sidebar_open = !self.sidebar_open,
            
            _ => {},
        };

        Task::none()
    }

    /// the view logic of the application
    /// essentially, it renders the ui
    pub fn view(&self) -> Element<'_, Message> {
        let mut container = row![]
            .padding(10)
            .spacing(10);

        let mut elements: Vec<Element<'_, Message>> = Vec::new();
        
        elements.push(sidebar::render(self));
        elements.push(central::render(self));

        container = container.extend(elements);
        container.into()
    }
}
