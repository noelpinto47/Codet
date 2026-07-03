use iced::{
    widget::{container, row, text_editor},
    Background, Border, Color, Element, Length, Shadow,
};

use crate::{
    app::message::Message,
    app::state::AppState,
    widgets::{extended_sidebar, sidebar},
};

const CHROME_BG: Color = Color::from_rgb8(19, 19, 23);
const EDITOR_BG: Color = Color::from_rgb8(30, 30, 36);

pub fn view(app: &AppState) -> Element<'_, Message> {
    let content = if app.sidebar_open {
        row![
            sidebar::view(app.selected_sidebar).map(Message::Sidebar),
            extended_sidebar::view(app.selected_sidebar),
            main_content(app),
        ]
    } else {
        row![
            sidebar::view(app.selected_sidebar).map(Message::Sidebar),
            main_content(app),
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

fn main_content(app: &AppState) -> iced::widget::Container<'_, Message> {
    container(
        text_editor(&app.editor)
        .placeholder("Start typing...")
        .on_action(Message::EditorEdit)
        .height(Length::Fill)
        .style(|_theme, _status| iced::widget::text_editor::Style {
            background: Background::Color(EDITOR_BG),
            border: Border {
                width: 0.0,
                radius: 0.0.into(),
                color: Color::TRANSPARENT,
            },
            placeholder: Color::from_rgb8(100, 100, 100),
            value: Color::from_rgb8(255, 255, 255),
            selection: Color::from_rgb8(0, 120, 215),
        })
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .style(|_theme| iced::widget::container::Style {
        background: Some(Background::Color(EDITOR_BG)),
        text_color: None,
        border: Border::default(),
        shadow: Shadow::default(),
        snap: false,
    })
}