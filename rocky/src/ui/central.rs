// imports
use iced::{
    Element, Length,
    widget::container,
};
use crate::app::{App, Message};

// functions
pub fn render<'a>(_app: &App) -> Element<'a, Message> {
    let root = container("")
        .padding(10)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(container::bordered_box);

    root.into()
}
