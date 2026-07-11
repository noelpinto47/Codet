use iced::{
    alignment,
    widget::{button, column, container, row, scrollable, text},
    Background, Border, Color, Element, Length, Shadow,
};

use crate::{
    app::message::Message,
    app::state::AppState,
    models::file_tree::FileNode,
};

const EXPLORER_BG: Color = Color::from_rgb8(14, 14, 20);
const TEXT_PRIMARY: Color = Color::from_rgb8(220, 220, 225);
const TEXT_MUTED: Color = Color::from_rgb8(160, 160, 170);
const FOLDER_COLOR: Color = Color::from_rgb8(240, 196, 92);
const FILE_COLOR: Color = Color::from_rgb8(205, 205, 215);

pub fn view(app: &AppState) -> Element<'_, Message> {
    let mut content = column![
        text("Files")
            .size(24)
            .color(TEXT_PRIMARY)
    ]
    .spacing(10)
    .padding([16, 12]);

    let tree = render_nodes(&app.file_tree, 0, vec![]);

    content = content.push(scrollable(tree).height(Length::Fill));

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_theme| iced::widget::container::Style {
            background: Some(Background::Color(EXPLORER_BG)),
            text_color: Some(TEXT_PRIMARY),
            border: Border::default(),
            shadow: Shadow::default(),
            snap: false,
        })
        .into()
}

fn render_nodes<'a>(
    nodes: &'a [FileNode],
    depth: usize,
    path: Vec<usize>,
) -> Element<'a, Message> {
    let mut col = column![] as iced::widget::Column<'_, Message>;
    for (i, node) in nodes.iter().enumerate() {
        let mut p = path.clone();
        p.push(i);
        col = col.push(render_node(node, depth, p.clone()));
        if let FileNode::Folder { expanded, children, .. } = node {
            if *expanded {
                col = col.push(render_nodes(children, depth + 1, p));
            }
        }
    }
    col.into()
}

fn render_node<'a>(
    node: &'a FileNode,
    depth: usize,
    path: Vec<usize>,
) -> Element<'a, Message> {
    let left_pad = (depth as u16) * 16;

    match node {
        FileNode::Folder { name, expanded, .. } => {
            container(
                button(
                    row![
                        text(if *expanded { "⌄" } else { "›" })
                            .size(16)
                            .color(TEXT_MUTED),
                        text("📁")
                            .size(16)
                            .color(FOLDER_COLOR),
                        text(name)
                            .size(16)
                            .color(FOLDER_COLOR),
                    ]
                    .spacing(8)
                    .align_y(alignment::Vertical::Center)
                )
                .padding(0)
                .width(Length::Fill)
                .on_press(Message::ToggleFolder(path))
                .style(|_theme, _status| iced::widget::button::Style {
                    background: None,
                    text_color: TEXT_PRIMARY,
                    border: Border::default(),
                    shadow: Shadow::default(),
                    snap: false,
                })
            )
            .padding([4, left_pad])
            .width(Length::Fill)
            .into()
        }

        FileNode::File { name, path } => {
            container(
                button(
                    row![
                        text("🦀").size(15).color(FILE_COLOR),
                        text(name).size(15).color(TEXT_MUTED),
                    ]
                    .spacing(8)
                    .align_y(alignment::Vertical::Center)
                )
                .padding(0)
                .width(Length::Fill)
                .on_press(Message::OpenFile(path.clone()))
                .style(|_theme, _status| iced::widget::button::Style {
                    background: None,
                    text_color: TEXT_MUTED,
                    border: Border::default(),
                    shadow: Shadow::default(),
                    snap: false,
                })
            )
            .padding([4, left_pad + 24])
            .width(Length::Fill)
            .into()
        }
    }
}