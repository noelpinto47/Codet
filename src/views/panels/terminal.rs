use iced::{
    widget::{container, text},
    Background, Border, Color, Length, Shadow,
};

use crate::{app::message::Message, app::state::AppState};

const PANEL_BG: Color = Color::from_rgb8(12, 12, 16);
const PANEL_BORDER: Color = Color::from_rgb8(34, 34, 40);
const TEXT_TERMINAL: Color = Color::from_rgb8(190, 220, 255);

pub fn view(_app: &AppState) -> iced::widget::Container<'_, Message> {
    container(
        text("○ ➜  Codet git:(main) ✗ ▯")
            .size(14)
            .color(TEXT_TERMINAL)
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .padding(20)
    .style(|_theme| iced::widget::container::Style {
        background: Some(Background::Color(PANEL_BG)),
        text_color: Some(TEXT_TERMINAL),
        border: Border {
            width: 1.0,
            radius: 0.0.into(),
            color: PANEL_BORDER,
        },
        shadow: Shadow::default(),
        snap: false,
    })
}