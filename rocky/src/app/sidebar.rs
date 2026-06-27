// imports
use iced::{
    Element, Length,
    widget::{container, column, button},
};

use super::{App, Message};

// structs
struct SidebarButton<'a> {
    pub text: &'a str,
    pub message: Option<Message>,
}

// constants
const CHARACTER_LENGTH: usize = 25;

const BUTTONS: &[SidebarButton] = &[
    SidebarButton {
        text: "test",
        message: Some(Message::Button1),
    },

    SidebarButton {
        text: "test 2",
        message: None,
    }
];

// functions
pub fn render<'a>(app: &App) -> Element<'a, Message> {
    let mut buttons = column![]
        .padding(10)
        .spacing(10);

    let mut elements: Vec<Element<'_, Message>> = Vec::new();

    // open/close sidebar button
    let toggle_button = button(
        if app.sidebar_open {
            "<"
        } else {
            ">"
        }
    )
        .style(button::subtle)
        .on_press(Message::ToggleSidebar);

    elements.push(toggle_button.into());

    if app.sidebar_open {
        // preprocess stage
        // this finds the button with the largest horizontal size so that every button can adopt it's size
        // however, it will always be capped to 200px
        let mut largest_size: usize = 0;

        for button_data in BUTTONS {
            largest_size = button_data.text.len() * CHARACTER_LENGTH;
        }

        largest_size = largest_size.min(200);

        // render stage
        // this is the stage that actually pushes the buttons to the elements list
        for button_data in BUTTONS {
            let rendered_button = button(button_data.text)
                .width(Length::Fixed(largest_size as f32))
                .on_press_maybe(button_data.message);

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
