use iced::{
    alignment,
    widget::{button, column, container, svg, Space},
    Element, Length,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SidebarItem {
    Files,
    Git,
    Search,
    Debugger,
    Packages,
    Settings,
}

#[derive(Debug, Clone)]
pub enum SidebarMessage {
    Pressed(SidebarItem),
}

pub fn view(selected: SidebarItem) -> Element<'static, SidebarMessage> {
    let top_icons = column![
        icon_button(
            include_bytes!("../../assets/icons/sidebar/files.svg"),
            SidebarItem::Files,
            selected,
        ),
        icon_button(
            include_bytes!("../../assets/icons/sidebar/git.svg"),
            SidebarItem::Git,
            selected,
        ),
        icon_button(
            include_bytes!("../../assets/icons/sidebar/search.svg"),
            SidebarItem::Search,
            selected,
        ),
        icon_button(
            include_bytes!("../../assets/icons/sidebar/debugger.svg"),
            SidebarItem::Debugger,
            selected,
        ),
        icon_button(
            include_bytes!("../../assets/icons/sidebar/packages.svg"),
            SidebarItem::Packages,
            selected,
        ),
    ]
    .spacing(10.0)
    .align_x(alignment::Horizontal::Center);

    let bottom_icon = icon_button(
        include_bytes!("../../assets/icons/sidebar/settings.svg"),
        SidebarItem::Settings,
        selected,
    );

    container(
        column![
            Space::new().height(Length::Fixed(18.0)),
            top_icons,
            Space::new().height(Length::Fill),
            bottom_icon,
            Space::new().height(Length::Fixed(18.0)),
        ]
        .width(Length::Fill)
        .align_x(alignment::Horizontal::Center),
    )
    .width(Length::Fixed(74.0))
    .height(Length::Fill)
    .style(sidebar_style)
    .into()
}

fn icon_button(
    icon_bytes: &'static [u8],
    item: SidebarItem,
    selected: SidebarItem,
) -> Element<'static, SidebarMessage> {
    let is_selected = item == selected;

    let icon = svg(svg::Handle::from_memory(icon_bytes))
        .width(Length::Fixed(22.0))
        .height(Length::Fixed(22.0));

    let button = button(icon)
        .width(Length::Fixed(44.0))
        .height(Length::Fixed(44.0))
        .padding(12.0)
        .style(move |_theme, status| {
            use iced::widget::button;
            use iced::{Background, Border, Color, Shadow};

            let bg = match status {
                button::Status::Hovered => {
                    Some(Background::Color(Color::from_rgba8(255, 255, 255, 0.05)))
                }
                button::Status::Pressed => {
                    Some(Background::Color(Color::from_rgba8(255, 255, 255, 0.08)))
                }
                _ if is_selected => {
                    Some(Background::Color(Color::from_rgba8(255, 255, 255, 0.06)))
                }
                _ => None,
            };

            button::Style {
                background: bg,
                text_color: if is_selected {
                    Color::from_rgb8(230, 230, 240)
                } else {
                    Color::from_rgb8(122, 122, 138)
                },
                border: Border {
                    radius: 12.0.into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                shadow: Shadow::default(),
                snap: false,
            }
        })
        .on_press(SidebarMessage::Pressed(item));

    container(button).into()
}

fn sidebar_style(_theme: &iced::Theme) -> iced::widget::container::Style {
    use iced::{Background, Border, Color, Shadow};

    iced::widget::container::Style {
        background: Some(Background::Color(Color::from_rgb8(10, 10, 20))),
        text_color: None,
        border: Border {
            radius: 0.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: Shadow::default(),
        snap: false,
    }
}