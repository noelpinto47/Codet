use iced::widget::text_editor;
use crate::widgets::sidebar::SidebarMessage;

#[derive(Debug, Clone)]
pub enum Message {
    Sidebar(SidebarMessage),
    EditorEdit(text_editor::Action),
    CloseSettings,
}