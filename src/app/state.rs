use crate::widgets::sidebar::SidebarItem;
use iced::widget::text_editor;

pub struct AppState {
    pub selected_sidebar: SidebarItem,
    pub sidebar_open: bool,
    pub editor: text_editor::Content,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            selected_sidebar: SidebarItem::Files,
            sidebar_open: true,
            editor: text_editor::Content::new(),
        }
    }
}