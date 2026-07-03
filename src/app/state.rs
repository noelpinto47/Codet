use crate::widgets::sidebar::SidebarItem;

pub struct App {
    pub selected_sidebar: SidebarItem,
    pub sidebar_open: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            selected_sidebar: SidebarItem::Files,
            sidebar_open: true,
        }
    }
}