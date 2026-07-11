use iced::{
    Background, Border, Color, Element, Length, Shadow, alignment,
    widget::{button, column, container, row, text},
};

use crate::app::message::Message;

const MENUBAR_BG: Color = Color::from_rgb8(16, 16, 20);
const MENU_TEXT: Color = Color::from_rgb8(170, 170, 180);
const DROPDOWN_BG: Color = Color::from_rgb8(20, 20, 28);
const DROPDOWN_BORDER: Color = Color::from_rgb8(50, 50, 60);
const DROPDOWN_TEXT: Color = Color::from_rgb8(190, 190, 200);
const DROPDOWN_MUTED: Color = Color::from_rgb8(100, 100, 110);

pub fn view(show_file_menu: bool) -> Element<'static, Message> {
    let items = row![
        menu_button("File", show_file_menu),
        menu_label("Edit"),
        menu_label("View"),
        menu_label("Help"),
    ]
    .spacing(4)
    .align_y(alignment::Vertical::Center)
    .padding([0, 10]);

    container(items)
        .width(Length::Fill)
        .height(Length::Fixed(32.0))
        .style(|_theme| iced::widget::container::Style {
            background: Some(Background::Color(MENUBAR_BG)),
            text_color: Some(MENU_TEXT),
            border: Border::default(),
            shadow: Shadow::default(),
            snap: false,
        })
        .into()
}

pub fn file_dropdown() -> Element<'static, Message> {
    let items = column![
        dropdown_item("New File", false),
        dropdown_item("Open File", false),
        dropdown_button("Open Folder", Message::OpenFolderClicked),
        separator(),
        dropdown_item("Save", false),
        dropdown_item("Save As", false),
    ]
    .spacing(2)
    .padding(4);
    container(items)
        .width(Length::Fixed(200.0))
        .style(|_theme| iced::widget::container::Style {
            background: Some(Background::Color(DROPDOWN_BG)),
            text_color: Some(DROPDOWN_TEXT),
            border: Border {
                width: 1.0,
                radius: 8.0.into(),
                color: DROPDOWN_BORDER,
            },
            shadow: Shadow::default(),
            snap: false,
        })
        .into()
}

fn menu_button(label: &str, active: bool) -> iced::widget::Button<'_, Message> {
    button(text(label).size(13).color(MENU_TEXT))
        .on_press(Message::ToggleFileMenu)
        .padding([4, 10])
        .style(move |_theme, _status| iced::widget::button::Style {
            background: if active {
                Some(Background::Color(Color::from_rgb8(25, 25, 25)))
            } else {
                None
            },
            text_color: MENU_TEXT,
            border: Border {
                width: 0.0,
                radius: 6.0.into(),
                color: Color::TRANSPARENT,
            },
            shadow: Shadow::default(),
            snap: false,
        })
}

fn menu_label(label: &str) -> iced::widget::Button<'_, Message> {
    button(text(label).size(13).color(MENU_TEXT))
        .padding([4, 10])
        .style(|_theme, _status| iced::widget::button::Style {
            background: None,
            text_color: MENU_TEXT,
            border: Border {
                width: 0.0,
                radius: 6.0.into(),
                color: Color::TRANSPARENT,
            },
            shadow: Shadow::default(),
            snap: false,
        })
}

fn dropdown_button(label: &str, msg: Message) -> iced::widget::Button<'_, Message> {
    button(text(label).size(13).color(DROPDOWN_TEXT))
        .width(Length::Fill)
        .on_press(msg)
        .padding([6, 12])
        .style(|_theme, _status| iced::widget::button::Style {
            background: None,
            text_color: DROPDOWN_TEXT,
            border: Border {
                width: 0.0,
                radius: 4.0.into(),
                color: Color::TRANSPARENT,
            },
            shadow: Shadow::default(),
            snap: false,
        })
}

fn dropdown_item(label: &str, _enabled: bool) -> iced::widget::Button<'_, Message> {
    button(text(label).size(13).color(DROPDOWN_MUTED))
        .width(Length::Fill)
        .padding([6, 12])
        .style(|_theme, _status| iced::widget::button::Style {
            background: None,
            text_color: DROPDOWN_MUTED,
            border: Border {
                width: 0.0,
                radius: 4.0.into(),
                color: Color::TRANSPARENT,
            },
            shadow: Shadow::default(),
            snap: false,
        })
}

fn separator() -> iced::widget::Container<'static, Message> {
    container("")
        .width(Length::Fill)
        .height(1)
        .style(|_theme| iced::widget::container::Style {
            background: Some(Background::Color(DROPDOWN_BORDER)),
            text_color: None,
            border: Border::default(),
            shadow: Shadow::default(),
            snap: false,
        })
}
