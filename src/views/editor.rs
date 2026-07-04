use iced::{
    widget::{column, container, row, text, text_editor},
    Background, Border, Color, Element, Length, Shadow,
};

use crate::{
    app::message::Message,
    app::state::AppState,
    views::settings,
    widgets::{extended_sidebar, sidebar},
};

const CHROME_BG: Color = Color::from_rgb8(19, 19, 23);
const EDITOR_BG: Color = Color::from_rgb8(30, 30, 36);

const TABBAR_BG: Color = Color::from_rgb8(20, 20, 26);
const TAB_ACTIVE_BG: Color = Color::from_rgb8(30, 30, 40);
const TAB_INACTIVE_BG: Color = Color::from_rgb8(24, 24, 30);
const TAB_BORDER_BLUE: Color = Color::from_rgb8(54, 116, 255);

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

    let base = container(content)
        .height(Length::Fill)
        .width(Length::Fill)
        .style(|_theme| iced::widget::container::Style {
            background: Some(Background::Color(CHROME_BG)),
            text_color: None,
            border: Border::default(),
            shadow: Shadow::default(),
            snap: false,
        });

    if app.show_settings {
        settings::modal(base)
    } else {
        base.into()
    }
}

fn main_content(app: &AppState) -> iced::widget::Container<'_, Message> {
    let tabs = container(
        row![
            tab("Main.js", true),
            tab(".gitignore", false),
            tab("builders.py", false),
        ]
        .spacing(6)
        .padding([6, 10])
    )
    .width(Length::Fill)
    .height(Length::Fixed(48.0))
    .style(|_theme| iced::widget::container::Style {
        background: Some(Background::Color(TABBAR_BG)),
        text_color: None,
        border: Border::default(),
        shadow: Shadow::default(),
        snap: false,
    });

    let editor = container(
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
    });

    container(column![tabs, editor])
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

fn tab<'a>(title: &'a str, active: bool) -> iced::widget::Container<'a, Message> {
    container(
        row![
            text(title).size(14).color(if active {
                Color::from_rgb8(140, 180, 255)
            } else {
                Color::from_rgb8(170, 170, 180)
            }),
            text("×").size(14).color(Color::from_rgb8(120, 120, 130)),
        ]
        .spacing(10)
    )
    .padding([10, 14])
    .style(move |_theme| iced::widget::container::Style {
        background: Some(Background::Color(if active {
            TAB_ACTIVE_BG
        } else {
            TAB_INACTIVE_BG
        })),
        text_color: None,
        border: Border {
            width: if active { 1.0 } else { 0.0 },
            radius: 10.0.into(),
            color: if active {
                TAB_BORDER_BLUE
            } else {
                Color::TRANSPARENT
            },
        },
        shadow: Shadow::default(),
        snap: false,
    })
}