use iced::{Background, Border, Color, Element, Length, Shadow};
use iced::widget::{column, container, row, text};

use crate::{
    app::message::Message,
    widgets::sidebar::SidebarItem,
};

const PANEL_BG: Color = Color::from_rgb8(23, 23, 28);
const PANEL_BORDER: Color = Color::from_rgb8(43, 43, 54);

pub fn view(selected: SidebarItem) -> Element<'static, Message> {
    let title = match selected {
        SidebarItem::Files => "Files",
        SidebarItem::Git => "Git",
        SidebarItem::Search => "Search",
        SidebarItem::Debugger => "Debugger",
        SidebarItem::Packages => "Packages",
        SidebarItem::Settings => "Settings",
    };

    let body = column![
        text(title).size(18),
        text("").size(14),
    ]
    .spacing(8);

    row![
        container(body)
            .width(Length::Fixed(280.0))
            .height(Length::Fill)
            .padding(16)
            .style(|_theme| iced::widget::container::Style {
                background: Some(Background::Color(PANEL_BG)),
                text_color: None,
                border: Border::default(),
                shadow: Shadow::default(),
                snap: false,
            }),

        container("")
            .width(1)
            .height(Length::Fill)
            .style(|_theme| iced::widget::container::Style {
                background: Some(Background::Color(PANEL_BORDER)),
                ..Default::default()
            }),
    ]
    .height(Length::Fill)
    .into()
}