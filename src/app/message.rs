use iced::widget::text_editor;
use crate::widgets::sidebar::SidebarMessage;
use crate::app::state::PanelTab;

#[derive(Debug, Clone)]
pub enum Message {
    Sidebar(SidebarMessage),
    EditorEdit(text_editor::Action),
    CloseSettings,
    ToggleTerminal,
    OpenPanel(PanelTab),
    ClosePanel,
}