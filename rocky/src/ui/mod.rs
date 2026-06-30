// modules
pub(crate) mod sidebar;
pub(crate) mod central;

// imports
use iced::{
    Element,
    widget::row,
};
use crate::app::{App, Message};

// functions
pub fn render<'a>(app: &App) -> Element<'a, Message> {
    let mut container = row![]
        .padding(10)
        .spacing(10);

    let mut elements: Vec<Element<'_, Message>> = Vec::new();
    
    elements.push(sidebar::render(app));
    elements.push(central::render(app));

    container = container.extend(elements);
    container.into()
}