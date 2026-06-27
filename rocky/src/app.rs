use iced::{
    Center, Element, Length, Task,
    widget::{Space, column, row, text},
};

#[derive(Debug, Clone, Copy)]
pub enum Message {}

pub struct App;

impl Default for App {
    fn default() -> Self {
        Self
    }
}

impl App {
    /// the update logic of the application
    /// essentially, it processes messages and updates the state of the app
    pub fn update(&mut self, _message: Message) -> Task<Message> {
        Task::none()
    }

    /// the view logic of the application
    /// essentially, it renders the ui
    pub fn view(&self) -> Element<'_, Message> {
        row![
            Space::new().width(Length::Fill),
            column![
                Space::new().height(Length::Fill),
                text("AHHHHH").size(30),
                Space::new().height(Length::Fill),
            ]
            .padding(20)
            .align_x(Center),
            Space::new().width(Length::Fill),
        ]
        .into()
    }
}
