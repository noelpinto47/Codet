use crate::app::state::PanelTab;
use crate::models::file_tree::FileNode; 
use crate::{
    app::{message::Message, state::AppState},
    widgets::sidebar::{SidebarItem, SidebarMessage},
};
use iced::Task;
use std::fs;
use std::path::Path;

pub fn update(app: &mut AppState, message: Message) -> Task<Message> {
    match message {
        Message::EditorEdit(action) => {
            app.editor.perform(action);
            Task::none()
        }

        Message::Sidebar(sidebar_message) => match sidebar_message {
            SidebarMessage::Pressed(item) => {
                if app.selected_sidebar == item {
                    app.sidebar_open = !app.sidebar_open;
                } else {
                    app.selected_sidebar = item;
                    app.sidebar_open = true;
                }

                if item == SidebarItem::Settings {
                    app.show_settings = true;
                } else {
                    app.selected_sidebar = item;
                    app.show_settings = false;
                }

                Task::none()
            }
        },

        Message::CloseSettings => {
            app.show_settings = false;
            Task::none()
        }

        Message::ToggleTerminal => {
            app.show_panel = true;
            app.active_panel = PanelTab::Terminal;
            Task::none()
        }

        Message::OpenPanel(tab) => {
            app.show_panel = true;
            app.active_panel = tab;
            Task::none()
        }

        Message::ClosePanel => {
            app.show_panel = false;
            Task::none()
        }

        Message::OpenFolderClicked => {
            app.show_file_menu = false;
            Task::perform(
                async {
                    let path = rfd::AsyncFileDialog::new()
                        .set_title("Open Folder")
                        .pick_folder()
                        .await
                        .map(|p| p.path().to_path_buf());
                    path
                },
                |path| Message::FolderOpened(path),
            )
        }

        Message::FolderOpened(path) => {
            if let Some(path) = path {
                app.file_tree = build_file_nodes(&path);
            } else {
                println!("folder selection cancelled.");
            }
            Task::none()
        }

        Message::ToggleFileMenu => {
            app.show_file_menu = !app.show_file_menu;
            Task::none()
        }

        Message::ToggleFolder(path) => {
            toggle_folder_at_path(&mut app.file_tree, &path);
            Task::none()
        }

        Message::OpenFile(path) => {
            match std::fs::read_to_string(&path) {
                Ok(contents) => {
                    // Preferred: if Content implements From<String>
                    app.editor = iced::widget::text_editor::Content::with_text(&contents);
                    // If that constructor doesn't exist in your iced version,
                    // try whatever constructor your iced exposes (e.g. `new_with_value`),
                    // or perform editor actions to insert the text.
                }
                Err(err) => {
                    eprintln!("Failed to open {}: {}", path.display(), err);
                }
            }
            Task::none()
        }
    }
}

fn toggle_folder_at_path(nodes: &mut [crate::models::file_tree::FileNode], path: &[usize]) {
    if path.is_empty() {
        return;
    }

    let index = path[0];

    if let Some(node) = nodes.get_mut(index) {
        match node {
            crate::models::file_tree::FileNode::Folder {
                expanded,
                children,
                ..
            } => {
                if path.len() == 1 {
                    *expanded = !*expanded;
                } else {
                    toggle_folder_at_path(children, &path[1..]);
                }
            }
            crate::models::file_tree::FileNode::File { .. } => {}
        }
    }
}

fn build_file_nodes(path: &Path) -> Vec<FileNode> {
    let mut nodes = Vec::new();

    if let Ok(entries) = fs::read_dir(path) {
        let mut entries: Vec<_> = entries.filter_map(Result::ok).collect();
        entries.sort_by_key(|e| e.file_name());

        for entry in entries {
            let name = entry.file_name().to_string_lossy().into_owned();
            let p = entry.path();
            if p.is_dir() {
                let children = build_file_nodes(&p);
                nodes.push(FileNode::Folder {
                    name,
                    expanded: false,
                    children,
                });
            } else {
                nodes.push(FileNode::File { name, path: p });
            }
        }
    }

    nodes
}