use iced::{
    alignment,
    widget::{container, row, text},
    Background, Border, Color, Length, Shadow,
};

use crate::{app::message::Message, app::state::AppState};

const STATUS_BG: Color = Color::from_rgb8(16, 16, 20);
const STATUS_BORDER: Color = Color::from_rgb8(38, 38, 44);
const STATUS_TEXT: Color = Color::from_rgb8(160, 160, 170);
const STATUS_ACCENT: Color = Color::from_rgb8(220, 220, 228);

pub fn view(_app: &AppState) -> iced::widget::Container<'_, Message> {
    let left = row![
        item("⌘", Some(Color::from_rgb8(150, 150, 160))),
        item("main", Some(STATUS_ACCENT)),
        item("⟳", Some(Color::from_rgb8(140, 140, 150))),
        item("✕ 0", Some(Color::from_rgb8(170, 170, 180))),
        item("⚠ 0", Some(Color::from_rgb8(170, 170, 180))),
    ]
    .spacing(18)
    .align_y(alignment::Vertical::Center);

    container(left)
        .width(Length::Fill)
        .height(Length::Fixed(30.0))
        .padding([0, 12])
        .style(|_theme| iced::widget::container::Style {
            background: Some(Background::Color(STATUS_BG)),
            text_color: Some(STATUS_TEXT),
            border: Border {
                width: 1.0,
                radius: 0.0.into(),
                color: STATUS_BORDER,
            },
            shadow: Shadow::default(),
            snap: false,
        })
}

fn item<'a>(label: &'a str, color: Option<Color>) -> iced::widget::Container<'a, Message> {
    container(
        text(label)
            .size(14)
            .color(color.unwrap_or(STATUS_TEXT))
    )
    .height(Length::Fixed(30.0))
    .center_y(Length::Fill)
}