use iced::{
    widget::{container, row, text},
    Background, Border, Color, Element, Length, Shadow,
};

use crate::{
    app::message::Message,
    app::state::App,
    widgets::{extended_sidebar, sidebar},
};

const CHROME_BG: Color = Color::from_rgb8(19, 19, 23);
const EDITOR_BG: Color = Color::from_rgb8(30, 30, 36);

pub fn view(app: &App) -> Element<'static, Message> {
    let content = if app.sidebar_open {
        row![
            sidebar::view(app.selected_sidebar).map(Message::Sidebar),
            extended_sidebar::view(app.selected_sidebar),
            main_content(),
        ]
    } else {
        row![
            sidebar::view(app.selected_sidebar).map(Message::Sidebar),
            main_content(),
        ]
    };

    container(content)
    .height(Length::Fill)
    .width(Length::Fill)
    .style(|_theme| iced::widget::container::Style {
        background: Some(Background::Color(CHROME_BG)),
        text_color: None,
        border: Border::default(),
        shadow: Shadow::default(),
        snap: false,
    })
    .into()
}

fn main_content() -> iced::widget::Container<'static, Message> {
    container(text("Main content"))
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(|_theme| iced::widget::container::Style {
            background: Some(Background::Color(EDITOR_BG)),
            text_color: None,
            border: Border::default(),
            shadow: Shadow::default(),
            snap: false,
        })
}
