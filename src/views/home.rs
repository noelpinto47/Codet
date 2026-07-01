use iced::{
    Element, Length,
    widget::{container, row, text},
};

use crate::{
    app::message::Message,
    widgets::sidebar::{self, SidebarItem},
};

pub fn view(selected: SidebarItem) -> Element<'static, Message> {
    row![
        sidebar::view(selected).map(Message::Sidebar),
        container(text("Main content"))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill),
    ]
    .height(Length::Fill)
    .into()
}