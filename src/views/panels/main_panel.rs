use iced::{
    alignment,
    widget::{button, container, row, text, Space},
    Background, Border, Color, Length, Shadow,
};

use crate::{
    app::{
        message::Message,
        state::{AppState, PanelTab},
    },
    views::panels::terminal,
};

const PANEL_BG: Color = Color::from_rgb8(12, 12, 16);
const PANEL_HEADER_BG: Color = Color::from_rgb8(16, 16, 20);
const PANEL_BORDER: Color = Color::from_rgb8(34, 34, 40);
const TEXT_MUTED: Color = Color::from_rgb8(140, 140, 150);
const TEXT_ACTIVE: Color = Color::from_rgb8(235, 235, 240);
const CLOSE_COLOR: Color = Color::from_rgb8(170, 170, 180);

pub fn view(app: &AppState) -> iced::widget::Container<'_, Message> {
    let tabs = row![
        tab("Problems", PanelTab::Problems, app.active_panel),
        tab("Output", PanelTab::Output, app.active_panel),
        tab("Debug Console", PanelTab::DebugConsole, app.active_panel),
        tab("Terminal", PanelTab::Terminal, app.active_panel),
        tab("Ports", PanelTab::Ports, app.active_panel),
        Space::new().width(Length::Fill),
        close_button(),
    ]
    .spacing(26)
    .align_y(alignment::Vertical::Center);

    let header = container(tabs)
        .width(Length::Fill)
        .height(Length::Fixed(40.0))
        .padding([0, 16])
        .style(|_theme| iced::widget::container::Style {
            background: Some(Background::Color(PANEL_HEADER_BG)),
            text_color: None,
            border: Border {
                width: 1.0,
                radius: 0.0.into(),
                color: PANEL_BORDER,
            },
            shadow: Shadow::default(),
            snap: false,
        });

    let body = match app.active_panel {
        PanelTab::Terminal => terminal::view(app).into(),
        PanelTab::Problems => placeholder("Problems"),
        PanelTab::Output => placeholder("Output"),
        PanelTab::DebugConsole => placeholder("Debug Console"),
        PanelTab::Ports => placeholder("Ports"),
    };

    container(iced::widget::column![header, body])
        .width(Length::Fill)
        .height(Length::Fixed(260.0))
        .style(|_theme| iced::widget::container::Style {
            background: Some(Background::Color(PANEL_BG)),
            text_color: None,
            border: Border {
                width: 1.0,
                radius: 0.0.into(),
                color: PANEL_BORDER,
            },
            shadow: Shadow::default(),
            snap: false,
        })
}

fn tab<'a>(
    label: &'a str,
    tab: PanelTab,
    active: PanelTab,
) -> iced::widget::Button<'a, Message> {
    button(
        container(
            text(label)
                .size(14)
                .color(if tab == active { TEXT_ACTIVE } else { TEXT_MUTED })
        )
        .height(Length::Fixed(28.0))
        .center_y(Length::Fill)
    )
    .padding(0)
    .on_press(Message::OpenPanel(tab))
    .style(move |_theme, _status| iced::widget::button::Style {
        background: None,
        text_color: if tab == active { TEXT_ACTIVE } else { TEXT_MUTED },
        border: Border {
            width: 0.0,
            radius: 0.0.into(),
            color: Color::TRANSPARENT,
        },
        shadow: Shadow::default(),
        snap: false,
    })
}

fn close_button<'a>() -> iced::widget::Button<'a, Message> {
    button(
        text("✕")
            .size(14)
            .color(CLOSE_COLOR)
    )
    .on_press(Message::ClosePanel)
    .style(|_theme, _status| iced::widget::button::Style {
        background: None,
        text_color: CLOSE_COLOR,
        border: Border {
            width: 0.0,
            radius: 0.0.into(),
            color: Color::TRANSPARENT,
        },
        shadow: Shadow::default(),
        snap: false,
    })
}

fn placeholder<'a>(label: &'a str) -> iced::Element<'a, Message> {
    container(
        text(label)
            .size(16)
            .color(TEXT_MUTED)
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .into()
}