use crate::app::state::PanelTab;
use crate::models::file_tree::FileNode;
use crate::models::open_tab::OpenTab;
use crate::{
    app::{message::Message, state::AppState},
    widgets::sidebar::{SidebarItem, SidebarMessage},
};
use iced::{window, Task};
use std::fs;
use std::path::Path;
use strip_ansi_escapes::strip_str;

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
                    rfd::AsyncFileDialog::new()
                        .set_title("Open Folder")
                        .pick_folder()
                        .await
                        .map(|p| p.path().to_path_buf())
                },
                Message::FolderOpened,
            )
        }

        Message::FolderOpened(path) => {
            if let Some(path) = path {
                app.file_tree = build_file_nodes(&path);
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
            match fs::read_to_string(&path) {
                Ok(contents) => {
                    if let Some(idx) = app.open_tabs.iter().position(|t| t.path == path) {
                        app.active_tab_index = idx;
                    } else {
                        app.open_tabs.push(OpenTab::new(path.clone()));
                        app.active_tab_index = app.open_tabs.len() - 1;
                    }
                    app.editor = iced::widget::text_editor::Content::with_text(&contents);
                    app.highlight_settings = crate::highlight::settings_for_path(&path);
                    app.active_file_path = Some(path);
                }
                Err(err) => eprintln!("Failed to open {}: {}", path.display(), err),
            }
            Task::none()
        }

        Message::SwitchTab(index) => {
            if let Some(tab) = app.open_tabs.get(index) {
                let path = tab.path.clone();
                if let Ok(contents) = fs::read_to_string(&path) {
                    app.active_tab_index = index;
                    app.editor = iced::widget::text_editor::Content::with_text(&contents);
                    app.highlight_settings = crate::highlight::settings_for_path(&path);
                    app.active_file_path = Some(path);
                }
            }
            Task::none()
        }

        Message::CloseTab(index) => {
            if index < app.open_tabs.len() {
                app.open_tabs.remove(index);
                if app.open_tabs.is_empty() {
                    app.editor = iced::widget::text_editor::Content::new();
                    app.active_file_path = None;
                    app.highlight_settings = crate::highlight::default_settings();
                    app.active_tab_index = 0;
                } else {
                    app.active_tab_index = app.active_tab_index.min(app.open_tabs.len() - 1);
                    let path = app.open_tabs[app.active_tab_index].path.clone();
                    if let Ok(contents) = fs::read_to_string(&path) {
                        app.editor = iced::widget::text_editor::Content::with_text(&contents);
                        app.highlight_settings = crate::highlight::settings_for_path(&path);
                        app.active_file_path = Some(path);
                    }
                }
            }
            Task::none()
        }

        // ── Window controls ───────────────────────────────────────────────────────────
        Message::WindowDrag => {
            window::latest().and_then(window::drag)
        }

        Message::WindowMinimize => {
            window::latest().and_then(|id| window::minimize(id, true))
        }

        Message::WindowMaximize => {
            window::latest().and_then(window::toggle_maximize)
        }

        Message::WindowClose => {
            window::latest().and_then(window::close)
        }

        Message::TerminalStarted(pty, writer) => {
            app.terminal_pty = Some(pty.0);       // ← .0 to unwrap PtyHandle
            app.terminal_writer = Some(writer.0); // ← .0 to unwrap WriterHandle
            Task::none()
        }

        Message::TerminalOutput(data) => {
            let clean = strip_str(&data);
            app.terminal_output.push_str(&clean);
            if app.terminal_output.len() > 50_000 {
                let trim_at = app.terminal_output.len() - 50_000;
                app.terminal_output = app.terminal_output[trim_at..].to_string();
            }
            Task::none()
        }

        Message::TerminalInputChanged(s) => {
            app.terminal_input = s;
            Task::none()
        }

        Message::TerminalInputSubmit => {
            if let Some(writer) = &app.terminal_writer {
                let mut w = writer.lock().unwrap();
                let _ = write!(w, "{}\n", app.terminal_input);
            }
            app.terminal_input.clear();
            Task::none()
        }
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn toggle_folder_at_path(nodes: &mut [FileNode], path: &[usize]) {
    if path.is_empty() { return; }
    if let Some(node) = nodes.get_mut(path[0]) {
        match node {
            FileNode::Folder { expanded, children, .. } => {
                if path.len() == 1 { *expanded = !*expanded; }
                else { toggle_folder_at_path(children, &path[1..]); }
            }
            FileNode::File { .. } => {}
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
                nodes.push(FileNode::Folder {
                    name,
                    expanded: false,
                    children: build_file_nodes(&p),
                });
            } else {
                nodes.push(FileNode::File { name, path: p });
            }
        }
    }
    nodes
}
