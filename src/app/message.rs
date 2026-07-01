use crate::widgets::sidebar::SidebarMessage;

#[derive(Debug, Clone)]
pub enum Message {
    Sidebar(SidebarMessage),
}