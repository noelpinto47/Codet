use iced::widget::text_editor;

use crate::widgets::sidebar::SidebarItem;

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
        }
    }
}