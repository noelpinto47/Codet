use iced::{
    Background, Border, Color, Element, Length, Padding, Shadow, alignment,
    widget::{button, column, container, mouse_area, row, text, Space},
};

use crate::app::message::Message;

// ── Colors ────────────────────────────────────────────────────────────────────

const TITLEBAR_BG: Color = Color::from_rgb8(16, 16, 20);
const MENU_TEXT: Color = Color::from_rgb8(170, 170, 180);
const DROPDOWN_BG: Color = Color::from_rgb8(20, 20, 28);
const DROPDOWN_BORDER: Color = Color::from_rgb8(50, 50, 60);
const DROPDOWN_TEXT: Color = Color::from_rgb8(190, 190, 200);
const DROPDOWN_MUTED: Color = Color::from_rgb8(100, 100, 110);

const CLOSE_COLOR: Color = Color::from_rgb8(255, 95, 87);
const MINIMIZE_COLOR: Color = Color::from_rgb8(255, 189, 46);
const MAXIMIZE_COLOR: Color = Color::from_rgb8(40, 200, 64);

// ── Public view ───────────────────────────────────────────────────────────────

pub fn view(show_file_menu: bool) -> Element<'static, Message> {
    // macOS traffic lights
    let traffic_lights = row![
        traffic_light(CLOSE_COLOR,    Message::WindowClose),
        traffic_light(MINIMIZE_COLOR, Message::WindowMinimize),
        traffic_light(MAXIMIZE_COLOR, Message::WindowMaximize),
    ]
    .spacing(8)
    .align_y(alignment::Vertical::Center)
    .padding(Padding::from([0u16, 16]));

    // Menu items
    let menu_items = row![
        menu_button("File", show_file_menu),
        menu_label("Edit"),
        menu_label("View"),
        menu_label("Help"),
    ]
    .spacing(4)
    .align_y(alignment::Vertical::Center);

    // App name — right side
    let app_title = container(
        text("Codet")
            .size(13)
            .color(Color::from_rgb8(100, 100, 110)),
    )
    .padding(Padding::from([0u16, 16]));

    // Drag region fills the space between menu items and title
    let drag_region = mouse_area(
        Space::new().width(Length::Fill).height(Length::Fill),
    )
    .on_press(Message::WindowDrag);

    let bar = row![traffic_lights, menu_items, drag_region, app_title]
        .align_y(alignment::Vertical::Center);

    container(bar)
        .width(Length::Fill)
        .height(Length::Fixed(40.0))
        .style(|_theme| iced::widget::container::Style {
            background: Some(Background::Color(TITLEBAR_BG)),
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

// ── Traffic light ─────────────────────────────────────────────────────────────

fn traffic_light(color: Color, msg: Message) -> Element<'static, Message> {
    button(Space::new())
        .width(Length::Fixed(12.0))
        .height(Length::Fixed(12.0))
        .on_press(msg)
        .style(move |_theme, _status| iced::widget::button::Style {
            background: Some(Background::Color(color)),
            border: Border {
                radius: 6.0.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
            text_color: Color::TRANSPARENT,
            shadow: Shadow::default(),
            snap: false,
        })
        .into()
}

// ── Menu buttons ──────────────────────────────────────────────────────────────

fn menu_button(label: &str, active: bool) -> iced::widget::Button<'_, Message> {
    button(text(label).size(13).color(MENU_TEXT))
        .on_press(Message::ToggleFileMenu)
        .padding([4u16, 10])
        .style(move |_theme, _status| iced::widget::button::Style {
            background: if active {
                Some(Background::Color(Color::from_rgb8(30, 30, 38)))
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
        .padding([4u16, 10])
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

// ── Dropdown helpers ──────────────────────────────────────────────────────────

fn dropdown_button(label: &str, msg: Message) -> iced::widget::Button<'_, Message> {
    button(text(label).size(13).color(DROPDOWN_TEXT))
        .width(Length::Fill)
        .on_press(msg)
        .padding([6u16, 12])
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
        .padding([6u16, 12])
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
    container(Space::new())
        .width(Length::Fill)
        .height(Length::Fixed(1.0))
        .style(|_theme| iced::widget::container::Style {
            background: Some(Background::Color(DROPDOWN_BORDER)),
            text_color: None,
            border: Border::default(),
            shadow: Shadow::default(),
            snap: false,
        })
}
