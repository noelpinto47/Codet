use crate::app::state::PanelTab;
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
                println!("\n--- Folder tree for : {}---", path.display());
                print_folder_tree(&path, "");
            } else {
                println!("folder selection cancelled.");
            }
            Task::none()
        }
        Message::ToggleFileMenu => {
            app.show_file_menu = !app.show_file_menu;
            Task::none()
        }
    }
}

fn print_folder_tree(path: &Path, prefix: &str) {
    if let Ok(entries) = fs::read_dir(path) {
        let mut entries: Vec<_> = entries.filter_map(Result::ok).collect();
        entries.sort_by_key(|e| e.file_name());

        let count = entries.len();
        for (index, entry) in entries.iter().enumerate() {
            let is_last = index == count - 1;
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();
            let path = entry.path();

            let marker = if is_last { "└── " } else { "├── " };
            println!("{}{}{}", prefix, marker, file_name_str);

            if path.is_dir() {
                let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
                print_folder_tree(&path, &new_prefix);
            }
        }
    }
}
