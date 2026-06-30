// imports
use iced::{
    Element, Length,
    widget::{container, column, button, text},
};
use crate::app::{App, Message};

// structs
pub struct SidebarButton {
    pub text: String,
    pub message: Option<Message>,
}

#[allow(dead_code)]
impl SidebarButton {
    pub fn new(text: String, message: Option<Message>) -> Self {
        Self { text, message }
    }

    pub fn text(&self) -> &String {
        &self.text
    }

    pub fn message(&self) -> &Option<Message> {
        &self.message
    }
}

// constants
const CHARACTER_LENGTH: usize = 25;

// functions
pub fn render<'a>(app: &App) -> Element<'a, Message> {
    let sidebar_open = app.sidebar_open();

    // note(swell): these are placeholder buttons! they'll be changed soon-ish </3
    let buttons_list = vec![
        SidebarButton::new("test".to_string(), Some(Message::Button1)),
        SidebarButton::new("test 2".to_string(), None),
    ];

    let mut buttons = column![]
        .padding(10)
        .spacing(10);

    let mut elements: Vec<Element<'_, Message>> = Vec::new();

    // open/close sidebar button
    let toggle_button = button(
        if *sidebar_open {
            "<"
        } else {
            ">"
        }
    )
        .style(button::primary)
        .on_press(Message::ToggleSidebar);

    elements.push(toggle_button.into());

    if *sidebar_open {
        // "preprocess" stage
        // this finds the button with the largest horizontal size so that every button can adopt it's size
        // however, it will always be capped to 200px
        let mut largest_size: usize = 0;

        // loop through every button to try and find the biggest size
        // todo(swell): optimize this somehow? i'm not sure this is a good idea by any means
        for button_data in buttons_list.iter() {
            largest_size = button_data.text.len() * CHARACTER_LENGTH;
        }

        // cap largest button size to 200px
        largest_size = largest_size.min(200);

        // "render" stage
        // this is the stage that actually pushes the buttons to the elements list
        for button_data in buttons_list.iter() {
            let rendered_button = button(text(button_data.text.clone()))
                .width(Length::Fixed(largest_size as f32))
                .on_press_maybe(button_data.message.clone());

            elements.push(rendered_button.into());
        }
    }

    buttons = buttons.extend(elements);

    let root = container(buttons)
        .padding(10)
        .width(Length::Shrink)
        .height(Length::Fill)
        .style(container::bordered_box);

    root.into()
}
