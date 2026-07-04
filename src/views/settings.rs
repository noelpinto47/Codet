use iced::{
    alignment,
    widget::{button, column, container, mouse_area, opaque, row, scrollable, stack, text, Space},
    Background, Border, Color, Element, Length, Shadow,
};

use crate::app::message::Message;

const SCRIM_BG: Color = Color::from_rgba8(0, 0, 0, 0.55);
const MODAL_BG: Color = Color::from_rgb8(10, 10, 20);
const BORDER: Color = Color::from_rgb8(45, 45, 54);
const MUTED: Color = Color::from_rgb8(150, 150, 160);
const INPUT_BG: Color = Color::from_rgb8(28, 28, 34);

pub fn modal<'a>(base: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    stack![
        base.into(),
        mouse_area(
            container(Space::new())
                .width(Length::Fill)
                .height(Length::Fill)
                .style(|_theme| iced::widget::container::Style {
                    background: Some(Background::Color(SCRIM_BG)),
                    text_color: None,
                    border: Border::default(),
                    shadow: Shadow::default(),
                    snap: false,
                })
        )
        .on_press(Message::CloseSettings),
        container(opaque(panel()))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
    ]
    .into()
}

fn panel<'a>() -> Element<'a, Message> {
    let header = row![
        text("Settings").size(24),
        container(
            button(text("✕").size(16))
                .on_press(Message::CloseSettings)
                .style(|_theme, _status| iced::widget::button::Style {
                    background: None,
                    text_color: Color::from_rgb8(190, 190, 200),
                    border: Border {
                        width: 0.0,
                        radius: 8.0.into(),
                        color: Color::TRANSPARENT,
                    },
                    shadow: Shadow::default(),
                    snap: false,
                })
        )
        .width(Length::Fill)
        .align_x(alignment::Horizontal::Right)
    ]
    .align_y(alignment::Vertical::Center);

    let search = container(text("Search settings").size(14).color(MUTED))
        .padding([10, 12])
        .width(Length::Fill)
        .style(|_theme| iced::widget::container::Style {
            background: Some(Background::Color(INPUT_BG)),
            text_color: None,
            border: Border {
                width: 1.0,
                radius: 8.0.into(),
                color: BORDER,
            },
            shadow: Shadow::default(),
            snap: false,
        });

    let left_nav = container(
        column![
            text("Text Editor").size(14).color(MUTED),
            text("Packages").size(14).color(MUTED),
        ]
        .spacing(14)
    )
    .width(Length::Fixed(220.0));

    let right_content = scrollable(
        column![
            setting_item("Editor: Font Size", "14"),
            setting_item("Editor: Default Formatter", "None"),
            setting_item("Editor: Font Family", "'Readex Pro', 'Courier New'"),
            setting_item("Editor: Word Wrap", "on"),
        ]
        .spacing(24)
    );

    container(
        column![
            header,
            search,
            row![
                left_nav,
                container(right_content)
                    .width(Length::Fill)
                    .padding(20),
            ]
            .spacing(24)
            .height(Length::Fill),
        ]
        .spacing(20)
    )
    .width(Length::Fixed(1100.0))
    .height(Length::Fixed(720.0))
    .padding(24)
    .style(|_theme| iced::widget::container::Style {
        background: Some(Background::Color(MODAL_BG)),
        text_color: Some(Color::WHITE),
        border: Border {
            width: 1.0,
            radius: 14.0.into(),
            color: BORDER,
        },
        shadow: Shadow::default(),
        snap: false,
    })
    .into()
}

fn setting_item<'a>(label: &'a str, value: &'a str) -> iced::widget::Container<'a, Message> {
    container(
        column![
            text(label).size(18),
            container(text(value).size(14))
                .padding([8, 10])
                .width(Length::Fixed(240.0))
                .style(|_theme| iced::widget::container::Style {
                    background: Some(Background::Color(INPUT_BG)),
                    text_color: Some(Color::WHITE),
                    border: Border {
                        width: 1.0,
                        radius: 6.0.into(),
                        color: BORDER,
                    },
                    shadow: Shadow::default(),
                    snap: false,
                }),
        ]
        .spacing(8)
    )
}