use crate::widgets::sidebar::SidebarMessage;
use iced::widget::text_editor;

#[derive(Debug, Clone)]
pub enum Message {
    Sidebar(SidebarMessage),
    EditorEdit(text_editor::Action)
}