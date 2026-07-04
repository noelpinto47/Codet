use iced::widget::text_editor;

use crate::widgets::sidebar::SidebarItem;

pub struct AppState {
    pub editor: text_editor::Content,
    pub sidebar_open: bool,
    pub selected_sidebar: SidebarItem,
    pub show_settings: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            editor: text_editor::Content::new(),
            sidebar_open: true,
            selected_sidebar: SidebarItem::Files,
            show_settings: false,
        }
    }
}