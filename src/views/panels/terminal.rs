use iced::{
    widget::{column, container, row, scrollable, text, text_input},
    Background, Border, Color, Length, Shadow,
};
use crate::{app::message::Message, app::state::AppState};

const PANEL_BG: Color    = Color::from_rgb8(12, 12, 16);
const PANEL_BORDER: Color = Color::from_rgb8(34, 34, 40);
const TEXT_COLOR: Color   = Color::from_rgb8(190, 220, 255);
const PROMPT_COLOR: Color = Color::from_rgb8(100, 200, 140);

pub fn view(app: &AppState) -> iced::widget::Container<'_, Message> {
    let output = scrollable(
        text(&app.terminal_output)
            .size(13)
            .font(iced::Font::MONOSPACE)
            .color(TEXT_COLOR),
    )
    .height(Length::Fill)
    .width(Length::Fill);

    let prompt = row![
        text("❯ ").size(13).font(iced::Font::MONOSPACE).color(PROMPT_COLOR),
        text_input("", &app.terminal_input)
            .on_input(Message::TerminalInputChanged)
            .on_submit(Message::TerminalInputSubmit)
            .size(13)
            .font(iced::Font::MONOSPACE)
            .style(|_theme, _status| iced::widget::text_input::Style {
                background: Background::Color(Color::TRANSPARENT),
                border: Border::default(),
                icon: Color::TRANSPARENT,
                placeholder: Color::from_rgb8(80, 80, 90),
                value: TEXT_COLOR,
                selection: Color::from_rgb8(54, 116, 255),
            }),
    ]
    .align_y(iced::alignment::Vertical::Center);

    container(column![output, prompt].spacing(4))
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(12)
        .style(|_theme| iced::widget::container::Style {
            background: Some(Background::Color(PANEL_BG)),
            text_color: Some(TEXT_COLOR),
            border: Border { width: 1.0, radius: 0.0.into(), color: PANEL_BORDER },
            shadow: Shadow::default(),
            snap: false,
        })
}