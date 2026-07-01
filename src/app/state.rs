use crate::widgets::sidebar::SidebarItem;

pub struct App {
    pub selected_sidebar: SidebarItem,
}

impl Default for App {
    fn default() -> Self {
        Self {
            selected_sidebar: SidebarItem::Editor,
        }
    }
}