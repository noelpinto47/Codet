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
        text("Files").size(24).color(TEXT_PRIMARY)
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

fn render_nodes<'a>(nodes: &'a [FileNode], depth: usize, path: Vec<usize>) -> Element<'a, Message> {
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

fn render_node<'a>(node: &'a FileNode, depth: usize, path: Vec<usize>) -> Element<'a, Message> {
    let left_pad = (depth as u16) * 16;

    match node {
        FileNode::Folder { name, expanded, .. } => {
            let folder_icon = if *expanded { "📂" } else { "📁" };
            let chevron    = if *expanded { "v" } else { ">" };

            container(
                button(
                    row![
                        text(chevron).size(12).color(TEXT_MUTED),
                        text(folder_icon).size(16).color(FOLDER_COLOR),
                        text(name).size(15).color(FOLDER_COLOR),
                    ]
                    .spacing(6)
                    .align_y(alignment::Vertical::Center),
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
                }),
            )
            .padding([4, left_pad])
            .width(Length::Fill)
            .into()
        }

        FileNode::File { name, path } => {
            let icon = file_icon(name);
            let (icon_color, label_color) = file_colors(name);

            container(
                button(
                    row![
                        text(icon).size(15).color(icon_color),
                        text(name).size(15).color(label_color),
                    ]
                    .spacing(8)
                    .align_y(alignment::Vertical::Center),
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
                }),
            )
            .padding([4, left_pad + 24])
            .width(Length::Fill)
            .into()
        }
    }
}

fn file_icon(name: &str) -> &'static str {
    // Full filename matches first
    match name {
        ".gitignore" | ".gitattributes" | ".gitmodules" => return "🔀",
        ".env" | ".env.local" | ".env.production"       => return "🔑",
        "Makefile" | "makefile" | "GNUmakefile"         => return "🔧",
        "Dockerfile"                                     => return "🐳",
        "LICENSE" | "LICENSE.md" | "LICENSE.txt"        => return "⚖",
        _ => {}
    }

    let ext = name.rsplit('.').next().unwrap_or("").to_lowercase();
    match ext.as_str() {
        "rs"                              => "🦀",
        "html" | "htm"                    => "🌐",
        "css" | "scss" | "sass" | "less"  => "🎨",
        "js" | "mjs" | "cjs"              => "📜",
        "ts"                              => "📘",
        "tsx" | "jsx"                     => "⚛",
        "json" | "jsonc"                  => "{ }",
        "toml"                            => "⚙",
        "yaml" | "yml"                    => "⚙",
        "xml"                             => "📋",
        "csv"                             => "📊",
        "sql"                             => "🗄",
        "md" | "mdx"                      => "📝",
        "txt"                             => "📄",
        "pdf"                             => "📑",
        "sh" | "bash" | "zsh" | "fish"    => "🐚",
        "py"                              => "🐍",
        "hs" | "lhs"                      => "λ",
        "go"                              => "🐹",
        "java"                            => "☕",
        "kt" | "kts"                      => "K",
        "swift"                           => "🍎",
        "c" | "h"                         => "C",
        "cpp" | "cc" | "cxx" | "hpp"      => "C+",
        "cs"                              => "C#",
        "rb"                              => "💎",
        "php"                             => "🐘",
        "lua"                             => "🌙",
        "dart"                            => "🎯",
        "lock"                            => "🔒",
        "cabal"                           => "🦀",
        "png" | "jpg" | "jpeg"
        | "gif" | "webp" | "bmp"          => "🖼",
        "svg"                             => "✏",
        "ico" | "icns"                    => "🖼",
        "zip" | "tar" | "gz"
        | "bz2" | "xz" | "7z" | "rar"    => "📦",
        _                                 => "📄",
    }
}

fn file_colors(name: &str) -> (Color, Color) {
    let ext = name.rsplit('.').next().unwrap_or("").to_lowercase();
    match ext.as_str() {
        "rs"                           => (Color::from_rgb8(255, 120,  80), Color::from_rgb8(255, 160, 120)),
        "js" | "mjs" | "cjs"           => (Color::from_rgb8(240, 210,  80), Color::from_rgb8(210, 200, 140)),
        "ts"                           => (Color::from_rgb8( 80, 140, 220), Color::from_rgb8(140, 180, 220)),
        "tsx" | "jsx"                  => (Color::from_rgb8( 80, 200, 210), Color::from_rgb8(130, 200, 210)),
        "py"                           => (Color::from_rgb8( 80, 160, 220), Color::from_rgb8(140, 190, 220)),
        "hs" | "lhs"                   => (Color::from_rgb8(160, 100, 220), Color::from_rgb8(180, 140, 220)),
        "go"                           => (Color::from_rgb8(100, 200, 220), Color::from_rgb8(150, 210, 220)),
        "md" | "mdx"                   => (Color::from_rgb8(100, 180, 120), Color::from_rgb8(140, 200, 150)),
        "json"|"toml"|"yaml"|"yml"     => (Color::from_rgb8(180, 180, 100), Color::from_rgb8(190, 190, 140)),
        "html" | "htm"                 => (Color::from_rgb8(220, 100,  80), Color::from_rgb8(210, 150, 130)),
        "css" | "scss" | "sass"        => (Color::from_rgb8( 80, 140, 220), Color::from_rgb8(130, 170, 220)),
        "sh" | "bash" | "zsh"          => (Color::from_rgb8(100, 200, 140), Color::from_rgb8(140, 200, 160)),
        "lock"                         => (Color::from_rgb8(140, 140, 140), Color::from_rgb8(130, 130, 130)),
        "pdf"                          => (Color::from_rgb8(220,  80,  80), Color::from_rgb8(200, 130, 130)),
        "png"|"jpg"|"jpeg"|"gif"|"svg" => (Color::from_rgb8(180, 120, 220), Color::from_rgb8(190, 160, 220)),
        _                              => (FILE_COLOR, TEXT_MUTED),
    }
}