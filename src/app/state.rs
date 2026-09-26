use std::path::PathBuf;

use iced::widget::text_editor;
use iced_highlighter::Settings as HighlightSettings;
use crate::models::file_tree::FileNode;
use crate::widgets::sidebar::SidebarItem;
use crate::highlight::default_settings;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PanelTab {
    Problems,
    Output,
    DebugConsole,
    Terminal,
    Ports,
}

pub struct AppState {
    pub editor: text_editor::Content,
    pub sidebar_open: bool,
    pub selected_sidebar: SidebarItem,
    pub show_settings: bool,
    pub show_panel: bool,
    pub active_panel: PanelTab,
    pub file_tree: Vec<FileNode>,
    pub show_file_menu: bool,
    // Syntax highlighting
    pub active_file_path: Option<PathBuf>,
    pub highlight_settings: HighlightSettings,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            editor: text_editor::Content::new(),
            sidebar_open: true,
            selected_sidebar: SidebarItem::Files,
            show_settings: false,
            show_panel: false,
            active_panel: PanelTab::Terminal,
            file_tree: Vec::new(),
            show_file_menu: false,
            active_file_path: None,
            highlight_settings: default_settings(),
        }
    }
}
