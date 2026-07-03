use iced::{
    widget::{container, row, text},
    Background, Border, Color, Element, Length, Shadow,
};

use crate::{
    app::message::Message,
    widgets::sidebar::{self, SidebarItem},
};

const CHROME_BG: Color = Color::from_rgb8(10, 10, 20);

pub fn view(selected: SidebarItem) -> Element<'static, Message> {
    container(
    row![
        sidebar::view(selected).map(Message::Sidebar),
        container(text("Main content"))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill),
    ]
    )
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